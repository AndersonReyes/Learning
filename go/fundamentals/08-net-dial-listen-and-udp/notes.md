# 08. The `net` Package (Dial/Listen)

## TCP: `Listen`, `Accept`, `Dial`

```go
l, err := net.Listen("tcp", "127.0.0.1:0") // ":0" picks an ephemeral port
defer l.Close()

for {
    conn, err := l.Accept() // blocks until a client connects
    if err != nil {
        if errors.Is(err, net.ErrClosed) {
            return nil // l.Close() was called elsewhere — clean shutdown
        }
        return err
    }
    go handle(conn) // one goroutine per connection (topic 6)
}
```

```go
conn, err := net.Dial("tcp", "127.0.0.1:54321")
defer conn.Close()
```

- `l.Addr()` and `conn.LocalAddr()`/`conn.RemoteAddr()` return `net.Addr`
  (topic 5). With `"127.0.0.1:0"`, `l.Addr().String()` gives the actual
  ephemeral port the OS assigned — read it back to know where to `Dial`.
- A `net.Conn` returned by `Accept` or `Dial` implements `io.Reader` and
  `io.Writer` (topic 5) plus the deadline methods from topic 7
  (`SetDeadline`, `SetReadDeadline`, `SetWriteDeadline`).
- Closing a `net.Listener` from another goroutine makes a blocked `Accept`
  return an error satisfying `errors.Is(err, net.ErrClosed)` — the standard
  way to stop an accept loop.

## TCP is a byte stream, not messages

RFC 793 defines TCP as delivering an ordered, reliable **stream of bytes** —
it has **no concept of message boundaries**. A single `Write` of 100 bytes
on one end might arrive as one `Read` of 100 bytes, or as several smaller
`Read`s, or coalesced with the *next* `Write`'s bytes into one `Read`. Any
protocol that needs discrete messages must define its own framing on top
(length prefixes, delimiters, fixed sizes — topic 9's `bufio`/
`encoding/binary`).

### Half-close with `CloseWrite`

Without framing, how does a reader know "the sender is done, no more bytes
are coming"? `*net.TCPConn` exposes:

```go
func (c *TCPConn) CloseWrite() error
```

This sends a TCP FIN on the write side only — the connection's read side
stays open. The peer's next `Read` (after draining buffered data) returns
`io.EOF`, which `io.ReadAll` treats as a normal end-of-stream, not an error.
Meanwhile this side can still read the peer's response. This "I'm done
sending, but still listening" signal is a one-shot substitute for framing,
useful for simple request/response exchanges (e.g. send a request, half-close,
read the full response, then close the connection entirely).

## UDP: `ListenUDP`, `DialUDP`, `ReadFromUDP`/`WriteToUDP`

UDP (RFC 768) is **connectionless**: there's no handshake and no persistent
"connection" object on the server side. Each datagram arrives with its own
source address.

```go
addr, _ := net.ResolveUDPAddr("udp", "127.0.0.1:0")
conn, err := net.ListenUDP("udp", addr) // a *net.UDPConn, not a Listener
defer conn.Close()

buf := make([]byte, 1024)
n, from, err := conn.ReadFromUDP(buf) // from identifies the sender
conn.WriteToUDP(buf[:n], from)        // reply to that sender
```

A client side can use `net.DialUDP` to fix the remote address once, then
use plain `Read`/`Write`:

```go
raddr, _ := net.ResolveUDPAddr("udp", "127.0.0.1:9999")
conn, err := net.DialUDP("udp", nil, raddr)
defer conn.Close()
conn.Write(data)
conn.SetReadDeadline(time.Now().Add(time.Second)) // topic 7: bound the wait
n, _ := conn.Read(buf)
```

- UDP delivers whole datagrams or nothing — no partial reads of a single
  datagram, no stream-splitting like TCP. But datagrams can be **lost,
  duplicated, or reordered** — UDP gives no delivery guarantees at all.
- `*net.UDPConn` also satisfies `net.Conn`'s deadline methods, so
  `SetReadDeadline` works exactly as in topic 7 to bound how long `Read`
  waits for a reply that may never come.

---

## Networking: TCP vs UDP at a glance

| | TCP (RFC 793) | UDP (RFC 768) |
|---|---|---|
| Connection | Handshake (`Dial`/`Accept`) establishes a connection | Connectionless — no handshake |
| Delivery | Reliable, ordered byte stream | Best-effort, unordered datagrams |
| Boundaries | None — stream, needs app framing | Each `Write`/`WriteToUDP` is one datagram |
| Loss handling | Automatic retransmission | None — application must handle loss |
| Go API | `net.Listen`/`Dial` → `net.Conn` | `net.ListenUDP`/`DialUDP` → `*net.UDPConn` |
| Per-message sender | N/A (one peer per connection) | `ReadFromUDP` returns sender's `*net.UDPAddr` |

This topic's exercise builds: a TCP echo server using the accept-loop +
`CloseWrite` half-close pattern, and a UDP request/response server using
`ReadFromUDP`/`WriteToUDP` with a client-side read timeout.

## Further Reading

- [`net.Listen`](https://pkg.go.dev/net#Listen), [`net.Dial`](https://pkg.go.dev/net#Dial)
- [`net.Conn`](https://pkg.go.dev/net#Conn), [`net.TCPConn.CloseWrite`](https://pkg.go.dev/net#TCPConn.CloseWrite)
- [`net.ListenUDP`](https://pkg.go.dev/net#ListenUDP), [`net.UDPConn`](https://pkg.go.dev/net#UDPConn)
- [`net.ErrClosed`](https://pkg.go.dev/net#ErrClosed)
- [RFC 793 (TCP)](https://www.rfc-editor.org/rfc/rfc793)
- [RFC 768 (UDP)](https://www.rfc-editor.org/rfc/rfc768)
