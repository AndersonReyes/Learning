use mini_mq::broker::{Partition, Record, Registry, Topic};
use mini_mq::error::Error;
use tempfile::TempDir;

// ── helpers ──────────────────────────────────────────────────────────────────

fn temp_dir() -> TempDir {
    TempDir::new().unwrap()
}

// ── Partition: basic ops ──────────────────────────────────────────────────────

#[test]
fn partition_open_append_read() {
    let dir = temp_dir();
    let mut p = Partition::open(dir.path(), "my-topic", 0).unwrap();
    assert_eq!(p.next_offset(), 0);
    assert_eq!(p.id(), 0);

    let off = p.append(b"hello").unwrap();
    assert_eq!(off, 0);
    p.flush().unwrap();

    assert_eq!(p.read(0).unwrap(), b"hello");
    assert_eq!(p.next_offset(), 1);
}

#[test]
fn partition_append_multiple_and_read() {
    let dir = temp_dir();
    let mut p = Partition::open(dir.path(), "t", 2).unwrap();
    p.append(b"a").unwrap();
    p.append(b"b").unwrap();
    p.append(b"c").unwrap();
    p.flush().unwrap();

    assert_eq!(p.read(0).unwrap(), b"a");
    assert_eq!(p.read(1).unwrap(), b"b");
    assert_eq!(p.read(2).unwrap(), b"c");
    assert_eq!(p.next_offset(), 3);
}

#[test]
fn partition_scan_all() {
    let dir = temp_dir();
    let mut p = Partition::open(dir.path(), "t", 0).unwrap();
    p.append(b"x").unwrap();
    p.append(b"y").unwrap();
    p.append(b"z").unwrap();
    p.flush().unwrap();

    let records: Vec<Record> = p.scan(0).map(|r| r.unwrap()).collect();
    assert_eq!(records.len(), 3);
    assert_eq!(records[0], Record { offset: 0, payload: b"x".to_vec() });
    assert_eq!(records[1], Record { offset: 1, payload: b"y".to_vec() });
    assert_eq!(records[2], Record { offset: 2, payload: b"z".to_vec() });
}

#[test]
fn partition_scan_from_middle() {
    let dir = temp_dir();
    let mut p = Partition::open(dir.path(), "t", 0).unwrap();
    for i in 0u8..5 {
        p.append(&[i]).unwrap();
    }
    p.flush().unwrap();

    let records: Vec<Record> = p.scan(3).map(|r| r.unwrap()).collect();
    assert_eq!(records.len(), 2);
    assert_eq!(records[0].offset, 3);
    assert_eq!(records[1].offset, 4);
}

#[test]
fn partition_reopen_recovery() {
    let dir = temp_dir();
    {
        let mut p = Partition::open(dir.path(), "events", 0).unwrap();
        p.append(b"first").unwrap();
        p.append(b"second").unwrap();
        p.flush().unwrap();
    }
    // Reopen and verify state is recovered.
    let mut p = Partition::open(dir.path(), "events", 0).unwrap();
    assert_eq!(p.next_offset(), 2);
    assert_eq!(p.read(0).unwrap(), b"first");
    assert_eq!(p.read(1).unwrap(), b"second");

    // Can still append after reopen.
    let off = p.append(b"third").unwrap();
    assert_eq!(off, 2);
    p.flush().unwrap();
    assert_eq!(p.read(2).unwrap(), b"third");
}

// ── Topic: create, produce, fetch, scan ─────────────────────────────────────

#[test]
fn topic_create_with_n_partitions() {
    let dir = temp_dir();
    let t = Topic::open(dir.path(), "orders", 3).unwrap();
    assert_eq!(t.name(), "orders");
    assert_eq!(t.num_partitions(), 3);
}

