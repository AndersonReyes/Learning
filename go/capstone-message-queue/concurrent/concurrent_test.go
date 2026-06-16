package concurrent

import (
	"fmt"
	"sync"
	"testing"
	"time"

	"github.com/andersonreyes/learning/go/capstone-message-queue/broker"
)

// helper: open an underlying registry in a temp dir.
func openBroker(t *testing.T) (*broker.Registry, string) {
	t.Helper()
	dir := t.TempDir()
	r, err := broker.OpenRegistry(dir)
	if err != nil {
		t.Fatalf("broker.OpenRegistry: %v", err)
	}
	return r, dir
}

// ── Basic delegation ──────────────────────────────────────────────────────────

func TestCreateTopicAndProduce(t *testing.T) {
	reg, _ := openBroker(t)
	s := NewSharedRegistry(reg, time.Hour) // long interval — flush goroutine won't fire
	defer func() {
		if err := s.Close(); err != nil {
			t.Fatalf("Close(): %v", err)
		}
	}()

	if err := s.CreateTopic("t", 2); err != nil {
		t.Fatalf("CreateTopic: %v", err)
	}

	_, off, err := s.Produce("t", []byte("hello"), nil)
	if err != nil {
		t.Fatalf("Produce: %v", err)
	}
	if off != 0 {
		t.Errorf("first produce offset = %d, want 0", off)
	}
}

func TestFetchAfterProduce(t *testing.T) {
	reg, _ := openBroker(t)
	s := NewSharedRegistry(reg, time.Hour)
	defer s.Close() //nolint:errcheck

	if err := s.CreateTopic("t", 1); err != nil {
		t.Fatal(err)
	}

	payload := []byte("world")
	part, off, err := s.Produce("t", payload, nil)
	if err != nil {
		t.Fatal(err)
	}

	got, err := s.Fetch("t", part, off)
	if err != nil {
		t.Fatalf("Fetch: %v", err)
	}
	if string(got) != string(payload) {
		t.Errorf("Fetch = %q, want %q", got, payload)
	}
}

func TestTopicNamesAndNumPartitions(t *testing.T) {
	reg, _ := openBroker(t)
	s := NewSharedRegistry(reg, time.Hour)
	defer s.Close() //nolint:errcheck

	if err := s.CreateTopic("aaa", 3); err != nil {
		t.Fatal(err)
	}
	if err := s.CreateTopic("bbb", 1); err != nil {
		t.Fatal(err)
	}

	names := s.TopicNames()
	if len(names) != 2 || names[0] != "aaa" || names[1] != "bbb" {
		t.Errorf("TopicNames() = %v, want [aaa bbb]", names)
	}

	n, err := s.NumPartitions("aaa")
	if err != nil {
		t.Fatal(err)
	}
	if n != 3 {
		t.Errorf("NumPartitions(aaa) = %d, want 3", n)
	}
}

// ── Concurrent producers ──────────────────────────────────────────────────────

func TestConcurrentProducers(t *testing.T) {
	reg, _ := openBroker(t)
	s := NewSharedRegistry(reg, time.Hour)
	defer s.Close() //nolint:errcheck

	if err := s.CreateTopic("concurrent", 4); err != nil {
		t.Fatal(err)
	}

	const goroutines = 10
	const msgsPerGoroutine = 50
	var wg sync.WaitGroup
	errCh := make(chan error, goroutines*msgsPerGoroutine)

	for g := range goroutines {
		wg.Add(1)
		go func(g int) {
			defer wg.Done()
			for m := range msgsPerGoroutine {
				payload := []byte(fmt.Sprintf("g%d-m%d", g, m))
				_, _, err := s.Produce("concurrent", payload, nil)
				if err != nil {
					errCh <- err
				}
			}
		}(g)
	}
	wg.Wait()
	close(errCh)

	for err := range errCh {
		t.Errorf("concurrent Produce error: %v", err)
	}
}

// ── Concurrent readers during writes ─────────────────────────────────────────
// Run with: go test -race ./concurrent/

