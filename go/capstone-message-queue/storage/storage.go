// Package storage implements an append-only log storage engine.
//
// On-disk layout:
//
// data.log — sequence of fixed-header records:
//
//	[offset: uint64 BE][length: uint32 BE][payload: length bytes]
//
// data.idx — sparse index, one entry every 64 records:
//
//	[offset: uint64 BE][file_position: uint64 BE]  (16 bytes each)
//
// To read offset O:
//  1. Binary-search the index for the largest indexed offset <= O.
//  2. Seek to its file position.
//  3. Scan forward record by record until the offset header matches O.
package storage

import (
	"encoding/binary"
	"fmt"
	"io"
	"os"
	"path/filepath"
)

// indexInterval is how many records between sparse index entries.
const indexInterval = 64

// headerLen is the fixed record header size: 8 (offset) + 4 (length) = 12 bytes.
const headerLen = 12

// ErrOffsetOutOfRange is returned when reading an offset that doesn't exist.
type ErrOffsetOutOfRange struct {
	Offset uint64
}

func (e *ErrOffsetOutOfRange) Error() string {
	return fmt.Sprintf("offset %d out of range", e.Offset)
}

// ErrCorruptRecord is returned when the log file is malformed.
type ErrCorruptRecord struct {
	Msg string
}

func (e *ErrCorruptRecord) Error() string {
	return fmt.Sprintf("corrupt record: %s", e.Msg)
}

// ── Index ─────────────────────────────────────────────────────────────────────

// indexEntry maps a logical offset to its byte position in the .log file.
type indexEntry struct {
	offset   uint64
	position uint64
}

// index is a sparse in-memory index backed by a file on disk.
type index struct {
	path    string
	entries []indexEntry // sorted by offset
}

func openIndex(path string) (*index, error) {
	idx := &index{path: path}

	f, err := os.Open(path)
	if os.IsNotExist(err) {
		return idx, nil
	}
	if err != nil {
		return nil, err
	}
	defer f.Close()

	var buf [16]byte
	for {
		_, err := io.ReadFull(f, buf[:])
		if err == io.EOF || err == io.ErrUnexpectedEOF {
			break
		}
		if err != nil {
			return nil, err
		}
		off := binary.BigEndian.Uint64(buf[0:8])
		pos := binary.BigEndian.Uint64(buf[8:16])
		idx.entries = append(idx.entries, indexEntry{off, pos})
	}
	return idx, nil
}

// lookup returns the file position of the largest indexed offset <= offset,
// or (0, 0) if no such entry exists.
func (idx *index) lookup(offset uint64) (uint64, uint64) {
	// Binary search for largest entry with entry.offset <= offset.
	lo, hi := 0, len(idx.entries)
	for lo < hi {
		mid := (lo + hi) / 2
		if idx.entries[mid].offset <= offset {
			lo = mid + 1
		} else {
			hi = mid
		}
	}
	if lo == 0 {
		return 0, 0
	}
	e := idx.entries[lo-1]
	return e.offset, e.position
}

// append adds an entry to the in-memory index and appends it to the index file.
func (idx *index) append(offset, position uint64) error {
	idx.entries = append(idx.entries, indexEntry{offset, position})

	f, err := os.OpenFile(idx.path, os.O_CREATE|os.O_APPEND|os.O_WRONLY, 0o644)
	if err != nil {
		return err
	}
	defer f.Close()

	var buf [16]byte
	binary.BigEndian.PutUint64(buf[0:8], offset)
	binary.BigEndian.PutUint64(buf[8:16], position)
	_, err = f.Write(buf[:])
	return err
}

// ── Log ───────────────────────────────────────────────────────────────────────

// Log is an append-only log with offset-based random reads.
//
// Append returns the offset assigned to the new record. Offsets start at 0
// and increment by 1 per record (sequence numbers, not byte positions).
type Log struct {
	writer             *os.File
	logPath            string
	idx                *index
	nextOffset         uint64
	writePos           uint64
	recordsSinceIndex  uint64
}

