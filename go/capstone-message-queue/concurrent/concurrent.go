// Package concurrent wraps broker.Registry with concurrent-safe access.
//
// Writes (Produce, CreateTopic) take a write lock.
// Reads (Fetch, FetchBatch, TopicNames, NumPartitions) take a read lock.
//
// A background goroutine calls reg.Flush() every flushInterval. Close signals
// it to stop and waits for it to exit before closing the underlying registry.
package concurrent

import (
	"errors"
	"time"

	"github.com/andersonreyes/learning/go/capstone-message-queue/broker"
)

// SharedRegistry is a thread-safe wrapper around *broker.Registry.
type SharedRegistry struct {
	// unexported fields: *broker.Registry, sync.RWMutex, shutdown chan
	reg *broker.Registry
}

// NewSharedRegistry wraps reg with concurrent access and starts a background
// goroutine that calls reg.Flush() every flushInterval. The goroutine stops
// when Close is called.
func NewSharedRegistry(reg *broker.Registry, flushInterval time.Duration) *SharedRegistry {
	_ = flushInterval // suppress unused warning in stub
	return &SharedRegistry{reg: reg}
}

// CreateTopic creates a topic with n partitions under a write lock.
// Delegates to Registry.CreateTopic — same idempotency rules apply.
func (s *SharedRegistry) CreateTopic(name string, partitions int) error {
	return errors.New("not implemented")
}

// Produce appends payload to topic under a write lock.
// Returns partition index and offset.
func (s *SharedRegistry) Produce(topic string, payload []byte, key []byte) (partition uint32, offset uint64, err error) {
	return 0, 0, errors.New("not implemented")
}

// Fetch returns the payload at offset on topic/partition under a read lock.
func (s *SharedRegistry) Fetch(topic string, partition uint32, offset uint64) ([]byte, error) {
	return nil, errors.New("not implemented")
}

// FetchBatch returns up to maxCount records from topic/partition starting at
// offset under a read lock.
func (s *SharedRegistry) FetchBatch(topic string, partition uint32, offset uint64, maxCount int) ([]broker.Record, error) {
	return nil, errors.New("not implemented")
}

// TopicNames returns a sorted list of all topic names under a read lock.
func (s *SharedRegistry) TopicNames() []string {
	return nil
}

// NumPartitions returns the partition count for topic under a read lock.
func (s *SharedRegistry) NumPartitions(topic string) (int, error) {
	return 0, errors.New("not implemented")
}

// Close signals the flush goroutine to stop, waits for it to exit, then
// closes the underlying registry.
func (s *SharedRegistry) Close() error {
	return errors.New("not implemented")
}
