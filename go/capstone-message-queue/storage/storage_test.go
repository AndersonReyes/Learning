package storage

import (
	"bytes"
	"os"
	"path/filepath"
	"testing"
)

// helper: open a log in a fresh temp dir.
func openTempLog(t *testing.T) (*Log, string) {
	t.Helper()
	dir := t.TempDir()
	l, err := OpenLog(dir)
	if err != nil {
		t.Fatalf("OpenLog(%q) error: %v", dir, err)
	}
	return l, dir
}

// helper: close log, asserting no error.
func closeLog(t *testing.T, l *Log) {
	t.Helper()
	if err := l.Close(); err != nil {
		t.Fatalf("Close() error: %v", err)
	}
}

// ── Phase 1: basic append + read ─────────────────────────────────────────────

func TestAppendAndReadSingleRecord(t *testing.T) {
	l, _ := openTempLog(t)
	defer closeLog(t, l)

	payload := []byte("hello world")
	off, err := l.Append(payload)
	if err != nil {
		t.Fatalf("Append() error: %v", err)
	}
	if off != 0 {
		t.Fatalf("first Append() offset = %d, want 0", off)
	}

	got, err := l.Read(0)
	if err != nil {
		t.Fatalf("Read(0) error: %v", err)
	}
	if !bytes.Equal(got, payload) {
		t.Errorf("Read(0) = %q, want %q", got, payload)
	}
}

func TestSequentialOffsets(t *testing.T) {
	l, _ := openTempLog(t)
	defer closeLog(t, l)

	for i := range 10 {
		off, err := l.Append([]byte{byte(i)})
		if err != nil {
			t.Fatalf("Append(%d) error: %v", i, err)
		}
		if off != uint64(i) {
			t.Fatalf("Append(%d) offset = %d, want %d", i, off, i)
		}
	}
}

func TestReadEachOffset(t *testing.T) {
	l, _ := openTempLog(t)
	defer closeLog(t, l)

	payloads := [][]byte{
		[]byte("alpha"),
		[]byte("beta"),
		[]byte("gamma"),
		[]byte("delta"),
		[]byte("epsilon"),
	}
	for _, p := range payloads {
		if _, err := l.Append(p); err != nil {
			t.Fatalf("Append(%q) error: %v", p, err)
		}
	}

	for i, want := range payloads {
		got, err := l.Read(uint64(i))
		if err != nil {
			t.Fatalf("Read(%d) error: %v", i, err)
		}
		if !bytes.Equal(got, want) {
			t.Errorf("Read(%d) = %q, want %q", i, got, want)
		}
	}
}

// ── Phase 2: out-of-range read ────────────────────────────────────────────────

func TestReadOutOfRangeEmpty(t *testing.T) {
	l, _ := openTempLog(t)
	defer closeLog(t, l)

	_, err := l.Read(0)
	if err == nil {
		t.Fatal("Read(0) on empty log: expected error, got nil")
	}
}

func TestReadOutOfRangeHigh(t *testing.T) {
	l, _ := openTempLog(t)
	defer closeLog(t, l)

	if _, err := l.Append([]byte("a")); err != nil {
		t.Fatal(err)
	}

	_, err := l.Read(1)
	if err == nil {
		t.Fatal("Read(1) on single-record log: expected error, got nil")
	}
}

func TestReadOutOfRangeFarBeyond(t *testing.T) {
	l, _ := openTempLog(t)
	defer closeLog(t, l)

	for range 5 {
		if _, err := l.Append([]byte("x")); err != nil {
			t.Fatal(err)
		}
	}

	_, err := l.Read(9999)
	if err == nil {
		t.Fatal("Read(9999) on 5-record log: expected error, got nil")
	}
}

// ── Phase 3: scan ─────────────────────────────────────────────────────────────

func TestScanAll(t *testing.T) {
	l, _ := openTempLog(t)
	defer closeLog(t, l)

	want := [][]byte{[]byte("a"), []byte("b"), []byte("c")}
	for _, p := range want {
		if _, err := l.Append(p); err != nil {
			t.Fatal(err)
		}
	}

	recs, err := l.Scan(0, 100)
	if err != nil {
		t.Fatalf("Scan(0, 100) error: %v", err)
	}
	if len(recs) != 3 {
		t.Fatalf("Scan returned %d records, want 3", len(recs))
	}
	for i, r := range recs {
		if r.Offset != uint64(i) {
			t.Errorf("recs[%d].Offset = %d, want %d", i, r.Offset, i)
		}
		if !bytes.Equal(r.Payload, want[i]) {
			t.Errorf("recs[%d].Payload = %q, want %q", i, r.Payload, want[i])
		}
	}
}

