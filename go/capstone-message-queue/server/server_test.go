package server_test

import (
	"bufio"
	"encoding/base64"
	"encoding/json"
	"fmt"
	"net"
	"sort"
	"testing"
	"time"

	"github.com/andersonreyes/learning/go/capstone-message-queue/broker"
	"github.com/andersonreyes/learning/go/capstone-message-queue/concurrent"
	"github.com/andersonreyes/learning/go/capstone-message-queue/protocol"
	"github.com/andersonreyes/learning/go/capstone-message-queue/server"
)

// ── helpers ───────────────────────────────────────────────────────────────────

func makeRegistry(t *testing.T) *concurrent.SharedRegistry {
	t.Helper()
	dir := t.TempDir()
	reg, err := broker.Open(dir)
	if err != nil {
		t.Fatalf("Open: %v", err)
	}
	sr := concurrent.New(reg, 100*time.Millisecond)
	t.Cleanup(sr.Close)
	return sr
}

func makeHandle(t *testing.T) *server.BrokerHandle {
	return server.NewBrokerHandle(makeRegistry(t))
}

func b64(s string) string {
	return base64.StdEncoding.EncodeToString([]byte(s))
}

func fromB64(s string) string {
	b, _ := base64.StdEncoding.DecodeString(s)
	return string(b)
}

// processReq calls ProcessRequest and returns decoded JSON responses.
func processReq(t *testing.T, handle *server.BrokerHandle, req *protocol.Request) []map[string]interface{} {
	t.Helper()
	responses := server.ProcessRequest(handle, req)
	result := make([]map[string]interface{}, len(responses))
	for i, r := range responses {
		b, err := json.Marshal(r)
		if err != nil {
			t.Fatalf("marshal response: %v", err)
		}
		var m map[string]interface{}
		json.Unmarshal(b, &m)
		result[i] = m
	}
	return result
}

// ── process_request unit tests ────────────────────────────────────────────────

func TestCreateTopicResponse(t *testing.T) {
	handle := makeHandle(t)
	resp := processReq(t, handle, &protocol.Request{Type: "create_topic", Topic: "events", Partitions: 3})
	if len(resp) != 1 {
		t.Fatalf("expected 1 response, got %d", len(resp))
	}
	if resp[0]["type"] != "topic_created" {
		t.Fatalf("expected type=topic_created, got %v", resp[0]["type"])
	}
	if resp[0]["topic"] != "events" {
		t.Fatalf("expected topic=events, got %v", resp[0]["topic"])
	}
	if resp[0]["partitions"].(float64) != 3 {
		t.Fatalf("expected partitions=3, got %v", resp[0]["partitions"])
	}
}

func TestCreateTopicIdempotent(t *testing.T) {
	handle := makeHandle(t)
	processReq(t, handle, &protocol.Request{Type: "create_topic", Topic: "t", Partitions: 2})
	resp := processReq(t, handle, &protocol.Request{Type: "create_topic", Topic: "t", Partitions: 2})
	if resp[0]["type"] != "topic_created" {
		t.Fatalf("expected topic_created, got %v", resp[0]["type"])
	}
}

func TestProduceReturnsPartitionAndOffset(t *testing.T) {
	handle := makeHandle(t)
	processReq(t, handle, &protocol.Request{Type: "create_topic", Topic: "t", Partitions: 1})

	resp := processReq(t, handle, &protocol.Request{Type: "produce", Topic: "t", Payload: b64("hello")})
	if resp[0]["type"] != "produced" {
		t.Fatalf("expected produced, got %v", resp[0]["type"])
	}
	if resp[0]["partition"].(float64) != 0 || resp[0]["offset"].(float64) != 0 {
		t.Fatalf("expected partition=0, offset=0, got %v", resp[0])
	}

	resp2 := processReq(t, handle, &protocol.Request{Type: "produce", Topic: "t", Payload: b64("world")})
	if resp2[0]["offset"].(float64) != 1 {
		t.Fatalf("expected offset=1, got %v", resp2[0]["offset"])
	}
}

func TestProduceUnknownTopicReturnsError(t *testing.T) {
	handle := makeHandle(t)
	resp := processReq(t, handle, &protocol.Request{Type: "produce", Topic: "nope", Payload: b64("x")})
	if resp[0]["type"] != "error" {
		t.Fatalf("expected error, got %v", resp[0]["type"])
	}
}