// Open opens (or creates) a log in dir.
// On re-open, scans the existing .log file to recover nextOffset and writePos.
func Open(dir string) (*Log, error) {
	if err := os.MkdirAll(dir, 0o755); err != nil {
		return nil, err
	}

	logPath := filepath.Join(dir, "data.log")
	idxPath := filepath.Join(dir, "data.idx")

	idx, err := openIndex(idxPath)
	if err != nil {
		return nil, err
	}

	nextOffset, writePos, recordsSinceIndex, err := recover(logPath, idx)
	if err != nil {
		return nil, err
	}

	f, err := os.OpenFile(logPath, os.O_CREATE|os.O_APPEND|os.O_WRONLY, 0o644)
	if err != nil {
		return nil, err
	}

	// Truncate any partial trailing write from a previous unclean shutdown.
	actualEnd, err := f.Seek(0, io.SeekEnd)
	if err != nil {
		f.Close()
		return nil, err
	}
	if uint64(actualEnd) != writePos {
		if err := f.Truncate(int64(writePos)); err != nil {
			f.Close()
			return nil, err
		}
	}

	return &Log{
		writer:            f,
		logPath:           logPath,
		idx:               idx,
		nextOffset:        nextOffset,
		writePos:          writePos,
		recordsSinceIndex: recordsSinceIndex,
	}, nil
}

// recover scans from the last indexed position to recover (nextOffset, writePos, recordsSinceIndex).
func recover(logPath string, idx *index) (nextOffset, writePos, recordsSinceIndex uint64, err error) {
	info, statErr := os.Stat(logPath)
	if os.IsNotExist(statErr) {
		return 0, 0, 0, nil
	}
	if statErr != nil {
		return 0, 0, 0, statErr
	}
	if info.Size() == 0 {
		return 0, 0, 0, nil
	}

	// Find the last indexed position to start scanning from.
	var startOffset, startPos uint64
	if len(idx.entries) > 0 {
		last := idx.entries[len(idx.entries)-1]
		startOffset = last.offset
		startPos = last.position
	}

	f, err := os.Open(logPath)
	if err != nil {
		return 0, 0, 0, err
	}
	defer f.Close()

	if _, err := f.Seek(int64(startPos), io.SeekStart); err != nil {
		return 0, 0, 0, err
	}

	var curOffset uint64 = startOffset
	var pos uint64 = startPos
	var recsSinceIdx uint64
	foundAny := false

	var header [12]byte
	for {
		_, readErr := io.ReadFull(f, header[:])
		if readErr == io.EOF || readErr == io.ErrUnexpectedEOF {
			break
		}
		if readErr != nil {
			return 0, 0, 0, readErr
		}

		recOffset := binary.BigEndian.Uint64(header[0:8])
		length := uint32(binary.BigEndian.Uint32(header[8:12]))

		// Validate: offset must be monotonically increasing.
		if foundAny && recOffset != curOffset+1 {
			return 0, 0, 0, &ErrCorruptRecord{
				Msg: fmt.Sprintf("expected offset %d got %d", curOffset+1, recOffset),
			}
		}

		// Skip the payload.
		if _, err := f.Seek(int64(length), io.SeekCurrent); err != nil {
			return 0, 0, 0, err
		}

		pos += headerLen + uint64(length)
		curOffset = recOffset
		foundAny = true
		recsSinceIdx = (recsSinceIdx + 1) % indexInterval
	}

	if foundAny {
		return curOffset + 1, pos, recsSinceIdx, nil
	}
	return startOffset, pos, recsSinceIdx, nil
}

