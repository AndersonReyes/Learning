// Package server implements the TCP broker server: newline-delimited JSON over TCP.
package server

import (
	"bufio"
	"encoding/base64"
	"encoding/json"
	"fmt"
	"net"
	"sort"
	"strings"

	"github.com/andersonreyes/learning/go/capstone-message-queue/concurrent"
	"github.com/andersonreyes/learning/go/capstone-message-queue/groups"
	"github.com/andersonreyes/learning/go/capstone-message-queue/protocol"
)

// ── BrokerHandle ──────────────────────────────────────────────────────────────

// BrokerHandle holds shared state threaded through every connection handler.
type BrokerHandle struct {
	Registry *concurrent.SharedRegistry
	Groups   *groups.GroupCoordinator
}

// NewBrokerHandle creates a BrokerHandle with a new GroupCoordinator.
func NewBrokerHandle(registry *concurrent.SharedRegistry) *BrokerHandle {
	return &BrokerHandle{
		Registry: registry,
		Groups:   groups.New(),
	}
}

// ── Response encoding ─────────────────────────────────────────────────────────

// rawResponse is a JSON-encodable response. We use a custom map-based approach
// to handle the nullable offset field (committed_offset) cleanly.
type rawResponse map[string]interface{}

func (r rawResponse) Encode() ([]byte, error) {
	return json.Marshal(map[string]interface{}(r))
}

func responseTopicCreated(topic string, partitions uint32) rawResponse {
	return rawResponse{"type": "topic_created", "topic": topic, "partitions": partitions}
}

func responseProduced(partition uint32, offset uint64) rawResponse {
	return rawResponse{"type": "produced", "partition": partition, "offset": offset}
}

func responseRecord(offset uint64, payload string) rawResponse {
	return rawResponse{"type": "record", "offset": offset, "payload": payload}
}

func responseEnd() rawResponse {
	return rawResponse{"type": "end"}
}

func responseMetadata(topics []protocol.TopicMeta) rawResponse {
	// Convert to serializable form.
	ts := make([]map[string]interface{}, len(topics))
	for i, t := range topics {
		ts[i] = map[string]interface{}{"name": t.Name, "partitions": t.Partitions}
	}
	return rawResponse{"type": "metadata", "topics": ts}
}

func responseJoined(group, memberID string, assignments []protocol.AssignedPartition) rawResponse {
	as := make([]map[string]interface{}, len(assignments))
	for i, a := range assignments {
		as[i] = map[string]interface{}{"topic": a.Topic, "partition": a.Partition}
	}
	return rawResponse{"type": "joined", "group": group, "member_id": memberID, "assignments": as}
}

func responseLeftGroup(group, memberID string) rawResponse {
	return rawResponse{"type": "left_group", "group": group, "member_id": memberID}
}

func responseOffsetCommitted(group, topic string, partition uint32, offset uint64) rawResponse {
	return rawResponse{"type": "offset_committed", "group": group, "topic": topic, "partition": partition, "offset": offset}
}

func responseCommittedOffset(group, topic string, partition uint32, offset *uint64) rawResponse {
	r := rawResponse{"type": "committed_offset", "group": group, "topic": topic, "partition": partition, "offset": nil}
	if offset != nil {
		r["offset"] = *offset
	}
	return r
}

func responseError(message string) rawResponse {
	return rawResponse{"type": "error", "message": message}
}

// ── Request processing ─────────────────────────────────────────────────────────