func TestProduceInvalidBase64ReturnsError(t *testing.T) {
	handle := makeHandle(t)
	processReq(t, handle, &protocol.Request{Type: "create_topic", Topic: "t", Partitions: 1})
	resp := processReq(t, handle, &protocol.Request{Type: "produce", Topic: "t", Payload: "!!!not_b64"})
	if resp[0]["type"] != "error" {
		t.Fatalf("expected error, got %v", resp[0]["type"])
	}
}

func TestFetchRoundtrip(t *testing.T) {
	handle := makeHandle(t)
	processReq(t, handle, &protocol.Request{Type: "create_topic", Topic: "t", Partitions: 1})
	processReq(t, handle, &protocol.Request{Type: "produce", Topic: "t", Payload: b64("ping")})

	resp := processReq(t, handle, &protocol.Request{Type: "fetch", Topic: "t", Partition: 0, Offset: 0})
	if len(resp) != 2 {
		t.Fatalf("expected 2 responses, got %d", len(resp))
	}
	if resp[1]["type"] != "end" {
		t.Fatalf("expected end, got %v", resp[1]["type"])
	}
	if resp[0]["type"] != "record" {
		t.Fatalf("expected record, got %v", resp[0]["type"])
	}
	if resp[0]["offset"].(float64) != 0 {
		t.Fatalf("expected offset=0, got %v", resp[0]["offset"])
	}
	if fromB64(resp[0]["payload"].(string)) != "ping" {
		t.Fatalf("expected 'ping', got %q", resp[0]["payload"])
	}
}

func TestFetchOutOfRangeReturnsError(t *testing.T) {
	handle := makeHandle(t)
	processReq(t, handle, &protocol.Request{Type: "create_topic", Topic: "t", Partitions: 1})
	resp := processReq(t, handle, &protocol.Request{Type: "fetch", Topic: "t", Partition: 0, Offset: 99})
	if resp[0]["type"] != "error" {
		t.Fatalf("expected error, got %v", resp[0]["type"])
	}
}

func TestFetchBatchReturnsRecordsAndEnd(t *testing.T) {
	handle := makeHandle(t)
	processReq(t, handle, &protocol.Request{Type: "create_topic", Topic: "t", Partitions: 1})
	for i := 0; i < 5; i++ {
		processReq(t, handle, &protocol.Request{Type: "produce", Topic: "t", Payload: b64(fmt.Sprintf("%d", i))})
	}

	resp := processReq(t, handle, &protocol.Request{Type: "fetch_batch", Topic: "t", Partition: 0, Offset: 1, MaxCount: 3})
	// 3 records + End
	if len(resp) != 4 {
		t.Fatalf("expected 4 responses, got %d", len(resp))
	}
	if resp[3]["type"] != "end" {
		t.Fatalf("expected end, got %v", resp[3]["type"])
	}
	for i := 0; i < 3; i++ {
		if resp[i]["type"] != "record" {
			t.Fatalf("resp[%d] expected record, got %v", i, resp[i]["type"])
		}
		if resp[i]["offset"].(float64) != float64(i+1) {
			t.Fatalf("resp[%d] offset = %v, want %d", i, resp[i]["offset"], i+1)
		}
		if fromB64(resp[i]["payload"].(string)) != fmt.Sprintf("%d", i+1) {
			t.Fatalf("resp[%d] payload wrong", i)
		}
	}
}

func TestFetchBatchEmptyReturnsJustEnd(t *testing.T) {
	handle := makeHandle(t)
	processReq(t, handle, &protocol.Request{Type: "create_topic", Topic: "t", Partitions: 1})
	resp := processReq(t, handle, &protocol.Request{Type: "fetch_batch", Topic: "t", Partition: 0, Offset: 0, MaxCount: 10})
	if len(resp) != 1 || resp[0]["type"] != "end" {
		t.Fatalf("expected [end], got %v", resp)
	}
}

func TestMetadataListsAllTopics(t *testing.T) {
	handle := makeHandle(t)
	processReq(t, handle, &protocol.Request{Type: "create_topic", Topic: "beta", Partitions: 2})
	processReq(t, handle, &protocol.Request{Type: "create_topic", Topic: "alpha", Partitions: 1})

	resp := processReq(t, handle, &protocol.Request{Type: "metadata"})
	if len(resp) != 1 || resp[0]["type"] != "metadata" {
		t.Fatalf("expected metadata response, got %v", resp)
	}
	topics := resp[0]["topics"].([]interface{})
	names := make([]string, len(topics))
	for i, t := range topics {
		names[i] = t.(map[string]interface{})["name"].(string)
	}
	sort.Strings(names)
	if len(names) != 2 || names[0] != "alpha" || names[1] != "beta" {
		t.Fatalf("expected ['alpha', 'beta'], got %v", names)
	}
	for _, tp := range topics {
		tm := tp.(map[string]interface{})
		switch tm["name"] {
		case "alpha":
			if tm["partitions"].(float64) != 1 {
				t.Errorf("alpha: expected 1 partition, got %v", tm["partitions"])
			}
		case "beta":
			if tm["partitions"].(float64) != 2 {
				t.Errorf("beta: expected 2 partitions, got %v", tm["partitions"])
			}
		}
	}
}

