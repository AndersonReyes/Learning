// Command main demonstrates structs, pointers, methods, and
// encoding/binary concepts used in this topic's exercise: struct literals,
// pointer vs value receivers, encoding/binary BigEndian reads/writes, and
// fixed-size byte arrays — applied to an Ethernet frame header and a UDP
// header, deliberately *not* the exercise (an IPv4 header) in exercise.go.
package main

import (
	"encoding/binary"
	"fmt"
)

// EthernetFrame represents the fixed 14-byte header of an Ethernet II frame.
type EthernetFrame struct {
	DstMAC    [6]byte
	SrcMAC    [6]byte
	EtherType uint16
}

// macString formats a MAC address as "aa:bb:cc:dd:ee:ff".
func macString(mac [6]byte) string {
	return fmt.Sprintf("%02x:%02x:%02x:%02x:%02x:%02x",
		mac[0], mac[1], mac[2], mac[3], mac[4], mac[5])
}

// parseEthernetFrame demonstrates copying into a fixed-size array and
// encoding/binary.BigEndian.Uint16.
func parseEthernetFrame(data []byte) (*EthernetFrame, error) {
	if len(data) < 14 {
		return nil, fmt.Errorf("frame too short: %d bytes", len(data))
	}
	f := &EthernetFrame{}
	copy(f.DstMAC[:], data[0:6])
	copy(f.SrcMAC[:], data[6:12])
	f.EtherType = binary.BigEndian.Uint16(data[12:14])
	return f, nil
}

// UDPHeader represents the 8-byte UDP header (RFC 768).
type UDPHeader struct {
	SrcPort  uint16
	DstPort  uint16
	Length   uint16
	Checksum uint16
}

// PayloadLength uses a value receiver: it only reads h, so a copy is fine.
func (h UDPHeader) PayloadLength() int {
	return int(h.Length) - 8
}

// SetChecksum uses a pointer receiver: it mutates h in place.
func (h *UDPHeader) SetChecksum(c uint16) {
	h.Checksum = c
}

// marshal demonstrates encoding/binary.BigEndian.PutUint16.
func (h UDPHeader) marshal() []byte {
	out := make([]byte, 8)
	binary.BigEndian.PutUint16(out[0:2], h.SrcPort)
	binary.BigEndian.PutUint16(out[2:4], h.DstPort)
	binary.BigEndian.PutUint16(out[4:6], h.Length)
	binary.BigEndian.PutUint16(out[6:8], h.Checksum)
	return out
}

func main() {
	frame, err := parseEthernetFrame([]byte{
		0xff, 0xff, 0xff, 0xff, 0xff, 0xff, // dst MAC: broadcast
		0x00, 0x1b, 0x63, 0x84, 0x45, 0xe6, // src MAC
		0x08, 0x00, // EtherType: IPv4
	})
	if err != nil {
		fmt.Println("parse error:", err)
		return
	}
	fmt.Printf("dst MAC: %s\n", macString(frame.DstMAC))
	fmt.Printf("src MAC: %s\n", macString(frame.SrcMAC))
	fmt.Printf("EtherType: 0x%04x\n", frame.EtherType)

	// struct literal with named fields
	udp := UDPHeader{SrcPort: 53, DstPort: 12345, Length: 40}
	fmt.Println("UDP payload length:", udp.PayloadLength())

	// pointer receiver: Go auto-takes the address of an addressable value
	udp.SetChecksum(0x1234)
	fmt.Printf("UDP checksum after SetChecksum: 0x%04x\n", udp.Checksum)
	fmt.Printf("marshaled UDP header: % x\n", udp.marshal())

	// pointers: zero value, address-of, dereference
	var p *UDPHeader
	fmt.Println("nil pointer:", p == nil)
	p = &udp
	fmt.Println("dereferenced SrcPort:", (*p).SrcPort, "via auto-deref:", p.SrcPort)
}
