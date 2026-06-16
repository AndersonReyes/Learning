// Producer CLI — sends a single message to a topic.
//
// Usage:
//
//	producer [--host <host>] [--port <port>] [--topic <topic>] [--message <msg>] [--key <key>]
//
// Defaults: host=127.0.0.1, port=9092
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
	fmt.Fprintln(os.Stderr, "Usage: producer [--host <host>] [--port <port>] [--topic <topic>] [--message <msg>] [--key <key>]")
	os.Exit(1)
}

func parseArgs() (host string, port uint16, topic, message, key string) {
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
		case "--message":
			i++
			if i >= len(args) {
				usage()
			}
			message = args[i]
		case "--key":
			i++
			if i >= len(args) {
				usage()
			}
			key = args[i]
		default:
			usage()
		}
	}
	if topic == "" || message == "" {
		fmt.Fprintln(os.Stderr, "error: --topic and --message are required")
		usage()
	}
	return
}

func main() {
	host, port, topic, message, key := parseArgs()
	addr := fmt.Sprintf("%s:%d", host, port)

	conn, err := net.Dial("tcp", addr)
	if err != nil {
		fmt.Fprintf(os.Stderr, "connect %s: %v\n", addr, err)
		os.Exit(1)
	}
	defer conn.Close()

	payload := base64.StdEncoding.EncodeToString([]byte(message))

	var req map[string]interface{}
	if key != "" {
		encodedKey := base64.StdEncoding.EncodeToString([]byte(key))
		req = map[string]interface{}{
			"type":    "produce",
			"topic":   topic,
			"key":     encodedKey,
			"payload": payload,
		}
	} else {
		req = map[string]interface{}{
			"type":    "produce",
			"topic":   topic,
			"payload": payload,
		}
	}

	b, _ := json.Marshal(req)
	fmt.Fprintf(conn, "%s\n", b)

	reader := bufio.NewReader(conn)
	line, err := reader.ReadString('\n')
	if err != nil {
		fmt.Fprintf(os.Stderr, "read response: %v\n", err)
		os.Exit(1)
	}

	var resp map[string]interface{}
	if err := json.Unmarshal([]byte(line), &resp); err != nil {
		fmt.Fprintf(os.Stderr, "parse response: %v\n", err)
		os.Exit(1)
	}

	if resp["type"] == "error" {
		fmt.Fprintf(os.Stderr, "error: %v\n", resp["message"])
		os.Exit(1)
	}

	fmt.Printf("Produced to topic=%q partition=%v offset=%v\n", topic, resp["partition"], resp["offset"])
}