func TestMetadataEmptyRegistry(t *testing.T) {
	handle := makeHandle(t)
	resp := processReq(t, handle, &protocol.Request{Type: "metadata"})
	if resp[0]["type"] != "metadata" {
		t.Fatalf("expected metadata, got %v", resp[0]["type"])
	}
	topics := resp[0]["topics"]
	// topics can be nil or empty slice
	if topics != nil {
		ts := topics.([]interface{})
		if len(ts) != 0 {
			t.Fatalf("expected empty topics, got %v", ts)
		}
	}
}

func TestProduceWithKeyRoutesDeterministically(t *testing.T) {
	handle := makeHandle(t)
	processReq(t, handle, &protocol.Request{Type: "create_topic", Topic: "t", Partitions: 4})

	key := b64("user-42")
	r1 := processReq(t, handle, &protocol.Request{Type: "produce", Topic: "t", Key: &key, Payload: b64("a")})
	r2 := processReq(t, handle, &protocol.Request{Type: "produce", Topic: "t", Key: &key, Payload: b64("b")})

	p1 := r1[0]["partition"].(float64)
	p2 := r2[0]["partition"].(float64)
	if p1 != p2 {
		t.Fatalf("same key should produce same partition: got %v and %v", p1, p2)
	}
}

// ── Consumer group via process_request ────────────────────────────────────────

func TestProcessRequestJoinGroup(t *testing.T) {
	handle := makeHandle(t)
	handle.Registry.CreateTopic("events", 2)

	resp := processReq(t, handle, &protocol.Request{Type: "join_group", Group: "g", Topics: []string{"events"}})
	if len(resp) != 1 || resp[0]["type"] != "joined" {
		t.Fatalf("expected joined, got %v", resp)
	}
	if resp[0]["group"] != "g" {
		t.Fatalf("expected group=g, got %v", resp[0]["group"])
	}
	if resp[0]["member_id"] != "member-0" {
		t.Fatalf("expected member_id=member-0, got %v", resp[0]["member_id"])
	}
	assignments := resp[0]["assignments"].([]interface{})
	if len(assignments) != 2 {
		t.Fatalf("expected 2 assignments, got %d", len(assignments))
	}
}

func TestProcessRequestLeaveGroup(t *testing.T) {
	handle := makeHandle(t)
	handle.Registry.CreateTopic("t", 1)

	joinResp := processReq(t, handle, &protocol.Request{Type: "join_group", Group: "g", Topics: []string{"t"}})
	memberID := joinResp[0]["member_id"].(string)

	leaveResp := processReq(t, handle, &protocol.Request{Type: "leave_group", Group: "g", MemberID: memberID})
	if leaveResp[0]["type"] != "left_group" {
		t.Fatalf("expected left_group, got %v", leaveResp[0]["type"])
	}
	if leaveResp[0]["group"] != "g" || leaveResp[0]["member_id"] != memberID {
		t.Fatalf("wrong left_group response: %v", leaveResp[0])
	}
}

func TestProcessRequestCommitAndFetchOffset(t *testing.T) {
	handle := makeHandle(t)

	// fetch_offset before any commit → offset: null
	resp := processReq(t, handle, &protocol.Request{Type: "fetch_offset", Group: "g", Topic: "t", Partition: 0})
	if resp[0]["type"] != "committed_offset" {
		t.Fatalf("expected committed_offset, got %v", resp[0]["type"])
	}
	if resp[0]["offset"] != nil {
		t.Fatalf("expected null offset, got %v", resp[0]["offset"])
	}

	// commit
	resp = processReq(t, handle, &protocol.Request{Type: "commit_offset", Group: "g", Topic: "t", Partition: 0, Offset: 99})
	if resp[0]["type"] != "offset_committed" {
		t.Fatalf("expected offset_committed, got %v", resp[0]["type"])
	}
	if resp[0]["offset"].(float64) != 99 {
		t.Fatalf("expected offset=99, got %v", resp[0]["offset"])
	}

	// fetch after commit
	resp = processReq(t, handle, &protocol.Request{Type: "fetch_offset", Group: "g", Topic: "t", Partition: 0})
	if resp[0]["offset"].(float64) != 99 {
		t.Fatalf("expected offset=99, got %v", resp[0]["offset"])
	}
}

