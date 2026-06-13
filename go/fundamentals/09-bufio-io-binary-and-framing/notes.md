# 09. `bufio`/`io`/`encoding/binary`

## `io.Reader`/`io.Writer` recap and `io.ReadFull`

Topic 5 introduced `io.Reader`/`io.Writer` as the interfaces `net.Conn`
satisfies. A single `Read` can return **fewer bytes than the buffer's
length** even without an error — the caller must loop. `io.ReadFull`
does that loop for you:

```go
func ReadFull(r Reader, buf []byte) (n int, err error)
```

- Returns `nil` error only if `len(buf)` bytes were read.
- Returns `io.EOF` if **zero** bytes were read before the stream ended —
  a "clean" end-of-stream, the expected case when the caller stops asking
  for more.
- Returns `io.ErrUnexpectedEOF` if **1..len(buf)-1** bytes were read before
  the stream ended — the stream ended *mid-value*, which is always an
  error (a truncated message).

This EOF-vs-ErrUnexpectedEOF distinction is the standard way to detect
"clean end of stream" vs. "truncated/corrupted stream" in Go.

## `bufio.Reader` / `bufio.Writer`

Raw `net.Conn`/`os.File` reads and writes are syscalls — expensive if done
one byte (or one small struct) at a time. `bufio` wraps an `io.Reader`/
`io.Writer` with an in-memory buffer:

```go
r := bufio.NewReader(conn) // default 4096-byte buffer
w := bufio.NewWriter(conn)

line, err := r.ReadString('\n') // buffered reads
n, err := w.Write(data)         // buffered — may not hit conn yet
err = w.Flush()                 // force buffered data out to conn
```

- `bufio.Reader` satisfies `io.Reader` — `io.ReadFull(bufioReader, buf)`
  works exactly as it would on the unwrapped reader, just faster across
  many small reads, because `bufio` does the syscall in larger chunks
  internally.
- **`bufio.Writer` buffers writes in memory.** Data isn't guaranteed to
  reach the underlying writer until `Flush` is called (or the buffer
  fills). Forgetting `Flush` is a classic bug: the program looks correct
  but the peer never receives the bytes (or only receives them once the
  buffer happens to fill).
- `bufio.Scanner` (seen in topic 8's chat server example) is built on the
  same idea for line- or token-oriented input.

## `encoding/binary`: byte order

Network protocols define a fixed byte order for multi-byte integers —
usually **big-endian** ("network byte order"). `encoding/binary` converts
between integers and their byte representations:

```go
var header [4]byte
binary.BigEndian.PutUint32(header[:], 42)  // header = [0x00 0x00 0x00 0x2a]
n := binary.BigEndian.Uint32(header[:])    // n == 42
```

- `binary.BigEndian` and `binary.LittleEndian` both implement
  `binary.ByteOrder`, with `PutUint16/32/64` (encode) and
  `Uint16/32/64` (decode) methods operating on fixed-size `[]byte`.
- `binary.Write`/`binary.Read` do the same via `io.Writer`/`io.Reader` and
  reflection, for fixed-size structs — convenient, but the explicit
  `PutUint32`/`Uint32` form above is faster and makes the wire format
  obvious.
- Variable-length integers (`binary.Uvarint`/`PutUvarint`) are an
  alternative used by protocols like Protocol Buffers — smaller for small
  values, but more complex to parse. This topic uses a fixed-size header.

---

## Networking: length-prefixed framing

Topic 8 established that TCP delivers a byte **stream** with no message
boundaries. The standard fix is **length-prefixed framing**: before each
message's payload, write a fixed-size header containing the payload's
length in bytes.

```
+----------------+------------------------+
| length (4B BE) | payload (length bytes) |
+----------------+------------------------+
```

To read one message: read the fixed-size header with `io.ReadFull`, decode
the length with `binary.BigEndian.Uint32`, then `io.ReadFull` exactly that
many payload bytes. Because both reads use `io.ReadFull`, a clean `io.EOF`
on the header read means "no more messages" (the stream ended exactly at a
boundary), while `io.ErrUnexpectedEOF` anywhere else means the stream was
truncated mid-message — a clear protocol error.

This isn't a toy: **DNS over TCP** ([RFC 1035 §4.2.2](https://www.rfc-editor.org/rfc/rfc1035#section-4.2.2))
prefixes each DNS message with a 2-byte length, for exactly this reason —
DNS messages were originally designed for UDP's datagram framing (topic 8),
and TCP needs an explicit length to recover those boundaries. Many other
binary protocols (gRPC, Thrift, SSH) use the same length-prefix pattern,
often with a max-size check on the decoded length to reject corrupted or
malicious headers before allocating a buffer for the payload.

This topic's exercise builds exactly this: `WriteMessage`/`ReadMessage` for
a single length-prefixed message, `ReadMessageLimit` to bound the
allocation, and `WriteMessages`/`ReadAllMessages` to frame a whole stream of
messages with `bufio`.

## Further Reading

- [`io.ReadFull`](https://pkg.go.dev/io#ReadFull), [`io.EOF`](https://pkg.go.dev/io#EOF), [`io.ErrUnexpectedEOF`](https://pkg.go.dev/io#ErrUnexpectedEOF)
- [`bufio.Reader`](https://pkg.go.dev/bufio#Reader), [`bufio.Writer`](https://pkg.go.dev/bufio#Writer)
- [`encoding/binary`](https://pkg.go.dev/encoding/binary), [`binary.ByteOrder`](https://pkg.go.dev/encoding/binary#ByteOrder)
- [RFC 1035 §4.2.2 (DNS over TCP framing)](https://www.rfc-editor.org/rfc/rfc1035#section-4.2.2)
