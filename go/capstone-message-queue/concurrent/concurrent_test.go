package concurrent_test

import (
	"sync"
	"testing"
	"time"

	"github.com/andersonreyes/learning/go/capstone-message-queue/broker"
	"github.com/andersonreyes/learning/go/capstone-message-queue/concurrent"
)

// ── helpers ───────────────────────────────────────────────────────────────────

func makeShared(t *testing.T, topics []struct{ name string; parts uint32 }) *concurrent.SharedRegistry {
	t.Helper()
	dir := t.TempDir()
	reg, err := broker.Open(dir)
	if err != nil {
		t.Fatalf("Open: %v", err)
	}
	for _, tp := range topics {
		if err := reg.CreateTopic(tp.name, tp.parts); err != nil {
			t.Fatalf("CreateTopic: %v", err)
		}
	}
	sr := concurrent.New(reg, 50*time.Millisecond)
	t.Cleanup(sr.Close)
	return sr
}

func totalOffsets(t *testing.T, sr *concurrent.SharedRegistry, topic string, numPartitions uint32) uint64 {
	t.Helper()
	var total uint64
	for p := uint32(0); p < numPartitions; p++ {
		off, err := sr.NextOffset(topic, p)
		if err != nil {
			t.Fatalf("NextOffset(%s, %d): %v", topic, p, err)
		}
		total += off
	}
	return total
}

// ── single-threaded API sanity ─────────────────────────────────────────────────

func TestCreateTopicAndProduceFetch(t *testing.T) {
	dir := t.TempDir()
	reg, _ := broker.Open(dir)
	sr := concurrent.New(reg, 100*time.Millisecond)
	defer sr.Close()

	if err := sr.CreateTopic("greetings", 1); err != nil {
		t.Fatalf("CreateTopic: %v", err)
	}
	pid, off, err := sr.Produce("greetings", []byte("hello"), nil)
	if err != nil {
		t.Fatalf("Produce: %v", err)
	}
	if pid != 0 || off != 0 {
		t.Fatalf("expected (0, 0), got (%d, %d)", pid, off)
	}
	got, err := sr.Fetch("greetings", 0, 0)
	if err != nil {
		t.Fatalf("Fetch: %v", err)
	}
	if string(got) != "hello" {
		t.Fatalf("expected 'hello', got %q", got)
	}
}

func TestNumPartitionsReflectsCreation(t *testing.T) {
	dir := t.TempDir()
	reg, _ := broker.Open(dir)
	sr := concurrent.New(reg, 100*time.Millisecond)
	defer sr.Close()

	sr.CreateTopic("t", 3)
	n, err := sr.NumPartitions("t")
	if err != nil {
		t.Fatalf("NumPartitions: %v", err)
	}
	if n != 3 {
		t.Fatalf("expected 3, got %d", n)
	}
}

func TestTopicNamesListsAllTopics(t *testing.T) {
	dir := t.TempDir()
	reg, _ := broker.Open(dir)
	sr := concurrent.New(reg, 100*time.Millisecond)
	defer sr.Close()

	sr.CreateTopic("beta", 1)
	sr.CreateTopic("alpha", 2)

	names := sr.TopicNames()
	if len(names) != 2 || names[0] != "alpha" || names[1] != "beta" {
		t.Fatalf("expected ['alpha', 'beta'], got %v", names)
	}
}

func TestProduceUnknownTopicErrors(t *testing.T) {
	dir := t.TempDir()
	reg, _ := broker.Open(dir)
	sr := concurrent.New(reg, 100*time.Millisecond)
	defer sr.Close()

	if _, _, err := sr.Produce("nope", []byte("x"), nil); err == nil {
		t.Fatal("expected error, got nil")
	}
}

func TestFetchUnknownTopicErrors(t *testing.T) {
	dir := t.TempDir()
	reg, _ := broker.Open(dir)
	sr := concurrent.New(reg, 100*time.Millisecond)
	defer sr.Close()

	if _, err := sr.Fetch("nope", 0, 0); err == nil {
		t.Fatal("expected error, got nil")
	}
}

// ── FetchBatch ────────────────────────────────────────────────────────────────

