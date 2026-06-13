// Package ipaddr implements IPv4 address parsing and CIDR subnet arithmetic.
package ipaddr

import "errors"

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
	return 0, errors.New("not implemented")
}

// IPv4ToString converts a 32-bit unsigned integer into its dotted-decimal
// IPv4 string representation.
//
//	IPv4ToString(3232235777) -> "192.168.1.1"
//	IPv4ToString(0)          -> "0.0.0.0"
//	IPv4ToString(4294967295) -> "255.255.255.255"
func IPv4ToString(ip uint32) string {
	return ""
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
	return 0, errors.New("not implemented")
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
	return 0, errors.New("not implemented")
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
	return 0, errors.New("not implemented")
}
