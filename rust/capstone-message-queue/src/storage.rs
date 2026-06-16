//! Append-only log storage engine.
//!
//! ## On-disk layout
//!
//! **`<dir>/data.log`** — sequence of fixed-header records:
//! ```text
//! [offset: u64 BE][length: u32 BE][payload: length bytes]
//! ```
//!
//! **`<dir>/data.idx`** — sparse index, one entry every INDEX_INTERVAL records:
//! ```text
//! [offset: u64 BE][file_position: u64 BE]   (16 bytes each)
//! ```
//!
//! To read offset O:
//!   1. Binary-search the index for the largest indexed offset ≤ O.
//!   2. Seek to its file position.
//!   3. Scan forward record by record until the offset header matches O.

use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

use crate::error::{Error, Result};

/// How many records between sparse index entries.
const INDEX_INTERVAL: u64 = 64;

/// Record header size: 8 (offset) + 4 (length) = 12 bytes.
const HEADER_LEN: u64 = 12;

// ── Index ────────────────────────────────────────────────────────────────────

/// Sparse in-memory index backed by a file on disk.
///
/// Each entry maps a logical offset to the byte position of that record's
/// header in the `.log` file.
struct Index {
    path: PathBuf,
    /// Sorted by offset.
    entries: Vec<(u64, u64)>,
}

impl Index {
    fn open(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref().to_owned();
        let mut entries = Vec::new();

        if path.exists() {
            let mut file = File::open(&path)?;
            let mut buf = [0u8; 16];
            loop {
                match file.read_exact(&mut buf) {
                    Ok(()) => {
                        let offset = u64::from_be_bytes(buf[0..8].try_into().unwrap());
                        let pos = u64::from_be_bytes(buf[8..16].try_into().unwrap());
                        entries.push((offset, pos));
                    }
                    Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => break,
                    Err(e) => return Err(e.into()),
                }
            }
        }

        Ok(Index { path, entries })
    }

    /// Returns the file position of the largest indexed offset ≤ `offset`,
    /// or `(0, 0)` (start of file) if no such entry exists.
    fn lookup(&self, offset: u64) -> (u64, u64) {
        match self.entries.partition_point(|&(o, _)| o <= offset) {
            0 => (0, 0),
            i => self.entries[i - 1],
        }
    }

    fn append(&mut self, offset: u64, position: u64) -> Result<()> {
        self.entries.push((offset, position));
        let mut file = OpenOptions::new().create(true).append(true).open(&self.path)?;
        let mut buf = [0u8; 16];
        buf[0..8].copy_from_slice(&offset.to_be_bytes());
        buf[8..16].copy_from_slice(&position.to_be_bytes());
        file.write_all(&buf)?;
        Ok(())
    }
}

// ── Log ──────────────────────────────────────────────────────────────────────

/// Append-only log with offset-based random reads.
///
/// `append` returns the offset assigned to the new record. Offsets start at 0
/// and increment by 1 per record (they are sequence numbers, not byte
/// positions).
pub struct Log {
    writer: BufWriter<File>,
    log_path: PathBuf,
    index: Index,
    next_offset: u64,
    write_pos: u64,
    records_since_index: u64,
}

impl Log {
    /// Opens (or creates) a log in `dir`.
    ///
    /// On re-open, scans the existing `.log` file to recover `next_offset`
    /// and `write_pos`.
    pub fn open(dir: impl AsRef<Path>) -> Result<Self> {
        let dir = dir.as_ref();
        std::fs::create_dir_all(dir)?;

        let log_path = dir.join("data.log");
        let idx_path = dir.join("data.idx");

        let index = Index::open(&idx_path)?;

        // Recover state by scanning from the last indexed position.
        let (next_offset, write_pos, records_since_index) =
            Self::recover(&log_path, &index)?;

        // If the file doesn't exist yet, next_offset=0, write_pos=0.
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_path)?;

        // Truncate any partial trailing write from a previous unclean shutdown.
        let actual_end = file.metadata()?.len();
        if actual_end != write_pos {
            file.set_len(write_pos)?;
        }

