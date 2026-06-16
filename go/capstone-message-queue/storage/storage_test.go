package storage_test

import (
	"testing"

	"github.com/andersonreyes/learning/go/capstone-message-queue/storage"
)

func tempLog(t *testing.T) *storage.Log {
	t.Helper()
	dir := t.TempDir()
	log, err := storage.Open(dir)
	if err != nil {
		t.Fatalf("Open: %v", err)
	}
	t.Cleanup(func() { log.Close() })
	return log
}

// ── append / NextOffset ───────────────────────────────────────────────────────

func TestOffsetsStartAtZero(t *testing.T) {
	log := tempLog(t)
	if log.NextOffset() != 0 {
		t.Fatalf("expected NextOffset=0, got %d", log.NextOffset())
	}
}

func TestAppendReturnsSequentialOffsets(t *testing.T) {
	log := tempLog(t)
	for i, b := range [][]byte{[]byte("a"), []byte("b"), []byte("c")} {
		off, err := log.Append(b)
		if err != nil {
			t.Fatalf("Append: %v", err)
		}
		if off != uint64(i) {
			t.Fatalf("expected offset %d, got %d", i, off)
		}
	}
	if log.NextOffset() != 3 {
		t.Fatalf("expected NextOffset=3, got %d", log.NextOffset())
	}
}

func TestAppendEmptyPayload(t *testing.T) {
	log := tempLog(t)
	off, err := log.Append([]byte{})
	if err != nil {
		t.Fatalf("Append empty: %v", err)
	}
	log.Flush()
	payload, err := log.Read(off)
	if err != nil {
		t.Fatalf("Read empty: %v", err)
	}
	if len(payload) != 0 {
		t.Fatalf("expected empty payload, got %v", payload)
	}
}

func TestAppendLargePayload(t *testing.T) {
	log := tempLog(t)
	big := make([]byte, 64*1024)
	for i := range big {
		big[i] = 0xAB
	}
	off, err := log.Append(big)
	if err != nil {
		t.Fatalf("Append large: %v", err)
	}
	log.Flush()
	got, err := log.Read(off)
	if err != nil {
		t.Fatalf("Read large: %v", err)
	}
	if len(got) != len(big) {
		t.Fatalf("length mismatch: got %d, want %d", len(got), len(big))
	}
	for i, b := range got {
		if b != big[i] {
			t.Fatalf("payload mismatch at index %d", i)
		}
	}
}

// ── Read ──────────────────────────────────────────────────────────────────────

func TestReadFirstRecord(t *testing.T) {
	log := tempLog(t)
	log.Append([]byte("hello"))
	log.Flush()
	got, err := log.Read(0)
	if err != nil {
		t.Fatalf("Read(0): %v", err)
	}
	if string(got) != "hello" {
		t.Fatalf("expected 'hello', got %q", got)
	}
}

func TestReadMiddleRecord(t *testing.T) {
	log := tempLog(t)
	log.Append([]byte("first"))
	log.Append([]byte("second"))
	log.Append([]byte("third"))
	log.Flush()
	got, err := log.Read(1)
	if err != nil {
		t.Fatalf("Read(1): %v", err)
	}
	if string(got) != "second" {
		t.Fatalf("expected 'second', got %q", got)
	}
}

func TestReadLastRecord(t *testing.T) {
	log := tempLog(t)
	for i := byte(0); i < 10; i++ {
		log.Append([]byte{i})
	}
	log.Flush()
	got, err := log.Read(9)
	if err != nil {
		t.Fatalf("Read(9): %v", err)
	}
	if len(got) != 1 || got[0] != 9 {
		t.Fatalf("expected [9], got %v", got)
	}
}

func TestReadOutOfRangeReturnsError(t *testing.T) {
	log := tempLog(t)
	log.Append([]byte("x"))
	log.Flush()
	if _, err := log.Read(1); err == nil {
		t.Fatal("expected error for Read(1), got nil")
	}
	if _, err := log.Read(100); err == nil {
		t.Fatal("expected error for Read(100), got nil")
	}
}

func TestReadEmptyLogReturnsError(t *testing.T) {
	log := tempLog(t)
	if _, err := log.Read(0); err == nil {
		t.Fatal("expected error reading empty log, got nil")
	}
}

// ── ScanAll ───────────────────────────────────────────────────────────────────

func TestScanAllRecords(t *testing.T) {
	log := tempLog(t)
	messages := [][]byte{[]byte("alpha"), []byte("beta"), []byte("gamma")}
	for _, m := range messages {
		log.Append(m)
	}
	log.Flush()

	records, err := log.ScanAll(0, 0)
	if err != nil {
		t.Fatalf("ScanAll: %v", err)
	}
	if len(records) != 3 {
		t.Fatalf("expected 3 records, got %d", len(records))
	}
	for i, r := range records {
		if r.Offset != uint64(i) {
			t.Errorf("record[%d].Offset = %d, want %d", i, r.Offset, i)
		}
		if string(r.Payload) != string(messages[i]) {
			t.Errorf("record[%d].Payload = %q, want %q", i, r.Payload, messages[i])
		}
	}
}

