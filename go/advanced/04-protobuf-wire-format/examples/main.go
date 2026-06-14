// Command main demonstrates this topic's exercise on a different "service"
// than exercise_test.go: an EchoService request/response exchange, with a
// nested Metadata message (field 3, decoded via DecodeNestedMessage's
// messageFields — exercise_test.go exercises this on its own, not combined
// with a network round trip), framed gRPC-style (1-byte compression flag +
// 4-byte big-endian length, see notes.md) and sent over a real TCP loopback
// connection — illustrating "Service-to-Service Networking" at the level
// protoc-gen-go-grpc normally generates for you.
//
// Against the stub exercise.go, EncodeMessage returns its "not implemented"
// error before any bytes are sent or any connection is made; this program
// prints that instead of treating it as fatal.
package main

import (
	"encoding/binary"
	"fmt"
	"io"
	"net"

	minipb "github.com/andersonreyes/learning/go/advanced/04-protobuf-wire-format"
)

const (
	fieldRequestID = 1
	fieldName      = 2
	fieldMetadata  = 3

	fieldMetaTimestamp = 1

	fieldResponseID = 1
	fieldGreeting   = 2
)

// writeFrame writes msg with a gRPC-style frame header: 1 byte (compression
// flag, always 0 here) followed by 4 bytes (big-endian payload length).
func writeFrame(w io.Writer, msg []byte) error {
	header := make([]byte, 5)
	binary.BigEndian.PutUint32(header[1:], uint32(len(msg)))
	if _, err := w.Write(header); err != nil {
		return err
	}
	_, err := w.Write(msg)
	return err
}

// readFrame reads a frame written by writeFrame and returns its payload.
func readFrame(r io.Reader) ([]byte, error) {
	header := make([]byte, 5)
	if _, err := io.ReadFull(r, header); err != nil {
		return nil, err
	}
	payload := make([]byte, binary.BigEndian.Uint32(header[1:]))
	if _, err := io.ReadFull(r, payload); err != nil {
		return nil, err
	}
	return payload, nil
}

// serveEcho accepts a single connection, decodes one framed EchoService
// request (including its nested Metadata message), and writes back a framed
// response greeting the requested name.
func serveEcho(listener net.Listener) {
	conn, err := listener.Accept()
	if err != nil {
		return
	}
	defer conn.Close()

	request, err := readFrame(conn)
	if err != nil {
		fmt.Printf("server: readFrame error: %v\n", err)
		return
	}

	fields, err := minipb.DecodeNestedMessage(request, map[int]bool{fieldMetadata: true})
	if err != nil {
		fmt.Printf("server: DecodeNestedMessage error: %v\n", err)
		return
	}
	fmt.Printf("server decoded request: %+v\n", fields)

	name, _ := fields[fieldName].([]byte)
	response, err := minipb.EncodeMessage(map[int]any{
		fieldResponseID: fields[fieldRequestID],
		fieldGreeting:   []byte("Hello, " + string(name) + "!"),
	})
	if err != nil {
		fmt.Printf("server: EncodeMessage error: %v\n", err)
		return
	}

	if err := writeFrame(conn, response); err != nil {
		fmt.Printf("server: writeFrame error: %v\n", err)
	}
}

func main() {
	fmt.Println("--- encoding an EchoService request (with a nested Metadata message) ---")

	metadata, err := minipb.EncodeMessage(map[int]any{
		fieldMetaTimestamp: uint64(1718000000),
	})
	if err != nil {
		fmt.Printf("EncodeMessage (metadata) error: %v\n", err)
		return
	}

	request, err := minipb.EncodeMessage(map[int]any{
		fieldRequestID: uint64(42),
		fieldName:      []byte("Ada"),
		fieldMetadata:  metadata,
	})
	if err != nil {
		fmt.Printf("EncodeMessage (request) error: %v\n", err)
		return
	}
	fmt.Printf("request bytes: % x\n", request)

	fmt.Println("\n--- sending the request to an EchoService over TCP loopback ---")
	listener, err := net.Listen("tcp", "127.0.0.1:0")
	if err != nil {
		fmt.Printf("net.Listen error: %v\n", err)
		return
	}
	defer listener.Close()
	go serveEcho(listener)

	conn, err := net.Dial("tcp", listener.Addr().String())
	if err != nil {
		fmt.Printf("net.Dial error: %v\n", err)
		return
	}
	defer conn.Close()

	if err := writeFrame(conn, request); err != nil {
		fmt.Printf("writeFrame error: %v\n", err)
		return
	}

	response, err := readFrame(conn)
	if err != nil {
		fmt.Printf("readFrame error: %v\n", err)
		return
	}
	fmt.Printf("response bytes: % x\n", response)

	fmt.Println("\n--- decoding the response ---")
	fields, err := minipb.DecodeMessage(response)
	if err != nil {
		fmt.Printf("DecodeMessage error: %v\n", err)
		return
	}
	fmt.Printf("response fields: %+v\n", fields)
	if greeting, ok := fields[fieldGreeting].([]byte); ok {
		fmt.Printf("greeting: %q\n", greeting)
	}
}
