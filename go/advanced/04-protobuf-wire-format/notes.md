# Advanced 4. Varint Encoding & Type Switches + the Protocol Buffers Wire Format (gRPC, by Hand)

## Why a hand-rolled wire format, not `google.golang.org/protobuf`/`grpc-go`

The roadmap originally scoped this topic as gRPC + Protocol Buffers via
`google.golang.org/grpc` and `google.golang.org/protobuf`, generated from a
`.proto` file with `protoc`. Those are both new module dependencies and a
new toolchain dependency (`protoc`, `protoc-gen-go`, `protoc-gen-go-grpc`),
none of which are available here. Generated code also *hides* the wire
format — exactly the part worth understanding.

So this topic implements the core of the **proto3 binary wire format**
directly: base-128 varints, the `tag = (field_number<<3)|wire_type` scheme,
and messages built from wire types 0 (VARINT) and 2 (LENGTH_DELIMITED). This
is precisely the format `protoc-gen-go` generates `Marshal`/`Unmarshal` code
for, and the format gRPC sends inside HTTP/2 `DATA` frames. If you come back
to this with the real modules available, `exercise.go`'s functions are a
correctness reference for what `proto.Marshal`/`proto.Unmarshal` do.

## New: base-128 varints (LEB128), vs. QUIC's varints (topic 2)

`advanced/02-quic-and-http3` introduced QUIC's varint: a **fixed-width**
encoding where the top 2 bits of the first byte select a length class (1, 2,
4, or 8 bytes), so the encoded length is knowable from byte 0 alone.

Protocol Buffers varints (LEB128 — "Little Endian Base 128") are
**unbounded** and self-terminating instead: each byte holds 7 value bits in
its low bits, with the high bit (`0x80`) as a **continuation flag** — 1 means
"another byte follows", 0 means "this is the last byte". Groups are emitted
**least-significant-first**:

```go
// encode: peel off 7 bits at a time, LSB first
for v > 0 {
    b := byte(v & 0x7f)
    v >>= 7
    if v > 0 {
        b |= 0x80 // more bytes follow
    }
    out = append(out, b)
}

// decode: accumulate 7-bit groups, shifting each group further left
var result uint64
for i, b := range data {
    result |= uint64(b&0x7f) << uint(7*i)
    if b&0x80 == 0 {
        return result, i + 1, nil // done
    }
}
```

150 (`0b1001_0110`) splits into low 7 bits `001_0110` (`0x16`) and remaining
bit `1` (`0x01`); the low group gets the continuation bit set, giving
`[0x96, 0x01]` — the canonical example from the protobuf docs.

**Gotchas**:

- A `uint64` needs **at most 10 bytes**: 9 bytes carry 63 bits, the 10th
  carries the last bit. Decoding must reject an 11th continuation byte
  ("too long") and reject a 10th byte whose low 7 bits exceed 1 ("overflows
  uint64") — both are malformed input, not just large-but-valid numbers.
- `0` still encodes as one byte (`0x00`) — the `for v > 0` loop never
  executes, so it needs an explicit `v == 0` case.
- Varints have **no inherent length limit on the input side**: decoding must
  consume only as many bytes as needed and report `n` back to the caller, so
  the caller can continue parsing the rest of the message (see "ignores
  trailing bytes" in `exercise_test.go`).

## New: `map[int]any` and type switches for a schema-less message

Real protobuf messages are typed by a `.proto` schema (`int64 id = 1;`,
`string name = 2;`, ...) that `protoc` compiles into Go structs. Without a
schema, `EncodeMessage`/`DecodeMessage` represent a message as
`map[int]any` — field number to value — and use a **type switch** to decide
how to encode each value:

```go
switch v := fields[num].(type) {
case uint64:
    // wire type 0 (VARINT)
case []byte:
    // wire type 2 (LENGTH_DELIMITED)
default:
    return nil, fmt.Errorf("unsupported value type %T", v)
}
```

This is the same `any` + type-switch pattern as decoding JSON into
`map[string]any` (`fundamentals/11`) — a dynamically-typed view of data
whose real type is defined elsewhere (a `.proto` file, here; a JSON Schema,
there).

## Networking: the Protocol Buffers wire format

A proto3 message is a flat sequence of **(tag, value)** pairs, back to back
— there's no message-level length or field count, just "decode until the
bytes run out". Each pair is:

```
tag   = varint encoding (field_number << 3) | wire_type
value = encoding determined by wire_type
```

This topic implements two of proto3's wire types:

- **Wire type 0, VARINT**: the value *is* a varint (see above). Used for
  `int32`/`int64`/`uint32`/`uint64`/`bool`/`enum` fields.
- **Wire type 2, LENGTH_DELIMITED**: a varint length, then exactly that many
  raw bytes. Used for `string`, `bytes`, repeated packed fields, and
  **embedded messages** — a nested message is encoded by recursively running
  this same algorithm and treating the result as opaque bytes.

`EncodeMessage` visits field numbers in ascending order for **deterministic
output** (real protobuf encoders don't guarantee field order, but a fixed
order makes this topic's encode/decode round trip byte-for-byte
predictable and testable). `DecodeMessage` makes no such assumption — it
decodes whatever order the bytes are in, which is why field numbers come
from the tag, not from position.

## `DecodeNestedMessage`: proto3's schema ambiguity

Given just the bytes, `DecodeMessage` **cannot tell** whether a
LENGTH_DELIMITED field is a `string`, `bytes`, or an embedded message — all
three are "a varint length, then that many bytes". A real `.proto` schema
resolves this per field number; `DecodeNestedMessage`'s `messageFields
map[int]bool` is a minimal stand-in for that schema: "field 3 is itself a
message, recurse into it; everything else, return raw bytes."

This is also why `DecodeNestedMessage` can fail in a way `DecodeMessage`
can't: a field marked in `messageFields` might contain bytes that parse fine
as raw LENGTH_DELIMITED content but aren't a valid nested message (e.g. a
truncated inner varint) — the schema's *claim* about the field can be wrong
relative to the bytes.

## Connection to gRPC: HTTP/2 framing

A gRPC call sends one or more protobuf-encoded messages as the body of an
HTTP/2 request/response, each prefixed with a 5-byte **gRPC frame header**:
1 byte (compression flag) + 4 bytes (big-endian message length) — the same
length-prefix framing idea as `fundamentals/09-bufio-io-binary-and-framing`'s
`WriteMessage`/`ReadMessage`, just with an extra leading flag byte. `protoc-
gen-go-grpc` generates client/server stubs that: frame each message with
that 5-byte header, write it to an HTTP/2 `DATA` frame, and on the other
side, read the header, read exactly that many bytes, and call
`DecodeMessage` (well, `proto.Unmarshal`) on them. `examples/main.go` builds
exactly this over a real TCP connection, using `EncodeMessage`/
`DecodeMessage` from this package and `ReadMessage`/`WriteMessage`-style
length-prefix framing for transport.

## Further Reading

- [Protocol Buffers Encoding reference](https://protobuf.dev/programming-guides/encoding/) —
  the spec this package implements a subset of
- [`encoding/binary`](https://pkg.go.dev/encoding/binary), [`sort`](https://pkg.go.dev/sort) —
  used by `EncodeMessage` for deterministic field ordering
- `fundamentals/09-bufio-io-binary-and-framing` — the length-prefix framing
  pattern gRPC's 5-byte header generalizes
- `advanced/02-quic-and-http3` — QUIC's fixed-width varint, for contrast with
  LEB128