func TestScanFromMiddle(t *testing.T) {
	log := tempLog(t)
	for i := byte(0); i < 5; i++ {
		log.Append([]byte{i})
	}
	log.Flush()

	records, err := log.ScanAll(2, 0)
	if err != nil {
		t.Fatalf("ScanAll(2): %v", err)
	}
	if len(records) != 3 {
		t.Fatalf("expected 3 records, got %d", len(records))
	}
	if records[0].Offset != 2 {
		t.Fatalf("expected offset 2, got %d", records[0].Offset)
	}
	if records[2].Offset != 4 {
		t.Fatalf("expected offset 4, got %d", records[2].Offset)
	}
}

func TestScanEmptyLog(t *testing.T) {
	log := tempLog(t)
	records, err := log.ScanAll(0, 0)
	if err != nil {
		t.Fatalf("ScanAll on empty: %v", err)
	}
	if len(records) != 0 {
		t.Fatalf("expected empty, got %d records", len(records))
	}
}

// ── sparse index: across the INDEX_INTERVAL boundary (64 records) ─────────────

func TestReadRecordBeyondIndexInterval(t *testing.T) {
	log := tempLog(t)
	// Append 100 records so the index fires at offsets 0 and 64.
	for i := uint64(0); i < 100; i++ {
		b := make([]byte, 8)
		for j := 0; j < 8; j++ {
			b[7-j] = byte(i >> (j * 8))
		}
		log.Append(b)
	}
	log.Flush()

	// Read a record that's between the two index entries.
	payload, err := log.Read(75)
	if err != nil {
		t.Fatalf("Read(75): %v", err)
	}
	var val uint64
	for _, b := range payload {
		val = val<<8 | uint64(b)
	}
	if val != 75 {
		t.Fatalf("expected 75, got %d", val)
	}
}

func TestScanAcrossIndexBoundary(t *testing.T) {
	log := tempLog(t)
	for i := uint64(0); i < 130; i++ {
		b := make([]byte, 8)
		for j := 0; j < 8; j++ {
			b[7-j] = byte(i >> (j * 8))
		}
		log.Append(b)
	}
	log.Flush()

	// Start scan past the second index entry at offset 128.
	records, err := log.ScanAll(120, 0)
	if err != nil {
		t.Fatalf("ScanAll(120): %v", err)
	}
	if len(records) != 10 {
		t.Fatalf("expected 10 records, got %d", len(records))
	}
	if records[0].Offset != 120 {
		t.Fatalf("expected offset 120, got %d", records[0].Offset)
	}
	if records[9].Offset != 129 {
		t.Fatalf("expected offset 129, got %d", records[9].Offset)
	}
}

// ── reopen / recovery ─────────────────────────────────────────────────────────

func TestReopenRecoversNextOffset(t *testing.T) {
	dir := t.TempDir()
	{
		log, err := storage.Open(dir)
		if err != nil {
			t.Fatalf("Open: %v", err)
		}
		log.Append([]byte("first"))
		log.Append([]byte("second"))
		log.Flush()
		log.Close()
	}
	log, err := storage.Open(dir)
	if err != nil {
		t.Fatalf("Reopen: %v", err)
	}
	defer log.Close()
	if log.NextOffset() != 2 {
		t.Fatalf("expected NextOffset=2, got %d", log.NextOffset())
	}
}

func TestReopenCanReadExistingRecords(t *testing.T) {
	dir := t.TempDir()
	{
		log, _ := storage.Open(dir)
		log.Append([]byte("persist me"))
		log.Flush()
		log.Close()
	}
	log, _ := storage.Open(dir)
	defer log.Close()
	got, err := log.Read(0)
	if err != nil {
		t.Fatalf("Read after reopen: %v", err)
	}
	if string(got) != "persist me" {
		t.Fatalf("expected 'persist me', got %q", got)
	}
}

func TestReopenContinuesAppending(t *testing.T) {
	dir := t.TempDir()
	{
		log, _ := storage.Open(dir)
		log.Append([]byte("before"))
		log.Flush()
		log.Close()
	}
	{
		log, _ := storage.Open(dir)
		defer log.Close()
		off, err := log.Append([]byte("after"))
		if err != nil {
			t.Fatalf("Append after reopen: %v", err)
		}
		log.Flush()
		if off != 1 {
			t.Fatalf("expected offset 1, got %d", off)
		}
		b0, _ := log.Read(0)
		if string(b0) != "before" {
			t.Fatalf("expected 'before', got %q", b0)
		}
		b1, _ := log.Read(1)
		if string(b1) != "after" {
			t.Fatalf("expected 'after', got %q", b1)
		}
	}
}

func TestReopenAcrossIndexBoundary(t *testing.T) {
	dir := t.TempDir()
	{
		log, _ := storage.Open(dir)
		for i := uint64(0); i < 70; i++ {
			b := make([]byte, 8)
			for j := 0; j < 8; j++ {
				b[7-j] = byte(i >> (j * 8))
			}
			log.Append(b)
		}
		log.Flush()
		log.Close()
	}
	{
		log, _ := storage.Open(dir)
		defer log.Close()
		if log.NextOffset() != 70 {
			t.Fatalf("expected NextOffset=70, got %d", log.NextOffset())
		}
		off, _ := log.Append([]byte("new"))
		if off != 70 {
			t.Fatalf("expected offset 70, got %d", off)
		}
		log.Flush()
		got, _ := log.Read(70)
		if string(got) != "new" {
			t.Fatalf("expected 'new', got %q", got)
		}
	}
}
