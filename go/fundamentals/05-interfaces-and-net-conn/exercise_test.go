package transport

import (
	"bytes"
	"errors"
	"io"
	"testing"
)

// fakeAddr is a third Addr implementation, defined here to show that any
// type with Network() and String() methods satisfies Addr — not just
// TCPAddr and UDPAddr.
type fakeAddr struct {
	network string
	addr    string
}

func (a fakeAddr) Network() string { return a.network }
func (a fakeAddr) String() string  { return a.addr }

func TestBufferConnWrite(t *testing.T) {
	c := NewBufferConn(nil, TCPAddr{IP: "127.0.0.1", Port: 9000})

	n, err := c.Write([]byte("hello"))
	if err != nil || n != 5 {
		t.Fatalf("Write(%q) = %d, %v, want 5, nil", "hello", n, err)
	}
	n, err = c.Write([]byte(" world"))
	if err != nil || n != 6 {
		t.Fatalf("Write(%q) = %d, %v, want 6, nil", " world", n, err)
	}
	if !bytes.Equal(c.data, []byte("hello world")) {
		t.Fatalf("c.data = %q, want %q", c.data, "hello world")
	}

	if err := c.Close(); err != nil {
		t.Fatalf("Close() = %v, want nil", err)
	}
	if _, err := c.Write([]byte("more")); !errors.Is(err, ErrConnClosed) {
		t.Errorf("Write() on closed conn: errors.Is(err, ErrConnClosed) = false, err = %v", err)
	}
}

func TestBufferConnRead(t *testing.T) {
	c := NewBufferConn([]byte("hello"), TCPAddr{IP: "127.0.0.1", Port: 9000})

	buf := make([]byte, 3)
	n, err := c.Read(buf)
	if err != nil || n != 3 || string(buf[:n]) != "hel" {
		t.Fatalf("Read() = %d, %q, %v, want 3, %q, nil", n, buf[:n], err, "hel")
	}

	n, err = c.Read(buf)
	if err != nil || n != 2 || string(buf[:n]) != "lo" {
		t.Fatalf("Read() = %d, %q, %v, want 2, %q, nil", n, buf[:n], err, "lo")
	}

	n, err = c.Read(buf)
	if n != 0 || err != io.EOF {
		t.Fatalf("Read() at EOF = %d, %v, want 0, io.EOF", n, err)
	}

	if err := c.Close(); err != nil {
		t.Fatalf("Close() = %v, want nil", err)
	}
	if _, err := c.Read(buf); !errors.Is(err, ErrConnClosed) {
		t.Errorf("Read() on closed conn: errors.Is(err, ErrConnClosed) = false, err = %v", err)
	}
}

func TestBufferConnClose(t *testing.T) {
	c := NewBufferConn(nil, TCPAddr{IP: "127.0.0.1", Port: 9000})

	if err := c.Close(); err != nil {
		t.Fatalf("first Close() = %v, want nil", err)
	}
	if err := c.Close(); !errors.Is(err, ErrConnClosed) {
		t.Errorf("second Close(): errors.Is(err, ErrConnClosed) = false, err = %v", err)
	}
}

func TestCopyAll(t *testing.T) {
	t.Run("copies all data", func(t *testing.T) {
		addr := TCPAddr{IP: "127.0.0.1", Port: 9000}
		src := NewBufferConn([]byte("hello world"), addr)
		dst := NewBufferConn(nil, addr)

		n, err := CopyAll(dst, src)
		if err != nil {
			t.Fatalf("CopyAll() error = %v", err)
		}
		if n != 11 {
			t.Errorf("CopyAll() n = %d, want 11", n)
		}
		if !bytes.Equal(dst.data, []byte("hello world")) {
			t.Errorf("dst.data = %q, want %q", dst.data, "hello world")
		}
	})

	t.Run("empty source", func(t *testing.T) {
		addr := TCPAddr{IP: "127.0.0.1", Port: 9000}
		src := NewBufferConn(nil, addr)
		dst := NewBufferConn(nil, addr)

		n, err := CopyAll(dst, src)
		if err != nil {
			t.Fatalf("CopyAll() error = %v", err)
		}
		if n != 0 {
			t.Errorf("CopyAll() n = %d, want 0", n)
		}
		if len(dst.data) != 0 {
			t.Errorf("dst.data = %q, want empty", dst.data)
		}
	})

	t.Run("closed source propagates error", func(t *testing.T) {
		addr := TCPAddr{IP: "127.0.0.1", Port: 9000}
		src := NewBufferConn([]byte("x"), addr)
		if err := src.Close(); err != nil {
			t.Fatalf("src.Close() = %v", err)
		}
		dst := NewBufferConn(nil, addr)

		n, err := CopyAll(dst, src)
		if n != 0 {
			t.Errorf("CopyAll() n = %d, want 0", n)
		}
		if !errors.Is(err, ErrConnClosed) {
			t.Errorf("CopyAll(): errors.Is(err, ErrConnClosed) = false, err = %v", err)
		}
	})
}

func TestValidateConns(t *testing.T) {
	t.Run("all valid", func(t *testing.T) {
		conns := []Conn{
			NewBufferConn(nil, TCPAddr{IP: "127.0.0.1", Port: 80}),
			NewBufferConn(nil, UDPAddr{IP: "127.0.0.1", Port: 53}),
		}
		if err := ValidateConns(conns...); err != nil {
			t.Errorf("ValidateConns() = %v, want nil", err)
		}
	})

	t.Run("no conns", func(t *testing.T) {
		if err := ValidateConns(); err != nil {
			t.Errorf("ValidateConns() = %v, want nil", err)
		}
	})

	t.Run("nil addr", func(t *testing.T) {
		conns := []Conn{NewBufferConn(nil, nil)}
		err := ValidateConns(conns...)
		if !errors.Is(err, ErrNilAddr) {
			t.Errorf("ValidateConns(): errors.Is(err, ErrNilAddr) = false, err = %v", err)
		}
	})

	t.Run("unknown network", func(t *testing.T) {
		conns := []Conn{NewBufferConn(nil, fakeAddr{network: "sctp", addr: "x"})}
		err := ValidateConns(conns...)
		if !errors.Is(err, ErrUnknownNetwork) {
			t.Errorf("ValidateConns(): errors.Is(err, ErrUnknownNetwork) = false, err = %v", err)
		}
	})

	t.Run("multiple failures joined", func(t *testing.T) {
		conns := []Conn{
			NewBufferConn(nil, nil),
			NewBufferConn(nil, fakeAddr{network: "sctp", addr: "x"}),
		}
		err := ValidateConns(conns...)
		if !errors.Is(err, ErrNilAddr) {
			t.Errorf("ValidateConns(): errors.Is(err, ErrNilAddr) = false, err = %v", err)
		}
		if !errors.Is(err, ErrUnknownNetwork) {
			t.Errorf("ValidateConns(): errors.Is(err, ErrUnknownNetwork) = false, err = %v", err)
		}
	})
}
