package server

import (
	"bufio"
	"encoding/base64"
	"encoding/json"
	"fmt"
	"net"
	"strings"
	"testing"
	"time"

	"github.com/andersonreyes/learning/go/capstone-message-queue/broker"
	"github.com/andersonreyes/learning/go/capstone-message-queue/concurrent"
	"github.com/andersonreyes/learning/go/capstone-message-queue/groups"
	"github.com/andersonreyes/learning/go/capstone-message-queue/protocol"
)

// ── helpers ───────────────────────────────────────────────────────────────────

func makeHandle(t *testing.T) *BrokerHandle {
	t.Helper()
	dir := t.TempDir()
	reg, err := broker.OpenRegistry(dir)
	if err != nil {
		t.Fatalf("broker.OpenRegistry: %v", err)
	}
	s := concurrent.NewSharedRegistry(reg, time.Hour)
	t.Cleanup(func() { s.Close() }) //nolint:errcheck
	return &BrokerHandle{
		Registry: s,
		Groups:   groups.NewGroupCoordinator(),
	}
}

// startServer spins up a RunServer goroutine on a random port and returns
// a connected client net.Conn plus a cleanup function that closes the listener.
func startServer(t *testing.T, h *BrokerHandle) (conn net.Conn, cleanup func()) {
	t.Helper()
	l, err := net.Listen("tcp", "127.0.0.1:0")
	if err != nil {
		t.Fatalf("Listen: %v", err)
	}
	go RunServer(l, h)

	conn, err = net.Dial("tcp", l.Addr().String())
	if err != nil {
		l.Close()
		t.Fatalf("Dial: %v", err)
	}
	conn.SetDeadline(time.Now().Add(5 * time.Second))
	return conn, func() {
		conn.Close()
		l.Close()
	}
}

// sendLine writes a JSON-encoded value followed by '\n'.
func sendLine(t *testing.T, conn net.Conn, v interface{}) {
	t.Helper()
	b, err := json.Marshal(v)
	if err != nil {
		t.Fatalf("json.Marshal: %v", err)
	}
	if _, err := fmt.Fprintf(conn, "%s\n", b); err != nil {
		t.Fatalf("write request: %v", err)
	}
}

// recvLine reads and decodes the next JSON line.
func recvLine(t *testing.T, conn net.Conn) map[string]interface{} {
	t.Helper()
	scanner := bufio.NewScanner(conn)
	if !scanner.Scan() {
		t.Fatalf("reading response: %v", scanner.Err())
	}
	var m map[string]interface{}
	if err := json.Unmarshal(scanner.Bytes(), &m); err != nil {
		t.Fatalf("unmarshal response %q: %v", scanner.Text(), err)
	}
	return m
}

// b64 encodes bytes to standard base64.
func b64(b []byte) string { return base64.StdEncoding.EncodeToString(b) }

// assertType checks the "type" field of a response map.
func assertType(t *testing.T, resp map[string]interface{}, want string) {
	t.Helper()
	got, _ := resp["type"].(string)
	if got != want {
		t.Errorf("response type = %q, want %q (full response: %v)", got, want, resp)
	}
}

func assertNoError(t *testing.T, resp map[string]interface{}) {
	t.Helper()
	if tp, _ := resp["type"].(string); tp == "error" {
		t.Errorf("unexpected error response: %v", resp)
	}
}

// ── ProcessRequest unit tests ─────────────────────────────────────────────────

func TestProcessRequestCreateTopic(t *testing.T) {
	h := makeHandle(t)

	resp := ProcessRequest(h, &protocol.Request{
		Type:       "create_topic",
		Topic:      "events",
		Partitions: 3,
	})
	if len(resp) != 1 {
		t.Fatalf("ProcessRequest returned %d responses, want 1", len(resp))
	}
	assertType(t, resp[0], "ok")
}

