// Package nettools applies the net package's Listen/Accept/Dial and UDP
// APIs to build a TCP echo server (with half-close framing) and a UDP
// request/response server, exercising connection deadlines from topic 7.
package nettools

import (
	"errors"
	"io"
	"net"
	"time"
)

// Serve accepts connections from l in a loop, calling handler in its own
// goroutine for each accepted connection. Serve blocks until l.Accept
// returns an error: if that error indicates l was closed (errors.Is(err,
// net.ErrClosed)), Serve returns nil; otherwise it returns the error.
func Serve(l net.Listener, handler func(net.Conn)) error {
	for {
		conn, err := l.Accept()
		if err != nil {
			if errors.Is(err, net.ErrClosed) {
				return nil
			}
			return err
		}

		go EchoHandler(conn)

	}
}

// EchoHandler reads all data sent on conn until the peer half-closes its
// write side (io.EOF), writes that data back unchanged, and closes conn.
func EchoHandler(conn net.Conn) error {
	data, err := io.ReadAll(conn)
	if err != nil {
		return err
	}

	bytesWritten, err := conn.Write(data)
	if err != nil {
		return err
	}

	if bytesWritten != len(data) {
		return errors.New("failed to write all the received data")
	}

	return conn.Close()
}

// DialAndSend connects to addr over network (e.g. "tcp"), writes data,
// half-closes its write side via (*net.TCPConn).CloseWrite, reads the
// peer's full response (bounded by timeout via SetDeadline), and returns
// it.
func DialAndSend(network, addr string, data []byte, timeout time.Duration) ([]byte, error) {
	conn, err := net.Dial(network, addr)
	if err != nil {
		return nil, err
	}

	bytesWritten, err := conn.Write(data)
	if err != nil {
		return nil, err
	}
	if bytesWritten != len(data) {
		return nil, errors.New("failed to write all the received data")
	}

	if network == "tcp" {
		err := conn.(*net.TCPConn).CloseWrite()
		if err != nil {
			return nil, err
		}
	}

	err = conn.SetReadDeadline(time.Now().Add(timeout))
	if err != nil {
		return nil, err
	}

	resp, err := io.ReadAll(conn)
	if err != nil {
		return nil, err
	}

	return resp, nil
}

// ServeUDP reads datagrams from conn in a loop, calls handler with each
// datagram's payload and sender address, and — if handler returns a
// non-nil response — writes that response back to the sender. ServeUDP
// returns nil once conn is closed (errors.Is(err, net.ErrClosed)),
// otherwise it returns the read error.
func ServeUDP(conn *net.UDPConn, handler func(data []byte, from *net.UDPAddr) []byte) error {
	buf := make([]byte, 1024)

	for {
		bytesRead, addr, err := conn.ReadFrom(buf)
		if err != nil {
			if errors.Is(err, net.ErrClosed) {
				return nil
			}
			return err
		}

		resp := handler(buf[:bytesRead], addr.(*net.UDPAddr))

		if resp != nil {
			_, err = conn.WriteToUDP(resp, addr.(*net.UDPAddr))

			if err != nil {
				return err
			}
		}
	}
}

// SendUDP sends data to addr over UDP, waits up to timeout (via
// SetReadDeadline) for a single response datagram, and returns its
// payload.
func SendUDP(addr string, data []byte, timeout time.Duration) ([]byte, error) {
	udpAddr, err := net.ResolveUDPAddr("udp", addr)
	if err != nil {
		return nil, err
	}

	conn, err := net.DialUDP("udp", nil, udpAddr)
	if err != nil {
		return nil, err
	}

	_, err = conn.Write(data)
	if err != nil {
		return nil, err
	}

	conn.SetReadDeadline(time.Now().Add(timeout))

	resp := make([]byte, 1024)
	bytesRead, err := conn.Read(resp)
	if err != nil && !errors.Is(err, net.ErrClosed) {
		return nil, err
	}

	return resp[:bytesRead], nil
}
