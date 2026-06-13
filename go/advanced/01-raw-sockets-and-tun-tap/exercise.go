// Package rawnet implements the packet-construction and -parsing primitives
// behind raw IP networking — the Internet checksum (RFC 1071), IPv4 packet
// encoding/decoding (RFC 791), and ICMP Echo Request messages (RFC 792) —
// plus OpenTUN, which creates a Linux TUN device via /dev/net/tun and the
// TUNSETIFF ioctl. Together these are the building blocks of a userspace
// "ping": construct a raw IP/ICMP packet, write it to a TUN device (which
// the kernel treats as a packet arriving on that interface), and read back
// whatever the kernel sends in response.
package rawnet

import (
	"errors"
	"net"
	"os"
)

// IPv4 protocol numbers relevant to this package (RFC 790).
const (
	ProtocolICMP = 1
)

// ICMP message types relevant to this package (RFC 792).
const (
	ICMPTypeEchoReply   = 0
	ICMPTypeEchoRequest = 8
)

// ipv4HeaderLen is the length in bytes of an IPv4 header with no options.
const ipv4HeaderLen = 20

// icmpHeaderLen is the length in bytes of an ICMP message's fixed header
// (type, code, checksum, identifier, sequence number), excluding payload.
const icmpHeaderLen = 8

// Linux TUN/TAP ioctl flags and request number, from <linux/if_tun.h>.
const (
	iffTUN  = 0x0001 // device is a TUN (IP-level) device, not a TAP (Ethernet-level) device
	iffNoPI = 0x1000 // don't prefix packets with a 4-byte flags/protocol header

	// tunSetIff is the TUNSETIFF ioctl request number, defined by the
	// kernel as _IOW('T', 202, int):
	//
	//	(IOC_WRITE<<30) | (sizeof(int)<<16) | ('T'<<8) | 202
	//	= (1<<30) | (4<<16) | (0x54<<8) | 0xCA
	//	= 0x400454CA
	tunSetIff = 0x400454ca
)

// ifReqSize is sizeof(struct ifreq) on Linux/amd64: a 16-byte interface-name
// field (IFNAMSIZ) followed by a union whose largest member is 24 bytes.
const ifReqSize = 40

// ifReq mirrors enough of Linux's struct ifreq for the TUNSETIFF ioctl: an
// interface name followed by a flags field, padded to the kernel's struct
// size. Field order and sizes must match the kernel's C struct exactly —
// the kernel reads this memory directly via the pointer passed to ioctl.
type ifReq struct {
	Name  [16]byte
	Flags uint16
	_     [ifReqSize - 16 - 2]byte
}

// IPv4Header is a parsed IPv4 header (RFC 791). Version is assumed to be 4
// and IHL is assumed to be 5 (no options); both are validated by
// ParseIPv4Packet but not retained here.
type IPv4Header struct {
	TotalLen uint16
	ID       uint16
	TTL      uint8
	Protocol uint8
	Checksum uint16
	Src      net.IP
	Dst      net.IP
}

// Checksum computes the Internet checksum (RFC 1071) of data: the
// one's-complement of the one's-complement sum of data's 16-bit words (in
// network byte order). If data has an odd length, the final byte is treated
// as the high byte of a zero-padded trailing 16-bit word.
func Checksum(data []byte) uint16 {
	return 0
}

// BuildIPv4Packet returns a complete IPv4 packet: a 20-byte header (version
// 4, no options) followed by payload, addressed from src to dst, carrying
// protocol, with the given time-to-live and identification field. The
// header's checksum field is computed over the header itself, such that
// Checksum of the returned packet's first 20 bytes is 0. BuildIPv4Packet
// returns an error if src or dst is not representable as an IPv4 address.
func BuildIPv4Packet(src, dst net.IP, protocol uint8, payload []byte, ttl uint8, id uint16) ([]byte, error) {
	return nil, errors.New("not implemented")
}

// ParseIPv4Packet parses a 20-byte IPv4 header (version 4, no options) from
// the start of data, returning the parsed header and the remaining bytes of
// data up to TotalLen (the payload). ParseIPv4Packet returns an error if
// data is shorter than the header (or than TotalLen), the version is not 4,
// the header has options (IHL != 5), or the header checksum is invalid.
func ParseIPv4Packet(data []byte) (IPv4Header, []byte, error) {
	return IPv4Header{}, nil, errors.New("not implemented")
}

// BuildICMPEchoRequest returns an ICMP Echo Request message (type 8, code 0)
// with the given identifier, sequence number, and payload, and a
// correctly-computed checksum, such that Checksum of the returned bytes is
// 0.
func BuildICMPEchoRequest(id, seq uint16, payload []byte) []byte {
	return nil
}

// OpenTUN creates a Linux TUN device named name and returns an *os.File for
// reading and writing raw IPv4 packets on it.
//
// Reads from the returned file yield IP packets the kernel routes out this
// interface; writes inject IP packets as if they arrived on this interface
// from the network. Packets carry no link-layer framing (IFF_NO_PI): each
// Read/Write deals with exactly one raw IPv4 packet, starting at the version
// header byte.
//
// The returned interface exists but is down and has no assigned address;
// callers configure it (e.g. via "ip addr add" and "ip link set ... up", run
// with os/exec) before sending or receiving traffic. OpenTUN requires
// CAP_NET_ADMIN (typically root) and a kernel with /dev/net/tun.
func OpenTUN(name string) (*os.File, error) {
	return nil, errors.New("not implemented")
}
