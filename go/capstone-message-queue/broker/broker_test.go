package broker_test

import (
	"testing"

	"github.com/andersonreyes/learning/go/capstone-message-queue/broker"
)

// ── Partition: basic ops ──────────────────────────────────────────────────────

func TestPartitionOpenAppendRead(t *testing.T) {
	dir := t.TempDir()
	p, err := broker.OpenPartition(dir, "my-topic", 0)
	if err != nil {
		t.Fatalf("OpenPartition: %v", err)
	}
	if p.NextOffset() != 0 {
		t.Fatalf("expected NextOffset=0, got %d", p.NextOffset())
	}
	if p.ID() != 0 {
		t.Fatalf("expected ID=0, got %d", p.ID())
	}

	off, err := p.Append([]byte("hello"))
	if err != nil {
		t.Fatalf("Append: %v", err)
	}
	if off != 0 {
		t.Fatalf("expected offset 0, got %d", off)
	}
	p.Flush()

	got, err := p.Read(0)
	if err != nil {
		t.Fatalf("Read: %v", err)
	}
	if string(got) != "hello" {
		t.Fatalf("expected 'hello', got %q", got)
	}
	if p.NextOffset() != 1 {
		t.Fatalf("expected NextOffset=1, got %d", p.NextOffset())
	}
}

func TestPartitionAppendMultipleAndRead(t *testing.T) {
	dir := t.TempDir()
	p, _ := broker.OpenPartition(dir, "t", 2)
	p.Append([]byte("a"))
	p.Append([]byte("b"))
	p.Append([]byte("c"))
	p.Flush()

	for i, want := range []string{"a", "b", "c"} {
		got, err := p.Read(uint64(i))
		if err != nil {
			t.Fatalf("Read(%d): %v", i, err)
		}
		if string(got) != want {
			t.Fatalf("Read(%d) = %q, want %q", i, got, want)
		}
	}
	if p.NextOffset() != 3 {
		t.Fatalf("expected NextOffset=3, got %d", p.NextOffset())
	}
}

func TestPartitionScanAll(t *testing.T) {
	dir := t.TempDir()
	p, _ := broker.OpenPartition(dir, "t", 0)
	p.Append([]byte("x"))
	p.Append([]byte("y"))
	p.Append([]byte("z"))
	p.Flush()

	records, err := p.ScanAll(0, 0)
	if err != nil {
		t.Fatalf("ScanAll: %v", err)
	}
	if len(records) != 3 {
		t.Fatalf("expected 3 records, got %d", len(records))
	}
	expected := []broker.Record{
		{Offset: 0, Payload: []byte("x")},
		{Offset: 1, Payload: []byte("y")},
		{Offset: 2, Payload: []byte("z")},
	}
	for i, r := range records {
		if r.Offset != expected[i].Offset || string(r.Payload) != string(expected[i].Payload) {
			t.Errorf("record[%d] = {%d, %q}, want {%d, %q}", i, r.Offset, r.Payload, expected[i].Offset, expected[i].Payload)
		}
	}
}

func TestPartitionScanFromMiddle(t *testing.T) {
	dir := t.TempDir()
	p, _ := broker.OpenPartition(dir, "t", 0)
	for i := byte(0); i < 5; i++ {
		p.Append([]byte{i})
	}
	p.Flush()

	records, err := p.ScanAll(3, 0)
	if err != nil {
		t.Fatalf("ScanAll(3): %v", err)
	}
	if len(records) != 2 {
		t.Fatalf("expected 2 records, got %d", len(records))
	}
	if records[0].Offset != 3 {
		t.Fatalf("expected offset 3, got %d", records[0].Offset)
	}
	if records[1].Offset != 4 {
		t.Fatalf("expected offset 4, got %d", records[1].Offset)
	}
}

