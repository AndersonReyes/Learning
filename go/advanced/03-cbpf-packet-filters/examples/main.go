// Command main demonstrates this topic's exercise on inputs different from
// exercise_test.go: the bitwise-OR opcode encodings from notes.md, a 4-byte
// (BPF_W) absolute load matching an IPv4 source address (exercise_test.go
// only exercises BPF_H/BPF_B), and attaching the assembled filter to a real
// (non-privileged) UDP socket via AttachFilter.
//
// Against the stub exercise.go, AssembleFilter returns its "not implemented"
// error, which this program prints instead of treating as fatal.
package main

import (
	"encoding/binary"
	"fmt"
	"syscall"

	cbpf "github.com/andersonreyes/learning/go/advanced/03-cbpf-packet-filters"
)

func main() {
	fmt.Println("--- cBPF opcode encodings (Code = class | size/mode | operand source) ---")
	fmt.Printf("BPF_LD|BPF_ABS|BPF_W  = %#02x\n", syscall.BPF_LD|syscall.BPF_ABS|syscall.BPF_W)
	fmt.Printf("BPF_LD|BPF_ABS|BPF_H  = %#02x\n", syscall.BPF_LD|syscall.BPF_ABS|syscall.BPF_H)
	fmt.Printf("BPF_LD|BPF_ABS|BPF_B  = %#02x\n", syscall.BPF_LD|syscall.BPF_ABS|syscall.BPF_B)
	fmt.Printf("BPF_JMP|BPF_JEQ|BPF_K = %#02x\n", syscall.BPF_JMP|syscall.BPF_JEQ|syscall.BPF_K)
	fmt.Printf("BPF_RET|BPF_K         = %#02x\n", syscall.BPF_RET|syscall.BPF_K)

	fmt.Println("\n--- assembling a filter: EtherType == IPv4 && source IP == 10.0.0.1 ---")
	const (
		etherTypeOffset = 12
		etherTypeIPv4   = 0x0800
		ipSrcOffset     = 26 // 14-byte Ethernet header + 12 (IPv4 source address field)
	)
	srcIP := binary.BigEndian.Uint32([]byte{10, 0, 0, 1})

	prog, err := cbpf.AssembleFilter([]cbpf.Match{
		{Offset: etherTypeOffset, Size: syscall.BPF_H, Value: etherTypeIPv4},
		{Offset: ipSrcOffset, Size: syscall.BPF_W, Value: srcIP},
	})
	if err != nil {
		fmt.Printf("AssembleFilter error: %v\n", err)
		return
	}
	for i, insn := range prog {
		fmt.Printf("[%d] %+v\n", i, insn)
	}

	fmt.Println("\n--- attaching the filter to a UDP socket (SO_ATTACH_FILTER, no root needed) ---")
	fd, err := syscall.Socket(syscall.AF_INET, syscall.SOCK_DGRAM, 0)
	if err != nil {
		fmt.Printf("syscall.Socket error: %v\n", err)
		return
	}
	defer syscall.Close(fd)

	if err := cbpf.AttachFilter(fd, prog); err != nil {
		fmt.Printf("AttachFilter error: %v\n", err)
		return
	}
	fmt.Println("SO_ATTACH_FILTER succeeded")
}
