// Package broker implements the broker layer: Partition, Topic, and Registry.
//
// Directory layout on disk:
//
//	<base_dir>/<topic_name>/<partition_id>/data.log
//	<base_dir>/<topic_name>/<partition_id>/data.idx
package broker

import (
	"errors"
	"fmt"
	"os"
	"path/filepath"
	"sort"
	"strconv"

	"github.com/andersonreyes/learning/go/capstone-message-queue/storage"
)

// ── Errors ────────────────────────────────────────────────────────────────────

// ErrTopicNotFound is returned when a topic doesn't exist.
type ErrTopicNotFound struct{ Name string }

func (e *ErrTopicNotFound) Error() string { return fmt.Sprintf("topic not found: %s", e.Name) }

// ErrPartitionOutOfRange is returned when a partition ID is invalid.
type ErrPartitionOutOfRange struct{ ID uint32 }

func (e *ErrPartitionOutOfRange) Error() string {
	return fmt.Sprintf("partition %d out of range", e.ID)
}

// ── FNV-1a ────────────────────────────────────────────────────────────────────

func fnv1a(data []byte) uint64 {
	h := uint64(14695981039346656037)
	for _, b := range data {
		h ^= uint64(b)
		h *= 1099511628211
	}
	return h
}

// ── Record ────────────────────────────────────────────────────────────────────

// Record is a record returned by scan iterators.
type Record struct {
	Offset  uint64
	Payload []byte
}

// ── Partition ─────────────────────────────────────────────────────────────────

// Partition is a single log partition. Wraps storage.Log and tracks its ID.
type Partition struct {
	log *storage.Log
	id  uint32
}

// OpenPartition opens (or creates) a partition at <baseDir>/<topic>/<id>/.
func OpenPartition(baseDir, topic string, id uint32) (*Partition, error) {
	dir := filepath.Join(baseDir, topic, strconv.Itoa(int(id)))
	log, err := storage.Open(dir)
	if err != nil {
		return nil, err
	}
	return &Partition{log: log, id: id}, nil
}

// Append appends payload, returning the assigned offset.
// Flushes immediately so subsequent reads (which open a fresh file handle) see the bytes.
func (p *Partition) Append(payload []byte) (uint64, error) {
	offset, err := p.log.Append(payload)
	if err != nil {
		return 0, err
	}
	if err := p.log.Flush(); err != nil {
		return 0, err
	}
	return offset, nil
}

// Read reads the payload at offset.
func (p *Partition) Read(offset uint64) ([]byte, error) {
	return p.log.Read(offset)
}

// ScanAll returns up to maxCount records starting at startOffset.
func (p *Partition) ScanAll(startOffset uint64, maxCount int) ([]Record, error) {
	recs, err := p.log.ScanAll(startOffset, maxCount)
	if err != nil {
		return nil, err
	}
	result := make([]Record, len(recs))
	for i, r := range recs {
		result[i] = Record{Offset: r.Offset, Payload: r.Payload}
	}
	return result, nil
}

// NextOffset returns the offset that will be assigned to the next record.
func (p *Partition) NextOffset() uint64 {
	return p.log.NextOffset()
}

// ID returns the partition ID.
func (p *Partition) ID() uint32 {
	return p.id
}

// Flush flushes buffered writes to the OS.
func (p *Partition) Flush() error {
	return p.log.Flush()
}

// ── Topic ─────────────────────────────────────────────────────────────────────

// Topic is a named topic with N partitions.
type Topic struct {
	name       string
	partitions []*Partition
	roundRobin int
}

// OpenTopic opens (or creates) a topic directory with numPartitions partitions.
//
// If the directory already has more partitions than numPartitions, all existing
// ones are opened (never shrinks). numPartitions is only used when creating a
// fresh topic.
func OpenTopic(baseDir, name string, numPartitions uint32) (*Topic, error) {
	topicDir := filepath.Join(baseDir, name)
	existingCount := countPartitionDirs(topicDir)
	count := existingCount
	if numPartitions > count {
		count = numPartitions
	}

	partitions := make([]*Partition, count)
	for id := uint32(0); id < count; id++ {
		p, err := OpenPartition(baseDir, name, id)
		if err != nil {
			return nil, err
		}
		partitions[id] = p
	}

	return &Topic{name: name, partitions: partitions}, nil
}

// Produce appends payload to a partition.
//
//   - key != nil: routes to partition fnv1a(key) % n.
//   - key == nil: round-robin across all partitions.
//
// Returns (partitionID, offset).
func (t *Topic) Produce(payload, key []byte) (uint32, uint64, error) {
	n := len(t.partitions)
	var partitionIdx int
	if key != nil {
		partitionIdx = int(fnv1a(key) % uint64(n))
	} else {
		partitionIdx = t.roundRobin % n
		t.roundRobin++
	}
	p := t.partitions[partitionIdx]
	offset, err := p.Append(payload)
	if err != nil {
		return 0, 0, err
	}
	return p.ID(), offset, nil
}

// Fetch reads the payload at offset from partition p.
func (t *Topic) Fetch(partition uint32, offset uint64) ([]byte, error) {
	if int(partition) >= len(t.partitions) {
		return nil, &ErrPartitionOutOfRange{ID: partition}
	}
	return t.partitions[partition].Read(offset)
}

