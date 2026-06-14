// Command main demonstrates two pieces of this topic's exercise on inputs
// different from exercise_test.go: RFC 9000 §16 variable-length integers
// applied to values that look like real QUIC stream IDs/offsets/packet
// numbers (rather than the RFC's own worked examples), and the
// crypto/tls.QUICEncryptionLevel values RunHandshake switches on. Neither
// requires root or a network connection.
package main

import (
	"crypto/tls"
	"fmt"

	miniquic "github.com/andersonreyes/learning/go/advanced/02-quic-and-http3"
)

func main() {
	fmt.Println("--- RFC 9000 §16 variable-length integers ---")
	for _, v := range []uint64{0, 100, 20000, 2_000_000_000, (1 << 62) - 1} {
		enc, err := miniquic.EncodeVarint(v)
		if err != nil {
			fmt.Printf("EncodeVarint(%d) error: %v\n", v, err)
			continue
		}
		fmt.Printf("EncodeVarint(%d) = % x (%d bytes)\n", v, enc, len(enc))

		dec, n, err := miniquic.DecodeVarint(enc)
		if err != nil {
			fmt.Printf("DecodeVarint(% x) error: %v\n", enc, err)
			continue
		}
		fmt.Printf("DecodeVarint(% x) = %d, consumed %d bytes\n", enc, dec, n)
	}

	fmt.Println("\n--- A QUIC PING frame (RFC 9000 §19.2) is just a 1-byte varint frame type ---")
	if ping, err := miniquic.EncodeVarint(0x01); err != nil {
		fmt.Printf("EncodeVarint(0x01) error: %v\n", err)
	} else {
		fmt.Printf("PING frame = % x\n", ping)
	}

	fmt.Println("\n--- QUIC encryption levels (RFC 9001 §4), as switched on by RunHandshake ---")
	levels := []tls.QUICEncryptionLevel{
		tls.QUICEncryptionLevelInitial,
		tls.QUICEncryptionLevelEarly,
		tls.QUICEncryptionLevelHandshake,
		tls.QUICEncryptionLevelApplication,
	}
	for _, l := range levels {
		fmt.Printf("%d: %s\n", int(l), l)
	}
}