func TestScanFromMiddle(t *testing.T) {
	l, _ := openTempLog(t)
	defer closeLog(t, l)

	for i := range 10 {
		if _, err := l.Append([]byte{byte(i)}); err != nil {
			t.Fatal(err)
		}
	}

	recs, err := l.Scan(5, 3)
	if err != nil {
		t.Fatalf("Scan(5, 3) error: %v", err)
	}
	if len(recs) != 3 {
		t.Fatalf("Scan(5, 3) returned %d records, want 3", len(recs))
	}
	for i, r := range recs {
		wantOff := uint64(5 + i)
		if r.Offset != wantOff {
			t.Errorf("recs[%d].Offset = %d, want %d", i, r.Offset, wantOff)
		}
		if !bytes.Equal(r.Payload, []byte{byte(5 + i)}) {
			t.Errorf("recs[%d].Payload = %v, want [%d]", i, r.Payload, 5+i)
		}
	}
}

func TestScanMaxCountLimits(t *testing.T) {
	l, _ := openTempLog(t)
	defer closeLog(t, l)

	for range 20 {
		if _, err := l.Append([]byte("x")); err != nil {
			t.Fatal(err)
		}
	}

	recs, err := l.Scan(0, 5)
	if err != nil {
		t.Fatalf("Scan(0, 5) error: %v", err)
	}
	if len(recs) != 5 {
		t.Fatalf("Scan(0, 5) returned %d records, want 5", len(recs))
	}
}

func TestScanEmptyLog(t *testing.T) {
	l, _ := openTempLog(t)
	defer closeLog(t, l)

	recs, err := l.Scan(0, 100)
	if err != nil {
		t.Fatalf("Scan on empty log error: %v", err)
	}
	if len(recs) != 0 {
		t.Fatalf("Scan on empty log returned %d records, want 0", len(recs))
	}
}

func TestScanBeyondEnd(t *testing.T) {
	l, _ := openTempLog(t)
	defer closeLog(t, l)

	if _, err := l.Append([]byte("only")); err != nil {
		t.Fatal(err)
	}

	recs, err := l.Scan(100, 10)
	if err != nil {
		t.Fatalf("Scan(100, 10) on 1-record log error: %v", err)
	}
	if len(recs) != 0 {
		t.Fatalf("Scan(100, 10) returned %d records, want 0", len(recs))
	}
}

func TestScanTruncatedByEnd(t *testing.T) {
	l, _ := openTempLog(t)
	defer closeLog(t, l)

	for range 3 {
		if _, err := l.Append([]byte("y")); err != nil {
			t.Fatal(err)
		}
	}

	// Ask for more than available starting at offset 1 — should return 2.
	recs, err := l.Scan(1, 100)
	if err != nil {
		t.Fatalf("Scan(1, 100) error: %v", err)
	}
	if len(recs) != 2 {
		t.Fatalf("Scan(1, 100) returned %d records, want 2", len(recs))
	}
	if recs[0].Offset != 1 {
		t.Errorf("recs[0].Offset = %d, want 1", recs[0].Offset)
	}
	if recs[1].Offset != 2 {
		t.Errorf("recs[1].Offset = %d, want 2", recs[1].Offset)
	}
}

// ── Phase 4: reopen recovery ──────────────────────────────────────────────────