// Append appends payload to the log, returning its assigned offset.
func (l *Log) Append(payload []byte) (uint64, error) {
	offset := l.nextOffset
	length := uint32(len(payload))

	// Write: [offset uint64 BE][length uint32 BE][payload]
	var header [12]byte
	binary.BigEndian.PutUint64(header[0:8], offset)
	binary.BigEndian.PutUint32(header[8:12], length)

	if _, err := l.writer.Write(header[:]); err != nil {
		return 0, err
	}
	if _, err := l.writer.Write(payload); err != nil {
		return 0, err
	}

	// Sparse index: record every indexInterval-th entry.
	if l.recordsSinceIndex == 0 {
		if err := l.idx.append(offset, l.writePos); err != nil {
			return 0, err
		}
	}
	l.recordsSinceIndex = (l.recordsSinceIndex + 1) % indexInterval

	l.writePos += headerLen + uint64(length)
	l.nextOffset++
	return offset, nil
}

// Flush flushes buffered writes to the OS.
func (l *Log) Flush() error {
	return l.writer.Sync()
}

// Read reads the payload stored at offset.
func (l *Log) Read(offset uint64) ([]byte, error) {
	if offset >= l.nextOffset {
		return nil, &ErrOffsetOutOfRange{Offset: offset}
	}

	// Find the closest indexed position <= offset.
	_, startPos := l.idx.lookup(offset)

	f, err := os.Open(l.logPath)
	if err != nil {
		return nil, err
	}
	defer f.Close()

	if _, err := f.Seek(int64(startPos), io.SeekStart); err != nil {
		return nil, err
	}

	var header [12]byte
	for {
		if _, err := io.ReadFull(f, header[:]); err != nil {
			return nil, &ErrCorruptRecord{Msg: fmt.Sprintf("truncated header at offset %d", offset)}
		}
		recOffset := binary.BigEndian.Uint64(header[0:8])
		length := int(binary.BigEndian.Uint32(header[8:12]))

		if recOffset == offset {
			payload := make([]byte, length)
			if _, err := io.ReadFull(f, payload); err != nil {
				return nil, err
			}
			return payload, nil
		}

		// Skip this record's payload.
		if _, err := f.Seek(int64(length), io.SeekCurrent); err != nil {
			return nil, err
		}
	}
}

// NextOffset returns the offset that will be assigned to the next appended record.
func (l *Log) NextOffset() uint64 {
	return l.nextOffset
}

// Record is a (offset, payload) pair returned by ScanAll.
type Record struct {
	Offset  uint64
	Payload []byte
}

// ScanAll returns up to maxCount records starting from startOffset.
// Pass maxCount <= 0 to return all records.
func (l *Log) ScanAll(startOffset uint64, maxCount int) ([]Record, error) {
	if l.nextOffset == 0 {
		return nil, nil
	}

	// Find the closest indexed position <= startOffset.
	_, startPos := l.idx.lookup(startOffset)

	f, err := os.Open(l.logPath)
	if err != nil {
		return nil, err
	}
	defer f.Close()

	if _, err := f.Seek(int64(startPos), io.SeekStart); err != nil {
		return nil, err
	}

	var records []Record
	var header [12]byte

	for {
		if maxCount > 0 && len(records) >= maxCount {
			break
		}

		_, readErr := io.ReadFull(f, header[:])
		if readErr == io.EOF || readErr == io.ErrUnexpectedEOF {
			break
		}
		if readErr != nil {
			return nil, readErr
		}

		recOffset := binary.BigEndian.Uint64(header[0:8])
		length := int(binary.BigEndian.Uint32(header[8:12]))

		if recOffset >= l.nextOffset {
			break
		}

		if recOffset < startOffset {
			// Skip records before startOffset (started at an index entry that precedes startOffset).
			if _, err := f.Seek(int64(length), io.SeekCurrent); err != nil {
				return nil, err
			}
			continue
		}

		payload := make([]byte, length)
		if _, err := io.ReadFull(f, payload); err != nil {
			return nil, err
		}

		records = append(records, Record{Offset: recOffset, Payload: payload})
	}

	return records, nil
}

// Close closes the underlying file handle.
func (l *Log) Close() error {
	return l.writer.Close()
}
