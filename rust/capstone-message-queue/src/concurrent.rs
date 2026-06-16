//! Thread-safe broker handle with a background flush thread.
//!
//! # Locking strategy
//!
//! `produce` holds a **write lock** for the duration of the append (the whole
//! registry is serialized). This is simple and correct; a per-partition lock
//! map would allow more parallelism but adds significant complexity — that's a
//! natural Phase 4 / production concern, not a Phase 3 learning objective.
//!
//! `fetch` / `fetch_batch` / metadata queries hold a **read lock**, so
//! multiple consumers can read concurrently.
//!
//! # Background flush
//!
//! A background thread wakes every `flush_interval` and calls `flush_all` on
//! the registry. `Partition::append` already flushes after each write for
//! single-threaded correctness; the background thread provides a periodic
//! safety net once we relax that in Phase 4.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};
use std::time::Duration;

use crate::broker::{Record, Registry};
use crate::error::{Error, Result};

// ── lock-poison helper ────────────────────────────────────────────────────────

fn lock_err<T>(_: std::sync::PoisonError<T>) -> Error {
    Error::Io(std::io::Error::new(
        std::io::ErrorKind::Other,
        "registry lock poisoned",
    ))
}

// ── SharedRegistry ────────────────────────────────────────────────────────────

/// A thread-safe handle to a [`Registry`] with a background flush thread.
///
/// Always wrap in `Arc` and share clones across threads:
///
/// ```ignore
/// let shared = Arc::new(SharedRegistry::new(registry, Duration::from_millis(100)));
/// let clone = Arc::clone(&shared);
/// std::thread::spawn(move || clone.produce("events", b"hello", None)).join().unwrap();
/// ```
///
/// The background flush thread is stopped (and joined) when the
/// `SharedRegistry` is dropped.
pub struct SharedRegistry {
    inner: Arc<RwLock<Registry>>,
    shutdown: Arc<AtomicBool>,
    flush_handle: std::sync::Mutex<Option<std::thread::JoinHandle<()>>>,
}

impl SharedRegistry {
    /// Wraps `registry` and starts a background flush thread that wakes every
    /// `flush_interval`.
    pub fn new(registry: Registry, flush_interval: Duration) -> Self {
        let inner = Arc::new(RwLock::new(registry));
        let shutdown = Arc::new(AtomicBool::new(false));

        let inner_clone = Arc::clone(&inner);
        let shutdown_clone = Arc::clone(&shutdown);

        let handle = std::thread::spawn(move || {
            while !shutdown_clone.load(Ordering::Acquire) {
                std::thread::sleep(flush_interval);
                if shutdown_clone.load(Ordering::Acquire) {
                    break;
                }
                if let Ok(mut reg) = inner_clone.write() {
                    let _ = reg.flush_all();
                }
            }
        });

        SharedRegistry {
            inner,
            shutdown,
            flush_handle: std::sync::Mutex::new(Some(handle)),
        }
    }

    /// Signals the flush thread to stop and blocks until it exits.
    ///
    /// Called automatically by `Drop`. Can also be called explicitly.
    pub fn shutdown(&self) {
        self.shutdown.store(true, Ordering::Release);
        if let Ok(mut guard) = self.flush_handle.lock() {
            if let Some(handle) = guard.take() {
                let _ = handle.join();
            }
        }
    }

    // ── Topic management ──────────────────────────────────────────────────────

    /// Creates a topic (or opens it if it already exists). See
    /// [`Registry::create_topic`].
    pub fn create_topic(&self, name: &str, num_partitions: u32) -> Result<()> {
        self.inner
            .write()
            .map_err(lock_err)?
            .create_topic(name, num_partitions)
    }

    /// Returns a sorted list of all topic names.
    pub fn topic_names(&self) -> Result<Vec<String>> {
        Ok(self.inner.read().map_err(lock_err)?.topic_names())
    }

    /// Returns the number of partitions for `topic`.
    pub fn num_partitions(&self, topic: &str) -> Result<usize> {
        self.inner
            .read()
            .map_err(lock_err)?
            .get_topic(topic)
            .map(|t| t.num_partitions())
            .ok_or_else(|| Error::TopicNotFound(topic.to_owned()))
    }

    // ── Produce ───────────────────────────────────────────────────────────────

    /// Appends `payload` to `topic`.
    ///
    /// - `key = Some(k)` — routes to partition `fnv1a(k) % n`.
    /// - `key = None` — round-robin.
    ///
    /// Returns `(partition_id, offset)`.
    pub fn produce(
        &self,
        topic: &str,
        payload: &[u8],
        key: Option<&[u8]>,
    ) -> Result<(u32, u64)> {
        self.inner
            .write()
            .map_err(lock_err)?
            .get_topic_mut(topic)
            .ok_or_else(|| Error::TopicNotFound(topic.to_owned()))?
            .produce(payload, key)
    }

    // ── Fetch ─────────────────────────────────────────────────────────────────

    /// Reads the single payload at `offset` from `topic`/`partition`.
    pub fn fetch(&self, topic: &str, partition: u32, offset: u64) -> Result<Vec<u8>> {
        self.inner
            .read()
            .map_err(lock_err)?
            .get_topic(topic)
            .ok_or_else(|| Error::TopicNotFound(topic.to_owned()))?
            .fetch(partition, offset)
    }

    /// Collects up to `max_count` records from `topic`/`partition` starting at
    /// `start_offset`.
    ///
    /// Collects into a `Vec` and releases the read lock before returning, so
    /// callers can process records without holding any lock.
    pub fn fetch_batch(
        &self,
        topic: &str,
        partition: u32,
        start_offset: u64,
        max_count: usize,
    ) -> Result<Vec<Record>> {
        let reg = self.inner.read().map_err(lock_err)?;
        let t = reg
            .get_topic(topic)
            .ok_or_else(|| Error::TopicNotFound(topic.to_owned()))?;
        // Bind iter explicitly so it (and its borrow of `reg`) is dropped
        // after collect() completes, before `reg` is released.
        let iter = t.scan(partition, start_offset)?;
        iter.take(max_count).collect::<Result<Vec<_>>>()
    }

    /// Returns the next offset for `topic`/`partition`.
    pub fn next_offset(&self, topic: &str, partition: u32) -> Result<u64> {
        self.inner
            .read()
            .map_err(lock_err)?
            .get_topic(topic)
            .ok_or_else(|| Error::TopicNotFound(topic.to_owned()))?
            .next_offset(partition)
    }

    /// Flushes all partition logs immediately (in addition to the background
    /// flush).
    pub fn flush_all(&self) -> Result<()> {
        self.inner.write().map_err(lock_err)?.flush_all()
    }
}

impl Drop for SharedRegistry {
    fn drop(&mut self) {
        self.shutdown();
    }
}
