package tlsnet

import (
	"crypto/x509"
	"io"
	"net"
	"testing"
	"time"
)

// dialTimeout bounds how long a test waits on network operations. It's
// generous enough for a correct implementation (everything here is
// loopback) but keeps a not-yet-implemented stub from hanging the test
// run.
const dialTimeout = 2 * time.Second

func TestGenerateSelfSignedCert(t *testing.T) {
	t.Run("IP host", func(t *testing.T) {
		cert, err := GenerateSelfSignedCert("127.0.0.1")
		if err != nil {
			t.Fatalf("GenerateSelfSignedCert() error = %v", err)
		}
		if len(cert.Certificate) == 0 {
			t.Fatal("cert.Certificate is empty")
		}

		leaf, err := x509.ParseCertificate(cert.Certificate[0])
		if err != nil {
			t.Fatalf("x509.ParseCertificate() error = %v", err)
		}
		if len(leaf.IPAddresses) != 1 || !leaf.IPAddresses[0].Equal(net.ParseIP("127.0.0.1")) {
			t.Errorf("IPAddresses = %v, want [127.0.0.1]", leaf.IPAddresses)
		}
	})

	t.Run("DNS host", func(t *testing.T) {
		cert, err := GenerateSelfSignedCert("example.com")
		if err != nil {
			t.Fatalf("GenerateSelfSignedCert() error = %v", err)
		}

		leaf, err := x509.ParseCertificate(cert.Certificate[0])
		if err != nil {
			t.Fatalf("x509.ParseCertificate() error = %v", err)
		}
		if len(leaf.DNSNames) != 1 || leaf.DNSNames[0] != "example.com" {
			t.Errorf("DNSNames = %v, want [example.com]", leaf.DNSNames)
		}
	})
}

func TestTLSRoundTrip(t *testing.T) {
	cert, err := GenerateSelfSignedCert("127.0.0.1")
	if err != nil {
		t.Fatalf("GenerateSelfSignedCert() error = %v", err)
	}

	leaf, err := x509.ParseCertificate(cert.Certificate[0])
	if err != nil {
		t.Fatalf("x509.ParseCertificate() error = %v", err)
	}

	pool := x509.NewCertPool()
	pool.AddCert(leaf)

	l, err := NewTLSListener("127.0.0.1:0", cert)
	if err != nil {
		t.Fatalf("NewTLSListener() error = %v", err)
	}

	serveDone := make(chan error, 1)
	go func() {
		serveDone <- ServeEcho(l)
	}()

	conn, err := DialTLS(l.Addr().String(), pool)
	if err != nil {
		l.Close()
		t.Fatalf("DialTLS() error = %v", err)
	}

	t.Run("handshake negotiates TLS 1.3", func(t *testing.T) {
		info, err := GetHandshakeInfo(conn)
		if err != nil {
			t.Fatalf("GetHandshakeInfo() error = %v", err)
		}
		if info.Version != "TLS 1.3" {
			t.Errorf("Version = %q, want %q", info.Version, "TLS 1.3")
		}
		if info.CipherSuite == "" {
			t.Error("CipherSuite is empty, want a cipher suite name")
		}
	})

	t.Run("echoes data back", func(t *testing.T) {
		conn.SetDeadline(time.Now().Add(dialTimeout))

		want := "hello, tls"
		if _, err := conn.Write([]byte(want)); err != nil {
			t.Fatalf("Write() error = %v", err)
		}

		buf := make([]byte, len(want))
		if _, err := io.ReadFull(conn, buf); err != nil {
			t.Fatalf("ReadFull() error = %v", err)
		}
		if string(buf) != want {
			t.Errorf("echo = %q, want %q", buf, want)
		}
	})

	conn.Close()
	l.Close()

	select {
	case err := <-serveDone:
		if err != nil {
			t.Errorf("ServeEcho() = %v, want nil after listener closed", err)
		}
	case <-time.After(dialTimeout):
		t.Fatal("ServeEcho did not return after listener was closed")
	}
}

func TestDialTLSRejectsUntrustedCert(t *testing.T) {
	cert, err := GenerateSelfSignedCert("127.0.0.1")
	if err != nil {
		t.Fatalf("GenerateSelfSignedCert() error = %v", err)
	}

	l, err := NewTLSListener("127.0.0.1:0", cert)
	if err != nil {
		t.Fatalf("NewTLSListener() error = %v", err)
	}
	defer l.Close()

	go ServeEcho(l)

	// An empty pool trusts no certificates, so the self-signed cert
	// should fail verification.
	_, err = DialTLS(l.Addr().String(), x509.NewCertPool())
	if err == nil {
		t.Error("DialTLS() with untrusted cert error = nil, want non-nil")
	}
}