func TestProcessRequestProduce(t *testing.T) {
	h := makeHandle(t)
	ProcessRequest(h, &protocol.Request{Type: "create_topic", Topic: "t", Partitions: 1})

	resp := ProcessRequest(h, &protocol.Request{
		Type:    "produce",
		Topic:   "t",
		Payload: b64([]byte("hello")),
	})
	if len(resp) != 1 {
		t.Fatalf("got %d responses, want 1", len(resp))
	}
	assertType(t, resp[0], "produced")
	if _, ok := resp[0]["partition"]; !ok {
		t.Error("produced response missing 'partition' field")
	}
	if _, ok := resp[0]["offset"]; !ok {
		t.Error("produced response missing 'offset' field")
	}
}

func TestProcessRequestFetch(t *testing.T) {
	h := makeHandle(t)
	ProcessRequest(h, &protocol.Request{Type: "create_topic", Topic: "t", Partitions: 1})
	p := ProcessRequest(h, &protocol.Request{
		Type:    "produce",
		Topic:   "t",
		Payload: b64([]byte("world")),
	})
	assertType(t, p[0], "produced")

	partition := uint32(p[0]["partition"].(float64))
	offset := uint64(p[0]["offset"].(float64))

	resp := ProcessRequest(h, &protocol.Request{
		Type:      "fetch",
		Topic:     "t",
		Partition: partition,
		Offset:    offset,
	})
	assertType(t, resp[0], "fetched")

	rawPayload, _ := resp[0]["payload"].(string)
	decoded, err := base64.StdEncoding.DecodeString(rawPayload)
	if err != nil {
		t.Fatalf("decode payload: %v", err)
	}
	if string(decoded) != "world" {
		t.Errorf("fetched payload = %q, want \"world\"", decoded)
	}
}

func TestProcessRequestFetchBatch(t *testing.T) {
	h := makeHandle(t)
	ProcessRequest(h, &protocol.Request{Type: "create_topic", Topic: "b", Partitions: 1})
	for i := range 5 {
		ProcessRequest(h, &protocol.Request{
			Type:    "produce",
			Topic:   "b",
			Payload: b64([]byte{byte(i)}),
		})
	}

	resp := ProcessRequest(h, &protocol.Request{
		Type:      "fetch_batch",
		Topic:     "b",
		Partition: 0,
		Offset:    1,
		MaxCount:  3,
	})
	assertType(t, resp[0], "fetched_batch")

	records, _ := resp[0]["records"].([]interface{})
	if len(records) != 3 {
		t.Fatalf("fetched_batch records = %d, want 3", len(records))
	}
}

func TestProcessRequestMetadata(t *testing.T) {
	h := makeHandle(t)
	ProcessRequest(h, &protocol.Request{Type: "create_topic", Topic: "aaa", Partitions: 2})
	ProcessRequest(h, &protocol.Request{Type: "create_topic", Topic: "bbb", Partitions: 4})

	resp := ProcessRequest(h, &protocol.Request{Type: "metadata"})
	assertType(t, resp[0], "metadata")

	topics, _ := resp[0]["topics"].([]interface{})
	if len(topics) != 2 {
		t.Fatalf("metadata topics = %d, want 2", len(topics))
	}
	first := topics[0].(map[string]interface{})
	if first["name"] != "aaa" {
		t.Errorf("first topic name = %v, want \"aaa\"", first["name"])
	}
}

func TestProcessRequestUnknownType(t *testing.T) {
	h := makeHandle(t)
	resp := ProcessRequest(h, &protocol.Request{Type: "bogus"})
	assertType(t, resp[0], "error")
	msg, _ := resp[0]["message"].(string)
	if !strings.Contains(msg, "bogus") {
		t.Errorf("error message %q should mention the unknown type", msg)
	}
}

func TestProcessRequestFetchUnknownTopic(t *testing.T) {
	h := makeHandle(t)
	resp := ProcessRequest(h, &protocol.Request{
		Type:      "fetch",
		Topic:     "nope",
		Partition: 0,
		Offset:    0,
	})
	assertType(t, resp[0], "error")
}

