// Package protocol defines the wire protocol types for mini-mq.
//
// Protocol: newline-delimited JSON over TCP.
// Binary payloads are base64-encoded inside JSON strings.
//
// # Request types
//
//	{"type":"create_topic","topic":"events","partitions":3}
//	{"type":"produce","topic":"events","payload":"aGVsbG8="}
//	{"type":"produce","topic":"events","key":"dXNlcjE=","payload":"aGVsbG8="}
//	{"type":"fetch","topic":"events","partition":0,"offset":5}
//	{"type":"fetch_batch","topic":"events","partition":0,"offset":5,"max_count":100}
//	{"type":"metadata"}
//	{"type":"join_group","group":"my-group","topics":["events","orders"]}
//	{"type":"leave_group","group":"my-group","member_id":"member-0"}
//	{"type":"commit_offset","group":"my-group","topic":"events","partition":0,"offset":42}
//	{"type":"fetch_offset","group":"my-group","topic":"events","partition":0}
//
// # Response types
//
//	{"type":"topic_created","topic":"events","partitions":3}
//	{"type":"produced","partition":0,"offset":42}
//	{"type":"record","offset":5,"payload":"aGVsbG8="}
//	{"type":"end"}
//	{"type":"metadata","topics":[{"name":"events","partitions":3}]}
//	{"type":"joined","group":"my-group","member_id":"member-0","assignments":[{"topic":"events","partition":0}]}
//	{"type":"left_group","group":"my-group","member_id":"member-0"}
//	{"type":"offset_committed","group":"my-group","topic":"events","partition":0,"offset":42}
//	{"type":"committed_offset","group":"my-group","topic":"events","partition":0,"offset":42}
//	{"type":"committed_offset","group":"my-group","topic":"events","partition":0,"offset":null}
//	{"type":"error","message":"topic not found: orders"}
package protocol

// ── Request ───────────────────────────────────────────────────────────────────

// Request holds all possible fields for any request type.
// The Type field discriminates which operation to perform.
// Unmarshal into this struct, then dispatch on Type.
type Request struct {
	Type string `json:"type"`

	// create_topic
	Topic      string `json:"topic,omitempty"`
	Partitions uint32 `json:"partitions,omitempty"`

	// produce
	Key     *string `json:"key,omitempty"`     // base64-encoded, optional
	Payload string  `json:"payload,omitempty"` // base64-encoded

	// fetch / fetch_batch
	Partition uint32 `json:"partition,omitempty"`
	Offset    uint64 `json:"offset,omitempty"`
	MaxCount  int    `json:"max_count,omitempty"`

	// join_group
	Group  string   `json:"group,omitempty"`
	Topics []string `json:"topics,omitempty"`

	// leave_group
	MemberID string `json:"member_id,omitempty"`
}

// ── Response ──────────────────────────────────────────────────────────────────

// Response is the union of all response types.
// Use the Type field to discriminate.
type Response struct {
	Type string `json:"type"`

	// topic_created
	Topic      string `json:"topic,omitempty"`
	Partitions uint32 `json:"partitions,omitempty"`

	// produced
	Partition uint32 `json:"partition,omitempty"`
	Offset    uint64 `json:"offset,omitempty"`

	// record
	Payload string `json:"payload,omitempty"` // base64-encoded

	// metadata
	Topics []TopicMeta `json:"topics,omitempty"`

	// joined
	Group       string              `json:"group,omitempty"`
	MemberID    string              `json:"member_id,omitempty"`
	Assignments []AssignedPartition `json:"assignments,omitempty"`

	// left_group / offset_committed / committed_offset
	// (group, topic, partition, offset are reused)

	// committed_offset: offset is nullable (pointer)
	CommittedOffset *uint64 `json:"committed_offset,omitempty"` // unused field name; see server for actual encoding

	// error
	Message string `json:"message,omitempty"`
}

// ── Sub-types ─────────────────────────────────────────────────────────────────

// TopicMeta describes a topic's name and partition count.
type TopicMeta struct {
	Name       string `json:"name"`
	Partitions uint32 `json:"partitions"`
}

// AssignedPartition is a (topic, partition) assignment for a consumer group member.
type AssignedPartition struct {
	Topic     string `json:"topic"`
	Partition uint32 `json:"partition"`
}
