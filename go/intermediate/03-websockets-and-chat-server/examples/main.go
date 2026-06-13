// Command main demonstrates the WebSocket opening handshake (RFC 6455 §1.3)
// as a raw HTTP/1.1 Upgrade exchange: a net/http handler obtains the
// underlying net.Conn via http.Hijacker (topic 10's HTTP internals), computes
// Sec-WebSocket-Accept by hand, and the two ends then exchange a single
// minimal unmasked text frame over the upgraded connection — the "raw
// protocol" counterpart to this topic's ComputeAcceptKey/WriteFrame/
// ReadFrame/Hub exercise.
package main

import (
	"bufio"
	"crypto/sha1"
	"encoding/base64"
	"fmt"
	"io"
	"net"
	"net/http"
	"time"
)

// websocketGUID is appended to a client's Sec-WebSocket-Key before hashing
// to compute Sec-WebSocket-Accept (RFC 6455 §1.3, §4.2.2).
const websocketGUID = "258EAFA5-E914-47DA-95CA-C5AB0DC85B11"

func acceptKey(clientKey string) string {
	sum := sha1.Sum([]byte(clientKey + websocketGUID))
	return base64.StdEncoding.EncodeToString(sum[:])
}

func main() {
	ln, err := net.Listen("tcp", "127.0.0.1:0")
	if err != nil {
		fmt.Println("listen error:", err)
		return
	}
	defer ln.Close()

	done := make(chan struct{})

	srv := &http.Server{
		Handler: http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
			defer close(done)

			if r.Header.Get("Upgrade") != "websocket" {
				http.Error(w, "expected websocket upgrade", http.StatusBadRequest)
				return
			}

			conn, buf, err := w.(http.Hijacker).Hijack()
			if err != nil {
				fmt.Println("server: hijack error:", err)
				return
			}
			defer conn.Close()

			accept := acceptKey(r.Header.Get("Sec-WebSocket-Key"))
			fmt.Fprintf(buf, "HTTP/1.1 101 Switching Protocols\r\n"+
				"Upgrade: websocket\r\n"+
				"Connection: Upgrade\r\n"+
				"Sec-WebSocket-Accept: %s\r\n\r\n", accept)
			buf.Flush()

			// A single unmasked text frame: FIN=1/opcode=0x1 (byte 0),
			// MASK=0/length<=125 (byte 1), followed by the payload.
			header := make([]byte, 2)
			if _, err := io.ReadFull(buf, header); err != nil {
				fmt.Println("server: read header error:", err)
				return
			}
			payload := make([]byte, header[1]&0x7F)
			if _, err := io.ReadFull(buf, payload); err != nil {
				fmt.Println("server: read payload error:", err)
				return
			}
			fmt.Printf("server: received frame opcode=%#x payload=%q\n", header[0]&0x0F, payload)
		}),
	}
	go srv.Serve(ln)
	defer srv.Close()

	conn, err := net.Dial("tcp", ln.Addr().String())
	if err != nil {
		fmt.Println("dial error:", err)
		return
	}
	defer conn.Close()

	clientKey := "dGhlIHNhbXBsZSBub25jZQ=="
	fmt.Fprintf(conn, "GET /chat HTTP/1.1\r\n"+
		"Host: example.com\r\n"+
		"Upgrade: websocket\r\n"+
		"Connection: Upgrade\r\n"+
		"Sec-WebSocket-Key: %s\r\n"+
		"Sec-WebSocket-Version: 13\r\n\r\n", clientKey)

	resp, err := http.ReadResponse(bufio.NewReader(conn), nil)
	if err != nil {
		fmt.Println("read response error:", err)
		return
	}
	fmt.Println("status:", resp.Status)
	fmt.Println("Sec-WebSocket-Accept:", resp.Header.Get("Sec-WebSocket-Accept"))
	fmt.Println("expected (RFC 6455 worked example):", acceptKey(clientKey))

	// Unmasked text frame "hi": FIN=1/opcode=0x1, MASK=0/length=2.
	conn.Write([]byte{0x81, 0x02, 'h', 'i'})

	select {
	case <-done:
	case <-time.After(2 * time.Second):
		fmt.Println("timed out waiting for server")
	}
}
