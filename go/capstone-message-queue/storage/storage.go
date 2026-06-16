// Package storage implements an append-only log with a sparse index.
//
// On-disk format:
//
//   data.log — fixed-header records:
//     [offset  uint64 BE][length uint32 BE][payload bytes]
//
//   data.idx — sparse index, one entry every 64 records:
//     [offset uint64 BE][file_pos uint64 BE]
//
// Recovery on open: scan from the last index entry forward to find the last
// valid record; truncate any partial trailing write.
package storage

import "errors"

// Record is a single entry returned by Scan.
type Record struct {
	Offset  uint64
	Payload []byte
}

// Log is an append-only log stored in a directory on disk.
type Log struct {
	// unexported fields
	dir string
}

// OpenLog opens or creates a log at dir (creates dir if needed).
// On reopen, recovers state from data.log and data.idx: scans forward from
// the last index entry to find the last valid record and truncates any partial
// trailing write.
func OpenLog(dir string) (*Log, error) {
	return nil, errors.New("not implemented")
}

// Append writes payload to the log and returns its assigned offset.
// The first record appended to a new log has offset 0.
func (l *Log) Append(payload []byte) (uint64, error) {
	return 0, errors.New("not implemented")
}

// Read returns the payload at exactly offset, or an error if offset is out of
// range (i.e., no record with that offset exists in the log).
func (l *Log) Read(offset uint64) ([]byte, error) {
	return nil, errors.New("not implemented")
}

// Scan returns up to maxCount records starting at startOffset (inclusive).
// Returns fewer records (or zero) if fewer exist beyond startOffset.
// Never returns an error for an empty result — only for I/O failures.
// If startOffset is beyond the end of the log, returns an empty slice and nil.
func (l *Log) Scan(startOffset uint64, maxCount int) ([]Record, error) {
	return nil, errors.New("not implemented")
}

// Close flushes and closes the underlying files.
func (l *Log) Close() error {
	return errors.New("not implemented")
}
