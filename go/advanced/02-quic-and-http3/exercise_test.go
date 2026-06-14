package miniquic

import (
	"bytes"
	"crypto/ecdsa"
	"crypto/elliptic"
	"crypto/rand"
	"crypto/tls"
	"crypto/x509"
	"crypto/x509/pkix"
	"encoding/hex"
	"math/big"
	"net"
	"sync"
	"testing"
	"time"
)

func mustDecodeHex(t *testing.T, s string) []byte {
	b, err := hex.DecodeString(s)
	if err != nil {
		t.Fatalf("hex.DecodeString(%q): %v", s, err)
	}
	return b
}

// TestVarint checks EncodeVarint/DecodeVarint against the worked examples in
// RFC 9000 §16, plus boundary values for each of the four encoding lengths
// and the error cases.
func TestVarint(t *testing.T) {
	tests := []struct {
		name string
		val  uint64
		enc  []byte
	}{
		{"1-byte", 37, mustDecodeHex(t, "25")},
		{"2-byte", 15293, mustDecodeHex(t, "7bbd")},
		{"4-byte", 494878333, mustDecodeHex(t, "9d7f3e7d")},
		{"8-byte", 151288809941952652, mustDecodeHex(t, "c2197c5eff14e88c")},
		{"zero", 0, mustDecodeHex(t, "00")},
		{"max 1-byte", 63, mustDecodeHex(t, "3f")},
		{"min 2-byte", 64, mustDecodeHex(t, "4040")},
		{"max varint", maxVarint, mustDecodeHex(t, "ffffffffffffffff")},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			gotEnc, err := EncodeVarint(tt.val)
			if err != nil {
				t.Fatalf("EncodeVarint(%d) error = %v", tt.val, err)
			}
			if !bytes.Equal(gotEnc, tt.enc) {
				t.Errorf("EncodeVarint(%d) = %x, want %x", tt.val, gotEnc, tt.enc)
			}

			gotVal, gotN, err := DecodeVarint(tt.enc)
			if err != nil {
				t.Fatalf("DecodeVarint(%x) error = %v", tt.enc, err)
			}
			if gotVal != tt.val || gotN != len(tt.enc) {
				t.Errorf("DecodeVarint(%x) = (%d, %d), want (%d, %d)", tt.enc, gotVal, gotN, tt.val, len(tt.enc))
			}
		})
	}

	t.Run("EncodeVarint too large", func(t *testing.T) {
		if _, err := EncodeVarint(maxVarint + 1); err == nil {
			t.Error("EncodeVarint(maxVarint+1) error = nil, want error")
		}
	})

	t.Run("DecodeVarint empty", func(t *testing.T) {
		if _, _, err := DecodeVarint([]byte{}); err == nil {
			t.Error("DecodeVarint([]byte{}) error = nil, want error")
		}
	})

	t.Run("DecodeVarint truncated", func(t *testing.T) {
		// 0x40 has top bits 01 (2-byte encoding) but only 1 byte is present.
		if _, _, err := DecodeVarint([]byte{0x40}); err == nil {
			t.Error("DecodeVarint([]byte{0x40}) error = nil, want error")
		}
	})

	t.Run("DecodeVarint ignores trailing bytes", func(t *testing.T) {
		data := append(mustDecodeHex(t, "25"), 0xFF, 0xFF)
		gotVal, gotN, err := DecodeVarint(data)
		if err != nil {
			t.Fatalf("DecodeVarint(%x) error = %v", data, err)
		}
		if gotVal != 37 || gotN != 1 {
			t.Errorf("DecodeVarint(%x) = (%d, %d), want (37, 1)", data, gotVal, gotN)
		}
	})
}

// TestCryptoFrame checks EncodeCryptoFrame/DecodeCryptoFrame (RFC 9000
// §19.6) round trips, and DecodeCryptoFrame's error cases.
func TestCryptoFrame(t *testing.T) {
	tests := []struct {
		name   string
		offset uint64
		data   []byte
		want   []byte
	}{
		{"offset 0", 0, []byte("clienthello"), mustDecodeHex(t, "06000b636c69656e7468656c6c6f")},
		{"offset 1000", 1000, []byte("serverhello-and-cert"), mustDecodeHex(t, "0643e81473657276657268656c6c6f2d616e642d63657274")},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := EncodeCryptoFrame(tt.offset, tt.data)
			if !bytes.Equal(got, tt.want) {
				t.Errorf("EncodeCryptoFrame(%d, %q) = %x, want %x", tt.offset, tt.data, got, tt.want)
			}

			gotOffset, gotData, err := DecodeCryptoFrame(tt.want)
			if err != nil {
				t.Fatalf("DecodeCryptoFrame(%x) error = %v", tt.want, err)
			}
			if gotOffset != tt.offset || !bytes.Equal(gotData, tt.data) {
				t.Errorf("DecodeCryptoFrame(%x) = (%d, %q), want (%d, %q)", tt.want, gotOffset, gotData, tt.offset, tt.data)
			}
		})
	}

	t.Run("wrong type", func(t *testing.T) {
		bad := mustDecodeHex(t, "07000b636c69656e7468656c6c6f") // type 0x07, not CRYPTO (0x06)
		if _, _, err := DecodeCryptoFrame(bad); err == nil {
			t.Error("DecodeCryptoFrame(wrong type) error = nil, want error")
		}
	})

	t.Run("declared length exceeds remaining data", func(t *testing.T) {
		bad := mustDecodeHex(t, "0600056869") // length=5, but only 2 bytes ("hi") follow
		if _, _, err := DecodeCryptoFrame(bad); err == nil {
			t.Error("DecodeCryptoFrame(truncated) error = nil, want error")
		}
	})
}