func TestProcessRequestProduceWithKey(t *testing.T) {
	h := makeHandle(t)
	ProcessRequest(h, &protocol.Request{Type: "create_topic", Topic: "k", Partitions: 4})

	key := b64([]byte("user-1"))
	resp1 := ProcessRequest(h, &protocol.Request{
		Type:    "produce",
		Topic:   "k",
		Payload: b64([]byte("v")),
		Key:     key,
	})
	resp2 := ProcessRequest(h, &protocol.Request{
		Type:    "produce",
		Topic:   "k",
		Payload: b64([]byte("v")),
		Key:     key,
	})

	p1 := resp1[0]["partition"]
	p2 := resp2[0]["partition"]
	if p1 != p2 {
		t.Errorf("same key routed to different partitions: %v vs %v", p1, p2)
	}
}

func TestProcessRequestJoinGroup(t *testing.T) {
	h := makeHandle(t)
	ProcessRequest(h, &protocol.Request{Type: "create_topic", Topic: "t", Partitions: 2})

	resp := ProcessRequest(h, &protocol.Request{
		Type:   "join_group",
		Group:  "g",
		Topics: []string{"t"},
	})
	assertType(t, resp[0], "joined")
	if _, ok := resp[0]["member_id"]; !ok {
		t.Error("joined response missing 'member_id'")
	}
	if _, ok := resp[0]["assignments"]; !ok {
		t.Error("joined response missing 'assignments'")
	}
}

func TestProcessRequestLeaveGroup(t *testing.T) {
	h := makeHandle(t)
	ProcessRequest(h, &protocol.Request{Type: "create_topic", Topic: "t", Partitions: 1})
	joinResp := ProcessRequest(h, &protocol.Request{
		Type:   "join_group",
		Group:  "g",
		Topics: []string{"t"},
	})
	memberID, _ := joinResp[0]["member_id"].(string)

	resp := ProcessRequest(h, &protocol.Request{
		Type:     "leave_group",
		Group:    "g",
		MemberID: memberID,
	})
	assertType(t, resp[0], "ok")
}

func TestProcessRequestCommitAndFetchOffset(t *testing.T) {
	h := makeHandle(t)

	commitResp := ProcessRequest(h, &protocol.Request{
		Type:      "commit_offset",
		Group:     "g",
		Topic:     "t",
		Partition: 0,
		Offset:    42,
	})
	assertType(t, commitResp[0], "ok")

	fetchResp := ProcessRequest(h, &protocol.Request{
		Type:      "fetch_offset",
		Group:     "g",
		Topic:     "t",
		Partition: 0,
	})
	assertType(t, fetchResp[0], "offset")
	off, ok := fetchResp[0]["offset"].(float64)
	if !ok {
		t.Fatalf("fetch_offset response offset field: %v (type %T)", fetchResp[0]["offset"], fetchResp[0]["offset"])
	}
	if uint64(off) != 42 {
		t.Errorf("fetch_offset = %v, want 42", off)
	}
}

func TestProcessRequestFetchOffsetNullBeforeCommit(t *testing.T) {
	h := makeHandle(t)

	resp := ProcessRequest(h, &protocol.Request{
		Type:      "fetch_offset",
		Group:     "g",
		Topic:     "t",
		Partition: 0,
	})
	assertType(t, resp[0], "offset")

	// The offset field must be null (JSON null) when no offset has been committed.
	// In a map[string]interface{}, null JSON decodes as nil, and a key with nil
	// value is present. Check that the key exists and is nil.
	offVal, exists := resp[0]["offset"]
	if !exists {
		t.Fatal("fetch_offset response missing 'offset' field")
	}
	if offVal != nil {
		t.Errorf("fetch_offset before commit = %v, want null/nil", offVal)
	}
}

// ── TCP end-to-end tests ──────────────────────────────────────────────────────

