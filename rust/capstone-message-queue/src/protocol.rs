//! Wire protocol: newline-delimited JSON over TCP.
//!
//! **Payload encoding:** raw bytes are base64-encoded inside JSON strings so
//! the protocol stays human-readable and debuggable with `nc` / `telnet`.
//!
//! # Request types
//!
//! ```json
//! {"type":"create_topic","topic":"events","partitions":3}
//! {"type":"produce","topic":"events","payload":"aGVsbG8="}
//! {"type":"produce","topic":"events","key":"dXNlcjE=","payload":"aGVsbG8="}
//! {"type":"fetch","topic":"events","partition":0,"offset":5}
//! {"type":"fetch_batch","topic":"events","partition":0,"offset":5,"max_count":100}
//! {"type":"metadata"}
//! ```
//!
//! # Response types
//!
//! ```json
//! {"type":"topic_created","topic":"events","partitions":3}
//! {"type":"produced","partition":0,"offset":42}
//! {"type":"record","offset":5,"payload":"aGVsbG8="}
//! {"type":"end"}
//! {"type":"metadata","topics":[{"name":"events","partitions":3}]}
//! {"type":"error","message":"topic not found: orders"}
//! ```

use serde::{Deserialize, Serialize};

// ── Request ───────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Request {
    /// Create a topic with `partitions` partitions. Idempotent.
    CreateTopic { topic: String, partitions: u32 },

    /// Append `payload` (base64) to `topic`.
    /// Optional `key` (base64) determines the partition via FNV-1a hash;
    /// omitting `key` uses round-robin.
    Produce {
        topic: String,
        #[serde(default)]
        key: Option<String>,
        payload: String,
    },

    /// Read the single record at `offset` from `topic`/`partition`.
    Fetch {
        topic: String,
        partition: u32,
        offset: u64,
    },

    /// Read up to `max_count` records from `topic`/`partition` starting at
    /// `offset`. Responds with zero or more `record` messages then `end`.
    FetchBatch {
        topic: String,
        partition: u32,
        offset: u64,
        max_count: usize,
    },

    /// List all topics and their partition counts.
    Metadata,
}

// ── Response ──────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Response {
    TopicCreated { topic: String, partitions: u32 },
    Produced { partition: u32, offset: u64 },
    Record { offset: u64, payload: String },
    End,
    Metadata { topics: Vec<TopicMeta> },
    Error { message: String },
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct TopicMeta {
    pub name: String,
    pub partitions: u32,
}
