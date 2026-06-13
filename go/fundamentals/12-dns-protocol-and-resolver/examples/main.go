// Command main demonstrates DNS wire-format concepts used in this topic's
// exercise — encoding/decoding domain names, building a query header, and
// reading flags/RCODE out of a response — applied to a hand-built CHAOS
// TXT query for "version.bind" (a well-known diagnostic query supported by
// many resolvers) and to a manually-assembled response message. It also
// shows Go's idiomatic high-level DNS API (`net.Resolver`), which is what
// applications normally use instead of speaking the wire protocol
// directly. These are deliberately *not* the exercise (EncodeName/
// DecodeName/EncodeQuery/ParseResponse/Resolve in exercise.go).
package main

import (
	"context"
	"encoding/binary"
	"fmt"
	"net"
	"strings"
	"time"
)

// encodeLabels encodes a dot-separated name into DNS wire format, without
// any of the error-checking the exercise's EncodeName requires.
func encodeLabels(name string) []byte {
	var out []byte
	for _, label := range strings.Split(name, ".") {
		out = append(out, byte(len(label)))
		out = append(out, label...)
	}
	return append(out, 0)
}

func main() {
	// Build a query header by hand: ID=0xABCD, RD bit set, QDCOUNT=1.
	header := make([]byte, 12)
	binary.BigEndian.PutUint16(header[0:2], 0xABCD)
	binary.BigEndian.PutUint16(header[2:4], 0x0100)
	binary.BigEndian.PutUint16(header[4:6], 1)
	fmt.Printf("query header bytes: % x\n", header)
	fmt.Printf("query ID: %#04x, RD bit set: %v, QDCOUNT: %d\n",
		binary.BigEndian.Uint16(header[0:2]),
		binary.BigEndian.Uint16(header[2:4])&0x0100 != 0,
		binary.BigEndian.Uint16(header[4:6]))

	// Encode a question name.
	name := encodeLabels("version.bind")
	fmt.Printf("encoded %q: % x (%d bytes)\n", "version.bind", name, len(name))

	// Manually assemble a response header with RCODE=3 (NXDOMAIN) and
	// show how the flags field packs QR/RD/RCODE together.
	flags := uint16(0x8183) // QR=1, RD=1, RCODE=3
	respHeader := make([]byte, 12)
	binary.BigEndian.PutUint16(respHeader[2:4], flags)
	rcode := uint8(binary.BigEndian.Uint16(respHeader[2:4]) & 0x0F)
	isResponse := binary.BigEndian.Uint16(respHeader[2:4])&0x8000 != 0
	fmt.Printf("response flags %#04x: is response = %v, RCODE = %d (NXDOMAIN)\n", flags, isResponse, rcode)

	// A compression pointer is the top two bits of a length byte set to
	// 11, plus 14 bits of offset.
	pointer := []byte{0xC0, 0x0C}
	offset := int(pointer[0]&0x3F)<<8 | int(pointer[1])
	fmt.Printf("compression pointer % x -> offset %d\n", pointer, offset)

	fmt.Println()
	fmt.Println("--- net.Resolver: the idiomatic way to do DNS lookups ---")

	// net.Resolver wraps exactly this wire protocol: building queries,
	// sending them over UDP (falling back to TCP for large responses),
	// and parsing responses — all hidden behind a simple Go API.
	resolver := net.Resolver{}
	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	ips, err := resolver.LookupIPAddr(ctx, "localhost")
	if err != nil {
		fmt.Println("LookupIPAddr error:", err)
		return
	}
	for _, ip := range ips {
		fmt.Println("localhost resolves to:", ip.String())
	}
}
