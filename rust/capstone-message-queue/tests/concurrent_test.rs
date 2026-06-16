use mini_mq::broker::Registry;
use mini_mq::concurrent::SharedRegistry;
use std::sync::Arc;
use std::time::Duration;
use tempfile::TempDir;

// ── helpers ───────────────────────────────────────────────────────────────────

fn make_shared(dir: &TempDir, topics: &[(&str, u32)]) -> Arc<SharedRegistry> {
    let mut reg = Registry::open(dir.path()).unwrap();
    for &(name, parts) in topics {
        reg.create_topic(name, parts).unwrap();
    }
    Arc::new(SharedRegistry::new(reg, Duration::from_millis(50)))
}

fn total_offsets(shared: &SharedRegistry, topic: &str, num_partitions: u32) -> u64 {
    (0..num_partitions)
        .map(|p| shared.next_offset(topic, p).unwrap())
        .sum()
}

// ── single-threaded API sanity ────────────────────────────────────────────────

#[test]
fn create_topic_and_produce_fetch() {
    let dir = TempDir::new().unwrap();
    let shared = SharedRegistry::new(
        Registry::open(dir.path()).unwrap(),
        Duration::from_millis(100),
    );
    shared.create_topic("greetings", 1).unwrap();
    let (pid, off) = shared.produce("greetings", b"hello", None).unwrap();
    assert_eq!(pid, 0);
    assert_eq!(off, 0);
    assert_eq!(shared.fetch("greetings", 0, 0).unwrap(), b"hello");
}

#[test]
fn num_partitions_reflects_creation() {
    let dir = TempDir::new().unwrap();
    let shared = SharedRegistry::new(
        Registry::open(dir.path()).unwrap(),
        Duration::from_millis(100),
    );
    shared.create_topic("t", 3).unwrap();
    assert_eq!(shared.num_partitions("t").unwrap(), 3);
}

#[test]
fn topic_names_lists_all_topics() {
    let dir = TempDir::new().unwrap();
    let shared = SharedRegistry::new(
        Registry::open(dir.path()).unwrap(),
        Duration::from_millis(100),
    );
    shared.create_topic("beta", 1).unwrap();
    shared.create_topic("alpha", 2).unwrap();
    let names = shared.topic_names().unwrap();
    assert_eq!(names, vec!["alpha", "beta"]);
}

#[test]
fn produce_unknown_topic_errors() {
    let dir = TempDir::new().unwrap();
    let shared = SharedRegistry::new(
        Registry::open(dir.path()).unwrap(),
        Duration::from_millis(100),
    );
    assert!(shared.produce("nope", b"x", None).is_err());
}

#[test]
fn fetch_unknown_topic_errors() {
    let dir = TempDir::new().unwrap();
    let shared = SharedRegistry::new(
        Registry::open(dir.path()).unwrap(),
        Duration::from_millis(100),
    );
    assert!(shared.fetch("nope", 0, 0).is_err());
}

// ── fetch_batch ───────────────────────────────────────────────────────────────

#[test]
fn fetch_batch_returns_up_to_max_count() {
    let dir = TempDir::new().unwrap();
    let shared = make_shared(&dir, &[("batch", 1)]);
    for i in 0..50_u32 {
        shared.produce("batch", &i.to_be_bytes(), None).unwrap();
    }
    let batch = shared.fetch_batch("batch", 0, 10, 20).unwrap();
    assert_eq!(batch.len(), 20);
    assert_eq!(batch[0].offset, 10);
    assert_eq!(batch[19].offset, 29);
}

#[test]
fn fetch_batch_at_end_returns_fewer_than_max() {
    let dir = TempDir::new().unwrap();
    let shared = make_shared(&dir, &[("few", 1)]);
    for i in 0..5_u32 {
        shared.produce("few", &i.to_be_bytes(), None).unwrap();
    }
    let batch = shared.fetch_batch("few", 0, 3, 100).unwrap();
    assert_eq!(batch.len(), 2); // only offsets 3 and 4 exist
}

#[test]
fn fetch_batch_empty_topic() {
    let dir = TempDir::new().unwrap();
    let shared = make_shared(&dir, &[("empty", 1)]);
    let batch = shared.fetch_batch("empty", 0, 0, 10).unwrap();
    assert!(batch.is_empty());
}

// ── concurrent produce ────────────────────────────────────────────────────────

