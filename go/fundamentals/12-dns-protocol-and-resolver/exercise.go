// Package dns applies encoding/binary and the net package to the DNS wire
// protocol (RFC 1035): encoding/decoding domain names (including
// compression pointers), building queries, parsing responses, and a
// minimal resolver that sends a type-A query over UDP.
package dns

import (
	"bytes"
	"encoding/binary"
	"errors"
	"fmt"
	"net"
	"strings"
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
	if len(name) == 0 {
		return []byte{0}, nil
	}

	var buf bytes.Buffer

	parts := strings.SplitSeq(name, ".")
	for part := range parts {
		if len(part) == 0 {
			return nil, fmt.Errorf("empty part now allowed in [%s]\n", name)
		}

		lengthByte := len(part)
		if lengthByte >= 64 {
			return nil, fmt.Errorf("label [%s] is over 63 bytes long\n", part)
		}
		buf.WriteByte(byte(lengthByte))
		buf.WriteString(part)
	}
	buf.WriteByte(0)
	return buf.Bytes(), nil
}

// DecodeName decodes a domain name starting at data[offset] (RFC 1035
// §4.1.4), following compression pointers. It returns the dot-separated
// name and the offset of the first byte after the name in data — which,
// if a pointer was followed, is the offset immediately after that
// pointer's two bytes, not after the pointed-to data.
func DecodeName(data []byte, offset int) (name string, next int, err error) {
	var parts []string
	curr := offset
	done := false
	for !done {
		if curr >= len(data) {
			break
		}

		b := data[curr]
		//log.Printf("PRE: curr=%d, b=%x\n", curr, b)

		if b == 0 {
			// finished. Advace curr to the next offset
			curr += 1
			done = true
			break
		}

		// check if length byte is a pointer
		if (b & 0xC0) == 0xC0 {
			highBits := uint16(data[curr]&0x3F) << 8
			// set curr to next offset
			nextOffset := int(highBits | uint16(data[curr+1]))
			//log.Printf("PTR-pre-recursion: curr=%d, b=%x\n", curr, b)
			name, _, err := DecodeName(data, nextOffset)
			curr += 2 // processed curr and curr+1
			//log.Printf("PTR-post-recursion: curr=%d, b=%x\n", curr, b)
			parts = append(parts, name)

			if err != nil {
				return "", 0, err
			}

			done = true
		} else {
			// now its just normal length
			n := int(b)
			start := curr + 1
			end := curr + n + 1
			slice := data[start:end]

			parts = append(parts, string(slice))
			curr = end
			//log.Printf("LENGTH: curr=%d, b=%x\n", curr, b)
		}
	}
	return strings.Join(parts, "."), curr, nil
}

// EncodeQuery builds a complete DNS query message (RFC 1035 §4.1): a
// 12-byte header with the given id, the RD (recursion desired) bit set,
// and QDCOUNT=1, followed by a single question for name with the given
// qtype and class IN.
func EncodeQuery(id uint16, name string, qtype uint16) ([]byte, error) {
	header := make([]byte, 12)
	binary.BigEndian.PutUint16(header[0:2], id)
	binary.BigEndian.PutUint16(header[2:4], 0x0100)
	binary.BigEndian.PutUint16(header[4:6], 1)

	encodedName, err := EncodeName(name)
	if err != nil {
		return nil, err
	}
	qtypeBuf := []byte{1, 1}
	binary.BigEndian.PutUint16(qtypeBuf, qtype)

	qclassBuf := []byte{0, 0}
	binary.BigEndian.PutUint16(qclassBuf, ClassIN)

	var query []byte
	query = append(query, header...)
	query = append(query, encodedName...)
	query = append(query, qtypeBuf...)
	query = append(query, qclassBuf...)

	return query, nil
}

// ParseResponse parses a DNS response message: the header's ID and RCODE,
// and — for each answer record of type A — its IPv4 address. Non-A answer
// records are skipped.
func ParseResponse(data []byte) (*Response, error) {
	headerBytes := data[:12]
	id := binary.BigEndian.Uint16(headerBytes[:2])
	flags := binary.BigEndian.Uint16(headerBytes[2:4])
	answerCount := binary.BigEndian.Uint16(headerBytes[6:8])
	var ips []net.IP

	for range answerCount {
		
	}

	return nil, errors.New("not implemented")
}

// Resolve sends a type-A query for name to server (a "host:port" UDP
// address) and returns the resulting IP addresses. It returns an error if
// the response's ID doesn't match the query's, if RCODE is non-zero, or
// if no response arrives within timeout.
func Resolve(server, name string, timeout time.Duration) ([]net.IP, error) {
	return nil, errors.New("not implemented")
}
