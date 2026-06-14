# Advanced 2. QUIC / HTTP/3 — Wire-Format Primitives & the TLS 1.3 Handshake

## New: bit-twiddling for variable-length wire formats

QUIC's varint encoding packs a "length class" into the top 2 bits of the
first byte. Building and reading that requires shifts and masks you haven't
needed much so far:

```go
// top 2 bits of data[0] select the encoded length: 00->1, 01->2, 10->4, 11->8
length := 1 << (data[0] >> 6)

// mask off the top 2 bits to get the value bits of the first byte
v := uint64(data[0]) & 0x3f

// set the top 2 bits to mark a 2-byte encoding (01xxxxxx)
b[0] |= 0x40
```

**Gotchas**:

- `data[0] >> 6` shifts a `byte` (uint8); `1 << (that)` produces an `int` by
  default — fine for indexing/length math, but mixing it back into `uint64`
  arithmetic needs an explicit conversion.
- `|=` to set bits assumes the target bits are currently 0 — `PutUint16`/
  `PutUint32`/`PutUint64` from `encoding/binary` zero-fill, so this is safe
  immediately after a `Put*` call.
- Always mask the length/type bits back OUT when reading the value, and back
  IN when writing it — forgetting either corrupts the high bits of the value
  or the encoded length class.

## New: event-driven state-machine APIs (`crypto/tls.QUICConn`)

Earlier topics used `crypto/tls` synchronously: `tls.Dial`, `conn.Read`,
`conn.Write` — the library does its own I/O. QUIC can't work that way: QUIC
itself (not TLS) owns the wire, so `crypto/tls` exposes the TLS 1.3 state
machine through `tls.QUICConn` as a **poll-and-feed** API instead:

```go
conn := tls.QUICClient(&tls.QUICConfig{TLSConfig: cfg})
conn.Start(ctx)
for {
    switch e := conn.NextEvent(); e.Kind {
    case tls.QUICWriteData:           // conn produced bytes to send
    case tls.QUICNoEvent:             // conn is idle — feed it more input
    case tls.QUICHandshakeDone:       // done
    }
}
```

This pattern — call a method, drain a queue of resulting "events" via repeated
`NextEvent()` calls until you see `QUICNoEvent`, react to each event kind,
then block for external input — shows up across event-loop and actor-style
Go code, not just `crypto/tls`. The key discipline is: **never block waiting
for external input except on `QUICNoEvent`** — every other event kind is
either actionable immediately or safe to ignore, and `NextEvent()` itself
never blocks.

## Networking: QUIC variable-length integers (RFC 9000 §16)

QUIC's wire format is built almost entirely from a single integer encoding:
1, 2, 4, or 8 bytes, network byte order, where the two most-significant bits
of the first byte are `log2` of the length:

| Top 2 bits | Length  | Usable bits | Max value     |
|------------|---------|-------------|---------------|
| `00`       | 1 byte  | 6           | 63            |
| `01`       | 2 bytes | 14          | 16,383        |
| `10`       | 4 bytes | 30          | 1,073,741,823 |
| `11`       | 8 bytes | 62          | 2^62 - 1      |

The largest representable value, `2^62 - 1`, is `maxVarint` in this
exercise. Stream IDs, offsets, lengths, packet numbers, and frame type codes
throughout QUIC are all varints.

## Networking: CRYPTO frames (RFC 9000 §19.6)

During the handshake, QUIC carries TLS handshake bytes inside **CRYPTO
frames** — one of QUIC's general-purpose frame types, used here instead of
STREAM frames because the handshake happens before stream flow control is
set up:

```
+------+--------+--------+------+
| 0x06 | Offset | Length | Data |
+------+--------+--------+------+
 1 byte  varint   varint   Length bytes
```

`Offset` lets the TLS byte stream at a given encryption level be split across
multiple CRYPTO frames (and reassembled in order) — the same role a TCP
sequence number plays, but per-frame instead of per-packet.

## Networking: TLS 1.3 for QUIC (RFC 9001) and `tls.QUICConn`

RFC 9001 repurposes the TLS 1.3 handshake as QUIC's cryptographic handshake,
but **QUIC carries the TLS handshake bytes itself** (in CRYPTO frames, at
varying **encryption levels** as keys are established) rather than using
TLS's own record layer. `crypto/tls` models this with `tls.QUICConn`:

- **Setup**: `tls.QUICClient(&tls.QUICConfig{TLSConfig: cfg})` /
  `tls.QUICServer(&tls.QUICConfig{TLSConfig: cfg})` — `cfg.MinVersion` must be
  `tls.VersionTLS13`.
- **`Start(ctx) error`**: begins the handshake. Must be called exactly once,
  before the first `NextEvent`.
- **`NextEvent() QUICEvent`**: returns the next queued event, or
  `{Kind: QUICNoEvent}` once the queue is drained. Never blocks.
- **`HandleData(level QUICEncryptionLevel, data []byte) error`**: feeds
  handshake bytes received from the peer at the given level. May produce more
  events (call `NextEvent` again afterward).
- **`SetTransportParameters(params []byte)`**: sets the (QUIC, not TLS)
  transport parameters to send to the peer. `nil` is treated as empty.
- **`ConnectionState() tls.ConnectionState`**: `HandshakeComplete`,
  `Version`, etc., once `QUICHandshakeDone` fires.

### `QUICEncryptionLevel` (RFC 9001 §4)

```go
QUICEncryptionLevelInitial     // 0 — unprotected, derived from connection ID
QUICEncryptionLevelEarly       // 1 — 0-RTT
QUICEncryptionLevelHandshake   // 2 — handshake keys
QUICEncryptionLevelApplication // 3 — 1-RTT
```

CRYPTO frames at different levels are logically separate byte streams, each
with its own offset space — this is why `RunHandshake` tracks one running
offset per level.

### `QUICEventKind` values this exercise's `RunHandshake` handles

| Kind | Meaning | Action |
|------|---------|--------|
| `QUICWriteData` | `e.Data` is handshake bytes to send at `e.Level` | wrap in a CRYPTO frame, send to peer |
| `QUICTransportParametersRequired` | peer needs our transport params before continuing | `SetTransportParameters(nil)` |
| `QUICNoEvent` | nothing more to do without new input | receive from peer, `HandleData` |
| `QUICHandshakeDone` | handshake finished | return `nil` |
| (anything else) | secrets, peer transport params, session tickets, etc. | ignore |

Errors are **not** delivered as an event: `Start` and `HandleData` return
`error` directly, and a non-nil error means the handshake has failed.

## Further Reading

- [`crypto/tls`](https://pkg.go.dev/crypto/tls) — `QUICConn`, `QUICConfig`,
  `QUICEvent`, `QUICEventKind`, `QUICEncryptionLevel`
- [`encoding/binary`](https://pkg.go.dev/encoding/binary)
- [RFC 9000 (QUIC: A UDP-Based Multiplexed and Secure Transport)](https://www.rfc-editor.org/rfc/rfc9000)
  — §16 (variable-length integers), §19.6 (CRYPTO frames)
- [RFC 9001 (Using TLS to Secure QUIC)](https://www.rfc-editor.org/rfc/rfc9001)
  — §4 (encryption levels), §4.1 (handshake flow)
- [RFC 9114 (HTTP/3)](https://www.rfc-editor.org/rfc/rfc9114) — the protocol
  this topic's primitives ultimately support
