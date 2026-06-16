// Package broker implements a topic/partition registry on top of the storage
// package.
//
// Topics are routed by FNV-1a hash of the key (mod numPartitions) when a key
// is provided, or round-robin when the key is nil.
//
// FNV-1a constants:
//
//	offset basis: 14695981039346656037
//	prime:        1099511628211
package broker

import (
	"errors"

	"github.com/andersonreyes/learning/go/capstone-message-queue/storage"
)

// Record is a single fetched entry (re-exported for callers that only import
// broker).
type Record struct {
	Offset  uint64
	Payload []byte
}

// Registry manages topics and their partitions.
type Registry struct {
	// unexported fields
	dir string
}

// OpenRegistry opens (or creates) a registry rooted at dir.
// Layout: dir/<topic>/<partition_id>/ each containing a storage.Log.
func OpenRegistry(dir string) (*Registry, error) {
	return nil, errors.New("not implemented")
}

// CreateTopic creates a topic with n partitions.
// Idempotent — calling again with the same name and same partition count is a
// no-op (not an error).
// Returns an error if the topic already exists with a different partition count.
func (r *Registry) CreateTopic(name string, partitions int) error {
	return errors.New("not implemented")
}

// Produce appends payload to topic, routing by key (FNV-1a hash mod
// partitions) if key is non-nil, or round-robin across partitions if key is
// nil.
// Returns the partition index and the offset of the written record.
// Returns an error if the topic does not exist.
func (r *Registry) Produce(topic string, payload []byte, key []byte) (partition uint32, offset uint64, err error) {
	return 0, 0, errors.New("not implemented")
}

// Fetch returns the payload at exactly offset on topic/partition.
// Returns an error if the topic/partition does not exist or offset is out of
// range.
func (r *Registry) Fetch(topic string, partition uint32, offset uint64) ([]byte, error) {
	return nil, errors.New("not implemented")
}

// FetchBatch returns up to maxCount records from topic/partition starting at
// offset (inclusive).
// Returns fewer records (or zero) if fewer exist. Never errors for an empty
// result — only for I/O failures or unknown topic/partition.
func (r *Registry) FetchBatch(topic string, partition uint32, offset uint64, maxCount int) ([]Record, error) {
	return nil, errors.New("not implemented")
}

// TopicNames returns a sorted list of all topic names.
func (r *Registry) TopicNames() []string {
	return nil
}

// NumPartitions returns the partition count for topic, or an error if the
// topic does not exist.
func (r *Registry) NumPartitions(topic string) (int, error) {
	return 0, errors.New("not implemented")
}

// Flush flushes all partition logs to disk.
func (r *Registry) Flush() error {
	return errors.New("not implemented")
}

// Close flushes and closes all underlying logs.
func (r *Registry) Close() error {
	return errors.New("not implemented")
}

// storageRecord converts a storage.Record to a broker.Record.
// Used internally — exported for testing convenience.
func storageRecord(s storage.Record) Record {
	return Record{Offset: s.Offset, Payload: s.Payload}
}