// FetchBatch returns up to maxCount records from partition starting at startOffset.
func (t *Topic) FetchBatch(partition uint32, startOffset uint64, maxCount int) ([]Record, error) {
	if int(partition) >= len(t.partitions) {
		return nil, &ErrPartitionOutOfRange{ID: partition}
	}
	return t.partitions[partition].ScanAll(startOffset, maxCount)
}

// NextOffset returns the next offset for the given partition.
func (t *Topic) NextOffset(partition uint32) (uint64, error) {
	if int(partition) >= len(t.partitions) {
		return 0, &ErrPartitionOutOfRange{ID: partition}
	}
	return t.partitions[partition].NextOffset(), nil
}

// Name returns the topic name.
func (t *Topic) Name() string {
	return t.name
}

// NumPartitions returns the number of partitions.
func (t *Topic) NumPartitions() int {
	return len(t.partitions)
}

// FlushAll flushes all partition logs.
func (t *Topic) FlushAll() error {
	for _, p := range t.partitions {
		if err := p.Flush(); err != nil {
			return err
		}
	}
	return nil
}

// ── Registry ──────────────────────────────────────────────────────────────────

// Registry is the registry of all topics. Persists state via directory layout:
//
//	<baseDir>/<topicName>/<partitionID>/data.log + data.idx
type Registry struct {
	baseDir string
	topics  map[string]*Topic
}

// Open opens an existing registry (or creates a fresh one) at baseDir.
// Scans baseDir for existing topic directories and re-opens each topic.
func Open(baseDir string) (*Registry, error) {
	if err := os.MkdirAll(baseDir, 0o755); err != nil {
		return nil, err
	}

	topics := make(map[string]*Topic)

	entries, err := os.ReadDir(baseDir)
	if err != nil {
		return nil, err
	}
	for _, entry := range entries {
		if !entry.IsDir() {
			continue
		}
		name := entry.Name()
		topicDir := filepath.Join(baseDir, name)
		numPartitions := countPartitionDirs(topicDir)
		if numPartitions == 0 {
			continue
		}
		topic, err := OpenTopic(baseDir, name, numPartitions)
		if err != nil {
			return nil, err
		}
		topics[name] = topic
	}

	return &Registry{baseDir: baseDir, topics: topics}, nil
}

// CreateTopic creates (or idempotently opens) a topic.
// If the topic already exists, it is opened as-is (numPartitions is ignored).
func (r *Registry) CreateTopic(name string, numPartitions uint32) error {
	if _, ok := r.topics[name]; ok {
		return nil // idempotent
	}
	topic, err := OpenTopic(r.baseDir, name, numPartitions)
	if err != nil {
		return err
	}
	r.topics[name] = topic
	return nil
}

// GetTopic returns the topic with the given name, or an error if it doesn't exist.
func (r *Registry) GetTopic(name string) (*Topic, error) {
	t, ok := r.topics[name]
	if !ok {
		return nil, &ErrTopicNotFound{Name: name}
	}
	return t, nil
}

// TopicNames returns a sorted list of all topic names.
func (r *Registry) TopicNames() []string {
	names := make([]string, 0, len(r.topics))
	for name := range r.topics {
		names = append(names, name)
	}
	sort.Strings(names)
	return names
}

// Produce appends payload to topic/partition selected by key (or round-robin).
func (r *Registry) Produce(topic string, payload, key []byte) (uint32, uint64, error) {
	t, err := r.GetTopic(topic)
	if err != nil {
		return 0, 0, err
	}
	return t.Produce(payload, key)
}

// Fetch reads the single payload at offset from topic/partition.
func (r *Registry) Fetch(topic string, partition uint32, offset uint64) ([]byte, error) {
	t, err := r.GetTopic(topic)
	if err != nil {
		return nil, err
	}
	return t.Fetch(partition, offset)
}

// FetchBatch collects up to maxCount records from topic/partition starting at startOffset.
func (r *Registry) FetchBatch(topic string, partition uint32, startOffset uint64, maxCount int) ([]Record, error) {
	t, err := r.GetTopic(topic)
	if err != nil {
		return nil, err
	}
	return t.FetchBatch(partition, startOffset, maxCount)
}

// NumPartitions returns the number of partitions for topic.
func (r *Registry) NumPartitions(topic string) (int, error) {
	t, err := r.GetTopic(topic)
	if err != nil {
		return 0, err
	}
	return t.NumPartitions(), nil
}

// FlushAll flushes all partition logs across all topics.
func (r *Registry) FlushAll() error {
	for _, topic := range r.topics {
		if err := topic.FlushAll(); err != nil {
			return err
		}
	}
	return nil
}

// ── helpers ───────────────────────────────────────────────────────────────────

// countPartitionDirs counts how many numeric subdirectories exist in dir.
func countPartitionDirs(dir string) uint32 {
	entries, err := os.ReadDir(dir)
	if err != nil {
		return 0
	}
	var count uint32
	for _, e := range entries {
		if !e.IsDir() {
			continue
		}
		if _, err := strconv.ParseUint(e.Name(), 10, 32); err == nil {
			count++
		}
	}
	return count
}

// IsTopicNotFound reports whether err is an ErrTopicNotFound.
func IsTopicNotFound(err error) bool {
	var e *ErrTopicNotFound
	return errors.As(err, &e)
}

// IsPartitionOutOfRange reports whether err is an ErrPartitionOutOfRange.
func IsPartitionOutOfRange(err error) bool {
	var e *ErrPartitionOutOfRange
	return errors.As(err, &e)
}
