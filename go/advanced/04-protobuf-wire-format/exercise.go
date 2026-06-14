// Package minipb implements the core of the Protocol Buffers binary wire
// format (proto3 "Encoding" spec) by hand: base-128 varints, the
// tag = (field_number<<3)|wire_type encoding, and messages built from the
// VARINT (0) and LENGTH_DELIMITED (2) wire types — enough to encode/decode
// the int64 and string/bytes/nested-message fields of a real .proto message,
// without protoc, a .proto file, or the google.golang.org/protobuf module.
//
// This is the wire format gRPC sends inside each HTTP/2 DATA frame (after a
// 5-byte gRPC frame header: 1 compression-flag byte + a 4-byte big-endian
// length, conceptually the same length-prefix framing as
// fundamentals/09-bufio-io-binary-and-framing) — see examples/main.go for a
// client/server exchange built on these functions, illustrating
// "Service-to-Service Networking" at the level protoc-gen-go-grpc normally
// generates for you.
package minipb

import "errors"

// EncodeVarint returns v encoded as a Protocol Buffers base-128 varint: the
// 7 low bits of each group go in a byte's low 7 bits, with the high bit set
// on every byte except the last to mark "more bytes follow". Groups are
// emitted least-significant-first, so e.g. EncodeVarint(150) = []byte{0x96,
// 0x01}.
func EncodeVarint(v uint64) []byte {
	return nil
}

// DecodeVarint decodes a base-128 varint from the start of data (see
// EncodeVarint) and returns its value and the number of bytes consumed.
// DecodeVarint returns an error if data is empty, if data ends before a byte
// with its high bit clear is found, or if the varint would need more than 10
// bytes or overflow 64 bits to represent (i.e. the 10th byte's low 7 bits
// are not exactly 0 or 1).
func DecodeVarint(data []byte) (value uint64, n int, err error) {
	return 0, 0, errors.New("not implemented")
}

// EncodeMessage encodes fields as a Protocol Buffers message: for each field
// number (visited in ascending order, for deterministic output), a tag
// varint encoding (field_number<<3)|wire_type followed by the field's value.
// fields' values must be either uint64 (encoded as wire type 0, VARINT, via
// EncodeVarint) or []byte (encoded as wire type 2, LENGTH_DELIMITED: a
// varint length followed by the bytes themselves — this is how proto3
// encodes string, bytes, and embedded message fields). EncodeMessage returns
// an error if any field number is less than 1, or any value is neither
// uint64 nor []byte.
func EncodeMessage(fields map[int]any) ([]byte, error) {
	return nil, errors.New("not implemented")
}

// DecodeMessage decodes data as a sequence of Protocol Buffers
// (tag, value) pairs and returns them keyed by field number: a field with
// wire type 0 (VARINT) decodes to a uint64, and a field with wire type 2
// (LENGTH_DELIMITED) decodes to its raw []byte content (proto3 can't tell
// string/bytes/embedded-message fields apart without the .proto schema —
// see DecodeNestedMessage). DecodeMessage returns an error if data is
// malformed (a truncated tag or length varint, or a length-delimited field
// whose length exceeds the remaining data) or contains a wire type other
// than 0 or 2.
func DecodeMessage(data []byte) (map[int]any, error) {
	return nil, errors.New("not implemented")
}

// DecodeNestedMessage behaves like DecodeMessage, except that for any
// LENGTH_DELIMITED field whose number is a key of messageFields with value
// true, the field's bytes are recursively decoded with DecodeNestedMessage
// (using the same messageFields schema) instead of being returned raw — so
// the result's values are uint64 (wire type 0), []byte (wire type 2, not
// listed in messageFields), or map[int]any (wire type 2, listed in
// messageFields, successfully decoded as a nested message).
// DecodeNestedMessage returns an error under the same conditions as
// DecodeMessage, including when a field marked in messageFields does not
// itself decode as a valid message.
func DecodeNestedMessage(data []byte, messageFields map[int]bool) (map[int]any, error) {
	return nil, errors.New("not implemented")
}
