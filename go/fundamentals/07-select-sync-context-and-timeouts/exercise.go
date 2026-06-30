// Package netctx applies select, sync, and context to model connection
// timeouts and cancellation: a concurrency-safe stats tracker, running a
// batch of tasks to completion, racing an operation against a deadline,
// and a context-aware fan-in that stops early when canceled.
package netctx

import (
	"context"
	"fmt"
	"maps"
	"sync"
	"time"
)

// Stats tracks per-key event counts (e.g., requests per remote address)
// and is safe for concurrent use by multiple goroutines.
type Stats struct {
	mu     sync.Mutex
	counts map[string]int
}

// NewStats returns an empty Stats.
func NewStats() *Stats {
	return &Stats{counts: make(map[string]int)}
}

// Record increments the count for key by one. Record is safe to call
// concurrently from multiple goroutines.
func (s *Stats) Record(key string) {
	s.mu.Lock()
	defer s.mu.Unlock()

	s.counts[key]++
}

// Snapshot returns a copy of the current counts, safe for the caller to
// read or modify without affecting s or racing with concurrent calls to
// Record.
func (s *Stats) Snapshot() map[string]int {
	s.mu.Lock()
	defer s.mu.Unlock()

	out := make(map[string]int, len(s.counts))

	maps.Copy(out, s.counts)
	return out
}

// WaitAll runs each task in tasks concurrently and waits for all of them
// to finish. It returns a slice the same length as tasks, where
// result[i] is the error returned by tasks[i] (nil on success).
func WaitAll(tasks []func() error) []error {
	var wg sync.WaitGroup
	errors := make([]error, len(tasks))

	for taskId, task := range tasks {
		wg.Add(1)
		go func(task func() error, taskId int, errors []error) {
			defer wg.Done()
			err := task()
			if err != nil {
				errors[taskId] = err
			}
		}(task, taskId, errors)
	}

	wg.Wait()
	return errors
}

// DialWithTimeout runs dial in its own goroutine and returns its result
// if dial returns within timeout. If timeout elapses first,
// DialWithTimeout returns ("", context.DeadlineExceeded); dial's
// goroutine is left to finish on its own and its result is discarded.
// This mirrors net.Conn.SetDeadline: an operation that doesn't complete
// in time fails with a deadline error.
func DialWithTimeout(dial func() (string, error), timeout time.Duration) (string, error) {
	errCh := make(chan error)
	strCh := make(chan string)

	go func(ech chan error, strch chan string) {
		s, err := dial()

		if err != nil {
			ech <- err
		} else {
			strch <- s
		}
	}(errCh, strCh)

	select {
	case result := <-strCh:
		return result, nil
	case err := <-errCh:
		return "", err
	case <-time.After(timeout):
		return "", context.DeadlineExceeded
	}
}

// MergeWithContext merges zero or more read-only int channels into a
// single output channel — a context-aware fan-in. Every value received
// from an input channel before ctx is done is forwarded to the output.
// The output channel is closed once either every input channel has been
// closed and drained, or ctx is done, whichever happens first.
func MergeWithContext(ctx context.Context, inputs ...<-chan int) <-chan int {

	out := make(chan int)

	var wg sync.WaitGroup

	for i, ch := range inputs {
		wg.Add(1)
		go func(ch <-chan int, chId int) {
			defer wg.Done()
			done := false
			for {
				fmt.Printf("ch=%d is done: %t\n", chId, done)
				select {
				case v, ok := <-ch:
					fmt.Printf("getting from v=%d from ch=%d with ok=%t.\n", v, chId, ok)
					if !ok {
						return
					}
					out <- v

				case <-ctx.Done():
					fmt.Printf("context done ch=%d\n", chId)
					return
				}
			}
		}(ch, i)

	}

	// wait and close need to be on a separate coroutine so we can return out
	go func() {
		wg.Wait()
		close(out)
	}()

	return out
}
