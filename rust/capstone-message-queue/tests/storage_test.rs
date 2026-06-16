use mini_mq::storage::Log;
use tempfile::TempDir;

fn temp_log() -> (TempDir, Log) {
    let dir = TempDir::new().unwrap();
    let log = Log::open(dir.path()).unwrap();
    (dir, log)
}

// ── append / next_offset ──────────────────────────────────────────────────────

#[test]
fn offsets_start_at_zero() {
    let (_dir, log) = temp_log();
    assert_eq!(log.next_offset(), 0);
}

#[test]
fn append_returns_sequential_offsets() {
    let (_dir, mut log) = temp_log();
    assert_eq!(log.append(b"a").unwrap(), 0);
    assert_eq!(log.append(b"b").unwrap(), 1);
    assert_eq!(log.append(b"c").unwrap(), 2);
    assert_eq!(log.next_offset(), 3);
}

#[test]
fn append_empty_payload() {
    let (_dir, mut log) = temp_log();
    let off = log.append(b"").unwrap();
    log.flush().unwrap();
    let payload = log.read(off).unwrap();
    assert_eq!(payload, b"");
}

#[test]
fn append_large_payload() {
    let (_dir, mut log) = temp_log();
    let big = vec![0xAB_u8; 64 * 1024]; // 64 KB
    let off = log.append(&big).unwrap();
    log.flush().unwrap();
    assert_eq!(log.read(off).unwrap(), big);
}

// ── read ─────────────────────────────────────────────────────────────────────

#[test]
fn read_first_record() {
    let (_dir, mut log) = temp_log();
    log.append(b"hello").unwrap();
    log.flush().unwrap();
    assert_eq!(log.read(0).unwrap(), b"hello");
}

#[test]
fn read_middle_record() {
    let (_dir, mut log) = temp_log();
    log.append(b"first").unwrap();
    log.append(b"second").unwrap();
    log.append(b"third").unwrap();
    log.flush().unwrap();
    assert_eq!(log.read(1).unwrap(), b"second");
}

#[test]
fn read_last_record() {
    let (_dir, mut log) = temp_log();
    for i in 0_u8..10 {
        log.append(&[i]).unwrap();
    }
    log.flush().unwrap();
    assert_eq!(log.read(9).unwrap(), &[9]);
}

#[test]
fn read_out_of_range_returns_error() {
    let (_dir, mut log) = temp_log();
    log.append(b"x").unwrap();
    log.flush().unwrap();
    assert!(log.read(1).is_err()); // only offset 0 exists
    assert!(log.read(100).is_err());
}

#[test]
fn read_empty_log_returns_error() {
    let (_dir, log) = temp_log();
    assert!(log.read(0).is_err());
}

// ── scan ─────────────────────────────────────────────────────────────────────

#[test]
fn scan_all_records() {
    let (_dir, mut log) = temp_log();
    let messages: Vec<&[u8]> = vec![b"alpha", b"beta", b"gamma"];
    for m in &messages {
        log.append(m).unwrap();
    }
    log.flush().unwrap();

    let result: Vec<(u64, Vec<u8>)> = log.scan(0).map(|r| r.unwrap()).collect();
    assert_eq!(result.len(), 3);
    assert_eq!(result[0], (0, b"alpha".to_vec()));
    assert_eq!(result[1], (1, b"beta".to_vec()));
    assert_eq!(result[2], (2, b"gamma".to_vec()));
}

#[test]
fn scan_from_middle() {
    let (_dir, mut log) = temp_log();
    for i in 0..5_u8 {
        log.append(&[i]).unwrap();
    }
    log.flush().unwrap();

    let result: Vec<(u64, Vec<u8>)> = log.scan(2).map(|r| r.unwrap()).collect();
    assert_eq!(result.len(), 3);
    assert_eq!(result[0].0, 2);
    assert_eq!(result[2].0, 4);
}

#[test]
fn scan_empty_log() {
    let (_dir, log) = temp_log();
    let result: Vec<_> = log.scan(0).collect();
    assert!(result.is_empty());
}

// ── sparse index: across the INDEX_INTERVAL boundary (64 records) ─────────────

#[test]
fn read_record_beyond_index_interval() {
    let (_dir, mut log) = temp_log();
    // Append 100 records so the index fires at offsets 0 and 64.
    for i in 0..100_u64 {
        let payload = i.to_be_bytes().to_vec();
        log.append(&payload).unwrap();
    }
    log.flush().unwrap();

    // Read a record that's between the two index entries.
    let payload = log.read(75).unwrap();
    assert_eq!(u64::from_be_bytes(payload.try_into().unwrap()), 75);
}

#[test]
fn scan_across_index_boundary() {
    let (_dir, mut log) = temp_log();
    for i in 0..130_u64 {
        log.append(&i.to_be_bytes()).unwrap();
    }
    log.flush().unwrap();

    // Start scan past the second index entry at offset 128.
    let result: Vec<_> = log.scan(120).map(|r| r.unwrap()).collect();
    assert_eq!(result.len(), 10);
    assert_eq!(result[0].0, 120);
    assert_eq!(result[9].0, 129);
}

// ── reopen / recovery ─────────────────────────────────────────────────────────

#[test]
fn reopen_recovers_next_offset() {
    let dir = TempDir::new().unwrap();
    {
        let mut log = Log::open(dir.path()).unwrap();
        log.append(b"first").unwrap();
        log.append(b"second").unwrap();
        log.flush().unwrap();
    }
    let log = Log::open(dir.path()).unwrap();
    assert_eq!(log.next_offset(), 2);
}

#[test]
fn reopen_can_read_existing_records() {
    let dir = TempDir::new().unwrap();
    {
        let mut log = Log::open(dir.path()).unwrap();
        log.append(b"persist me").unwrap();
        log.flush().unwrap();
    }
    let log = Log::open(dir.path()).unwrap();
    assert_eq!(log.read(0).unwrap(), b"persist me");
}

#[test]
fn reopen_continues_appending() {
    let dir = TempDir::new().unwrap();
    {
        let mut log = Log::open(dir.path()).unwrap();
        log.append(b"before").unwrap();
        log.flush().unwrap();
    }
    {
        let mut log = Log::open(dir.path()).unwrap();
        let off = log.append(b"after").unwrap();
        log.flush().unwrap();
        assert_eq!(off, 1);
        assert_eq!(log.read(0).unwrap(), b"before");
        assert_eq!(log.read(1).unwrap(), b"after");
    }
}

#[test]
fn reopen_across_index_boundary() {
    let dir = TempDir::new().unwrap();
    // Write 70 records (index fires at 0, 64), close, reopen, append more.
    {
        let mut log = Log::open(dir.path()).unwrap();
        for i in 0..70_u64 {
            log.append(&i.to_be_bytes()).unwrap();
        }
        log.flush().unwrap();
    }
    {
        let mut log = Log::open(dir.path()).unwrap();
        assert_eq!(log.next_offset(), 70);
        let off = log.append(b"new").unwrap();
        assert_eq!(off, 70);
        log.flush().unwrap();
        assert_eq!(log.read(70).unwrap(), b"new");
    }
}
