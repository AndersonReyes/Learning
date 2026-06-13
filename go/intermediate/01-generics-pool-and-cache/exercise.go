// Package poolcache applies Go generics to two networking-adjacent data
// structures: a generic resource pool (Pool[T]), suited to reusing
// expensive-to-create values like net.Conn, and a generic fixed-capacity
// LRU cache (LRU[K, V]), suited to caching DNS lookups (topic 12) keyed by
// hostname.
package poolcache

import (
	"container/list"
	"errors"
	"sync"
)

// Pool is a concurrency-safe pool of reusable values of type T. Get
// returns an idle value if one is available, or creates one by calling
// New; Put returns a value to the pool for reuse, discarding it once the
// pool already holds max idle values.
type Pool[T any] struct {
	mu   sync.Mutex
	idle []T
	new  func() (T, error)
	max  int
}

// NewPool returns a new Pool that creates values with newFn and holds at
// most max idle values for reuse.
func NewPool[T any](max int, newFn func() (T, error)) *Pool[T] {
	return &Pool[T]{new: newFn, max: max}
}

// Get returns an idle value from the pool if one is available, or
// otherwise creates a new one by calling the pool's New function. Get is
// safe to call concurrently with Get and Put.
func (p *Pool[T]) Get() (T, error) {
	var zero T
	return zero, errors.New("not implemented")
}

// Put returns item to the pool for reuse by a future Get. If the pool
// already holds max idle items, Put discards item instead. Put is safe to
// call concurrently with Get and Put.
func (p *Pool[T]) Put(item T) {
}

// entry is one key/value pair stored in an LRU's eviction list.
type entry[K comparable, V any] struct {
	key   K
	value V
}

// LRU is a concurrency-safe, fixed-capacity cache that evicts the
// least-recently-used entry when a Put would exceed capacity. Both Get and
// Put count as a "use", moving the entry to the front of the eviction
// order.
type LRU[K comparable, V any] struct {
	mu       sync.Mutex
	capacity int
	items    map[K]*list.Element
	order    *list.List
}

// NewLRU returns a new LRU cache with the given capacity. capacity must be
// at least 1.
func NewLRU[K comparable, V any](capacity int) *LRU[K, V] {
	return &LRU[K, V]{
		capacity: capacity,
		items:    make(map[K]*list.Element),
		order:    list.New(),
	}
}

// Get returns the value stored for key and true, moving key to the front
// of the eviction order. It returns the zero value and false if key is not
// present. Get is safe to call concurrently with Get and Put.
func (c *LRU[K, V]) Get(key K) (V, bool) {
	var zero V
	return zero, false
}

// Put stores value under key, moving key to the front of the eviction
// order. If key is already present, its value is replaced. If inserting a
// new key would exceed capacity, Put first evicts the least-recently-used
// entry. Put is safe to call concurrently with Get and Put.
func (c *LRU[K, V]) Put(key K, value V) {
}

// Len returns the number of entries currently in the cache. Len is safe to
// call concurrently with Get and Put.
func (c *LRU[K, V]) Len() int {
	return 0
}