#[test]
fn concurrent_produces_single_partition() {
    let dir = TempDir::new().unwrap();
    let shared = make_shared(&dir, &[("events", 1)]);

    let mut handles = vec![];
    for i in 0..4_u32 {
        let s = Arc::clone(&shared);
        handles.push(std::thread::spawn(move || {
            for j in 0..50_u32 {
                let payload = format!("{i}-{j}");
                s.produce("events", payload.as_bytes(), Some(&i.to_be_bytes()))
                    .unwrap();
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }

    // 4 threads × 50 = 200 messages, all hashed to partition 0 (only 1 partition).
    assert_eq!(shared.next_offset("events", 0).unwrap(), 200);
}

#[test]
fn concurrent_produces_spread_across_partitions() {
    let dir = TempDir::new().unwrap();
    let shared = make_shared(&dir, &[("orders", 4)]);

    let mut handles = vec![];
    for i in 0..8_u32 {
        let s = Arc::clone(&shared);
        handles.push(std::thread::spawn(move || {
            for j in 0..25_u32 {
                let payload = format!("{i}-{j}");
                s.produce("orders", payload.as_bytes(), None).unwrap();
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }

    // 8 × 25 = 200 total across 4 partitions.
    assert_eq!(total_offsets(&shared, "orders", 4), 200);
}

#[test]
fn concurrent_produce_and_fetch_after_join() {
    let dir = TempDir::new().unwrap();
    let shared = make_shared(&dir, &[("log", 1)]);

    let s = Arc::clone(&shared);
    let producer = std::thread::spawn(move || {
        for i in 0..100_u32 {
            s.produce("log", &i.to_be_bytes(), None).unwrap();
        }
    });
    producer.join().unwrap();

    // After the producer finishes, all 100 messages must be fetchable in order.
    for i in 0..100_u64 {
        let payload = shared.fetch("log", 0, i).unwrap();
        assert_eq!(
            u32::from_be_bytes(payload.try_into().unwrap()),
            i as u32,
            "message at offset {i} has wrong value"
        );
    }
}

#[test]
fn concurrent_readers_while_producing() {
    // Writer thread and two reader threads run simultaneously.
    // After the writer finishes, readers verify the data they saw was consistent
    // (no torn reads — each payload is either absent or intact).
    let dir = TempDir::new().unwrap();
    let shared = make_shared(&dir, &[("mixed", 1)]);

    // Pre-write 50 records so readers have something to start from.
    for i in 0..50_u32 {
        shared.produce("mixed", &i.to_be_bytes(), None).unwrap();
    }

    let s_write = Arc::clone(&shared);
    let writer = std::thread::spawn(move || {
        for i in 50..150_u32 {
            s_write.produce("mixed", &i.to_be_bytes(), None).unwrap();
        }
    });

    // Two readers that fetch batches while the writer runs.
    let mut reader_handles = vec![];
    for _ in 0..2 {
        let s = Arc::clone(&shared);
        reader_handles.push(std::thread::spawn(move || {
            let batch = s.fetch_batch("mixed", 0, 0, 50).unwrap();
            // Every record we DID see must have the right value.
            for rec in &batch {
                let val = u32::from_be_bytes(rec.payload.clone().try_into().unwrap());
                assert_eq!(val, rec.offset as u32);
            }
        }));
    }

    writer.join().unwrap();
    for h in reader_handles {
        h.join().unwrap();
    }

    assert_eq!(shared.next_offset("mixed", 0).unwrap(), 150);
}

// ── background flush thread ───────────────────────────────────────────────────

#[test]
fn flush_thread_does_not_deadlock() {
    let dir = TempDir::new().unwrap();
    let shared = make_shared(&dir, &[("t", 1)]);

    for i in 0..20_u32 {
        shared.produce("t", &i.to_be_bytes(), None).unwrap();
    }
    // Sleep past two flush intervals; the flush thread must not deadlock.
    std::thread::sleep(Duration::from_millis(150));
    // Still readable after flush thread ran.
    assert_eq!(shared.next_offset("t", 0).unwrap(), 20);
}

#[test]
fn shutdown_joins_flush_thread_without_hanging() {
    let dir = TempDir::new().unwrap();
    let reg = Registry::open(dir.path()).unwrap();
    // Drop triggers shutdown; must not hang.
    let shared = SharedRegistry::new(reg, Duration::from_millis(50));
    shared.create_topic("x", 1).unwrap();
    shared.produce("x", b"data", None).unwrap();
    drop(shared); // ← must return promptly
}

#[test]
fn explicit_flush_all_visible_after() {
    let dir = TempDir::new().unwrap();
    let shared = make_shared(&dir, &[("f", 1)]);
    shared.produce("f", b"msg", None).unwrap();
    shared.flush_all().unwrap();
    assert_eq!(shared.fetch("f", 0, 0).unwrap(), b"msg");
}