        Ok(Log {
            writer: BufWriter::new(file),
            log_path,
            index,
            next_offset,
            write_pos,
            records_since_index,
        })
    }

    /// Scans from the last indexed position to recover (next_offset, write_pos,
    /// records_since_index).
    fn recover(log_path: &Path, index: &Index) -> Result<(u64, u64, u64)> {
        if !log_path.exists() {
            return Ok((0, 0, 0));
        }

        let file_len = log_path.metadata()?.len();
        if file_len == 0 {
            return Ok((0, 0, 0));
        }

        // Find the last indexed position to start scanning from.
        let (start_offset, start_pos) = if index.entries.is_empty() {
            (0, 0)
        } else {
            *index.entries.last().unwrap()
        };

        let mut file = File::open(log_path)?;
        file.seek(SeekFrom::Start(start_pos))?;
        let mut reader = BufReader::new(file);

        let mut offset = start_offset;
        let mut pos = start_pos;
        let mut records_since_index = 0u64;
        let mut found_any = false;

        loop {
            let mut header = [0u8; 12];
            match reader.read_exact(&mut header) {
                Ok(()) => {}
                Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => break,
                Err(e) => return Err(e.into()),
            }
            let rec_offset = u64::from_be_bytes(header[0..8].try_into().unwrap());
            let length = u32::from_be_bytes(header[8..12].try_into().unwrap()) as u64;

            // Validate: offset should be monotonically increasing.
            if found_any && rec_offset != offset + 1 {
                return Err(Error::CorruptRecord(format!(
                    "expected offset {} got {}",
                    offset + 1,
                    rec_offset
                )));
            }

            // Skip the payload.
            reader
                .seek_relative(length as i64)
                .or_else(|_| reader.seek(SeekFrom::Current(length as i64)).map(|_| ()))?;

            pos += HEADER_LEN + length;
            offset = rec_offset;
            found_any = true;
            records_since_index = (records_since_index + 1) % INDEX_INTERVAL;
        }

        let next_offset = if found_any { offset + 1 } else { start_offset };
        Ok((next_offset, pos, records_since_index))
    }

    /// Appends `payload` to the log, returning its assigned offset.
    pub fn append(&mut self, payload: &[u8]) -> Result<u64> {
        let offset = self.next_offset;
        let length = payload.len() as u32;

        // Write: [offset u64 BE][length u32 BE][payload]
        self.writer.write_all(&offset.to_be_bytes())?;
        self.writer.write_all(&length.to_be_bytes())?;
        self.writer.write_all(payload)?;

        // Sparse index: record every INDEX_INTERVAL-th entry.
        if self.records_since_index == 0 {
            self.index.append(offset, self.write_pos)?;
        }
        self.records_since_index = (self.records_since_index + 1) % INDEX_INTERVAL;

        self.write_pos += HEADER_LEN + payload.len() as u64;
        self.next_offset += 1;
        Ok(offset)
    }

    /// Flushes buffered writes to the OS.
    pub fn flush(&mut self) -> Result<()> {
        self.writer.flush()?;
        Ok(())
    }

    /// Reads the payload stored at `offset`.
    pub fn read(&self, offset: u64) -> Result<Vec<u8>> {
        if offset >= self.next_offset {
            return Err(Error::OffsetOutOfRange(offset));
        }

        // Find the closest indexed position ≤ offset.
        let (_, start_pos) = self.index.lookup(offset);

        let mut file = File::open(&self.log_path)?;
        file.seek(SeekFrom::Start(start_pos))?;
        let mut reader = BufReader::new(file);

        loop {
            let mut header = [0u8; 12];
            reader.read_exact(&mut header).map_err(|_| {
                Error::CorruptRecord(format!("truncated header at offset {offset}"))
            })?;
            let rec_offset = u64::from_be_bytes(header[0..8].try_into().unwrap());
            let length = u32::from_be_bytes(header[8..12].try_into().unwrap()) as usize;

            if rec_offset == offset {
                let mut payload = vec![0u8; length];
                reader.read_exact(&mut payload)?;
                return Ok(payload);
            }

            // Skip this record's payload.
            reader.seek_relative(length as i64).or_else(|_| {
                reader
                    .seek(SeekFrom::Current(length as i64))
                    .map(|_| ())
            })?;
        }
    }

    /// The offset that will be assigned to the next appended record.
    pub fn next_offset(&self) -> u64 {
        self.next_offset
    }

    /// Returns an iterator over `(offset, payload)` pairs starting from
    /// `start_offset`.
    pub fn scan(&self, start_offset: u64) -> impl Iterator<Item = Result<(u64, Vec<u8>)>> + '_ {
        LogScanner::new(self, start_offset)
    }
}

// ── LogScanner ────────────────────────────────────────────────────────────────

struct LogScanner<'a> {
    log: &'a Log,
    reader: Option<BufReader<File>>,
    current_offset: u64,
    done: bool,
}

impl<'a> LogScanner<'a> {
    fn new(log: &'a Log, start_offset: u64) -> Self {
        let reader = (|| -> Result<BufReader<File>> {
            let (_, start_pos) = log.index.lookup(start_offset);
            let mut file = File::open(&log.log_path)?;
            file.seek(SeekFrom::Start(start_pos))?;
            Ok(BufReader::new(file))
        })();

        let (reader, done) = match reader {
            Ok(r) => (Some(r), false),
            Err(_) => (None, true),
        };

        LogScanner {
            log,
            reader,
            current_offset: start_offset,
            done,
        }
    }
}

impl<'a> Iterator for LogScanner<'a> {
    type Item = Result<(u64, Vec<u8>)>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        let reader = self.reader.as_mut()?;

        loop {
            let mut header = [0u8; 12];
            match reader.read_exact(&mut header) {
                Ok(()) => {}
                Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => {
                    self.done = true;
                    return None;
                }
                Err(e) => {
                    self.done = true;
                    return Some(Err(e.into()));
                }
            }

            let rec_offset = u64::from_be_bytes(header[0..8].try_into().unwrap());
            let length = u32::from_be_bytes(header[8..12].try_into().unwrap()) as usize;

            if rec_offset < self.current_offset {
                // Skip records before start_offset (scanner started at an index entry
                // that precedes start_offset).
                if let Err(e) = reader.seek_relative(length as i64).or_else(|_| {
                    reader
                        .seek(SeekFrom::Current(length as i64))
                        .map(|_| ())
                }) {
                    self.done = true;
                    return Some(Err(e.into()));
                }
                continue;
            }

            let mut payload = vec![0u8; length];
            if let Err(e) = reader.read_exact(&mut payload) {
                self.done = true;
                return Some(Err(e.into()));
            }

            if rec_offset >= self.log.next_offset {
                self.done = true;
                return None;
            }

            return Some(Ok((rec_offset, payload)));
        }
    }
}
