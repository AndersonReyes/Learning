// Package resp implements a parser and encoder for RESP (REdis
// Serialization Protocol), the wire protocol used by Redis and
// Redis-compatible servers — a real-world recursive text protocol whose
// length-prefixed bulk strings make it a natural target for the fuzz
// testing (testing.F) introduced by this topic's test suite, alongside the
// table-driven tests and httptest patterns used throughout this curriculum.
package resp

import (
	"bufio"
	"errors"
)

// Type identifies a RESP value's type
// (https://redis.io/docs/latest/develop/reference/protocol-spec/).
type Type byte

// RESP type bytes.
const (
	SimpleString Type = '+'
	Error        Type = '-'
	Integer      Type = ':'
	BulkString   Type = '$'
	Array        Type = '*'
)

// Value is a parsed RESP value. Which fields are meaningful depends on
// Type:
//   - SimpleString, Error: Str holds the line's contents.
//   - Integer: Int holds the parsed number.
//   - BulkString: Str holds the binary-safe payload, unless Null is true
//     (the "$-1\r\n" null bulk string).
//   - Array: Array holds the parsed elements, unless Null is true (the
//     "*-1\r\n" null array).
type Value struct {
	Type  Type
	Str   string
	Int   int64
	Array []Value
	Null  bool
}

// ParseValue reads and parses a single RESP value from r (recursively, for
// arrays). To guard against a malicious or corrupt length triggering an
// out-of-memory allocation, ParseValue returns an error if a bulk string's
// length or an array's element count is negative (other than the -1 used
// for null) or implausibly large (more than 512 MiB for a bulk string, or
// more than 1<<20 elements for an array).
func ParseValue(r *bufio.Reader) (Value, error) {
	return Value{}, errors.New("not implemented")
}

// Encode returns v's RESP wire representation.
func (v Value) Encode() []byte {
	return nil
}

// String returns a human-readable representation of v, in the style of
// redis-cli: simple strings are printed as-is, errors are prefixed with
// "(error) ", integers with "(integer) ", bulk strings are double-quoted,
// arrays are numbered one element per line (or "(empty array)" if empty),
// and null bulk strings/arrays are printed as "(nil)".
func (v Value) String() string {
	return ""
}

// EncodeCommand encodes args as a RESP array of bulk strings — the format
// RESP clients use to send commands to a server.
func EncodeCommand(args ...string) []byte {
	return nil
}

// ParseCommand reads a single RESP value from r and returns its elements
// as strings. It returns an error if the value is not a non-null array of
// non-null bulk strings.
func ParseCommand(r *bufio.Reader) ([]string, error) {
	return nil, errors.New("not implemented")
}