func TestReopenRecovery(t *testing.T) {
	dir := t.TempDir()

	// Write 5 records, close.
	l1, err := OpenLog(dir)
	if err != nil {
		t.Fatalf("OpenLog (first open): %v", err)
	}
	payloads := [][]byte{
		[]byte("first"),
		[]byte("second"),
		[]byte("third"),
		[]byte("fourth"),
		[]byte("fifth"),
	}
	for _, p := range payloads {
		if _, err := l1.Append(p); err != nil {
			t.Fatalf("Append(%q): %v", p, err)
		}
	}
	if err := l1.Close(); err != nil {
		t.Fatalf("Close (first): %v", err)
	}

	// Reopen and verify all records readable.
	l2, err := OpenLog(dir)
	if err != nil {
		t.Fatalf("OpenLog (reopen): %v", err)
	}
	defer closeLog(t, l2)

	for i, want := range payloads {
		got, err := l2.Read(uint64(i))
		if err != nil {
			t.Fatalf("Read(%d) after reopen: %v", i, err)
		}
		if !bytes.Equal(got, want) {
			t.Errorf("Read(%d) = %q, want %q", i, got, want)
		}
	}
}

func TestReopenAndContinueAppending(t *testing.T) {
	dir := t.TempDir()

	l1, err := OpenLog(dir)
	if err != nil {
		t.Fatal(err)
	}
	off0, _ := l1.Append([]byte("before"))
	if off0 != 0 {
		t.Fatalf("first append offset = %d, want 0", off0)
	}
	if err := l1.Close(); err != nil {
		t.Fatal(err)
	}

	l2, err := OpenLog(dir)
	if err != nil {
		t.Fatal(err)
	}
	defer closeLog(t, l2)

	off1, err := l2.Append([]byte("after"))
	if err != nil {
		t.Fatalf("Append after reopen: %v", err)
	}
	if off1 != 1 {
		t.Fatalf("second append offset = %d, want 1", off1)
	}

	got0, _ := l2.Read(0)
	if !bytes.Equal(got0, []byte("before")) {
		t.Errorf("Read(0) = %q, want \"before\"", got0)
	}
	got1, _ := l2.Read(1)
	if !bytes.Equal(got1, []byte("after")) {
		t.Errorf("Read(1) = %q, want \"after\"", got1)
	}
}

// ── Phase 5: crash recovery (truncate partial record) ─────────────────────────

func TestCrashRecoveryPartialHeader(t *testing.T) {
	dir := t.TempDir()

	l, err := OpenLog(dir)
	if err != nil {
		t.Fatal(err)
	}
	if _, err := l.Append([]byte("good")); err != nil {
		t.Fatal(err)
	}
	if err := l.Close(); err != nil {
		t.Fatal(err)
	}

	// Corrupt: append 6 bytes (partial 12-byte header) to data.log.
	logPath := filepath.Join(dir, "data.log")
	f, err := os.OpenFile(logPath, os.O_APPEND|os.O_WRONLY, 0644)
	if err != nil {
		t.Fatalf("open data.log for corruption: %v", err)
	}
	if _, err := f.Write([]byte("partialhdr"[:6])); err != nil {
		t.Fatal(err)
	}
	f.Close()

	// Reopen — should recover, truncating the garbage.
	l2, err := OpenLog(dir)
	if err != nil {
		t.Fatalf("OpenLog after partial-header corruption: %v", err)
	}
	defer closeLog(t, l2)

	got, err := l2.Read(0)
	if err != nil {
		t.Fatalf("Read(0) after crash recovery: %v", err)
	}
	if !bytes.Equal(got, []byte("good")) {
		t.Errorf("Read(0) = %q, want \"good\"", got)
	}

	// No record at offset 1 — the partial header was truncated away.
	_, err = l2.Read(1)
	if err == nil {
		t.Error("Read(1) should error after crash recovery of partial header")
	}
}

