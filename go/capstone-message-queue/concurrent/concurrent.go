// Package concurrent provides a thread-safe broker handle with a background flush goroutine.
//
// # Locking strategy
//
// Produce holds a write lock for the duration of the append (the whole
// registry is serialized). This is simple and correct.
//
// Fetch / FetchBatch / metadata queries hold a read lock, so multiple
// consumers can read concurrently.
//
// # Background flush
//
// A background goroutine wakes every flushInterval and calls FlushAll on
// the registry.
package concurrent

import (
	"sync"
	"time"

	"github.com/andersonreyes/learning/go/capstone-message-queue/broker"
)

// SharedRegistry is a thread-safe handle to a Registry with a background flush goroutine.
//
// Always share a *SharedRegistry (or wrap in sync.Arc equivalent — just pass pointer).
// The background flush goroutine is stopped when Close is called or the struct is dropped.
type SharedRegistry struct {
	mu       sync.RWMutex
	registry *broker.Registry

	shutdown chan struct{}
	done     chan struct{}
}

// New wraps registry and starts a background flush goroutine that wakes every flushInterval.
func New(registry *broker.Registry, flushInterval time.Duration) *SharedRegistry {
	sr := &SharedRegistry{
		registry: registry,
		shutdown: make(chan struct{}),
		done:     make(chan struct{}),
	}

	go func() {
		defer close(sr.done)
		ticker := time.NewTicker(flushInterval)
		defer ticker.Stop()
		for {
			select {
			case <-sr.shutdown:
				return
			case <-ticker.C:
				sr.mu.Lock()
				_ = sr.registry.FlushAll()
				sr.mu.Unlock()
			}
		}
	}()

	return sr
}

// Close signals the flush goroutine to stop and waits for it to exit.
func (sr *SharedRegistry) Close() {
	select {
	case <-sr.shutdown:
		// already closed
	default:
		close(sr.shutdown)
	}
	<-sr.done
}

// ── Topic management ──────────────────────────────────────────────────────────

// CreateTopic creates a topic (or opens it if it already exists).
func (sr *SharedRegistry) CreateTopic(name string, numPartitions uint32) error {
	sr.mu.Lock()
	defer sr.mu.Unlock()
	return sr.registry.CreateTopic(name, numPartitions)
}

// TopicNames returns a sorted list of all topic names.
func (sr *SharedRegistry) TopicNames() []string {
	sr.mu.RLock()
	defer sr.mu.RUnlock()
	return sr.registry.TopicNames()
}

// NumPartitions returns the number of partitions for topic.
func (sr *SharedRegistry) NumPartitions(topic string) (int, error) {
	sr.mu.RLock()
	defer sr.mu.RUnlock()
	return sr.registry.NumPartitions(topic)
}

// ── Produce ───────────────────────────────────────────────────────────────────

// Produce appends payload to topic.
//
//   - key != nil: routes to partition fnv1a(key) % n.
//   - key == nil: round-robin.
//
// Returns (partitionID, offset).
func (sr *SharedRegistry) Produce(topic string, payload, key []byte) (uint32, uint64, error) {
	sr.mu.Lock()
	defer sr.mu.Unlock()
	return sr.registry.Produce(topic, payload, key)
}

// ── Fetch ─────────────────────────────────────────────────────────────────────

// Fetch reads the single payload at offset from topic/partition.
func (sr *SharedRegistry) Fetch(topic string, partition uint32, offset uint64) ([]byte, error) {
	sr.mu.RLock()
	defer sr.mu.RUnlock()
	return sr.registry.Fetch(topic, partition, offset)
}

// FetchBatch collects up to maxCount records from topic/partition starting at startOffset.
// Releases the read lock before returning.
func (sr *SharedRegistry) FetchBatch(topic string, partition uint32, startOffset uint64, maxCount int) ([]broker.Record, error) {
	sr.mu.RLock()
	defer sr.mu.RUnlock()
	return sr.registry.FetchBatch(topic, partition, startOffset, maxCount)
}

// NextOffset returns the next offset for topic/partition.
func (sr *SharedRegistry) NextOffset(topic string, partition uint32) (uint64, error) {
	sr.mu.RLock()
	defer sr.mu.RUnlock()
	t, err := sr.registry.GetTopic(topic)
	if err != nil {
		return 0, err
	}
	return t.NextOffset(partition)
}

// FlushAll flushes all partition logs immediately.
func (sr *SharedRegistry) FlushAll() error {
	sr.mu.Lock()
	defer sr.mu.Unlock()
	return sr.registry.FlushAll()
}