func TestFetchBatchReturnsUpToMaxCount(t *testing.T) {
	sr := makeShared(t, []struct{ name string; parts uint32 }{{"batch", 1}})
	for i := uint32(0); i < 50; i++ {
		b := make([]byte, 4)
		b[0] = byte(i >> 24); b[1] = byte(i >> 16); b[2] = byte(i >> 8); b[3] = byte(i)
		sr.Produce("batch", b, nil)
	}
	batch, err := sr.FetchBatch("batch", 0, 10, 20)
	if err != nil {
		t.Fatalf("FetchBatch: %v", err)
	}
	if len(batch) != 20 {
		t.Fatalf("expected 20 records, got %d", len(batch))
	}
	if batch[0].Offset != 10 {
		t.Fatalf("expected offset 10, got %d", batch[0].Offset)
	}
	if batch[19].Offset != 29 {
		t.Fatalf("expected offset 29, got %d", batch[19].Offset)
	}
}

func TestFetchBatchAtEndReturnsFewer(t *testing.T) {
	sr := makeShared(t, []struct{ name string; parts uint32 }{{"few", 1}})
	for i := uint32(0); i < 5; i++ {
		b := make([]byte, 4)
		b[3] = byte(i)
		sr.Produce("few", b, nil)
	}
	batch, err := sr.FetchBatch("few", 0, 3, 100)
	if err != nil {
		t.Fatalf("FetchBatch: %v", err)
	}
	if len(batch) != 2 {
		t.Fatalf("expected 2 records, got %d", len(batch))
	}
}

func TestFetchBatchEmptyTopic(t *testing.T) {
	sr := makeShared(t, []struct{ name string; parts uint32 }{{"empty", 1}})
	batch, err := sr.FetchBatch("empty", 0, 0, 10)
	if err != nil {
		t.Fatalf("FetchBatch: %v", err)
	}
	if len(batch) != 0 {
		t.Fatalf("expected empty, got %d records", len(batch))
	}
}

// ── concurrent produce ────────────────────────────────────────────────────────

func TestConcurrentProducesSinglePartition(t *testing.T) {
	sr := makeShared(t, []struct{ name string; parts uint32 }{{"events", 1}})

	var wg sync.WaitGroup
	for i := uint32(0); i < 4; i++ {
		wg.Add(1)
		go func(i uint32) {
			defer wg.Done()
			for j := uint32(0); j < 50; j++ {
				payload := []byte{byte(i), byte(j)}
				key := []byte{byte(i >> 24), byte(i >> 16), byte(i >> 8), byte(i)}
				if _, _, err := sr.Produce("events", payload, key); err != nil {
					t.Errorf("Produce: %v", err)
				}
			}
		}(i)
	}
	wg.Wait()

	// 4 threads × 50 = 200 messages, all hashed to partition 0 (only 1 partition).
	off, err := sr.NextOffset("events", 0)
	if err != nil {
		t.Fatalf("NextOffset: %v", err)
	}
	if off != 200 {
		t.Fatalf("expected 200, got %d", off)
	}
}

func TestConcurrentProducesSpreadAcrossPartitions(t *testing.T) {
	sr := makeShared(t, []struct{ name string; parts uint32 }{{"orders", 4}})

	var wg sync.WaitGroup
	for i := uint32(0); i < 8; i++ {
		wg.Add(1)
		go func(i uint32) {
			defer wg.Done()
			for j := uint32(0); j < 25; j++ {
				payload := []byte{byte(i), byte(j)}
				if _, _, err := sr.Produce("orders", payload, nil); err != nil {
					t.Errorf("Produce: %v", err)
				}
			}
		}(i)
	}
	wg.Wait()

	// 8 × 25 = 200 total across 4 partitions.
	total := totalOffsets(t, sr, "orders", 4)
	if total != 200 {
		t.Fatalf("expected 200 total, got %d", total)
	}
}

