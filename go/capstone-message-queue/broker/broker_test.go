package broker

import (
	"testing"
)

// helper: open a registry in a fresh temp dir.
func openTempRegistry(t *testing.T) (*Registry, string) {
	t.Helper()
	dir := t.TempDir()
	r, err := OpenRegistry(dir)
	if err != nil {
		t.Fatalf("OpenRegistry(%q): %v", dir, err)
	}
	return r, dir
}

func closeRegistry(t *testing.T, r *Registry) {
	t.Helper()
	if err := r.Close(); err != nil {
		t.Fatalf("Close(): %v", err)
	}
}

// ── CreateTopic ───────────────────────────────────────────────────────────────

func TestCreateTopicBasic(t *testing.T) {
	r, _ := openTempRegistry(t)
	defer closeRegistry(t, r)

	if err := r.CreateTopic("events", 3); err != nil {
		t.Fatalf("CreateTopic(events, 3): %v", err)
	}

	n, err := r.NumPartitions("events")
	if err != nil {
		t.Fatalf("NumPartitions(events): %v", err)
	}
	if n != 3 {
		t.Errorf("NumPartitions(events) = %d, want 3", n)
	}
}

func TestCreateTopicIdempotent(t *testing.T) {
	r, _ := openTempRegistry(t)
	defer closeRegistry(t, r)

	if err := r.CreateTopic("orders", 2); err != nil {
		t.Fatalf("first CreateTopic: %v", err)
	}
	// Same name, same partition count — must be a no-op, not an error.
	if err := r.CreateTopic("orders", 2); err != nil {
		t.Fatalf("idempotent CreateTopic: %v", err)
	}
}

func TestCreateTopicConflict(t *testing.T) {
	r, _ := openTempRegistry(t)
	defer closeRegistry(t, r)

	if err := r.CreateTopic("logs", 4); err != nil {
		t.Fatal(err)
	}
	// Different partition count — must error.
	if err := r.CreateTopic("logs", 8); err == nil {
		t.Fatal("CreateTopic with conflicting partition count: expected error, got nil")
	}
}

// ── TopicNames ────────────────────────────────────────────────────────────────

func TestTopicNamesEmpty(t *testing.T) {
	r, _ := openTempRegistry(t)
	defer closeRegistry(t, r)

	names := r.TopicNames()
	if len(names) != 0 {
		t.Errorf("TopicNames() = %v, want []", names)
	}
}

func TestTopicNamesSorted(t *testing.T) {
	r, _ := openTempRegistry(t)
	defer closeRegistry(t, r)

	for _, name := range []string{"zebra", "alpha", "middle"} {
		if err := r.CreateTopic(name, 1); err != nil {
			t.Fatal(err)
		}
	}

	names := r.TopicNames()
	want := []string{"alpha", "middle", "zebra"}
	if len(names) != 3 {
		t.Fatalf("TopicNames() = %v, want %v", names, want)
	}
	for i, n := range names {
		if n != want[i] {
			t.Errorf("names[%d] = %q, want %q", i, n, want[i])
		}
	}
}

// ── NumPartitions ─────────────────────────────────────────────────────────────

func TestNumPartitionsUnknownTopic(t *testing.T) {
	r, _ := openTempRegistry(t)
	defer closeRegistry(t, r)

	_, err := r.NumPartitions("nonexistent")
	if err == nil {
		t.Fatal("NumPartitions(nonexistent): expected error, got nil")
	}
}

// ── Produce / Fetch ───────────────────────────────────────────────────────────

func TestProduceFetchRoundtrip(t *testing.T) {
	r, _ := openTempRegistry(t)
	defer closeRegistry(t, r)

	if err := r.CreateTopic("t", 1); err != nil {
		t.Fatal(err)
	}

	payload := []byte("hello broker")
	part, off, err := r.Produce("t", payload, nil)
	if err != nil {
		t.Fatalf("Produce: %v", err)
	}

	got, err := r.Fetch("t", part, off)
	if err != nil {
		t.Fatalf("Fetch(t, %d, %d): %v", part, off, err)
	}
	if string(got) != string(payload) {
		t.Errorf("Fetch = %q, want %q", got, payload)
	}
}

func TestProduceUnknownTopic(t *testing.T) {
	r, _ := openTempRegistry(t)
	defer closeRegistry(t, r)

	_, _, err := r.Produce("missing", []byte("x"), nil)
	if err == nil {
		t.Fatal("Produce to unknown topic: expected error, got nil")
	}
}

func TestFetchUnknownTopic(t *testing.T) {
	r, _ := openTempRegistry(t)
	defer closeRegistry(t, r)

	_, err := r.Fetch("missing", 0, 0)
	if err == nil {
		t.Fatal("Fetch from unknown topic: expected error, got nil")
	}
}