func TestCrashRecoveryPartialPayload(t *testing.T) {
	dir := t.TempDir()

	l, err := OpenLog(dir)
	if err != nil {
		t.Fatal(err)
	}
	if _, err := l.Append([]byte("good")); err != nil {
		t.Fatal(err)
	}
	if err := l.Close(); err != nil {
		t.Fatal(err)
	}

	// Append a full 12-byte header claiming 1000-byte payload, then only 3 bytes.
	logPath := filepath.Join(dir, "data.log")
	f, err := os.OpenFile(logPath, os.O_APPEND|os.O_WRONLY, 0644)
	if err != nil {
		t.Fatalf("open data.log for corruption: %v", err)
	}
	// offset=1, length=1000 in big-endian, then 3 bytes of "payload"
	header := []byte{
		0, 0, 0, 0, 0, 0, 0, 1, // offset=1 uint64 BE
		0, 0, 3, 232,           // length=1000 uint32 BE
	}
	if _, err := f.Write(header); err != nil {
		t.Fatal(err)
	}
	if _, err := f.Write([]byte("pay")); err != nil { // only 3 of 1000 bytes
		t.Fatal(err)
	}
	f.Close()

	// Reopen — should truncate the partial payload record.
	l2, err := OpenLog(dir)
	if err != nil {
		t.Fatalf("OpenLog after partial-payload corruption: %v", err)
	}
	defer closeLog(t, l2)

	got, err := l2.Read(0)
	if err != nil {
		t.Fatalf("Read(0) after crash recovery: %v", err)
	}
	if !bytes.Equal(got, []byte("good")) {
		t.Errorf("Read(0) = %q, want \"good\"", got)
	}

	_, err = l2.Read(1)
	if err == nil {
		t.Error("Read(1) should error after crash recovery of partial payload")
	}
}

// ── Phase 6: large payload and sparse index ───────────────────────────────────

func TestLargePayload(t *testing.T) {
	l, _ := openTempLog(t)
	defer closeLog(t, l)

	// 1 MiB payload.
	payload := bytes.Repeat([]byte("X"), 1<<20)
	off, err := l.Append(payload)
	if err != nil {
		t.Fatalf("Append(1MiB): %v", err)
	}
	if off != 0 {
		t.Fatalf("Append offset = %d, want 0", off)
	}

	got, err := l.Read(0)
	if err != nil {
		t.Fatalf("Read(0): %v", err)
	}
	if !bytes.Equal(got, payload) {
		t.Errorf("Read(0) length = %d, want %d", len(got), len(payload))
	}
}

func TestSparseIndexEnables65PlusRecords(t *testing.T) {
	// Write > 64 records so the sparse index must be written and used.
	l, dir := openTempLog(t)

	for i := range 130 {
		p := []byte{byte(i)}
		if _, err := l.Append(p); err != nil {
			t.Fatalf("Append(%d): %v", i, err)
		}
	}
	if err := l.Close(); err != nil {
		t.Fatal(err)
	}

	// Reopen — index must be loaded correctly.
	l2, err := OpenLog(dir)
	if err != nil {
		t.Fatalf("OpenLog reopen: %v", err)
	}
	defer closeLog(t, l2)

	// Read from the second index block (offset 64+).
	for _, off := range []uint64{0, 63, 64, 65, 128, 129} {
		got, err := l2.Read(off)
		if err != nil {
			t.Fatalf("Read(%d) after reopen: %v", off, err)
		}
		if len(got) != 1 || got[0] != byte(off) {
			t.Errorf("Read(%d) = %v, want [%d]", off, got, off)
		}
	}
}

func TestScanAcrossSparseIndexBoundary(t *testing.T) {
	l, _ := openTempLog(t)
	defer closeLog(t, l)

	for i := range 70 {
		if _, err := l.Append([]byte{byte(i)}); err != nil {
			t.Fatalf("Append(%d): %v", i, err)
		}
	}

	// Scan starting at 62 — crosses the index boundary at 64.
	recs, err := l.Scan(62, 10)
	if err != nil {
		t.Fatalf("Scan(62, 10): %v", err)
	}
	if len(recs) != 8 { // 62..69 = 8 records
		t.Fatalf("Scan(62, 10) returned %d records, want 8", len(recs))
	}
	for i, r := range recs {
		wantOff := uint64(62 + i)
		if r.Offset != wantOff {
			t.Errorf("recs[%d].Offset = %d, want %d", i, r.Offset, wantOff)
		}
	}
}

// ── Phase 7: empty payload ────────────────────────────────────────────────────

func TestEmptyPayload(t *testing.T) {
	l, _ := openTempLog(t)
	defer closeLog(t, l)

	off, err := l.Append([]byte{})
	if err != nil {
		t.Fatalf("Append(empty): %v", err)
	}
	if off != 0 {
		t.Fatalf("offset = %d, want 0", off)
	}

	got, err := l.Read(0)
	if err != nil {
		t.Fatalf("Read(0): %v", err)
	}
	if len(got) != 0 {
		t.Errorf("Read(0) = %v, want empty slice", got)
	}
}
