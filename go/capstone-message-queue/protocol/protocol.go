// Package protocol defines the wire types for the message queue's
// newline-delimited JSON over TCP protocol.
//
// Every message has a "type" field. Payloads and keys are standard base64.
//
// Request types (client → broker):
//
//	{"type":"create_topic","topic":"events","partitions":3}
//	{"type":"produce","topic":"events","key":"dXNlcjE=","payload":"aGVsbG8="}
//	{"type":"fetch","topic":"events","partition":0,"offset":5}
//	{"type":"fetch_batch","topic":"events","partition":0,"offset":5,"max_count":100}
//	{"type":"metadata"}
//	{"type":"join_group","group":"g","topics":["events"]}
//	{"type":"leave_group","group":"g","member_id":"member-0"}
//	{"type":"commit_offset","group":"g","topic":"events","partition":0,"offset":42}
//	{"type":"fetch_offset","group":"g","topic":"events","partition":0}
package protocol

// Request is a decoded client request.
// Payload and Key are base64 strings in the wire format.
type Request struct {
	Type       string   `json:"type"`
	Topic      string   `json:"topic,omitempty"`
	Topics     []string `json:"topics,omitempty"`
	Partitions int      `json:"partitions,omitempty"`
	Partition  uint32   `json:"partition,omitempty"`
	Offset     uint64   `json:"offset,omitempty"`
	MaxCount   int      `json:"max_count,omitempty"`
	Key        string   `json:"key,omitempty"`     // base64-encoded bytes
	Payload    string   `json:"payload,omitempty"` // base64-encoded bytes
	Group      string   `json:"group,omitempty"`
	MemberID   string   `json:"member_id,omitempty"`
}

// TopicMeta describes a single topic in a metadata response.
type TopicMeta struct {
	Name       string `json:"name"`
	Partitions int    `json:"partitions"`
}

// Assignment is a (topic, partition) pair assigned to a consumer group member.
type Assignment struct {
	Topic     string `json:"topic"`
	Partition uint32 `json:"partition"`
}