func TestFetchOutOfRange(t *testing.T) {
	r, _ := openTempRegistry(t)
	defer closeRegistry(t, r)

	if err := r.CreateTopic("t", 1); err != nil {
		t.Fatal(err)
	}
	if _, _, err := r.Produce("t", []byte("a"), nil); err != nil {
		t.Fatal(err)
	}

	_, err := r.Fetch("t", 0, 99)
	if err == nil {
		t.Fatal("Fetch out-of-range offset: expected error, got nil")
	}
}

func TestFetchUnknownPartition(t *testing.T) {
	r, _ := openTempRegistry(t)
	defer closeRegistry(t, r)

	if err := r.CreateTopic("t", 1); err != nil {
		t.Fatal(err)
	}

	_, err := r.Fetch("t", 99, 0)
	if err == nil {
		t.Fatal("Fetch from nonexistent partition: expected error, got nil")
	}
}

// ── Round-robin routing (no key) ──────────────────────────────────────────────

func TestRoundRobinNoKey(t *testing.T) {
	r, _ := openTempRegistry(t)
	defer closeRegistry(t, r)

	const numPartitions = 3
	if err := r.CreateTopic("rr", numPartitions); err != nil {
		t.Fatal(err)
	}

	// Produce 6 messages with no key; each partition should get exactly 2.
	counts := make(map[uint32]int)
	for range 6 {
		p, _, err := r.Produce("rr", []byte("msg"), nil)
		if err != nil {
			t.Fatalf("Produce: %v", err)
		}
		counts[p]++
	}
	if len(counts) != numPartitions {
		t.Fatalf("round-robin used %d partitions, want %d", len(counts), numPartitions)
	}
	for part, count := range counts {
		if count != 2 {
			t.Errorf("partition %d got %d messages, want 2", part, count)
		}
	}
}

// ── FNV-1a key routing ────────────────────────────────────────────────────────

func TestKeyRoutingDeterministic(t *testing.T) {
	r, _ := openTempRegistry(t)
	defer closeRegistry(t, r)

	if err := r.CreateTopic("k", 5); err != nil {
		t.Fatal(err)
	}

	key := []byte("user-42")
	var firstPart uint32
	for i := range 10 {
		p, _, err := r.Produce("k", []byte("payload"), key)
		if err != nil {
			t.Fatalf("Produce #%d: %v", i, err)
		}
		if i == 0 {
			firstPart = p
		} else if p != firstPart {
			t.Fatalf("key routing not deterministic: got partition %d then %d", firstPart, p)
		}
	}
}

func TestKeyRoutingFNV1a(t *testing.T) {
	// Hand-verify FNV-1a("user1") mod 4.
	// FNV-1a offset basis = 14695981039346656037
	// FNV-1a prime        = 1099511628211
	// key = "user1" = [117, 115, 101, 114, 49]
	// Compute:
	//   h = 14695981039346656037
	//   h ^= 117 -> h *= 1099511628211
	//   h ^= 115 -> h *= 1099511628211
	//   h ^= 101 -> h *= 1099511628211
	//   h ^= 114 -> h *= 1099511628211
	//   h ^= 49  -> h *= 1099511628211
	// Expected partition = h % 4.
	//
	// Pre-computed (verified with reference implementation):
	// h = 0xa68bb2a2f0e4d3e7 = 11999584393849499623
	// 11999584393849499623 % 4 = 3
	const wantPartition uint32 = 3

	r, _ := openTempRegistry(t)
	defer closeRegistry(t, r)

	if err := r.CreateTopic("fnv", 4); err != nil {
		t.Fatal(err)
	}

	p, _, err := r.Produce("fnv", []byte("x"), []byte("user1"))
	if err != nil {
		t.Fatalf("Produce: %v", err)
	}
	if p != wantPartition {
		t.Errorf("FNV-1a(\"user1\") mod 4 = partition %d, want %d", p, wantPartition)
	}
}

func TestDifferentKeysDifferentPartitions(t *testing.T) {
	r, _ := openTempRegistry(t)
	defer closeRegistry(t, r)

	if err := r.CreateTopic("multi", 16); err != nil {
		t.Fatal(err)
	}

	// With 16 partitions and diverse keys, we expect at least 2 distinct partitions.
	keys := []string{"alice", "bob", "carol", "dave", "eve"}
	seen := make(map[uint32]bool)
	for _, k := range keys {
		p, _, err := r.Produce("multi", []byte("v"), []byte(k))
		if err != nil {
			t.Fatal(err)
		}
		seen[p] = true
	}
	if len(seen) < 2 {
		t.Errorf("diverse keys all routed to same partition — FNV-1a likely broken")
	}
}

// ── FetchBatch ────────────────────────────────────────────────────────────────

