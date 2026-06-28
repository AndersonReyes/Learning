// Package transport defines small interfaces mirroring net.Conn/net.Addr,
// an in-memory Conn implementation for testing, and helpers for copying
// data between Conns and validating their addresses.
package transport

import (
	"errors"
	"fmt"
	"io"
)

// ErrConnClosed is returned by Read, Write, and Close on a *BufferConn
// that has already been closed (and by Close itself on a second call).
var ErrConnClosed = errors.New("connection closed")

// ErrNilAddr indicates a Conn whose RemoteAddr returned nil.
var ErrNilAddr = errors.New("nil remote address")

// ErrUnknownNetwork indicates a Conn whose RemoteAddr().Network() is
// neither "tcp" nor "udp".
var ErrUnknownNetwork = errors.New("unknown network")

// Addr mirrors net.Addr: a network endpoint address.
type Addr interface {
	Network() string // "tcp", "udp", etc.
	String() string  // string form of the address, e.g. "192.0.2.1:80"
}

// TCPAddr and UDPAddr are concrete Addr implementations.
type TCPAddr struct {
	IP   string
	Port int
}

func (a TCPAddr) Network() string { return "tcp" }
func (a TCPAddr) String() string  { return fmt.Sprintf("%s:%d", a.IP, a.Port) }

// UDPAddr is the UDP counterpart of TCPAddr.
type UDPAddr struct {
	IP   string
	Port int
}

func (a UDPAddr) Network() string { return "udp" }
func (a UDPAddr) String() string  { return fmt.Sprintf("%s:%d", a.IP, a.Port) }

// Conn mirrors a minimal subset of net.Conn.
type Conn interface {
	Read(p []byte) (int, error)
	Write(p []byte) (int, error)
	Close() error
	RemoteAddr() Addr
}

// BufferConn is an in-memory Conn backed by a byte slice, useful for
// exercising code written against Conn without real sockets. Writes
// append to the buffer; reads consume it from the front.
type BufferConn struct {
	data   []byte
	pos    int
	addr   Addr
	closed bool
}

// NewBufferConn returns a *BufferConn whose buffer is pre-loaded with a
// copy of data (available to Read) and whose RemoteAddr returns addr.
func NewBufferConn(data []byte, addr Addr) *BufferConn {
	return &BufferConn{data: append([]byte(nil), data...), addr: addr}
}

// RemoteAddr returns the connection's remote address.
func (c *BufferConn) RemoteAddr() Addr {
	return c.addr
}

// Write appends p to c's buffer.
//
//	c is closed -> 0, error wrapping ErrConnClosed
//	otherwise   -> len(p), nil
func (c *BufferConn) Write(p []byte) (int, error) {
	if c.closed {
		return 0, errors.Join(errors.New("failed to write to buffercon"), ErrConnClosed)
	}

	c.data = append(c.data, p...)
	return len(p), nil
}

// Read copies up to len(p) bytes from c's unread buffer into p, advancing
// the read position.
//
//	all data already read -> 0, io.EOF
//	c is closed            -> 0, error wrapping ErrConnClosed
//	otherwise               -> n>0, nil   (n = min(len(p), unread bytes))
func (c *BufferConn) Read(p []byte) (int, error) {
	if c.closed {
		return 0, errors.Join(errors.New("onnection closed"), ErrConnClosed)
	}

	if c.pos >= len(c.data) {
		return 0, io.EOF

	}

	unread := len(c.data) - c.pos
	n := min(len(p), unread)
	end := c.pos + n
	copy(p, c.data[c.pos:end])
	c.pos = end
	return n, nil
}

// Close marks c as closed. The first call returns nil; any subsequent
// call returns ErrConnClosed.
func (c *BufferConn) Close() error {
	if c.closed {
		return errors.Join(errors.New("buffer conn already closed"), ErrConnClosed)
	}
	c.closed = true
	c.data = nil
	return nil
}

// CopyAll reads from src until io.EOF, writing everything read to dst,
// and returns the total number of bytes copied. A read error (other than
// io.EOF) or write error is wrapped with context and returned
// immediately. A short write (dst.Write returning n < len(p) with a nil
// error) is reported by wrapping io.ErrShortWrite.
func CopyAll(dst, src Conn) (int64, error) {
	buf := make([]byte, 1024)
	var bytesCopied int64 = 0

	for {

		nRead, err := src.Read(buf)

		fmt.Printf("num bytess: %d nRead=%d\n", len(buf), nRead)
		if err == io.EOF {
			break
		}

		if err != nil {
			return 0, err
		}

		nWritten, err := dst.Write(buf[:nRead])

		if err != nil {
			return 0, err
		}

		if nRead != nWritten {
			return 0, fmt.Errorf("failed to write all bytes")
		}

		bytesCopied += int64(nWritten)

	}
	return bytesCopied, nil
}

// ValidateConns checks each conn's RemoteAddr: it must be non-nil
// (otherwise ErrNilAddr) and have Network() == "tcp" or "udp" (otherwise
// ErrUnknownNetwork). Failures from all conns are combined with
// errors.Join; ValidateConns returns nil if every conn is valid. Callers
// can use errors.Is(err, ErrNilAddr) / errors.Is(err, ErrUnknownNetwork)
// to check which problems occurred.
func ValidateConns(conns ...Conn) error {

	var err error = nil

	for _, c := range conns {
		if c.RemoteAddr() == nil {
			err = errors.Join(ErrNilAddr, err)
		} else if c.RemoteAddr().Network() != "tcp" && c.RemoteAddr().Network() != "udp" {
			err = errors.Join(ErrUnknownNetwork, err)
		}
	}
	return err
}
