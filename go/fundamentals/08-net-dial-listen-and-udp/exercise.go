// Package nettools applies the net package's Listen/Accept/Dial and UDP
// APIs to build a TCP echo server (with half-close framing) and a UDP
// request/response server, exercising connection deadlines from topic 7.
package nettools

import (
	"errors"
	"net"
	"time"
)

// Serve accepts connections from l in a loop, calling handler in its own
// goroutine for each accepted connection. Serve blocks until l.Accept
// returns an error: if that error indicates l was closed (errors.Is(err,
// net.ErrClosed)), Serve returns nil; otherwise it returns the error.
func Serve(l net.Listener, handler func(net.Conn)) error {
	return errors.New("not implemented")
}

// EchoHandler reads all data sent on conn until the peer half-closes its
// write side (io.EOF), writes that data back unchanged, and closes conn.
func EchoHandler(conn net.Conn) error {
	return errors.New("not implemented")
}

// DialAndSend connects to addr over network (e.g. "tcp"), writes data,
// half-closes its write side via (*net.TCPConn).CloseWrite, reads the
// peer's full response (bounded by timeout via SetDeadline), and returns
// it.
func DialAndSend(network, addr string, data []byte, timeout time.Duration) ([]byte, error) {
	return nil, errors.New("not implemented")
}

// ServeUDP reads datagrams from conn in a loop, calls handler with each
// datagram's payload and sender address, and — if handler returns a
// non-nil response — writes that response back to the sender. ServeUDP
// returns nil once conn is closed (errors.Is(err, net.ErrClosed)),
// otherwise it returns the read error.
func ServeUDP(conn *net.UDPConn, handler func(data []byte, from *net.UDPAddr) []byte) error {
	return errors.New("not implemented")
}

// SendUDP sends data to addr over UDP, waits up to timeout (via
// SetReadDeadline) for a single response datagram, and returns its
// payload.
func SendUDP(addr string, data []byte, timeout time.Duration) ([]byte, error) {
	return nil, errors.New("not implemented")
}
