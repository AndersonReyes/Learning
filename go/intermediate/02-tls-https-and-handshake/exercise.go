// Package tlsnet applies crypto/tls and crypto/x509 to generate a
// self-signed certificate, run a TLS-wrapped echo server (built on the
// net.Conn abstraction from topic 5 and the Serve pattern from topic 8),
// dial it as a client, and inspect the negotiated TLS 1.3 connection.
package tlsnet

import (
	"crypto/tls"
	"crypto/x509"
	"errors"
	"net"
)

// HandshakeInfo describes a negotiated TLS connection.
type HandshakeInfo struct {
	// Version is the negotiated protocol version's name, e.g. "TLS 1.3"
	// (see tls.VersionName).
	Version string

	// CipherSuite is the negotiated cipher suite's name, e.g.
	// "TLS_AES_128_GCM_SHA256" (see tls.CipherSuiteName).
	CipherSuite string

	// ServerName is the server name indicated by the client (SNI), as
	// seen by the server, or the server name the client connected to.
	ServerName string
}

// GenerateSelfSignedCert returns a self-signed TLS certificate and its
// matching private key, valid for host. If host parses as an IP address,
// it is set as the certificate's IP SAN (Subject Alternative Name);
// otherwise host is set as its DNS SAN. The certificate is valid
// immediately and for 24 hours.
func GenerateSelfSignedCert(host string) (tls.Certificate, error) {
	return tls.Certificate{}, errors.New("not implemented")
}

// NewTLSListener returns a listener on addr that presents cert to clients
// and requires TLS 1.3.
func NewTLSListener(addr string, cert tls.Certificate) (net.Listener, error) {
	return nil, errors.New("not implemented")
}

// DialTLS dials addr over TLS 1.3, verifying the server's certificate
// against rootCAs.
func DialTLS(addr string, rootCAs *x509.CertPool) (*tls.Conn, error) {
	return nil, errors.New("not implemented")
}

// ServeEcho accepts connections on l until it is closed, and for each
// connection copies every byte read back to the same connection
// (io.Copy(conn, conn)) until the client's side is closed. ServeEcho
// returns nil when l is closed.
func ServeEcho(l net.Listener) error {
	return errors.New("not implemented")
}

// GetHandshakeInfo ensures conn's TLS handshake has completed and returns
// information about the negotiated connection.
func GetHandshakeInfo(conn *tls.Conn) (HandshakeInfo, error) {
	return HandshakeInfo{}, errors.New("not implemented")
}
