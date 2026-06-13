# Intermediate 3. WebSockets + Real-Time Chat Server

## Non-blocking channel sends: `select` with `default`

Topic 7 introduced `select` for waiting on multiple channel operations.
A `select` with a `default` case never blocks: if no other case is ready
*immediately*, `default` runs instead.

```go
select {
case ch <- msg:
    // sent
default:
    // ch's buffer is full (or nobody is receiving) — drop msg instead
    // of blocking the sender
}
```

This is the standard way to **broadcast** to many goroutine-owned channels
without one slow/stuck receiver blocking delivery to everyone else — central
to this topic's chat `Hub`, which sends each broadcast message to every
registered client's channel and drops it for any client whose buffer is
full rather than waiting.

## Bit manipulation, revisited: XOR masking

Topic 1 covered bitwise operators. WebSocket frames from a client **must**
be masked (RFC 6455 §5.3) by XOR-ing each payload byte with one byte of a
4-byte key, cycling through the key:

```go
out[i] = payload[i] ^ key[i%4]
```

XOR is its own inverse (`(a ^ b) ^ b == a`), so the *same* function masks
and unmasks — `MaskPayload` is used by both `WriteFrame` (to mask outgoing
client frames) and `ReadFrame` (to unmask incoming masked frames).

## New stdlib: `crypto/sha1` + `encoding/base64`

The WebSocket handshake (below) computes `base64(sha1(input))` — two
one-line stdlib calls:

```go
sum := sha1.Sum([]byte(input))      // [20]byte
encoded := base64.StdEncoding.EncodeToString(sum[:])
```

---

## Networking: the WebSocket protocol (RFC 6455)

WebSocket upgrades an HTTP/1.1 connection (topic 10) into a persistent,
bidirectional, message-framed connection — the opposite of HTTP's
request/response model.

### The handshake: an HTTP Upgrade

A WebSocket connection starts as an ordinary HTTP/1.1 request with special
headers:

```
GET /chat HTTP/1.1
Host: example.com
Upgrade: websocket
Connection: Upgrade
Sec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==
Sec-WebSocket-Version: 13
```

The server replies `101 Switching Protocols` with `Sec-WebSocket-Accept`
computed from the client's key (RFC 6455 §1.3, §4.2.2):

```
Sec-WebSocket-Accept = base64(sha1(Sec-WebSocket-Key + "258EAFA5-E914-47DA-95CA-C5AB0DC85B11"))
```

The fixed GUID string proves the server actually understood the
`Sec-WebSocket-Key` header (a proxy that blindly forwards an HTTP Upgrade
without WebSocket support can't compute this). After the `101` response,
both ends switch to framed messages on the *same* underlying `net.Conn` —
in `net/http`, a handler obtains that raw connection via
[`http.Hijacker`](https://pkg.go.dev/net/http#Hijacker).

### Frame format (RFC 6455 §5.2)

```
 0               1               2               3
 0 1 2 3 4 5 6 7 0 1 2 3 4 5 6 7 0 1 2 3 4 5 6 7 0 1 2 3 4 5 6 7
+-+-+-+-+-------+-+-------------+-------------------------------+
|F|R|R|R| opcode|M| Payload len |    Extended payload length    |
|I|S|S|S|  (4)  |A|     (7)     |   (16 bits, if len == 126)    |
|N|V|V|V|       |S|             |   (or 64 bits, if len == 127) |
| |1|2|3|       |K|             |                               |
+-+-+-+-+-------+-+-------------+ - - - - - - - - - - - - - - - +
|                  Masking-key (32 bits), if MASK set           |
+-----------------------------------------------------------------+
|                          Payload Data                          |
+-----------------------------------------------------------------+
```

- **FIN** (1 bit): 1 if this is the final fragment of a message. This
  exercise only handles unfragmented messages (FIN always 1).
- **opcode** (4 bits): frame type — `0x1` text, `0x2` binary, `0x8` close,
  `0x9` ping, `0xA` pong (RFC 6455 §11.8).
- **MASK** (1 bit) + **Masking-key** (32 bits, present only if MASK=1):
  client→server frames MUST set MASK=1; server→client frames MUST NOT
  (RFC 6455 §5.1).
- **Payload len** (7 bits): the length directly if ≤125; `126` means the
  next 2 bytes are a 16-bit length; `127` means the next 8 bytes are a
  64-bit length — the same "escape value for extended length" idea as
  DNS's compression pointers (topic 12) used the top bits of a byte for a
  different purpose.

### Real-time chat: the broadcast hub

A chat server holds one goroutine per connected client (reading frames in
a loop, topic 6's pattern) plus a central `Hub` that every client registers
with. When any client sends a chat message, the server calls
`Hub.Broadcast(msg)`, which fans the message out to every other client's
outgoing-message channel using the non-blocking `select`/`default` pattern
above — a slow client gets dropped messages, not a stalled broadcaster.

## Further Reading

- [RFC 6455 (The WebSocket Protocol)](https://www.rfc-editor.org/rfc/rfc6455)
- [RFC 6455 §1.3 (Opening Handshake)](https://www.rfc-editor.org/rfc/rfc6455#section-1.3)
- [RFC 6455 §5.2 (Base Framing Protocol)](https://www.rfc-editor.org/rfc/rfc6455#section-5.2)
- [RFC 6455 §11.8 (Opcode values)](https://www.rfc-editor.org/rfc/rfc6455#section-11.8)
- [`crypto/sha1`](https://pkg.go.dev/crypto/sha1), [`encoding/base64`](https://pkg.go.dev/encoding/base64)
- [`net/http.Hijacker`](https://pkg.go.dev/net/http#Hijacker)