func TestPartitionReopenRecovery(t *testing.T) {
	dir := t.TempDir()
	{
		p, _ := broker.OpenPartition(dir, "events", 0)
		p.Append([]byte("first"))
		p.Append([]byte("second"))
		p.Flush()
	}
	// Reopen and verify state is recovered.
	p, err := broker.OpenPartition(dir, "events", 0)
	if err != nil {
		t.Fatalf("reopen: %v", err)
	}
	if p.NextOffset() != 2 {
		t.Fatalf("expected NextOffset=2, got %d", p.NextOffset())
	}
	b0, _ := p.Read(0)
	if string(b0) != "first" {
		t.Fatalf("expected 'first', got %q", b0)
	}
	b1, _ := p.Read(1)
	if string(b1) != "second" {
		t.Fatalf("expected 'second', got %q", b1)
	}

	// Can still append after reopen.
	off, _ := p.Append([]byte("third"))
	if off != 2 {
		t.Fatalf("expected offset 2, got %d", off)
	}
	p.Flush()
	b2, _ := p.Read(2)
	if string(b2) != "third" {
		t.Fatalf("expected 'third', got %q", b2)
	}
}

// ── Topic: create, produce, fetch, scan ──────────────────────────────────────

func TestTopicCreateWithNPartitions(t *testing.T) {
	dir := t.TempDir()
	tp, err := broker.OpenTopic(dir, "orders", 3)
	if err != nil {
		t.Fatalf("OpenTopic: %v", err)
	}
	if tp.Name() != "orders" {
		t.Fatalf("expected name 'orders', got %q", tp.Name())
	}
	if tp.NumPartitions() != 3 {
		t.Fatalf("expected 3 partitions, got %d", tp.NumPartitions())
	}
}

func TestTopicProduceWithKeyIsDeterministic(t *testing.T) {
	dir := t.TempDir()
	tp, _ := broker.OpenTopic(dir, "orders", 4)

	pid1, _, _ := tp.Produce([]byte("msg1"), []byte("user-42"))
	pid2, _, _ := tp.Produce([]byte("msg2"), []byte("user-42"))
	pid3, _, _ := tp.Produce([]byte("msg3"), []byte("user-42"))

	if pid1 != pid2 || pid2 != pid3 {
		t.Fatalf("same key should produce same partition: got %d, %d, %d", pid1, pid2, pid3)
	}

	pidOther, _, _ := tp.Produce([]byte("other"), []byte("user-99"))
	if pidOther >= 4 {
		t.Fatalf("partition ID %d out of range", pidOther)
	}
}

func TestTopicProduceWithoutKeyIsRoundRobin(t *testing.T) {
	dir := t.TempDir()
	tp, _ := broker.OpenTopic(dir, "rr", 3)

	var pids []uint32
	for i := byte(0); i < 9; i++ {
		pid, _, err := tp.Produce([]byte{i}, nil)
		if err != nil {
			t.Fatalf("Produce: %v", err)
		}
		pids = append(pids, pid)
	}
	expected := []uint32{0, 1, 2, 0, 1, 2, 0, 1, 2}
	for i, pid := range pids {
		if pid != expected[i] {
			t.Errorf("pids[%d] = %d, want %d", i, pid, expected[i])
		}
	}
}

func TestTopicFetchReturnsCorrectPayload(t *testing.T) {
	dir := t.TempDir()
	tp, _ := broker.OpenTopic(dir, "t", 2)

	pidA, offA, _ := tp.Produce([]byte("payload-a"), []byte("key-a"))
	pidB, offB, _ := tp.Produce([]byte("payload-b"), []byte("key-b"))

	gotA, _ := tp.Fetch(pidA, offA)
	if string(gotA) != "payload-a" {
		t.Fatalf("expected 'payload-a', got %q", gotA)
	}
	gotB, _ := tp.Fetch(pidB, offB)
	if string(gotB) != "payload-b" {
		t.Fatalf("expected 'payload-b', got %q", gotB)
	}
}

func TestTopicScanPartition(t *testing.T) {
	dir := t.TempDir()
	tp, _ := broker.OpenTopic(dir, "events", 1)

	tp.Produce([]byte("e0"), nil)
	tp.Produce([]byte("e1"), nil)
	tp.Produce([]byte("e2"), nil)

	records, err := tp.FetchBatch(0, 0, 0)
	if err != nil {
		t.Fatalf("FetchBatch: %v", err)
	}
	if len(records) != 3 {
		t.Fatalf("expected 3 records, got %d", len(records))
	}
	if string(records[0].Payload) != "e0" {
		t.Fatalf("expected 'e0', got %q", records[0].Payload)
	}
	if string(records[2].Payload) != "e2" {
		t.Fatalf("expected 'e2', got %q", records[2].Payload)
	}
}