func TestConcurrentReadsDuringWrites(t *testing.T) {
	reg, _ := openBroker(t)
	s := NewSharedRegistry(reg, time.Hour)
	defer s.Close() //nolint:errcheck

	if err := s.CreateTopic("rw", 2); err != nil {
		t.Fatal(err)
	}

	// Seed one record so reads don't always fail.
	if _, _, err := s.Produce("rw", []byte("seed"), nil); err != nil {
		t.Fatal(err)
	}

	var wg sync.WaitGroup
	stop := make(chan struct{})

	// Writers.
	for range 4 {
		wg.Add(1)
		go func() {
			defer wg.Done()
			for {
				select {
				case <-stop:
					return
				default:
					s.Produce("rw", []byte("msg"), nil) //nolint:errcheck
				}
			}
		}()
	}

	// Readers.
	for range 4 {
		wg.Add(1)
		go func() {
			defer wg.Done()
			for {
				select {
				case <-stop:
					return
				default:
					s.TopicNames()
					s.NumPartitions("rw") //nolint:errcheck
					s.FetchBatch("rw", 0, 0, 5) //nolint:errcheck
				}
			}
		}()
	}

	// Let them race for a short time.
	time.Sleep(50 * time.Millisecond)
	close(stop)
	wg.Wait()
	// If the race detector trips, the test fails automatically.
}

// ── Flush goroutine ───────────────────────────────────────────────────────────

func TestFlushGoroutineRuns(t *testing.T) {
	reg, _ := openBroker(t)
	// Very short interval — flush goroutine should fire multiple times.
	s := NewSharedRegistry(reg, 5*time.Millisecond)

	if err := s.CreateTopic("fl", 1); err != nil {
		t.Fatal(err)
	}
	if _, _, err := s.Produce("fl", []byte("data"), nil); err != nil {
		t.Fatal(err)
	}

	// Give the flush goroutine time to fire.
	time.Sleep(30 * time.Millisecond)

	// Close must succeed (goroutine has been running without crashing).
	if err := s.Close(); err != nil {
		t.Fatalf("Close() after flush goroutine ran: %v", err)
	}
}

// ── Close waits for goroutine ─────────────────────────────────────────────────

func TestCloseWaitsForFlushGoroutine(t *testing.T) {
	reg, _ := openBroker(t)
	s := NewSharedRegistry(reg, time.Hour)

	// Close must complete without hanging.
	done := make(chan struct{})
	go func() {
		s.Close() //nolint:errcheck
		close(done)
	}()

	select {
	case <-done:
		// Good — Close returned promptly.
	case <-time.After(3 * time.Second):
		t.Fatal("Close() timed out — flush goroutine may not be stopping")
	}
}

func TestCloseIdempotent(t *testing.T) {
	reg, _ := openBroker(t)
	s := NewSharedRegistry(reg, time.Hour)

	if err := s.Close(); err != nil {
		t.Fatalf("first Close(): %v", err)
	}
	// A second close should not panic or deadlock — it may return an error.
	// We just ensure it doesn't hang.
	done := make(chan error, 1)
	go func() { done <- s.Close() }()
	select {
	case <-done:
	case <-time.After(2 * time.Second):
		t.Fatal("second Close() timed out")
	}
}

// ── FetchBatch delegation ─────────────────────────────────────────────────────

func TestFetchBatch(t *testing.T) {
	reg, _ := openBroker(t)
	s := NewSharedRegistry(reg, time.Hour)
	defer s.Close() //nolint:errcheck

	if err := s.CreateTopic("fb", 1); err != nil {
		t.Fatal(err)
	}
	for i := range 8 {
		if _, _, err := s.Produce("fb", []byte{byte(i)}, nil); err != nil {
			t.Fatal(err)
		}
	}

	recs, err := s.FetchBatch("fb", 0, 2, 4)
	if err != nil {
		t.Fatalf("FetchBatch: %v", err)
	}
	if len(recs) != 4 {
		t.Fatalf("FetchBatch returned %d records, want 4", len(recs))
	}
	for i, r := range recs {
		if r.Offset != uint64(2+i) {
			t.Errorf("recs[%d].Offset = %d, want %d", i, r.Offset, 2+i)
		}
	}
}
