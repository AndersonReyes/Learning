// Package ipaddr implements IPv4 address parsing and CIDR subnet arithmetic.
package ipaddr

import (
	"fmt"
	"strconv"
	"strings"
)

func pow(n uint32, e uint32) uint32 {
	var result uint32 = 1

	for e > 0 {
		result *= n
		e--
	}

	return result
}

// ParseIPv4 parses a dotted-decimal IPv4 address string (e.g. "192.168.1.1")
// into its 32-bit unsigned integer representation.
//
//	ParseIPv4("192.168.1.1")     -> 3232235777, nil
//	ParseIPv4("0.0.0.0")         -> 0, nil
//	ParseIPv4("255.255.255.255") -> 4294967295, nil
//
// Returns an error if s does not have exactly four dot-separated decimal
// octets, or if any octet is not in the range 0-255.
func ParseIPv4(s string) (uint32, error) {
	parts := strings.Split(s, ".")

	if len(parts) != 4 {
		return 0, fmt.Errorf("Expected 4 octets: %s", s)
	}

	var result uint32 = 0

	for i, octet := range parts {
		num, err := strconv.ParseUint(octet, 10, 8)

		if err != nil {
			return 0, fmt.Errorf("invalid octet: %s", octet)
		}

		result += (uint32(num) * (pow(2, uint32(24)-uint32(i*8))))

	}

	// fmt.Printf("powers: 2^24 = %d\n", pow(2, 24))

	return result, nil
}

// IPv4ToString converts a 32-bit unsigned integer into its dotted-decimal
// IPv4 string representation.
//
//	IPv4ToString(3232235777) -> "192.168.1.1"
//	IPv4ToString(0)          -> "0.0.0.0"
//	IPv4ToString(4294967295) -> "255.255.255.255"
func IPv4ToString(ip uint32) string {
	result := []string{
		strconv.Itoa(int(ip) & (0xFF << 24) >> 24),
		strconv.Itoa(int(ip) & (0xFF << 16) >> 16),
		strconv.Itoa(int(ip) & (0xFF << 8) >> 8),
		strconv.Itoa(int(ip) & 0xFF),
	}

	return strings.Join(result, ".")
}

// NetworkAddress returns the network address for ip under the given CIDR
// prefix length, i.e. ip with all host bits cleared.
//
//	NetworkAddress(ipFor("192.168.1.130"), 24) -> ipFor("192.168.1.0")
//	NetworkAddress(ipFor("192.168.1.130"), 26) -> ipFor("192.168.1.128")
//	NetworkAddress(ip, 32) -> ip
//	NetworkAddress(ip, 0)  -> 0
//
// Returns an error if prefixLen is not in the range 0-32.
func NetworkAddress(ip uint32, prefixLen int) (uint32, error) {
	if prefixLen < 0 || prefixLen > 32 {
		return 0, fmt.Errorf("invalid prefixLen: %d\n", prefixLen)
	}

	hostBits := (32 - prefixLen)
	network := ip & (0xFFFFFFFF << hostBits)

	return network, nil
}

// BroadcastAddress returns the broadcast address for ip under the given CIDR
// prefix length, i.e. the network address with all host bits set.
//
//	BroadcastAddress(ipFor("192.168.1.130"), 24) -> ipFor("192.168.1.255")
//	BroadcastAddress(ipFor("192.168.1.130"), 26) -> ipFor("192.168.1.191")
//	BroadcastAddress(ip, 32) -> ip
//	BroadcastAddress(ip, 0)  -> 4294967295 (255.255.255.255)
//
// Returns an error if prefixLen is not in the range 0-32.
func BroadcastAddress(ip uint32, prefixLen int) (uint32, error) {
	network, err := NetworkAddress(ip, prefixLen)
	if err != nil {
		return 0, fmt.Errorf("Invalid network address %d: %s", ip, err)
	}

	broadcast := network | (0xFFFFFFFF >> prefixLen)

	return broadcast, nil
}

// UsableHostCount returns the number of usable host addresses in a subnet
// with the given CIDR prefix length (the network and broadcast addresses are
// excluded from "usable").
//
// As special cases, per RFC 3021, /31 subnets have 2 usable addresses
// (point-to-point links) and /32 subnets have exactly 1.
//
//	UsableHostCount(24) -> 254
//	UsableHostCount(30) -> 2
//	UsableHostCount(31) -> 2
//	UsableHostCount(32) -> 1
//	UsableHostCount(0)  -> 4294967294
//
// Returns an error if prefixLen is not in the range 0-32.
func UsableHostCount(prefixLen int) (uint64, error) {
	if prefixLen < 0 || prefixLen > 32 {
		return 0, fmt.Errorf("invalid prefixLen: %d\n", prefixLen)
	}

	switch prefixLen {
	case 31:
		return 2, nil
	case 32:
		return 1, nil
	default:
		return (0xFFFFFFFF >> prefixLen) - 1, nil
	}

}