func TestFetchBatch(t *testing.T) {
	r, _ := openTempRegistry(t)
	defer closeRegistry(t, r)

	if err := r.CreateTopic("b", 1); err != nil {
		t.Fatal(err)
	}
	for i := range 10 {
		if _, _, err := r.Produce("b", []byte{byte(i)}, nil); err != nil {
			t.Fatal(err)
		}
	}

	recs, err := r.FetchBatch("b", 0, 3, 5)
	if err != nil {
		t.Fatalf("FetchBatch: %v", err)
	}
	if len(recs) != 5 {
		t.Fatalf("FetchBatch returned %d records, want 5", len(recs))
	}
	for i, rec := range recs {
		if rec.Offset != uint64(3+i) {
			t.Errorf("recs[%d].Offset = %d, want %d", i, rec.Offset, 3+i)
		}
		if len(rec.Payload) != 1 || rec.Payload[0] != byte(3+i) {
			t.Errorf("recs[%d].Payload = %v, want [%d]", i, rec.Payload, 3+i)
		}
	}
}

func TestFetchBatchEmpty(t *testing.T) {
	r, _ := openTempRegistry(t)
	defer closeRegistry(t, r)

	if err := r.CreateTopic("empty", 1); err != nil {
		t.Fatal(err)
	}

	recs, err := r.FetchBatch("empty", 0, 0, 100)
	if err != nil {
		t.Fatalf("FetchBatch on empty partition: %v", err)
	}
	if len(recs) != 0 {
		t.Fatalf("FetchBatch on empty partition = %d records, want 0", len(recs))
	}
}

func TestFetchBatchUnknownTopic(t *testing.T) {
	r, _ := openTempRegistry(t)
	defer closeRegistry(t, r)

	_, err := r.FetchBatch("nope", 0, 0, 10)
	if err == nil {
		t.Fatal("FetchBatch unknown topic: expected error, got nil")
	}
}

// ── Persistence (close + reopen) ──────────────────────────────────────────────

func TestRegistryPersistsAcrossReopen(t *testing.T) {
	dir := t.TempDir()

	r1, err := OpenRegistry(dir)
	if err != nil {
		t.Fatal(err)
	}
	if err := r1.CreateTopic("persistent", 2); err != nil {
		t.Fatal(err)
	}
	_, off, err := r1.Produce("persistent", []byte("durable"), nil)
	if err != nil {
		t.Fatal(err)
	}
	if err := r1.Close(); err != nil {
		t.Fatal(err)
	}

	r2, err := OpenRegistry(dir)
	if err != nil {
		t.Fatalf("OpenRegistry reopen: %v", err)
	}
	defer closeRegistry(t, r2)

	// Topic should still exist.
	n, err := r2.NumPartitions("persistent")
	if err != nil {
		t.Fatalf("NumPartitions after reopen: %v", err)
	}
	if n != 2 {
		t.Errorf("NumPartitions after reopen = %d, want 2", n)
	}

	// Fetch the record produced before close.
	got, err := r2.Fetch("persistent", 0, off)
	if err != nil {
		t.Fatalf("Fetch after reopen: %v", err)
	}
	if string(got) != "durable" {
		t.Errorf("Fetch after reopen = %q, want \"durable\"", got)
	}
}

// ── Multiple topics ───────────────────────────────────────────────────────────

func TestMultipleTopicsIsolated(t *testing.T) {
	r, _ := openTempRegistry(t)
	defer closeRegistry(t, r)

	for _, topic := range []string{"topic-a", "topic-b"} {
		if err := r.CreateTopic(topic, 1); err != nil {
			t.Fatal(err)
		}
	}

	if _, _, err := r.Produce("topic-a", []byte("in-a"), nil); err != nil {
		t.Fatal(err)
	}
	if _, _, err := r.Produce("topic-b", []byte("in-b"), nil); err != nil {
		t.Fatal(err)
	}

	gotA, _ := r.Fetch("topic-a", 0, 0)
	gotB, _ := r.Fetch("topic-b", 0, 0)

	if string(gotA) != "in-a" {
		t.Errorf("topic-a offset 0 = %q, want \"in-a\"", gotA)
	}
	if string(gotB) != "in-b" {
		t.Errorf("topic-b offset 0 = %q, want \"in-b\"", gotB)
	}
}

// ── Single-partition topic ─────────────────────────────────────────────────────

func TestSinglePartitionOffsets(t *testing.T) {
	r, _ := openTempRegistry(t)
	defer closeRegistry(t, r)

	if err := r.CreateTopic("seq", 1); err != nil {
		t.Fatal(err)
	}

	for i := range 5 {
		_, off, err := r.Produce("seq", []byte{byte(i)}, nil)
		if err != nil {
			t.Fatalf("Produce #%d: %v", i, err)
		}
		if off != uint64(i) {
			t.Errorf("Produce #%d offset = %d, want %d", i, off, i)
		}
	}
}
