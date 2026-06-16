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
//! {"type":"join_group","group":"my-group","topics":["events","orders"]}
//! {"type":"leave_group","group":"my-group","member_id":"member-0"}
//! {"type":"commit_offset","group":"my-group","topic":"events","partition":0,"offset":42}
//! {"type":"fetch_offset","group":"my-group","topic":"events","partition":0}
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
//! {"type":"joined","group":"my-group","member_id":"member-0","assignments":[{"topic":"events","partition":0}]}
//! {"type":"left_group","group":"my-group","member_id":"member-0"}
//! {"type":"offset_committed","group":"my-group","topic":"events","partition":0,"offset":42}
//! {"type":"committed_offset","group":"my-group","topic":"events","partition":0,"offset":42}
//! {"type":"committed_offset","group":"my-group","topic":"events","partition":0,"offset":null}
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
    /// Optional `key` (base64) routes via FNV-1a hash; omitting uses round-robin.
    Produce {
        topic: String,
        #[serde(default)]
        key: Option<String>,
        payload: String,
    },

    /// Read the single record at `offset` from `topic`/`partition`.
    Fetch { topic: String, partition: u32, offset: u64 },

    /// Read up to `max_count` records starting at `offset`. Responds with
    /// zero or more `record` messages then `end`.
    FetchBatch { topic: String, partition: u32, offset: u64, max_count: usize },

    /// List all topics and their partition counts.
    Metadata,

    /// Join `group`, subscribing to `topics`. Server assigns partitions and
    /// returns a `member_id` the client must use for subsequent group requests.
    JoinGroup { group: String, topics: Vec<String> },

    /// Leave `group`. Server reassigns the departing member's partitions.
    LeaveGroup { group: String, member_id: String },

    /// Record that `group` has successfully processed up to `offset` on
    /// `topic`/`partition`. Durable across reconnects within a session
    /// (in-memory in Phase 5; persisted storage is a natural follow-on).
    CommitOffset { group: String, topic: String, partition: u32, offset: u64 },

    /// Return the last committed offset for `group`/`topic`/`partition`.
    FetchOffset { group: String, topic: String, partition: u32 },
}

// ── Response ──────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Response {
    // ── broker / storage ──
    TopicCreated { topic: String, partitions: u32 },
    Produced { partition: u32, offset: u64 },
    Record { offset: u64, payload: String },
    End,
    Metadata { topics: Vec<TopicMeta> },

    // ── consumer groups ──
    /// Sent after a successful `join_group`.
    Joined {
        group: String,
        member_id: String,
        assignments: Vec<AssignedPartition>,
    },
    /// Sent after a successful `leave_group`.
    LeftGroup { group: String, member_id: String },
    /// Sent after a successful `commit_offset`.
    OffsetCommitted { group: String, topic: String, partition: u32, offset: u64 },
    /// Sent in reply to `fetch_offset`. `offset` is `None` if nothing has been
    /// committed yet for this `(group, topic, partition)` triple.
    CommittedOffset {
        group: String,
        topic: String,
        partition: u32,
        offset: Option<u64>,
    },

    // ── errors ──
    Error { message: String },
}

// ── shared sub-types ──────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct TopicMeta {
    pub name: String,
    pub partitions: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct AssignedPartition {
    pub topic: String,
    pub partition: u32,
}
