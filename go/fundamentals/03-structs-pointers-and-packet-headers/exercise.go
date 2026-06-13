// Package packet implements IPv4 header parsing and serialization
// (RFC 791 §3.1), without support for options (IHL is always 5).
package packet

import "errors"

// IPv4Header represents the fields of a 20-byte IPv4 header (no options).
type IPv4Header struct {
	Version        uint8
	IHL            uint8 // header length in 32-bit words
	TOS            uint8
	TotalLength    uint16
	ID             uint16
	Flags          uint8  // 3-bit value: bit1=DF, bit2=MF
	FragmentOffset uint16 // 13-bit value, in 8-byte units
	TTL            uint8
	Protocol       uint8
	Checksum       uint16
	SrcIP          uint32
	DstIP          uint32
}

// ParseIPv4Header parses the first 20+ bytes of data as an IPv4 header.
//
//	ParseIPv4Header(<20 valid bytes>) -> &IPv4Header{...}, nil
//	len(data) < 20                    -> nil, error
//	version != 4                      -> nil, error
//	IHL < 5                           -> nil, error
//	IHL*4 > len(data)                 -> nil, error
func ParseIPv4Header(data []byte) (*IPv4Header, error) {
	return nil, errors.New("not implemented")
}

// HeaderLength returns the header length in bytes (IHL * 4).
func (h *IPv4Header) HeaderLength() int {
	return 0
}

// PayloadLength returns the number of payload bytes following the header
// (TotalLength - HeaderLength()).
func (h *IPv4Header) PayloadLength() int {
	return 0
}

// DecrementTTL decrements TTL by 1. If TTL is already 0, it returns an
// error and leaves h unmodified.
func (h *IPv4Header) DecrementTTL() error {
	return errors.New("not implemented")
}

// MarshalBinary serializes h into a 20-byte IPv4 header (IHL is always
// written as 5; no options are supported).
func (h *IPv4Header) MarshalBinary() []byte {
	return nil
}
