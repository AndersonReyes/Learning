package nettools

import (
	"bytes"
	"net"
	"testing"
	"time"
)

// dialTimeout bounds how long a test waits on network operations. It's
// generous enough for a correct implementation (everything here is
// loopback) but keeps a not-yet-implemented stub from hanging the test
// run.
const dialTimeout = 2 * time.Second

// mustListenTCP starts a TCP listener on an ephemeral loopback port.
func mustListenTCP(t *testing.T) net.Listener {
	t.Helper()
	l, err := net.Listen("tcp", "127.0.0.1:0")
	if err != nil {
		t.Fatalf("net.Listen() error = %v", err)
	}
	return l
}

// mustListenUDP opens a UDP socket on an ephemeral loopback port.
func mustListenUDP(t *testing.T) *net.UDPConn {
	t.Helper()
	addr, err := net.ResolveUDPAddr("udp", "127.0.0.1:0")
	if err != nil {
		t.Fatalf("net.ResolveUDPAddr() error = %v", err)
	}
	conn, err := net.ListenUDP("udp", addr)
	if err != nil {
		t.Fatalf("net.ListenUDP() error = %v", err)
	}
	return conn
}

func TestServeAndEchoHandler(t *testing.T) {
	l := mustListenTCP(t)

	serveDone := make(chan error, 1)
	go func() {
		serveDone <- Serve(l, func(c net.Conn) {
			_ = EchoHandler(c)
		})
	}()

	t.Run("echoes data back", func(t *testing.T) {
		got, err := DialAndSend("tcp", l.Addr().String(), []byte("hello, server"), dialTimeout)
		if err != nil {
			t.Fatalf("DialAndSend() error = %v", err)
		}
		if string(got) != "hello, server" {
			t.Errorf("DialAndSend() = %q, want %q", got, "hello, server")
		}
	})

	t.Run("handles empty payload", func(t *testing.T) {
		got, err := DialAndSend("tcp", l.Addr().String(), nil, dialTimeout)
		if err != nil {
			t.Fatalf("DialAndSend() error = %v", err)
		}
		if len(got) != 0 {
			t.Errorf("DialAndSend() = %q, want empty", got)
		}
	})

	t.Run("handles multiple sequential connections", func(t *testing.T) {
		for _, msg := range []string{"first", "second", "third"} {
			got, err := DialAndSend("tcp", l.Addr().String(), []byte(msg), dialTimeout)
			if err != nil {
				t.Fatalf("DialAndSend(%q) error = %v", msg, err)
			}
			if string(got) != msg {
				t.Errorf("DialAndSend(%q) = %q, want %q", msg, got, msg)
			}
		}
	})

	t.Run("connection refused for closed listener", func(t *testing.T) {
		other := mustListenTCP(t)
		addr := other.Addr().String()
		other.Close()

		_, err := DialAndSend("tcp", addr, []byte("x"), dialTimeout)
		if err == nil {
			t.Error("DialAndSend() to a closed listener error = nil, want non-nil")
		}
	})

	l.Close()
	select {
	case err := <-serveDone:
		if err != nil {
			t.Errorf("Serve() = %v, want nil after listener closed", err)
		}
	case <-time.After(dialTimeout):
		t.Fatal("Serve did not return after listener was closed")
	}
}

func TestServeUDPAndSendUDP(t *testing.T) {
	conn := mustListenUDP(t)

	serveDone := make(chan error, 1)
	go func() {
		serveDone <- ServeUDP(conn, func(data []byte, from *net.UDPAddr) []byte {
			return bytes.ToUpper(data)
		})
	}()

	t.Run("responds with transformed payload", func(t *testing.T) {
		got, err := SendUDP(conn.LocalAddr().String(), []byte("hello"), dialTimeout)
		if err != nil {
			t.Fatalf("SendUDP() error = %v", err)
		}
		if string(got) != "HELLO" {
			t.Errorf("SendUDP() = %q, want %q", got, "HELLO")
		}
	})

	t.Run("times out when nothing responds", func(t *testing.T) {
		other := mustListenUDP(t)
		addr := other.LocalAddr().String()
		other.Close()

		_, err := SendUDP(addr, []byte("hello"), 50*time.Millisecond)
		if err == nil {
			t.Error("SendUDP() to an unresponsive address error = nil, want non-nil")
		}
	})

	conn.Close()
	select {
	case err := <-serveDone:
		if err != nil {
			t.Errorf("ServeUDP() = %v, want nil after conn closed", err)
		}
	case <-time.After(dialTimeout):
		t.Fatal("ServeUDP did not return after conn was closed")
	}
}
