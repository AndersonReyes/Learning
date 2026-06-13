# 03. Structs, Pointers & Methods

## Structs

A struct is a typed collection of named fields:

```go
type IPv4Header struct {
	Version uint8
	IHL     uint8
	TTL     uint8
	SrcIP   uint32
}

h := IPv4Header{Version: 4, IHL: 5, TTL: 64} // struct literal, named fields
h.TTL // 64 — field access with dot notation
```

Unset fields take their zero value (`0` for numeric types, `""` for
strings, `nil` for slices/maps/pointers).

## Pointers

`&x` takes the address of `x` (type `*T`). `*p` dereferences a pointer to
get the value it points to.

```go
h := IPv4Header{TTL: 64}
p := &h          // p is *IPv4Header
(*p).TTL = 63    // explicit dereference
p.TTL = 63       // Go auto-dereferences struct field access through a pointer
```

A `nil` pointer has no underlying value — dereferencing it panics.

## Methods: value vs pointer receivers

A method is a function with a receiver argument before its name:

```go
func (h IPv4Header) PayloadLength() int { ... }   // value receiver: gets a COPY of h
func (h *IPv4Header) DecrementTTL() error { ... } // pointer receiver: can mutate h
```

- **Value receiver**: the method operates on a copy. Fine for read-only
  methods on small structs.
- **Pointer receiver**: required to mutate the receiver's fields, or to
  avoid copying a large struct.
- Convention: if ANY method on a type needs a pointer receiver, give ALL
  its methods pointer receivers for consistency.
- Go auto-takes the address for pointer-receiver calls on addressable
  values: `h.DecrementTTL()` is shorthand for `(&h).DecrementTTL()` when
  `h` is a local variable (NOT when `h` is the result of a function call —
  that value isn't addressable).

## `encoding/binary`: byte order

Network protocols use **big-endian** ("network byte order") for
multi-byte integer fields — the most significant byte comes first.
`encoding/binary` reads/writes multi-byte integers from/to `[]byte`:

```go
import "encoding/binary"

binary.BigEndian.Uint16(data[2:4])        // read a 16-bit field
binary.BigEndian.Uint32(data[12:16])      // read a 32-bit field
binary.BigEndian.PutUint16(out[2:4], v)   // write a 16-bit field
binary.BigEndian.PutUint32(out[12:16], v) // write a 32-bit field
```

## Bit-field packing

Some header fields share a byte or word. Extract with shift (`>>`) and
mask (`&`); pack with shift and OR (`|`):

```go
// byte 0 of an IPv4 header: high nibble = version, low nibble = IHL
version := data[0] >> 4    // shift off the low nibble
ihl := data[0] & 0x0F      // mask off the high nibble
b0 := (version << 4) | ihl // pack back together

// a 16-bit value: high 3 bits = flags, low 13 bits = fragment offset
v := binary.BigEndian.Uint16(data[6:8])
flags := uint8(v >> 13)
fragOffset := v & 0x1FFF // 0x1FFF = 13 ones

packed := (uint16(flags) << 13) | (fragOffset & 0x1FFF)
```

---

## Networking: the IPv4 header (RFC 791 §3.1)

[RFC 791 §3.1](https://www.rfc-editor.org/rfc/rfc791#section-3.1) defines
the fixed 20-byte IPv4 header (options omitted here):

| Bytes | Field | Size | Notes |
|-------|-------|------|-------|
| 0 | Version (high nibble) \| IHL (low nibble) | 4+4 bits | IHL = header length in 32-bit **words** (5 = 20 bytes, no options) |
| 1 | Type of Service (TOS) | 8 bits | |
| 2-3 | Total Length | 16 bits | header + payload, in bytes |
| 4-5 | Identification | 16 bits | for reassembling fragments |
| 6-7 | Flags (high 3 bits) \| Fragment Offset (low 13 bits) | 3+13 bits | flags bit1=DF (don't fragment), bit2=MF (more fragments); offset in 8-byte units |
| 8 | Time To Live (TTL) | 8 bits | decremented by each router; packet dropped at 0 |
| 9 | Protocol | 8 bits | 6=TCP, 17=UDP, 1=ICMP |
| 10-11 | Header Checksum | 16 bits | covers the header only |
| 12-15 | Source Address | 32 bits | |
| 16-19 | Destination Address | 32 bits | |

### Worked example

Hex bytes `45 00 00 34 1c 46 40 00 40 06 b1 e6 c0 a8 01 01 c0 a8 01 02`:

- `0x45` -> version `4`, IHL `5` (5 * 4 = 20-byte header, no options)
- `0x00` -> TOS `0`
- `0x0034` -> total length `52` bytes (20-byte header + 32-byte payload)
- `0x1c46` -> identification `7238`
- `0x4000` -> flags `010` (DF set), fragment offset `0`
- `0x40` -> TTL `64`
- `0x06` -> protocol `6` (TCP)
- `0xb1e6` -> header checksum
- `0xc0a80101` -> source `192.168.1.1`
- `0xc0a80102` -> destination `192.168.1.2`

`HeaderLength()` = IHL * 4 = 20. `PayloadLength()` = TotalLength -
HeaderLength() = 52 - 20 = 32.

## Further Reading

- [Tour: Pointers](https://go.dev/tour/moretypes/1)
- [Tour: Structs](https://go.dev/tour/moretypes/2)
- [Tour: Methods](https://go.dev/tour/methods/1)
- [Tour: Pointer receivers](https://go.dev/tour/methods/4)
- [`encoding/binary`](https://pkg.go.dev/encoding/binary)
- [RFC 791 §3.1 — Internet Header Format](https://www.rfc-editor.org/rfc/rfc791#section-3.1)