#[test]
fn topic_produce_with_key_is_deterministic() {
    let dir = temp_dir();
    let mut t = Topic::open(dir.path(), "orders", 4).unwrap();

    // Produce with the same key always goes to the same partition.
    let (pid1, _) = t.produce(b"msg1", Some(b"user-42")).unwrap();
    let (pid2, _) = t.produce(b"msg2", Some(b"user-42")).unwrap();
    let (pid3, _) = t.produce(b"msg3", Some(b"user-42")).unwrap();
    assert_eq!(pid1, pid2);
    assert_eq!(pid2, pid3);

    // Different key may go to a different partition (just verify it's valid).
    let (pid_other, _) = t.produce(b"other", Some(b"user-99")).unwrap();
    assert!(pid_other < 4);
}

#[test]
fn topic_produce_without_key_is_round_robin() {
    let dir = temp_dir();
    let mut t = Topic::open(dir.path(), "rr", 3).unwrap();

    let mut pids = Vec::new();
    for i in 0..9u8 {
        let (pid, _) = t.produce(&[i], None).unwrap();
        pids.push(pid);
    }
    // With 3 partitions and 9 round-robin produces: 0,1,2,0,1,2,0,1,2
    assert_eq!(pids, vec![0, 1, 2, 0, 1, 2, 0, 1, 2]);
}

#[test]
fn topic_fetch_returns_correct_payload() {
    let dir = temp_dir();
    let mut t = Topic::open(dir.path(), "t", 2).unwrap();

    // Route specific messages to known partitions by key.
    // Find which partition key "a" hashes to.
    let (pid_a, off_a) = t.produce(b"payload-a", Some(b"key-a")).unwrap();
    let (pid_b, off_b) = t.produce(b"payload-b", Some(b"key-b")).unwrap();

    assert_eq!(t.fetch(pid_a, off_a).unwrap(), b"payload-a");
    assert_eq!(t.fetch(pid_b, off_b).unwrap(), b"payload-b");
}

#[test]
fn topic_scan_partition() {
    let dir = temp_dir();
    let mut t = Topic::open(dir.path(), "events", 1).unwrap();

    // With 1 partition everything goes to partition 0.
    t.produce(b"e0", None).unwrap();
    t.produce(b"e1", None).unwrap();
    t.produce(b"e2", None).unwrap();

    let records: Vec<Record> = t.scan(0, 0).unwrap().map(|r| r.unwrap()).collect();
    assert_eq!(records.len(), 3);
    assert_eq!(records[0].payload, b"e0");
    assert_eq!(records[2].payload, b"e2");
}

#[test]
fn topic_next_offset() {
    let dir = temp_dir();
    let mut t = Topic::open(dir.path(), "t", 1).unwrap();
    assert_eq!(t.next_offset(0).unwrap(), 0);
    t.produce(b"msg", None).unwrap();
    t.produce(b"msg2", None).unwrap();
    assert_eq!(t.next_offset(0).unwrap(), 2);
}

#[test]
fn topic_produce_key_routing_across_partitions() {
    // With 4 partitions and chosen keys, verify routing is FNV-1a % 4.
    let dir = temp_dir();
    let mut t = Topic::open(dir.path(), "keyed", 4).unwrap();

    // Pre-compute expected partition for a few keys using the same FNV-1a.
    fn fnv1a(data: &[u8]) -> u64 {
        let mut h = 14695981039346656037_u64;
        for &b in data {
            h ^= b as u64;
            h = h.wrapping_mul(1099511628211);
        }
        h
    }

    let keys: &[&[u8]] = &[b"alpha", b"beta", b"gamma", b"delta", b"epsilon"];
    for key in keys {
        let expected_pid = (fnv1a(key) % 4) as u32;
        let (actual_pid, _) = t.produce(b"x", Some(key)).unwrap();
        assert_eq!(
            actual_pid, expected_pid,
            "key {:?} should route to partition {}",
            key, expected_pid
        );
    }
}

// ── Registry: create, get, flush, reopen ─────────────────────────────────────

#[test]
fn registry_create_topic_and_get() {
    let dir = temp_dir();
    let mut reg = Registry::open(dir.path()).unwrap();

    reg.create_topic("users", 3).unwrap();

    let t = reg.get_topic("users").unwrap();
    assert_eq!(t.name(), "users");
    assert_eq!(t.num_partitions(), 3);
}