func TestTCPCreateTopicAndProduce(t *testing.T) {
	h := makeHandle(t)
	conn, cleanup := startServer(t, h)
	defer cleanup()

	// Create topic.
	sendLine(t, conn, map[string]interface{}{
		"type":       "create_topic",
		"topic":      "events",
		"partitions": 3,
	})
	resp := recvLine(t, conn)
	assertType(t, resp, "ok")

	// Produce.
	sendLine(t, conn, map[string]interface{}{
		"type":    "produce",
		"topic":   "events",
		"payload": b64([]byte("hello tcp")),
	})
	resp = recvLine(t, conn)
	assertType(t, resp, "produced")
	assertNoError(t, resp)
}

func TestTCPFetchRoundtrip(t *testing.T) {
	h := makeHandle(t)
	conn, cleanup := startServer(t, h)
	defer cleanup()

	sendLine(t, conn, map[string]interface{}{"type": "create_topic", "topic": "t", "partitions": 1})
	recvLine(t, conn) // ok

	sendLine(t, conn, map[string]interface{}{
		"type":    "produce",
		"topic":   "t",
		"payload": b64([]byte("tcp payload")),
	})
	produceResp := recvLine(t, conn)
	assertType(t, produceResp, "produced")

	partition := uint32(produceResp["partition"].(float64))
	offset := uint64(produceResp["offset"].(float64))

	sendLine(t, conn, map[string]interface{}{
		"type":      "fetch",
		"topic":     "t",
		"partition": partition,
		"offset":    offset,
	})
	fetchResp := recvLine(t, conn)
	assertType(t, fetchResp, "fetched")

	rawPayload, _ := fetchResp["payload"].(string)
	decoded, _ := base64.StdEncoding.DecodeString(rawPayload)
	if string(decoded) != "tcp payload" {
		t.Errorf("fetched payload = %q, want \"tcp payload\"", decoded)
	}
}

func TestTCPMetadata(t *testing.T) {
	h := makeHandle(t)
	conn, cleanup := startServer(t, h)
	defer cleanup()

	sendLine(t, conn, map[string]interface{}{"type": "create_topic", "topic": "mt", "partitions": 2})
	recvLine(t, conn)

	sendLine(t, conn, map[string]interface{}{"type": "metadata"})
	resp := recvLine(t, conn)
	assertType(t, resp, "metadata")
	topics, _ := resp["topics"].([]interface{})
	if len(topics) != 1 {
		t.Errorf("metadata topics count = %d, want 1", len(topics))
	}
}

func TestTCPParseErrorKeepsConnectionOpen(t *testing.T) {
	h := makeHandle(t)
	conn, cleanup := startServer(t, h)
	defer cleanup()

	// Send invalid JSON.
	fmt.Fprintf(conn, "not json at all\n")
	resp := recvLine(t, conn)
	assertType(t, resp, "error")

	// Connection still works: send a valid request after the error.
	sendLine(t, conn, map[string]interface{}{"type": "metadata"})
	resp = recvLine(t, conn)
	assertType(t, resp, "metadata")
}

func TestTCPMultipleRequestsOnOneConnection(t *testing.T) {
	h := makeHandle(t)
	conn, cleanup := startServer(t, h)
	defer cleanup()

	sendLine(t, conn, map[string]interface{}{"type": "create_topic", "topic": "multi", "partitions": 1})
	recvLine(t, conn)

	for i := range 5 {
		sendLine(t, conn, map[string]interface{}{
			"type":    "produce",
			"topic":   "multi",
			"payload": b64([]byte(fmt.Sprintf("msg-%d", i))),
		})
		resp := recvLine(t, conn)
		assertType(t, resp, "produced")
	}
}