func TestTopicNextOffset(t *testing.T) {
	dir := t.TempDir()
	tp, _ := broker.OpenTopic(dir, "t", 1)
	off, _ := tp.NextOffset(0)
	if off != 0 {
		t.Fatalf("expected 0, got %d", off)
	}
	tp.Produce([]byte("msg"), nil)
	tp.Produce([]byte("msg2"), nil)
	off, _ = tp.NextOffset(0)
	if off != 2 {
		t.Fatalf("expected 2, got %d", off)
	}
}

func TestTopicProduceKeyRoutingAcrossPartitions(t *testing.T) {
	dir := t.TempDir()
	tp, _ := broker.OpenTopic(dir, "keyed", 4)

	fnv1a := func(data []byte) uint64 {
		h := uint64(14695981039346656037)
		for _, b := range data {
			h ^= uint64(b)
			h *= 1099511628211
		}
		return h
	}

	keys := [][]byte{[]byte("alpha"), []byte("beta"), []byte("gamma"), []byte("delta"), []byte("epsilon")}
	for _, key := range keys {
		expectedPID := uint32(fnv1a(key) % 4)
		actualPID, _, _ := tp.Produce([]byte("x"), key)
		if actualPID != expectedPID {
			t.Errorf("key %q: expected partition %d, got %d", key, expectedPID, actualPID)
		}
	}
}

// ── Registry: create, get, flush, reopen ─────────────────────────────────────

func TestRegistryCreateTopicAndGet(t *testing.T) {
	dir := t.TempDir()
	reg, err := broker.Open(dir)
	if err != nil {
		t.Fatalf("Open: %v", err)
	}

	if err := reg.CreateTopic("users", 3); err != nil {
		t.Fatalf("CreateTopic: %v", err)
	}

	tp, err := reg.GetTopic("users")
	if err != nil {
		t.Fatalf("GetTopic: %v", err)
	}
	if tp.Name() != "users" {
		t.Fatalf("expected name 'users', got %q", tp.Name())
	}
	if tp.NumPartitions() != 3 {
		t.Fatalf("expected 3 partitions, got %d", tp.NumPartitions())
	}
}

func TestRegistryTopicNamesSorted(t *testing.T) {
	dir := t.TempDir()
	reg, _ := broker.Open(dir)
	reg.CreateTopic("zebra", 1)
	reg.CreateTopic("apple", 2)
	reg.CreateTopic("mango", 3)

	names := reg.TopicNames()
	expected := []string{"apple", "mango", "zebra"}
	if len(names) != len(expected) {
		t.Fatalf("expected %v, got %v", expected, names)
	}
	for i, name := range names {
		if name != expected[i] {
			t.Errorf("names[%d] = %q, want %q", i, name, expected[i])
		}
	}
}

func TestRegistryProduceAndFetch(t *testing.T) {
	dir := t.TempDir()
	reg, _ := broker.Open(dir)
	reg.CreateTopic("orders", 2)

	pid, off, err := reg.Produce("orders", []byte("order-1"), nil)
	if err != nil {
		t.Fatalf("Produce: %v", err)
	}
	got, err := reg.Fetch("orders", pid, off)
	if err != nil {
		t.Fatalf("Fetch: %v", err)
	}
	if string(got) != "order-1" {
		t.Fatalf("expected 'order-1', got %q", got)
	}
}

func TestRegistryFlushAll(t *testing.T) {
	dir := t.TempDir()
	reg, _ := broker.Open(dir)
	reg.CreateTopic("logs", 2)
	reg.Produce("logs", []byte("log1"), nil)
	reg.Produce("logs", []byte("log2"), nil)
	if err := reg.FlushAll(); err != nil {
		t.Fatalf("FlushAll: %v", err)
	}
}

