// Command main demonstrates two testing-toolkit ideas from this topic
// outside of _test.go files: running table-driven cases directly (here
// against a tiny length-prefixed message reader) and exercising an
// http.Handler with net/http/httptest. readMessage's length bound is the
// same defense ParseValue uses against the unbounded-allocation bug a fuzz
// test is built to catch.
package main

import (
	"bufio"
	"bytes"
	"encoding/binary"
	"fmt"
	"io"
	"net/http"
	"net/http/httptest"
)

const maxMessageLen = 1024

// readMessage reads a 2-byte big-endian length prefix followed by that many
// bytes, rejecting lengths over maxMessageLen before allocating.
func readMessage(r *bufio.Reader) ([]byte, error) {
	var length uint16
	if err := binary.Read(r, binary.BigEndian, &length); err != nil {
		return nil, err
	}
	if length > maxMessageLen {
		return nil, fmt.Errorf("message length %d exceeds max %d", length, maxMessageLen)
	}
	data := make([]byte, length)
	if _, err := io.ReadFull(r, data); err != nil {
		return nil, err
	}
	return data, nil
}

func main() {
	cases := []struct {
		name  string
		input []byte
	}{
		{"normal message", append([]byte{0x00, 0x05}, []byte("hello")...)},
		{"empty message", []byte{0x00, 0x00}},
		{"oversized length", []byte{0xFF, 0xFF}}, // 65535 > maxMessageLen
		{"truncated", []byte{0x00, 0x05, 'h', 'i'}},
	}

	fmt.Println("--- readMessage, run directly ---")
	for _, c := range cases {
		got, err := readMessage(bufio.NewReader(bytes.NewReader(c.input)))
		fmt.Printf("%-16s got=%-8q err=%v\n", c.name, got, err)
	}

	srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		msg, err := readMessage(bufio.NewReader(r.Body))
		if err != nil {
			http.Error(w, err.Error(), http.StatusBadRequest)
			return
		}
		fmt.Fprintf(w, "received %d bytes", len(msg))
	}))
	defer srv.Close()

	fmt.Println("--- same cases, over HTTP via httptest.NewServer ---")
	for _, c := range cases {
		resp, err := http.Post(srv.URL, "application/octet-stream", bytes.NewReader(c.input))
		if err != nil {
			fmt.Println("POST error:", err)
			continue
		}
		body, _ := io.ReadAll(resp.Body)
		resp.Body.Close()
		fmt.Printf("%-16s -> %s: %s\n", c.name, resp.Status, body)
	}
}
