# Intermediate 2. TLS (`crypto/tls`) + HTTPS Client/Server & the TLS 1.3 Handshake

## `crypto/tls`: wrapping a `net.Conn`

Topic 5 introduced `net.Conn` as the abstraction every transport implements.
`*tls.Conn` is itself a `net.Conn` — it wraps an underlying connection
(typically a `*net.TCPConn` from topic 8) and transparently
encrypts/decrypts `Read`/`Write`. This means topic 8's `Serve`/echo pattern
and topic 10's HTTP-over-`net.Conn` code work unchanged once you swap a
plain listener/dial for a TLS one.

```go
// Server side: wrap a listener.
ln, err := tls.Listen("tcp", addr, &tls.Config{
    Certificates: []tls.Certificate{cert},
    MinVersion:   tls.VersionTLS13,
})

// Client side: dial and (by default) verify the server's certificate.
conn, err := tls.Dial("tcp", addr, &tls.Config{
    RootCAs:    pool,
    MinVersion: tls.VersionTLS13,
})
```

`tls.Dial` performs the handshake before returning — if certificate
verification fails, `Dial` itself returns the error. `(*tls.Conn).Handshake`
is idempotent and mostly only useful for inspecting
`(*tls.Conn).ConnectionState()` before the first `Read`/`Write`.

## Certificates: `crypto/x509` + `encoding/pem`

A `tls.Certificate` is a certificate chain plus its private key. For
testing (no real CA), generate a **self-signed certificate**:

```go
key, _ := ecdsa.GenerateKey(elliptic.P256(), rand.Reader)

template := &x509.Certificate{
    SerialNumber: serial,
    Subject:      pkix.Name{CommonName: host},
    NotBefore:    time.Now().Add(-time.Hour),
    NotAfter:     time.Now().Add(24 * time.Hour),
    KeyUsage:     x509.KeyUsageDigitalSignature | x509.KeyUsageCertSign,
    ExtKeyUsage:  []x509.ExtKeyUsage{x509.ExtKeyUsageServerAuth},
    IsCA:         true,
    BasicConstraintsValid: true,
}
// SAN: an IP address or a DNS name, depending on host.
if ip := net.ParseIP(host); ip != nil {
    template.IPAddresses = []net.IP{ip}
} else {
    template.DNSNames = []string{host}
}

der, _ := x509.CreateCertificate(rand.Reader, template, template, &key.PublicKey, key)
```

**Gotcha**: since Go 1.15, certificate hostname verification checks the
**Subject Alternative Name** (SAN) extension — `DNSNames`/`IPAddresses` —
not the deprecated `CommonName` fallback. A cert with only a `CommonName`
fails verification against a real client.

A client verifies a self-signed cert by adding it to a `*x509.CertPool`:

```go
pool := x509.NewCertPool()
pool.AddCert(leafCert) // *x509.Certificate, parsed from the DER bytes
```

## Inspecting a negotiated connection

`(*tls.Conn).ConnectionState()` returns a `tls.ConnectionState` with the
negotiated protocol version and cipher suite as `uint16` IDs.
`tls.VersionName` and `tls.CipherSuiteName` (both stdlib helpers) turn
those IDs into readable strings ("TLS 1.3",
"TLS_AES_128_GCM_SHA256") without hand-maintained lookup tables.

---

## Networking: the TLS 1.3 handshake (RFC 8446)

TLS 1.3 reduced the handshake from TLS 1.2's 2 round trips to **1 round
trip** before application data can flow:

```
Client                                           Server
  ClientHello
  + key_share, supported_versions,
    signature_algorithms          -------->
                                              ServerHello
                                              + key_share
                                   {EncryptedExtensions}
                                   {Certificate}
                                   {CertificateVerify}
                                   {Finished}
                             <--------
  {Finished}
                             -------->
  [Application Data]        <------->  [Application Data]
```

(`{...}` = encrypted once both sides derive handshake keys from the
key-share exchange; `[...]` = encrypted with the final application keys.)

Key points vs. TLS 1.2:

- **Key exchange happens in the first round trip**: the client sends
  `key_share` entries (its (EC)DHE public values) in the *ClientHello*
  itself, so the server can derive shared secrets immediately and respond
  with its own `key_share` plus everything else encrypted.
- **Cipher suites are AEAD-only and decoupled from key exchange**: TLS 1.3
  cipher suite names like `TLS_AES_128_GCM_SHA256` specify only the bulk
  cipher and hash — not the key-exchange algorithm (always (EC)DHE) or
  signature algorithm (negotiated separately via
  `signature_algorithms`). This is why `cipherSuite` is just one `uint16`
  in `tls.ConnectionState` but no longer implies RSA vs. ECDHE key exchange
  the way TLS 1.2 suite names did.
- **0-RTT (optional)**: with a previous session's pre-shared key, a client
  can send application data in its *first* flight, before the handshake
  completes — at the cost of losing forward secrecy and replay protection
  for that early data. Go's `crypto/tls` supports 0-RTT only in limited
  configurations and it's off the beaten path for this exercise.

## Networking: HTTPS = HTTP/1.1 (topic 10) over TLS

HTTPS has no separate request/response format — it's exactly the HTTP/1.1
message format from topic 10, sent over a `*tls.Conn` instead of a raw
`*net.TCPConn`. `net/http`'s `Server.ServeTLS` and `Client` with an
`https://` URL do precisely this: terminate TLS, then run the same HTTP/1.1
(or HTTP/2, negotiated via TLS's ALPN extension) state machine on the
decrypted byte stream.

## Further Reading

- [`crypto/tls`](https://pkg.go.dev/crypto/tls)
- [`tls.Config`](https://pkg.go.dev/crypto/tls#Config), [`tls.Certificate`](https://pkg.go.dev/crypto/tls#Certificate), [`tls.ConnectionState`](https://pkg.go.dev/crypto/tls#ConnectionState)
- [`tls.VersionName`](https://pkg.go.dev/crypto/tls#VersionName), [`tls.CipherSuiteName`](https://pkg.go.dev/crypto/tls#CipherSuiteName)
- [`crypto/x509`](https://pkg.go.dev/crypto/x509), [`x509.CreateCertificate`](https://pkg.go.dev/crypto/x509#CreateCertificate)
- [RFC 8446 (TLS 1.3)](https://www.rfc-editor.org/rfc/rfc8446)
- [RFC 8446 §2 (Protocol Overview / handshake diagram)](https://www.rfc-editor.org/rfc/rfc8446#section-2)