func TestRegistryReopenRecoversTopicsAndOffsets(t *testing.T) {
	dir := t.TempDir()

	// Session 1: create topics, produce messages.
	{
		reg, _ := broker.Open(dir)
		reg.CreateTopic("events", 2)
		reg.CreateTopic("orders", 1)
		reg.Produce("events", []byte("ev0"), nil)
		reg.Produce("events", []byte("ev1"), nil)
		reg.Produce("orders", []byte("ord0"), nil)
		reg.FlushAll()
	}

	// Session 2: reopen, verify everything is recovered.
	{
		reg, err := broker.Open(dir)
		if err != nil {
			t.Fatalf("reopen: %v", err)
		}

		names := reg.TopicNames()
		hasEvents, hasOrders := false, false
		for _, n := range names {
			if n == "events" {
				hasEvents = true
			}
			if n == "orders" {
				hasOrders = true
			}
		}
		if !hasEvents || !hasOrders {
			t.Fatalf("missing topics: %v", names)
		}

		events, _ := reg.GetTopic("events")
		if events.NumPartitions() != 2 {
			t.Fatalf("expected 2 partitions, got %d", events.NumPartitions())
		}
		// The two events are distributed round-robin: partition 0 has "ev0", partition 1 has "ev1".
		off0, _ := events.NextOffset(0)
		if off0 != 1 {
			t.Fatalf("events partition 0: expected next_offset=1, got %d", off0)
		}
		off1, _ := events.NextOffset(1)
		if off1 != 1 {
			t.Fatalf("events partition 1: expected next_offset=1, got %d", off1)
		}
		ev0, _ := events.Fetch(0, 0)
		if string(ev0) != "ev0" {
			t.Fatalf("expected 'ev0', got %q", ev0)
		}
		ev1, _ := events.Fetch(1, 0)
		if string(ev1) != "ev1" {
			t.Fatalf("expected 'ev1', got %q", ev1)
		}

		orders, _ := reg.GetTopic("orders")
		if orders.NumPartitions() != 1 {
			t.Fatalf("expected 1 partition, got %d", orders.NumPartitions())
		}
		ord0, _ := orders.NextOffset(0)
		if ord0 != 1 {
			t.Fatalf("orders partition 0: expected next_offset=1, got %d", ord0)
		}
		o, _ := orders.Fetch(0, 0)
		if string(o) != "ord0" {
			t.Fatalf("expected 'ord0', got %q", o)
		}
	}
}

func TestRegistryCreateTopicIsIdempotent(t *testing.T) {
	dir := t.TempDir()
	reg, _ := broker.Open(dir)
	reg.CreateTopic("dup", 3)
	// Calling again should be OK.
	if err := reg.CreateTopic("dup", 3); err != nil {
		t.Fatalf("second CreateTopic: %v", err)
	}
	names := reg.TopicNames()
	if len(names) != 1 || names[0] != "dup" {
		t.Fatalf("expected ['dup'], got %v", names)
	}
	tp, _ := reg.GetTopic("dup")
	if tp.NumPartitions() != 3 {
		t.Fatalf("expected 3 partitions, got %d", tp.NumPartitions())
	}
}

// ── Error cases ───────────────────────────────────────────────────────────────

func TestFetchFromNonexistentPartitionErrors(t *testing.T) {
	dir := t.TempDir()
	tp, _ := broker.OpenTopic(dir, "t", 2)
	_, err := tp.Fetch(5, 0)
	if err == nil {
		t.Fatal("expected error, got nil")
	}
	if !broker.IsPartitionOutOfRange(err) {
		t.Fatalf("expected ErrPartitionOutOfRange, got %T: %v", err, err)
	}
}

func TestGetNonexistentTopicReturnsError(t *testing.T) {
	dir := t.TempDir()
	reg, _ := broker.Open(dir)
	_, err := reg.GetTopic("nope")
	if err == nil {
		t.Fatal("expected error, got nil")
	}
	if !broker.IsTopicNotFound(err) {
		t.Fatalf("expected ErrTopicNotFound, got %T: %v", err, err)
	}
	if len(reg.TopicNames()) != 0 {
		t.Fatalf("expected empty topic list, got %v", reg.TopicNames())
	}
}
