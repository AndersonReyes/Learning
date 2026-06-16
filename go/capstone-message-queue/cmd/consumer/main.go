// Consumer CLI — fetches messages from a topic.
//
// Usage:
//
//	consumer [--host <host>] [--port <port>] [--topic <topic>] [--group <group>] [--partition <p>] [--offset <o>]
//
// Defaults: host=127.0.0.1, port=9092, partition=0, offset=0
package main

import (
	"bufio"
	"encoding/base64"
	"encoding/json"
	"fmt"
	"net"
	"os"
	"strconv"
)

func usage() {
	fmt.Fprintln(os.Stderr, "Usage: consumer [--host <host>] [--port <port>] [--topic <topic>] [--group <group>] [--partition <p>] [--offset <o>]")
	os.Exit(1)
}

func parseArgs() (host string, port uint16, topic, group string, partition uint32, offset uint64) {
	host = "127.0.0.1"
	port = 9092

	args := os.Args[1:]
	for i := 0; i < len(args); i++ {
		switch args[i] {
		case "--host":
			i++
			if i >= len(args) {
				usage()
			}
			host = args[i]
		case "--port":
			i++
			if i >= len(args) {
				usage()
			}
			p, err := strconv.ParseUint(args[i], 10, 16)
			if err != nil {
				usage()
			}
			port = uint16(p)
		case "--topic":
			i++
			if i >= len(args) {
				usage()
			}
			topic = args[i]
		case "--group":
			i++
			if i >= len(args) {
				usage()
			}
			group = args[i]
		case "--partition":
			i++
			if i >= len(args) {
				usage()
			}
			p, err := strconv.ParseUint(args[i], 10, 32)
			if err != nil {
				usage()
			}
			partition = uint32(p)
		case "--offset":
			i++
			if i >= len(args) {
				usage()
			}
			o, err := strconv.ParseUint(args[i], 10, 64)
			if err != nil {
				usage()
			}
			offset = o
		default:
			usage()
		}
	}
	if topic == "" {
		fmt.Fprintln(os.Stderr, "error: --topic is required")
		usage()
	}
	return
}

func main() {
	host, port, topic, group, partition, startOffset := parseArgs()
	addr := fmt.Sprintf("%s:%d", host, port)

	conn, err := net.Dial("tcp", addr)
	if err != nil {
		fmt.Fprintf(os.Stderr, "connect %s: %v\n", addr, err)
		os.Exit(1)
	}
	defer conn.Close()

	reader := bufio.NewReader(conn)

	// If a group is provided, join it first.
	var memberID string
	if group != "" {
		req := map[string]interface{}{
			"type":   "join_group",
			"group":  group,
			"topics": []string{topic},
		}
		b, _ := json.Marshal(req)
		fmt.Fprintf(conn, "%s\n", b)

		line, err := reader.ReadString('\n')
		if err != nil {
			fmt.Fprintf(os.Stderr, "read join response: %v\n", err)
			os.Exit(1)
		}
		var resp map[string]interface{}
		json.Unmarshal([]byte(line), &resp)
		if resp["type"] == "error" {
			fmt.Fprintf(os.Stderr, "join error: %v\n", resp["message"])
			os.Exit(1)
		}
		memberID = resp["member_id"].(string)
		fmt.Printf("Joined group %q as %q\n", group, memberID)
	}

	// Fetch a batch of messages.
	req := map[string]interface{}{
		"type":      "fetch_batch",
		"topic":     topic,
		"partition": partition,
		"offset":    startOffset,
		"max_count": 100,
	}
	b, _ := json.Marshal(req)
	fmt.Fprintf(conn, "%s\n", b)

	for {
		line, err := reader.ReadString('\n')
		if err != nil {
			break
		}
		var resp map[string]interface{}
		if err := json.Unmarshal([]byte(line), &resp); err != nil {
			continue
		}
		switch resp["type"] {
		case "record":
			payload, _ := base64.StdEncoding.DecodeString(resp["payload"].(string))
			fmt.Printf("offset=%v message=%q\n", resp["offset"], string(payload))
		case "end":
			goto done
		case "error":
			fmt.Fprintf(os.Stderr, "error: %v\n", resp["message"])
			os.Exit(1)
		}
	}
done:

	// If a group, leave it.
	if group != "" && memberID != "" {
		req := map[string]interface{}{
			"type":      "leave_group",
			"group":     group,
			"member_id": memberID,
		}
		b, _ := json.Marshal(req)
		fmt.Fprintf(conn, "%s\n", b)
		reader.ReadString('\n') // discard response
	}
}
