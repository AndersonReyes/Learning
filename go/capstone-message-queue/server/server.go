// Package server implements the message queue's JSON-over-TCP network layer.
//
// Wire protocol: newline-delimited JSON. Each connection is handled by a
// dedicated goroutine. Requests are decoded line-by-line; responses are written
// back as JSON lines.
//
// All domain errors (unknown topic, out-of-range offset, …) produce a single
// error response line: {"type":"error","message":"…"}. The connection stays
// open after an error.
package server

import (
	"errors"
	"net"

	"github.com/andersonreyes/learning/go/capstone-message-queue/concurrent"
	"github.com/andersonreyes/learning/go/capstone-message-queue/groups"
	"github.com/andersonreyes/learning/go/capstone-message-queue/protocol"
)

// BrokerHandle bundles the shared registry and group coordinator for a running
// broker. Both fields must be non-nil.
type BrokerHandle struct {
	Registry *concurrent.SharedRegistry
	Groups   *groups.GroupCoordinator
}

// ProcessRequest handles a single decoded request and returns the JSON objects
// to send back to the client (one object per response line).
//
// Payload and key fields in Request are standard base64; decode them before
// passing to the registry. Encode outgoing payloads as standard base64.
//
// Responses for each request type:
//
//	create_topic  → {"type":"ok"}
//	produce       → {"type":"produced","partition":N,"offset":N}
//	fetch         → {"type":"fetched","payload":"<base64>"}
//	fetch_batch   → {"type":"fetched_batch","records":[{"offset":N,"payload":"<base64>"},…]}
//	metadata      → {"type":"metadata","topics":[{"name":"…","partitions":N},…]}
//	join_group    → {"type":"joined","member_id":"…","assignments":[{"topic":"…","partition":N},…]}
//	leave_group   → {"type":"ok"}
//	commit_offset → {"type":"ok"}
//	fetch_offset  → {"type":"offset","offset":N}  (offset field is null if none committed)
//	unknown type  → {"type":"error","message":"unknown request type: <type>"}
//	any error     → {"type":"error","message":"<message>"}
//
// Returns one element in the slice for all request types.
func ProcessRequest(h *BrokerHandle, req *protocol.Request) []map[string]interface{} {
	return []map[string]interface{}{
		{"type": "error", "message": errors.New("not implemented").Error()},
	}
}

// HandleConnection reads newline-delimited JSON from conn, calls ProcessRequest
// for each valid request, and writes responses back. Keeps the connection open
// until the client disconnects or an unrecoverable I/O error occurs.
//
// JSON parse errors produce {"type":"error","message":"…"} and keep the
// connection open (the client can send another request).
func HandleConnection(conn net.Conn, h *BrokerHandle) {
	// stub — close the connection immediately
	conn.Close()
}

// RunServer accepts connections from l and spawns a goroutine per connection
// calling HandleConnection. Blocks until l is closed (Accept returns an error).
func RunServer(l net.Listener, h *BrokerHandle) {
	// stub — return immediately
	_ = l
	_ = h
}