// ── end-to-end TCP tests ──────────────────────────────────────────────────────

// startTestBroker starts a test broker on an OS-assigned port.
func startTestBroker(t *testing.T) (int, *concurrent.SharedRegistry) {
	t.Helper()
	listener, err := net.Listen("tcp", "127.0.0.1:0")
	if err != nil {
		t.Fatalf("Listen: %v", err)
	}

	reg := makeRegistry(t)
	handle := server.NewBrokerHandle(reg)

	go server.RunServer(listener, handle)

	t.Cleanup(func() { listener.Close() })

	port := listener.Addr().(*net.TCPAddr).Port
	return port, reg
}

func tcpConnect(t *testing.T, port int) (*bufio.Reader, net.Conn) {
	t.Helper()
	conn, err := net.Dial("tcp", fmt.Sprintf("127.0.0.1:%d", port))
	if err != nil {
		t.Fatalf("Dial: %v", err)
	}
	t.Cleanup(func() { conn.Close() })
	return bufio.NewReader(conn), conn
}

func tcpSend(t *testing.T, conn net.Conn, line string) {
	t.Helper()
	if _, err := fmt.Fprintf(conn, "%s\n", line); err != nil {
		t.Fatalf("Send: %v", err)
	}
}

func tcpRecv(t *testing.T, r *bufio.Reader) map[string]interface{} {
	t.Helper()
	line, err := r.ReadString('\n')
	if err != nil {
		t.Fatalf("Recv: %v", err)
	}
	var m map[string]interface{}
	if err := json.Unmarshal([]byte(line), &m); err != nil {
		t.Fatalf("Unmarshal %q: %v", line, err)
	}
	return m
}

func TestTCPCreateAndProduceFetch(t *testing.T) {
	port, _ := startTestBroker(t)
	r, conn := tcpConnect(t, port)

	// Create topic.
	tcpSend(t, conn, `{"type":"create_topic","topic":"e","partitions":1}`)
	resp := tcpRecv(t, r)
	if resp["type"] != "topic_created" || resp["topic"] != "e" {
		t.Fatalf("unexpected response: %v", resp)
	}

	// Produce.
	payload := b64("hello tcp")
	tcpSend(t, conn, fmt.Sprintf(`{"type":"produce","topic":"e","payload":"%s"}`, payload))
	resp = tcpRecv(t, r)
	if resp["type"] != "produced" || resp["offset"].(float64) != 0 {
		t.Fatalf("unexpected response: %v", resp)
	}

	// Fetch.
	tcpSend(t, conn, `{"type":"fetch","topic":"e","partition":0,"offset":0}`)
	record := tcpRecv(t, r)
	end := tcpRecv(t, r)
	if record["type"] != "record" {
		t.Fatalf("expected record, got %v", record["type"])
	}
	if fromB64(record["payload"].(string)) != "hello tcp" {
		t.Fatalf("expected 'hello tcp', got %q", record["payload"])
	}
	if end["type"] != "end" {
		t.Fatalf("expected end, got %v", end["type"])
	}
}

func TestTCPFetchBatchMultipleRecords(t *testing.T) {
	port, _ := startTestBroker(t)
	r, conn := tcpConnect(t, port)

	tcpSend(t, conn, `{"type":"create_topic","topic":"b","partitions":1}`)
	tcpRecv(t, r) // topic_created

	for i := 0; i < 5; i++ {
		p := b64(fmt.Sprintf("%d", i))
		tcpSend(t, conn, fmt.Sprintf(`{"type":"produce","topic":"b","payload":"%s"}`, p))
		tcpRecv(t, r) // produced
	}

	tcpSend(t, conn, `{"type":"fetch_batch","topic":"b","partition":0,"offset":1,"max_count":3}`)

	r0 := tcpRecv(t, r)
	r1 := tcpRecv(t, r)
	r2 := tcpRecv(t, r)
	end := tcpRecv(t, r)

	if r0["offset"].(float64) != 1 || fromB64(r0["payload"].(string)) != "1" {
		t.Fatalf("r0 wrong: %v", r0)
	}
	if r1["offset"].(float64) != 2 {
		t.Fatalf("r1 offset wrong: %v", r1["offset"])
	}
	if r2["offset"].(float64) != 3 {
		t.Fatalf("r2 offset wrong: %v", r2["offset"])
	}
	if end["type"] != "end" {
		t.Fatalf("expected end, got %v", end["type"])
	}
}