// ProcessRequest processes one request and returns the JSON-encoded response lines to send.
// All domain errors are converted to error responses so the connection stays open.
func ProcessRequest(handle *BrokerHandle, req *protocol.Request) []rawResponse {
	reg := handle.Registry
	gc := handle.Groups

	switch req.Type {
	// ── broker / storage ──
	case "create_topic":
		if err := reg.CreateTopic(req.Topic, req.Partitions); err != nil {
			return []rawResponse{responseError(err.Error())}
		}
		return []rawResponse{responseTopicCreated(req.Topic, req.Partitions)}

	case "produce":
		payload, err := base64.StdEncoding.DecodeString(req.Payload)
		if err != nil {
			return []rawResponse{responseError("payload: invalid base64")}
		}
		var keyBytes []byte
		if req.Key != nil {
			keyBytes, err = base64.StdEncoding.DecodeString(*req.Key)
			if err != nil {
				return []rawResponse{responseError("key: invalid base64")}
			}
		}
		partition, offset, err := reg.Produce(req.Topic, payload, keyBytes)
		if err != nil {
			return []rawResponse{responseError(err.Error())}
		}
		return []rawResponse{responseProduced(partition, offset)}

	case "fetch":
		data, err := reg.Fetch(req.Topic, req.Partition, req.Offset)
		if err != nil {
			return []rawResponse{responseError(err.Error())}
		}
		return []rawResponse{
			responseRecord(req.Offset, base64.StdEncoding.EncodeToString(data)),
			responseEnd(),
		}

	case "fetch_batch":
		records, err := reg.FetchBatch(req.Topic, req.Partition, req.Offset, req.MaxCount)
		if err != nil {
			return []rawResponse{responseError(err.Error())}
		}
		out := make([]rawResponse, 0, len(records)+1)
		for _, r := range records {
			out = append(out, responseRecord(r.Offset, base64.StdEncoding.EncodeToString(r.Payload)))
		}
		out = append(out, responseEnd())
		return out

	case "metadata":
		names := reg.TopicNames()
		var topics []protocol.TopicMeta
		for _, name := range names {
			n, err := reg.NumPartitions(name)
			if err != nil {
				continue
			}
			topics = append(topics, protocol.TopicMeta{Name: name, Partitions: uint32(n)})
		}
		return []rawResponse{responseMetadata(topics)}

	// ── consumer groups ──
	case "join_group":
		memberID, assignment, err := gc.Join(req.Group, req.Topics, reg)
		if err != nil {
			return []rawResponse{responseError(err.Error())}
		}
		// Sort for determinism.
		sort.Slice(assignment, func(i, j int) bool {
			if assignment[i].Topic != assignment[j].Topic {
				return assignment[i].Topic < assignment[j].Topic
			}
			return assignment[i].Partition < assignment[j].Partition
		})
		return []rawResponse{responseJoined(req.Group, memberID, assignment)}

	case "leave_group":
		if err := gc.Leave(req.Group, req.MemberID, reg); err != nil {
			return []rawResponse{responseError(err.Error())}
		}
		return []rawResponse{responseLeftGroup(req.Group, req.MemberID)}

	case "commit_offset":
		if err := gc.CommitOffset(req.Group, req.Topic, req.Partition, req.Offset); err != nil {
			return []rawResponse{responseError(err.Error())}
		}
		return []rawResponse{responseOffsetCommitted(req.Group, req.Topic, req.Partition, req.Offset)}

	case "fetch_offset":
		offset, err := gc.FetchOffset(req.Group, req.Topic, req.Partition)
		if err != nil {
			return []rawResponse{responseError(err.Error())}
		}
		return []rawResponse{responseCommittedOffset(req.Group, req.Topic, req.Partition, offset)}

	default:
		return []rawResponse{responseError(fmt.Sprintf("unknown request type: %q", req.Type))}
	}
}

// ── Connection handler ────────────────────────────────────────────────────────

// HandleConnection handles a single client connection.
// Reads newline-delimited JSON requests and writes newline-delimited JSON responses.
func HandleConnection(conn net.Conn, handle *BrokerHandle) {
	defer conn.Close()
	scanner := bufio.NewScanner(conn)
	writer := bufio.NewWriter(conn)

	for scanner.Scan() {
		line := strings.TrimSpace(scanner.Text())
		if line == "" {
			continue
		}

		var req protocol.Request
		responses := func() []rawResponse {
			if err := json.Unmarshal([]byte(line), &req); err != nil {
				return []rawResponse{responseError(fmt.Sprintf("parse error: %v", err))}
			}
			return ProcessRequest(handle, &req)
		}()

		for _, resp := range responses {
			b, err := resp.Encode()
			if err != nil {
				// Should never happen.
				continue
			}
			writer.Write(b)
			writer.WriteByte('\n')
		}
		writer.Flush()
	}
}

// ── Server ─────────────────────────────────────────────────────────────────────

// RunServer runs the broker on a pre-bound listener.
// Accepts connections and spawns a goroutine per connection.
func RunServer(listener net.Listener, handle *BrokerHandle) error {
	for {
		conn, err := listener.Accept()
		if err != nil {
			return err
		}
		go HandleConnection(conn, handle)
	}
}
