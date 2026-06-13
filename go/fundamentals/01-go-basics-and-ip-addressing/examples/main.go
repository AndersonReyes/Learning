// Command main demonstrates the Go language features used in this topic's
// exercises: variables, basic types, bitwise operators, control flow,
// strings/strconv, and error handling — applied to small IPv4 warm-ups that
// are deliberately *not* the exercises in exercise.go.
package main

import (
	"fmt"
	"strconv"
	"strings"
)

func main() {
	// --- variables, constants, zero values ---
	const ipString = "192.168.1.130"
	var octets [4]byte // zero value: [0 0 0 0]
	fmt.Println("zero value array:", octets)

	// --- strings.Split + strconv + error handling (multiple return values) ---
	parts := strings.Split(ipString, ".")
	for i, p := range parts { // for-range over a slice: index + value
		n, err := strconv.Atoi(p)
		if err != nil {
			fmt.Println("bad octet:", err)
			continue
		}
		octets[i] = byte(n) // explicit conversion: int -> byte
	}
	fmt.Println("parsed octets:", octets)

	// --- bitwise OR + shift: pack 4 octets into a uint32 ---
	var packed uint32
	for _, o := range octets { // range, discarding the index with _
		packed = packed<<8 | uint32(o)
	}
	fmt.Printf("packed: %d (binary: %032b)\n", packed, packed)

	// --- classic three-clause for loop + bitwise AND with a mask ---
	for prefixLen := 32; prefixLen >= 24; prefixLen -= 8 {
		mask := uint32(0xFFFFFFFF) << (32 - prefixLen)
		fmt.Printf("/%d mask = %032b -> masked = %032b\n", prefixLen, mask, packed&mask)
	}

	// --- tagless switch: classify the (historical) address class from octet[0] ---
	switch first := octets[0]; {
	case first < 128:
		fmt.Println("class A")
	case first < 192:
		fmt.Println("class B")
	case first < 224:
		fmt.Println("class C")
	default:
		fmt.Println("class D/E")
	}

	// --- strconv.ParseUint validates numeric AND range in one call ---
	if _, err := strconv.ParseUint("256", 10, 8); err != nil {
		fmt.Println("ParseUint rejects out-of-range octet:", err)
	}
}