func TestTCPMetadata(t *testing.T) {
	port, _ := startTestBroker(t)
	r, conn := tcpConnect(t, port)

	tcpSend(t, conn, `{"type":"create_topic","topic":"x","partitions":2}`)
	tcpRecv(t, r)

	tcpSend(t, conn, `{"type":"metadata"}`)
	resp := tcpRecv(t, r)
	if resp["type"] != "metadata" {
		t.Fatalf("expected metadata, got %v", resp["type"])
	}
	topics := resp["topics"].([]interface{})
	if len(topics) != 1 {
		t.Fatalf("expected 1 topic, got %d", len(topics))
	}
	tp := topics[0].(map[string]interface{})
	if tp["name"] != "x" || tp["partitions"].(float64) != 2 {
		t.Fatalf("unexpected topic: %v", tp)
	}
}

func TestTCPErrorOnUnknownTopic(t *testing.T) {
	port, _ := startTestBroker(t)
	r, conn := tcpConnect(t, port)

	tcpSend(t, conn, fmt.Sprintf(`{"type":"produce","topic":"nope","payload":"%s"}`, b64("x")))
	resp := tcpRecv(t, r)
	if resp["type"] != "error" {
		t.Fatalf("expected error, got %v", resp["type"])
	}
}

func TestTCPParseErrorDoesNotCloseConnection(t *testing.T) {
	port, _ := startTestBroker(t)
	r, conn := tcpConnect(t, port)

	// Send garbage.
	tcpSend(t, conn, "not json at all")
	resp := tcpRecv(t, r)
	if resp["type"] != "error" {
		t.Fatalf("expected error, got %v", resp["type"])
	}

	// Connection must still be alive.
	tcpSend(t, conn, `{"type":"create_topic","topic":"y","partitions":1}`)
	resp2 := tcpRecv(t, r)
	if resp2["type"] != "topic_created" {
		t.Fatalf("expected topic_created, got %v", resp2["type"])
	}
}

func TestTCPMultipleConcurrentConnections(t *testing.T) {
	port, _ := startTestBroker(t)

	// Create topic first.
	r, conn := tcpConnect(t, port)
	tcpSend(t, conn, `{"type":"create_topic","topic":"c","partitions":1}`)
	tcpRecv(t, r)
	conn.Close()

	// 4 concurrent connections, each producing 10 messages.
	done := make(chan error, 4)
	for i := 0; i < 4; i++ {
		go func(i int) {
			conn2, err := net.Dial("tcp", fmt.Sprintf("127.0.0.1:%d", port))
			if err != nil {
				done <- err
				return
			}
			defer conn2.Close()
			rdr := bufio.NewReader(conn2)
			for j := 0; j < 10; j++ {
				payload := b64(fmt.Sprintf("%d-%d", i, j))
				fmt.Fprintf(conn2, `{"type":"produce","topic":"c","payload":"%s"}`+"\n", payload)
				line, err := rdr.ReadString('\n')
				if err != nil {
					done <- fmt.Errorf("read: %v", err)
					return
				}
				var m map[string]interface{}
				if err := json.Unmarshal([]byte(line), &m); err != nil {
					done <- fmt.Errorf("unmarshal: %v", err)
					return
				}
				if m["type"] != "produced" {
					done <- fmt.Errorf("expected produced, got %v", m["type"])
					return
				}
			}
			done <- nil
		}(i)
	}
	for i := 0; i < 4; i++ {
		if err := <-done; err != nil {
			t.Fatalf("concurrent connection %d: %v", i, err)
		}
	}

	// Fetch all 40 messages.
	r2, conn3 := tcpConnect(t, port)
	tcpSend(t, conn3, `{"type":"fetch_batch","topic":"c","partition":0,"offset":0,"max_count":50}`)
	count := 0
	for {
		resp := tcpRecv(t, r2)
		switch resp["type"] {
		case "record":
			count++
		case "end":
			goto done
		default:
			t.Fatalf("unexpected type: %v", resp["type"])
		}
	}
done:
	if count != 40 {
		t.Fatalf("expected 40 messages, got %d", count)
	}
}
