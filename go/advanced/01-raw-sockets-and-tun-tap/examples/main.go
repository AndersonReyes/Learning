// Command main demonstrates two mechanisms underlying this topic's
// exercise, on inputs different from exercise_test.go: the RFC 1071
// Internet checksum (also used by BuildIPv4Packet/ParseIPv4Packet/
// BuildICMPEchoRequest), and how OpenTUN's "magic" TUNSETIFF ioctl constant
// and ifreq struct layout are derived — without opening any real device, so
// this runs without root.
package main

import (
	"encoding/binary"
	"fmt"
	"unsafe"

	rawnet "github.com/andersonreyes/learning/go/advanced/01-raw-sockets-and-tun-tap"
)

func main() {
	fmt.Println("--- RFC 1071 Internet checksum ---")
	data := []byte{0x12, 0x34, 0x56, 0x78, 0x00, 0x00} // checksum field zeroed
	cksum := rawnet.Checksum(data)
	fmt.Printf("Checksum(% x) = %#04x\n", data, cksum)

	binary.BigEndian.PutUint16(data[4:6], cksum)
	fmt.Printf("with checksum filled in, % x\n", data)
	fmt.Printf("Checksum(% x) = %#04x (0 means the data is self-consistent)\n", data, rawnet.Checksum(data))

	fmt.Println("\n--- Deriving TUNSETIFF = _IOW('T', 202, sizeof(int)) ---")
	const (
		iocWrite     = 1   // _IOC_WRITE
		ifreqIOCType = 'T' // ioctl "type" byte for TUN/TAP requests
		tunSetIffNr  = 202 // request number for TUNSETIFF
		sizeofInt    = 4   // sizeof(int) on amd64
	)
	tunSetIff := uint32(iocWrite)<<30 | uint32(sizeofInt)<<16 | uint32(ifreqIOCType)<<8 | uint32(tunSetIffNr)
	fmt.Printf("_IOC(dir=%#x, type=%q, nr=%d, size=%d) = %#08x\n", iocWrite, rune(ifreqIOCType), tunSetIffNr, sizeofInt, tunSetIff)

	fmt.Println("\n--- struct ifreq layout (must match the kernel's C ABI) ---")
	type ifReq struct {
		Name  [16]byte
		Flags uint16
		_     [22]byte
	}
	fmt.Printf("unsafe.Sizeof(ifReq{}) = %d bytes (Linux's struct ifreq is also 40 bytes)\n", unsafe.Sizeof(ifReq{}))

	var req ifReq
	copy(req.Name[:], "tun0")
	req.Flags = 0x0001 | 0x1000 // IFF_TUN | IFF_NO_PI
	fmt.Printf("ifReq{Name: %q, Flags: %#04x}\n", req.Name[:4], req.Flags)
}
