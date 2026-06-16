//! Broker layer: Partition, Topic, and Registry.
//!
//! Directory layout on disk:
//!   `<base_dir>/<topic_name>/<partition_id>/data.log`
//!   `<base_dir>/<topic_name>/<partition_id>/data.idx`

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::error::{Error, Result};
use crate::storage::Log;

// ── FNV-1a ───────────────────────────────────────────────────────────────────

fn fnv1a(data: &[u8]) -> u64 {
    let mut h = 14695981039346656037_u64;
    for &b in data {
        h ^= b as u64;
        h = h.wrapping_mul(1099511628211);
    }
    h
}

// ── Record ───────────────────────────────────────────────────────────────────

/// A record returned by scan iterators.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Record {
    pub offset: u64,
    pub payload: Vec<u8>,
}

// ── Partition ────────────────────────────────────────────────────────────────

/// A single log partition. Wraps `Log` and tracks its topic/id for directory
/// routing.
pub struct Partition {
    log: Log,
    id: u32,
}

impl Partition {
    /// Opens (or creates) a partition at `<base_dir>/<topic>/<id>/`.
    pub fn open(base_dir: &Path, topic: &str, id: u32) -> Result<Self> {
        let dir = base_dir.join(topic).join(id.to_string());
        let log = Log::open(&dir)?;
        Ok(Partition { log, id })
    }

    /// Appends `payload`, returning the assigned offset.
    ///
    /// Flushes the underlying log immediately so that subsequent reads
    /// (which open a fresh file handle) see the written bytes.
    pub fn append(&mut self, payload: &[u8]) -> Result<u64> {
        let offset = self.log.append(payload)?;
        self.log.flush()?;
        Ok(offset)
    }

    /// Reads the payload at `offset`.
    pub fn read(&self, offset: u64) -> Result<Vec<u8>> {
        self.log.read(offset)
    }

    /// Returns an iterator over `Record`s starting from `start_offset`.
    pub fn scan(
        &self,
        start_offset: u64,
    ) -> impl Iterator<Item = crate::error::Result<Record>> + '_ {
        self.log.scan(start_offset).map(|r| {
            r.map(|(offset, payload)| Record { offset, payload })
        })
    }

    /// The offset that will be assigned to the next appended record.
    pub fn next_offset(&self) -> u64 {
        self.log.next_offset()
    }

    /// The partition id.
    pub fn id(&self) -> u32 {
        self.id
    }

    /// Flushes buffered writes to the OS.
    pub fn flush(&mut self) -> Result<()> {
        self.log.flush()
    }
}

// ── Topic ────────────────────────────────────────────────────────────────────

/// A named topic with N partitions.
pub struct Topic {
    name: String,
    partitions: Vec<Partition>,
    round_robin: usize,
}

impl Topic {
    /// Opens (or creates) a topic directory with `num_partitions` partitions.
    ///
    /// If the directory already has more partitions than `num_partitions`, all
    /// existing ones are opened (never shrinks). The `num_partitions` argument
    /// is only used when creating a fresh topic.
    pub fn open(base_dir: &Path, name: &str, num_partitions: u32) -> Result<Self> {
        let topic_dir = base_dir.join(name);

        // Count how many numeric subdirs already exist.
        let existing_count = count_partition_dirs(&topic_dir);
        let count = existing_count.max(num_partitions);

        let mut partitions = Vec::with_capacity(count as usize);
        for id in 0..count {
            partitions.push(Partition::open(base_dir, name, id)?);
        }

        Ok(Topic {
            name: name.to_owned(),
            partitions,
            round_robin: 0,
        })
    }

    /// Appends `payload` to a partition.
    ///
    /// - `key=Some(k)`: routes to partition `fnv1a(k) % n`.
    /// - `key=None`: round-robin across all partitions.
    ///
    /// Returns `(partition_id, offset)`.
    pub fn produce(&mut self, payload: &[u8], key: Option<&[u8]>) -> Result<(u32, u64)> {
        let n = self.partitions.len();
        let partition_idx = match key {
            Some(k) => (fnv1a(k) % n as u64) as usize,
            None => {
                let idx = self.round_robin % n;
                self.round_robin = self.round_robin.wrapping_add(1);
                idx
            }
        };
        let partition = &mut self.partitions[partition_idx];
        let offset = partition.append(payload)?;
        Ok((partition.id(), offset))
    }