func TestTCPConcurrentConnections(t *testing.T) {
	h := makeHandle(t)
	l, err := net.Listen("tcp", "127.0.0.1:0")
	if err != nil {
		t.Fatalf("Listen: %v", err)
	}
	go RunServer(l, h)
	defer l.Close()

	addr := l.Addr().String()

	// Pre-create the topic so all goroutines can produce without racing on
	// topic creation errors.
	setupConn, _ := net.DialTimeout("tcp", addr, 2*time.Second)
	setupConn.SetDeadline(time.Now().Add(2 * time.Second))
	fmt.Fprintf(setupConn, `{"type":"create_topic","topic":"cc","partitions":4}`+"\n")
	bufio.NewScanner(setupConn).Scan() // drain ok
	setupConn.Close()

	const clients = 5
	errs := make(chan error, clients*10)
	done := make(chan struct{})

	for range clients {
		go func() {
			c, err := net.DialTimeout("tcp", addr, 2*time.Second)
			if err != nil {
				errs <- err
				return
			}
			defer c.Close()
			c.SetDeadline(time.Now().Add(5 * time.Second))
			sc := bufio.NewScanner(c)
			for range 10 {
				fmt.Fprintf(c, `{"type":"produce","topic":"cc","payload":"%s"}`+"\n",
					b64([]byte("x")))
				if !sc.Scan() {
					errs <- fmt.Errorf("scan: %v", sc.Err())
					return
				}
				var resp map[string]interface{}
				if err := json.Unmarshal(sc.Bytes(), &resp); err != nil {
					errs <- err
					return
				}
				if resp["type"] != "produced" {
					errs <- fmt.Errorf("expected produced, got %v", resp)
				}
			}
		}()
	}

	// Wait for all goroutines to finish (with timeout).
	go func() {
		// crude: just wait enough time then signal done
		time.Sleep(4 * time.Second)
		close(done)
	}()

	<-done
	close(errs)
	for err := range errs {
		t.Errorf("concurrent client error: %v", err)
	}
}

func TestTCPFetchBatch(t *testing.T) {
	h := makeHandle(t)
	conn, cleanup := startServer(t, h)
	defer cleanup()

	sendLine(t, conn, map[string]interface{}{"type": "create_topic", "topic": "fb", "partitions": 1})
	recvLine(t, conn)

	for i := range 6 {
		sendLine(t, conn, map[string]interface{}{
			"type":    "produce",
			"topic":   "fb",
			"payload": b64([]byte{byte(i)}),
		})
		recvLine(t, conn) // drain produced
	}

	sendLine(t, conn, map[string]interface{}{
		"type":      "fetch_batch",
		"topic":     "fb",
		"partition": 0,
		"offset":    2,
		"max_count": 3,
	})
	resp := recvLine(t, conn)
	assertType(t, resp, "fetched_batch")
	records, _ := resp["records"].([]interface{})
	if len(records) != 3 {
		t.Errorf("fetched_batch records = %d, want 3", len(records))
	}
}

func TestTCPGroupJoinAndCommit(t *testing.T) {
	h := makeHandle(t)
	conn, cleanup := startServer(t, h)
	defer cleanup()

	sendLine(t, conn, map[string]interface{}{"type": "create_topic", "topic": "ev", "partitions": 2})
	recvLine(t, conn)

	sendLine(t, conn, map[string]interface{}{
		"type":   "join_group",
		"group":  "consumers",
		"topics": []string{"ev"},
	})
	joinResp := recvLine(t, conn)
	assertType(t, joinResp, "joined")
	memberID, _ := joinResp["member_id"].(string)
	if memberID == "" {
		t.Fatal("joined response missing member_id")
	}

	sendLine(t, conn, map[string]interface{}{
		"type":      "commit_offset",
		"group":     "consumers",
		"topic":     "ev",
		"partition": 0,
		"offset":    7,
	})
	recvLine(t, conn) // ok

	sendLine(t, conn, map[string]interface{}{
		"type":      "fetch_offset",
		"group":     "consumers",
		"topic":     "ev",
		"partition": 0,
	})
	offsetResp := recvLine(t, conn)
	assertType(t, offsetResp, "offset")
	off, _ := offsetResp["offset"].(float64)
	if uint64(off) != 7 {
		t.Errorf("fetch_offset = %v, want 7", offsetResp["offset"])
	}
}
