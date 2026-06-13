package netctx

import (
	"context"
	"errors"
	"reflect"
	"sort"
	"sync"
	"testing"
	"time"
)

// chanTimeout bounds how long a test waits on a channel produced by the
// function under test. It's generous enough for a correct implementation
// (which resolves in microseconds) but keeps a not-yet-implemented stub
// (which returns a nil channel) from hanging the test run.
const chanTimeout = 200 * time.Millisecond

// collectChan receives every value from ch until it's closed, or fails
// the test if timeout elapses without a value or a close (including the
// case where ch is nil, which never becomes ready).
func collectChan(t *testing.T, ch <-chan int, timeout time.Duration) []int {
	t.Helper()
	var got []int
	for {
		select {
		case v, ok := <-ch:
			if !ok {
				return got
			}
			got = append(got, v)
		case <-time.After(timeout):
			t.Fatalf("timed out waiting for channel to produce a value or close")
			return nil
		}
	}
}

// bufferedIntChan returns a channel pre-loaded with vals and already
// closed, so reading it never blocks regardless of who (if anyone) reads
// it.
func bufferedIntChan(vals ...int) <-chan int {
	ch := make(chan int, len(vals))
	for _, v := range vals {
		ch <- v
	}
	close(ch)
	return ch
}

func TestStatsRecordAndSnapshot(t *testing.T) {
	s := NewStats()
	const n = 100

	var wg sync.WaitGroup
	for i := 0; i < n; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			s.Record("a")
		}()
	}
	wg.Wait()

	snap := s.Snapshot()
	if snap["a"] != n {
		t.Errorf(`Snapshot()["a"] = %d, want %d`, snap["a"], n)
	}
}

func TestStatsMultipleKeys(t *testing.T) {
	s := NewStats()
	for i := 0; i < 3; i++ {
		s.Record("x")
	}
	for i := 0; i < 5; i++ {
		s.Record("y")
	}

	got := s.Snapshot()
	want := map[string]int{"x": 3, "y": 5}
	if !reflect.DeepEqual(got, want) {
		t.Errorf("Snapshot() = %v, want %v", got, want)
	}
}

func TestStatsSnapshotIsCopy(t *testing.T) {
	s := NewStats()
	s.Record("a")

	snap := s.Snapshot()
	if snap == nil {
		t.Fatal("Snapshot() = nil, want a non-nil map")
	}
	snap["a"] = 999
	snap["b"] = 1

	again := s.Snapshot()
	want := map[string]int{"a": 1}
	if !reflect.DeepEqual(again, want) {
		t.Errorf("Snapshot() after mutating a previous snapshot = %v, want %v", again, want)
	}
}

func TestWaitAll(t *testing.T) {
	errA := errors.New("task a failed")
	errB := errors.New("task b failed")

	t.Run("mix of success and failure, in order", func(t *testing.T) {
		tasks := []func() error{
			func() error { return nil },
			func() error { return errA },
			func() error { time.Sleep(5 * time.Millisecond); return nil },
			func() error { return errB },
		}
		want := []error{nil, errA, nil, errB}

		got := WaitAll(tasks)
		if !reflect.DeepEqual(got, want) {
			t.Errorf("WaitAll(tasks) = %v, want %v", got, want)
		}
	})

	t.Run("no tasks", func(t *testing.T) {
		got := WaitAll(nil)
		want := []error{}
		if !reflect.DeepEqual(got, want) {
			t.Errorf("WaitAll(nil) = %v, want %v", got, want)
		}
	})
}

func TestDialWithTimeout(t *testing.T) {
	t.Run("dial completes before timeout", func(t *testing.T) {
		addr, err := DialWithTimeout(func() (string, error) {
			return "192.0.2.1:80", nil
		}, 50*time.Millisecond)

		if err != nil || addr != "192.0.2.1:80" {
			t.Errorf("DialWithTimeout() = %q, %v, want %q, nil", addr, err, "192.0.2.1:80")
		}
	})

	t.Run("dial returns its own error before timeout", func(t *testing.T) {
		dialErr := errors.New("connection refused")
		addr, err := DialWithTimeout(func() (string, error) {
			return "", dialErr
		}, 50*time.Millisecond)

		if !errors.Is(err, dialErr) || addr != "" {
			t.Errorf("DialWithTimeout() = %q, %v, want %q, %v", addr, err, "", dialErr)
		}
	})

	t.Run("dial exceeds timeout", func(t *testing.T) {
		addr, err := DialWithTimeout(func() (string, error) {
			time.Sleep(50 * time.Millisecond)
			return "192.0.2.1:80", nil
		}, 5*time.Millisecond)

		if !errors.Is(err, context.DeadlineExceeded) || addr != "" {
			t.Errorf("DialWithTimeout() = %q, %v, want %q, %v", addr, err, "", context.DeadlineExceeded)
		}
	})
}

func TestMergeWithContext(t *testing.T) {
	t.Run("no inputs", func(t *testing.T) {
		got := collectChan(t, MergeWithContext(context.Background()), chanTimeout)
		if len(got) != 0 {
			t.Errorf("MergeWithContext(ctx) = %v, want empty", got)
		}
	})

	t.Run("merges all values when not canceled", func(t *testing.T) {
		a := bufferedIntChan(1, 2, 3)
		b := bufferedIntChan(4, 5)

		got := collectChan(t, MergeWithContext(context.Background(), a, b), chanTimeout)

		sort.Ints(got)
		want := []int{1, 2, 3, 4, 5}
		if !reflect.DeepEqual(got, want) {
			t.Errorf("MergeWithContext merged = %v, want %v (as a set)", got, want)
		}
	})

	t.Run("stops early when ctx is already canceled", func(t *testing.T) {
		ctx, cancel := context.WithCancel(context.Background())
		cancel()

		// a is never sent to or closed: only ctx cancellation can end this.
		a := make(chan int)

		got := collectChan(t, MergeWithContext(ctx, a), chanTimeout)
		if len(got) != 0 {
			t.Errorf("MergeWithContext(canceled ctx, a) = %v, want empty", got)
		}
	})
}