    /// Reads the payload at `offset` from partition `partition`.
    pub fn fetch(&self, partition: u32, offset: u64) -> Result<Vec<u8>> {
        let p = self
            .partitions
            .get(partition as usize)
            .ok_or(Error::PartitionOutOfRange(partition))?;
        p.read(offset)
    }

    /// Returns an iterator over `Record`s in partition `partition` starting
    /// from `start_offset`.
    pub fn scan(
        &self,
        partition: u32,
        start_offset: u64,
    ) -> Result<impl Iterator<Item = crate::error::Result<Record>> + '_> {
        let p = self
            .partitions
            .get(partition as usize)
            .ok_or(Error::PartitionOutOfRange(partition))?;
        Ok(p.scan(start_offset))
    }

    /// Returns the next offset for partition `partition`.
    pub fn next_offset(&self, partition: u32) -> Result<u64> {
        let p = self
            .partitions
            .get(partition as usize)
            .ok_or(Error::PartitionOutOfRange(partition))?;
        Ok(p.next_offset())
    }

    /// The topic name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The number of partitions.
    pub fn num_partitions(&self) -> usize {
        self.partitions.len()
    }

    /// Flushes all partition logs.
    pub fn flush_all(&mut self) -> Result<()> {
        for p in &mut self.partitions {
            p.flush()?;
        }
        Ok(())
    }
}

// ── Registry ─────────────────────────────────────────────────────────────────

/// Registry of all topics. Persists state via directory layout:
///   `<base_dir>/<topic_name>/<partition_id>/data.log + data.idx`
///
/// Opening the registry scans `base_dir` for existing topic directories and
/// counts their numeric subdirectories to recover partition counts.
pub struct Registry {
    base_dir: PathBuf,
    topics: HashMap<String, Topic>,
}

impl Registry {
    /// Opens an existing registry (or creates a fresh one) at `base_dir`.
    ///
    /// Scans `base_dir` for existing topic directories and re-opens each topic
    /// with all its partitions.
    pub fn open(base_dir: &Path) -> Result<Self> {
        std::fs::create_dir_all(base_dir)?;

        let mut topics = HashMap::new();

        // Scan for existing topic directories.
        let read_dir = std::fs::read_dir(base_dir)?;
        for entry in read_dir {
            let entry = entry?;
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }
            let name = match path.file_name().and_then(|n| n.to_str()) {
                Some(n) => n.to_owned(),
                None => continue,
            };
            let num_partitions = count_partition_dirs(&path);
            if num_partitions == 0 {
                continue;
            }
            let topic = Topic::open(base_dir, &name, num_partitions)?;
            topics.insert(name, topic);
        }

        Ok(Registry {
            base_dir: base_dir.to_owned(),
            topics,
        })
    }

    /// Creates (or idempotently opens) a topic.
    ///
    /// If the topic already exists, opens it as-is (the `num_partitions`
    /// argument is ignored for existing topics).
    pub fn create_topic(&mut self, name: &str, num_partitions: u32) -> Result<()> {
        if self.topics.contains_key(name) {
            // Idempotent: already open.
            return Ok(());
        }
        let topic = Topic::open(&self.base_dir, name, num_partitions)?;
        self.topics.insert(name.to_owned(), topic);
        Ok(())
    }

    /// Returns a shared reference to a topic, or `None` if it doesn't exist.
    pub fn get_topic(&self, name: &str) -> Option<&Topic> {
        self.topics.get(name)
    }

    /// Returns a mutable reference to a topic, or `None` if it doesn't exist.
    pub fn get_topic_mut(&mut self, name: &str) -> Option<&mut Topic> {
        self.topics.get_mut(name)
    }

    /// Returns a sorted list of all topic names.
    pub fn topic_names(&self) -> Vec<String> {
        let mut names: Vec<String> = self.topics.keys().cloned().collect();
        names.sort();
        names
    }

    /// Flushes all partition logs across all topics.
    pub fn flush_all(&mut self) -> Result<()> {
        for topic in self.topics.values_mut() {
            topic.flush_all()?;
        }
        Ok(())
    }
}

// ── helpers ──────────────────────────────────────────────────────────────────

/// Counts how many numeric subdirectories exist in `dir`.
fn count_partition_dirs(dir: &Path) -> u32 {
    if !dir.is_dir() {
        return 0;
    }
    let Ok(read_dir) = std::fs::read_dir(dir) else {
        return 0;
    };
    let mut count = 0u32;
    for entry in read_dir.flatten() {
        let path = entry.path();
        if path.is_dir() {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.parse::<u32>().is_ok() {
                    count += 1;
                }
            }
        }
    }
    count
}
