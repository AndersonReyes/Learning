// Package dns applies encoding/binary and the net package to the DNS wire
// protocol (RFC 1035): encoding/decoding domain names (including
// compression pointers), building queries, parsing responses, and a
// minimal resolver that sends a type-A query over UDP.
package dns

import (
	"errors"
	"net"
	"time"
)

const (
	// TypeA is the DNS resource record type for an IPv4 address (RFC 1035
	// §3.2.2).
	TypeA uint16 = 1

	// ClassIN is the DNS "Internet" class (RFC 1035 §3.2.4).
	ClassIN uint16 = 1
)

// Response is a parsed DNS response message (RFC 1035 §4.1): the header's
// ID and RCODE, and the A-record IP addresses from the answer section.
type Response struct {
	ID      uint16
	RCODE   uint8
	Answers []net.IP
}

// EncodeName encodes a dot-separated domain name into DNS wire format
// (RFC 1035 §4.1.2): each label prefixed by its length byte, terminated by
// a zero-length label. The root name "" encodes as a single zero byte.
// EncodeName returns an error if any label is empty or longer than 63
// bytes.
func EncodeName(name string) ([]byte, error) {
	return nil, errors.New("not implemented")
}

// DecodeName decodes a domain name starting at data[offset] (RFC 1035
// §4.1.4), following compression pointers. It returns the dot-separated
// name and the offset of the first byte after the name in data — which,
// if a pointer was followed, is the offset immediately after that
// pointer's two bytes, not after the pointed-to data.
func DecodeName(data []byte, offset int) (name string, next int, err error) {
	return "", 0, errors.New("not implemented")
}

// EncodeQuery builds a complete DNS query message (RFC 1035 §4.1): a
// 12-byte header with the given id, the RD (recursion desired) bit set,
// and QDCOUNT=1, followed by a single question for name with the given
// qtype and class IN.
func EncodeQuery(id uint16, name string, qtype uint16) ([]byte, error) {
	return nil, errors.New("not implemented")
}

// ParseResponse parses a DNS response message: the header's ID and RCODE,
// and — for each answer record of type A — its IPv4 address. Non-A answer
// records are skipped.
func ParseResponse(data []byte) (*Response, error) {
	return nil, errors.New("not implemented")
}

// Resolve sends a type-A query for name to server (a "host:port" UDP
// address) and returns the resulting IP addresses. It returns an error if
// the response's ID doesn't match the query's, if RCODE is non-zero, or
// if no response arrives within timeout.
func Resolve(server, name string, timeout time.Duration) ([]net.IP, error) {
	return nil, errors.New("not implemented")
}