func TestConcurrentProduceAndFetchAfterJoin(t *testing.T) {
	sr := makeShared(t, []struct{ name string; parts uint32 }{{"log", 1}})

	producer := make(chan struct{})
	go func() {
		defer close(producer)
		for i := uint32(0); i < 100; i++ {
			b := make([]byte, 4)
			b[0] = byte(i >> 24); b[1] = byte(i >> 16); b[2] = byte(i >> 8); b[3] = byte(i)
			if _, _, err := sr.Produce("log", b, nil); err != nil {
				t.Errorf("Produce: %v", err)
			}
		}
	}()
	<-producer

	// After the producer finishes, all 100 messages must be fetchable in order.
	for i := uint64(0); i < 100; i++ {
		payload, err := sr.Fetch("log", 0, i)
		if err != nil {
			t.Fatalf("Fetch(%d): %v", i, err)
		}
		val := uint32(payload[0])<<24 | uint32(payload[1])<<16 | uint32(payload[2])<<8 | uint32(payload[3])
		if uint64(val) != i {
			t.Fatalf("message at offset %d has wrong value %d", i, val)
		}
	}
}

func TestConcurrentReadersWhileProducing(t *testing.T) {
	sr := makeShared(t, []struct{ name string; parts uint32 }{{"mixed", 1}})

	// Pre-write 50 records.
	for i := uint32(0); i < 50; i++ {
		b := make([]byte, 4)
		b[0] = byte(i >> 24); b[1] = byte(i >> 16); b[2] = byte(i >> 8); b[3] = byte(i)
		sr.Produce("mixed", b, nil)
	}

	writerDone := make(chan struct{})
	go func() {
		defer close(writerDone)
		for i := uint32(50); i < 150; i++ {
			b := make([]byte, 4)
			b[0] = byte(i >> 24); b[1] = byte(i >> 16); b[2] = byte(i >> 8); b[3] = byte(i)
			sr.Produce("mixed", b, nil)
		}
	}()

	var readerWg sync.WaitGroup
	for r := 0; r < 2; r++ {
		readerWg.Add(1)
		go func() {
			defer readerWg.Done()
			batch, err := sr.FetchBatch("mixed", 0, 0, 50)
			if err != nil {
				t.Errorf("FetchBatch: %v", err)
				return
			}
			for _, rec := range batch {
				val := uint32(rec.Payload[0])<<24 | uint32(rec.Payload[1])<<16 | uint32(rec.Payload[2])<<8 | uint32(rec.Payload[3])
				if uint64(val) != rec.Offset {
					t.Errorf("offset %d has wrong value %d", rec.Offset, val)
				}
			}
		}()
	}

	<-writerDone
	readerWg.Wait()

	off, _ := sr.NextOffset("mixed", 0)
	if off != 150 {
		t.Fatalf("expected 150, got %d", off)
	}
}

// ── background flush goroutine ────────────────────────────────────────────────

func TestFlushGoroutineDoesNotDeadlock(t *testing.T) {
	sr := makeShared(t, []struct{ name string; parts uint32 }{{"t", 1}})

	for i := uint32(0); i < 20; i++ {
		b := make([]byte, 4)
		b[3] = byte(i)
		sr.Produce("t", b, nil)
	}
	// Sleep past two flush intervals; the flush goroutine must not deadlock.
	time.Sleep(150 * time.Millisecond)
	off, err := sr.NextOffset("t", 0)
	if err != nil {
		t.Fatalf("NextOffset: %v", err)
	}
	if off != 20 {
		t.Fatalf("expected 20, got %d", off)
	}
}

func TestShutdownJoinsFlushGoroutineWithoutHanging(t *testing.T) {
	dir := t.TempDir()
	reg, _ := broker.Open(dir)
	sr := concurrent.New(reg, 50*time.Millisecond)
	sr.CreateTopic("x", 1)
	sr.Produce("x", []byte("data"), nil)
	sr.Close() // must return promptly
}

func TestExplicitFlushAllVisibleAfter(t *testing.T) {
	sr := makeShared(t, []struct{ name string; parts uint32 }{{"f", 1}})
	sr.Produce("f", []byte("msg"), nil)
	if err := sr.FlushAll(); err != nil {
		t.Fatalf("FlushAll: %v", err)
	}
	got, err := sr.Fetch("f", 0, 0)
	if err != nil {
		t.Fatalf("Fetch: %v", err)
	}
	if string(got) != "msg" {
		t.Fatalf("expected 'msg', got %q", got)
	}
}