#[test]
fn registry_topic_names_sorted() {
    let dir = temp_dir();
    let mut reg = Registry::open(dir.path()).unwrap();

    reg.create_topic("zebra", 1).unwrap();
    reg.create_topic("apple", 2).unwrap();
    reg.create_topic("mango", 3).unwrap();

    assert_eq!(reg.topic_names(), vec!["apple", "mango", "zebra"]);
}

#[test]
fn registry_get_topic_mut_produce() {
    let dir = temp_dir();
    let mut reg = Registry::open(dir.path()).unwrap();
    reg.create_topic("orders", 2).unwrap();

    let t = reg.get_topic_mut("orders").unwrap();
    let (pid, off) = t.produce(b"order-1", None).unwrap();
    assert_eq!(t.fetch(pid, off).unwrap(), b"order-1");
}

#[test]
fn registry_flush_all() {
    let dir = temp_dir();
    let mut reg = Registry::open(dir.path()).unwrap();
    reg.create_topic("logs", 2).unwrap();

    {
        let t = reg.get_topic_mut("logs").unwrap();
        t.produce(b"log1", None).unwrap();
        t.produce(b"log2", None).unwrap();
    }

    // flush_all should not error.
    reg.flush_all().unwrap();
}

#[test]
fn registry_reopen_recovers_topics_and_offsets() {
    let dir = temp_dir();

    // Session 1: create topics, produce messages.
    {
        let mut reg = Registry::open(dir.path()).unwrap();
        reg.create_topic("events", 2).unwrap();
        reg.create_topic("orders", 1).unwrap();

        {
            let t = reg.get_topic_mut("events").unwrap();
            t.produce(b"ev0", None).unwrap();
            t.produce(b"ev1", None).unwrap();
        }
        {
            let t = reg.get_topic_mut("orders").unwrap();
            t.produce(b"ord0", None).unwrap();
        }
        reg.flush_all().unwrap();
    }

    // Session 2: reopen, verify everything is recovered.
    {
        let reg = Registry::open(dir.path()).unwrap();

        let names = reg.topic_names();
        assert!(names.contains(&"events".to_string()));
        assert!(names.contains(&"orders".to_string()));

        let events = reg.get_topic("events").unwrap();
        assert_eq!(events.num_partitions(), 2);
        // The two events are distributed across partitions round-robin (0,1).
        // Partition 0 has offset 0 with "ev0", partition 1 has offset 0 with "ev1".
        assert_eq!(events.next_offset(0).unwrap(), 1);
        assert_eq!(events.next_offset(1).unwrap(), 1);
        assert_eq!(events.fetch(0, 0).unwrap(), b"ev0");
        assert_eq!(events.fetch(1, 0).unwrap(), b"ev1");

        let orders = reg.get_topic("orders").unwrap();
        assert_eq!(orders.num_partitions(), 1);
        assert_eq!(orders.next_offset(0).unwrap(), 1);
        assert_eq!(orders.fetch(0, 0).unwrap(), b"ord0");
    }
}

#[test]
fn registry_create_topic_is_idempotent() {
    let dir = temp_dir();
    let mut reg = Registry::open(dir.path()).unwrap();

    reg.create_topic("dup", 3).unwrap();
    // Calling again with the same name should be OK (idempotent, not an error).
    reg.create_topic("dup", 3).unwrap();

    assert_eq!(reg.topic_names(), vec!["dup"]);
    assert_eq!(reg.get_topic("dup").unwrap().num_partitions(), 3);
}

// ── Error cases ──────────────────────────────────────────────────────────────

#[test]
fn fetch_from_nonexistent_partition_errors() {
    let dir = temp_dir();
    let t = Topic::open(dir.path(), "t", 2).unwrap();
    // Only partitions 0 and 1 exist.
    let err = t.fetch(5, 0).unwrap_err();
    assert!(matches!(err, Error::PartitionOutOfRange(5)));
}

#[test]
fn get_nonexistent_topic_returns_none() {
    let dir = temp_dir();
    let reg = Registry::open(dir.path()).unwrap();
    assert!(reg.get_topic("nope").is_none());
    assert_eq!(reg.topic_names(), Vec::<String>::new());
}