// mustGenerateCert returns a self-signed ECDSA certificate for 127.0.0.1
// and a pool containing it, for use as both the server's certificate and
// the client's root of trust.
func mustGenerateCert(t *testing.T) (tls.Certificate, *x509.CertPool) {
	t.Helper()

	priv, err := ecdsa.GenerateKey(elliptic.P256(), rand.Reader)
	if err != nil {
		t.Fatalf("ecdsa.GenerateKey() error = %v", err)
	}

	tmpl := &x509.Certificate{
		SerialNumber: big.NewInt(1),
		Subject:      pkix.Name{CommonName: "127.0.0.1"},
		NotBefore:    time.Now().Add(-time.Hour),
		NotAfter:     time.Now().Add(time.Hour),
		IPAddresses:  []net.IP{net.ParseIP("127.0.0.1")},
		KeyUsage:     x509.KeyUsageDigitalSignature,
		ExtKeyUsage:  []x509.ExtKeyUsage{x509.ExtKeyUsageServerAuth},
	}
	der, err := x509.CreateCertificate(rand.Reader, tmpl, tmpl, &priv.PublicKey, priv)
	if err != nil {
		t.Fatalf("x509.CreateCertificate() error = %v", err)
	}
	leaf, err := x509.ParseCertificate(der)
	if err != nil {
		t.Fatalf("x509.ParseCertificate() error = %v", err)
	}

	pool := x509.NewCertPool()
	pool.AddCert(leaf)
	return tls.Certificate{Certificate: [][]byte{der}, PrivateKey: priv, Leaf: leaf}, pool
}

// TestRunHandshake runs RunHandshake on a client and server tls.QUICConn
// concurrently, connected by a pair of channels, and checks that both sides
// complete a TLS 1.3 handshake (RFC 9001).
func TestRunHandshake(t *testing.T) {
	cert, pool := mustGenerateCert(t)

	client := tls.QUICClient(&tls.QUICConfig{
		TLSConfig: &tls.Config{
			ServerName: "127.0.0.1",
			RootCAs:    pool,
			MinVersion: tls.VersionTLS13,
		},
	})
	server := tls.QUICServer(&tls.QUICConfig{
		TLSConfig: &tls.Config{
			Certificates: []tls.Certificate{cert},
			MinVersion:   tls.VersionTLS13,
		},
	})
	defer client.Close()
	defer server.Close()

	clientToServer := make(chan []byte, 16)
	serverToClient := make(chan []byte, 16)

	var clientErr, serverErr error
	var wg sync.WaitGroup
	wg.Add(2)
	go func() {
		defer wg.Done()
		clientErr = RunHandshake(client, clientToServer, serverToClient)
	}()
	go func() {
		defer wg.Done()
		serverErr = RunHandshake(server, serverToClient, clientToServer)
	}()

	done := make(chan struct{})
	go func() {
		wg.Wait()
		close(done)
	}()

	select {
	case <-done:
	case <-time.After(5 * time.Second):
		t.Fatal("handshake did not complete within 5s")
	}

	if clientErr != nil {
		t.Errorf("client RunHandshake() error = %v", clientErr)
	}
	if serverErr != nil {
		t.Errorf("server RunHandshake() error = %v", serverErr)
	}

	cs := client.ConnectionState()
	if !cs.HandshakeComplete {
		t.Error("client ConnectionState().HandshakeComplete = false")
	}
	if cs.Version != tls.VersionTLS13 {
		t.Errorf("client ConnectionState().Version = %#x, want TLS 1.3 (%#x)", cs.Version, tls.VersionTLS13)
	}

	ss := server.ConnectionState()
	if !ss.HandshakeComplete {
		t.Error("server ConnectionState().HandshakeComplete = false")
	}
}
