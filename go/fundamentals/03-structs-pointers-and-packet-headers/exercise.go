// Package packet implements IPv4 header parsing and serialization
// (RFC 791 §3.1), without support for options (IHL is always 5).
package packet

import (
	"encoding/binary"
	"fmt"
)

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
	if len(data) < 20 {
		return nil, fmt.Errorf("data must be at least 20 bytes (header) but got: %d\n", len(data))
	}

	totalLength := binary.BigEndian.Uint16(data[2:4])
	tos := uint8(data[1])
	ihl := uint8(data[0]) & 0x0F
	if ihl < 5 || int(ihl*4) > len(data) {
		return nil, fmt.Errorf("header words needs to be at least 5 but got %d\n", ihl)
	}

	version := data[0] >> 4
	if version != 4 {
		return nil, fmt.Errorf("not an ipv4 version: %d", version)
	}

	id := binary.BigEndian.Uint16(data[4:6])
	flags := uint8(data[6] >> 5)
	fragmentOffset := uint16(((data[6] << 3) >> 3) | data[7])
	ttl := uint8(data[8])
	protocol := uint8(data[9])
	checksum := binary.BigEndian.Uint16(data[10:12])
	srcIp := binary.BigEndian.Uint32(data[12:16])
	dstIp := binary.BigEndian.Uint32(data[16:20])

	return &IPv4Header{Version: version, IHL: ihl, TOS: tos, TotalLength: totalLength, ID: id, Flags: flags, FragmentOffset: fragmentOffset, TTL: ttl, Protocol: protocol, Checksum: checksum, SrcIP: srcIp, DstIP: dstIp}, nil
}

// HeaderLength returns the header length in bytes (IHL * 4).
func (h *IPv4Header) HeaderLength() int {
	return int(h.IHL * 4)
}

// PayloadLength returns the number of payload bytes following the header
// (TotalLength - HeaderLength()).
func (h *IPv4Header) PayloadLength() int {
	return int(h.TotalLength) - h.HeaderLength()
}

// DecrementTTL decrements TTL by 1. If TTL is already 0, it returns an
// error and leaves h unmodified.
func (h *IPv4Header) DecrementTTL() error {
	if h.TTL == 0 {
		return fmt.Errorf("TTL %d is already <= 0\n", h.TTL)
	}

	h.TTL--

	return nil
}

// MarshalBinary serializes h into a 20-byte IPv4 header (IHL is always
// written as 5; no options are supported).
func (h *IPv4Header) MarshalBinary() []byte {
	var bytes []byte

	bytes = append(bytes, h.Version<<4|h.IHL)
	bytes = append(bytes, h.TOS)
	bytes = append(bytes, byte(h.TotalLength>>8))
	bytes = append(bytes, byte(h.TotalLength)&0xFF)
	bytes = append(bytes, byte(h.ID>>8))
	bytes = append(bytes, byte(h.ID&0xFF))
	bytes = append(bytes, ((h.Flags << 5) | byte(h.FragmentOffset&0x1F00)))
	bytes = append(bytes, (byte(h.FragmentOffset & 0xFF)))
	bytes = append(bytes, h.TTL)
	bytes = append(bytes, h.Protocol)
	bytes = append(bytes, byte(h.Checksum>>8))
	bytes = append(bytes, byte(h.Checksum&0xFF))
	bytes = append(bytes, byte((h.SrcIP&0xFF000000)>>24))
	bytes = append(bytes, byte((h.SrcIP&0x00FF0000)>>16))
	bytes = append(bytes, byte((h.SrcIP&0x0000FF00)>>8))
	bytes = append(bytes, byte(h.SrcIP&0x000000FF))

	bytes = append(bytes, byte((h.DstIP&0xFF000000)>>24))
	bytes = append(bytes, byte((h.DstIP&0x00FF0000)>>16))
	bytes = append(bytes, byte((h.DstIP&0x0000FF00)>>8))
	bytes = append(bytes, byte(h.DstIP&0x000000FF))

	return bytes
}
