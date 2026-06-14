// Package cbpf implements a small assembler for classic BPF (cBPF) packet
// filters and attaches them to a socket via SO_ATTACH_FILTER (man 7 socket;
// Linux Documentation/networking/filter.rst) — the filtering language used
// by tcpdump/libpcap and the direct ancestor of eBPF.
//
// A cBPF program is a flat array of syscall.SockFilter instructions, each
// either a load from a fixed byte offset in the packet (BPF_LD|BPF_ABS), a
// conditional jump (BPF_JMP|BPF_JEQ), or a verdict (BPF_RET): 0 drops the
// packet, any other value is how many bytes of it to deliver. AssembleFilter
// compiles a list of "packet[offset:offset+size] == value" conditions into
// such a program, computing the relative jump offsets so that the packet is
// accepted only if every condition holds.
package cbpf

import (
	"errors"
	"syscall"
)

// Match is one "packet[Offset:Offset+Size] == Value" condition in the
// AND-chain assembled by AssembleFilter. Size must be syscall.BPF_B (1
// byte), syscall.BPF_H (2 bytes), or syscall.BPF_W (4 bytes); Offset is a
// byte offset from the start of the packet, and Value is compared against
// the big-endian interpretation of those bytes.
type Match struct {
	Offset uint32
	Size   uint16
	Value  uint32
}

// LoadAbsolute returns a cBPF instruction that loads the big-endian Size-byte
// value at byte Offset of the packet into the implicit accumulator
// (BPF_LD|BPF_ABS|Size, K=Offset). Size must be syscall.BPF_B, syscall.BPF_H,
// or syscall.BPF_W; any other value is an error.
func LoadAbsolute(size uint16, offset uint32) (syscall.SockFilter, error) {
	return syscall.SockFilter{}, errors.New("not implemented")
}

// JumpIfEqual returns a cBPF instruction that compares the accumulator to
// value (BPF_JMP|BPF_JEQ|BPF_K, K=value), jumping forward jt instructions if
// they are equal or jf instructions if they are not. A jump of 0 falls
// through to the next instruction.
func JumpIfEqual(value uint32, jt, jf uint8) syscall.SockFilter {
	return syscall.SockFilter{}
}

// Return returns a cBPF instruction that terminates the program with verdict
// value (BPF_RET|BPF_K, K=value): 0 drops the packet, and any other value is
// the number of bytes of the packet to deliver (0xffff means "the whole
// packet").
func Return(value uint32) syscall.SockFilter {
	return syscall.SockFilter{}
}

// AssembleFilter compiles matches into a complete cBPF program that delivers
// the whole packet (Return(0xffff)) if every Match holds, and drops it
// (Return(0)) as soon as one doesn't — short-circuiting like &&. The
// returned program has exactly 2*len(matches)+2 instructions: a
// LoadAbsolute/JumpIfEqual pair per match, followed by the drop and accept
// Return instructions, with each JumpIfEqual's jt/jf computed so that:
//
//   - the last match's JumpIfEqual jumps forward 1 instruction (over the
//     drop) on success, and 0 (falling through to the drop) on failure;
//   - every earlier match's JumpIfEqual falls through (0) to the next
//     match's LoadAbsolute on success, and jumps forward over all remaining
//     match pairs to the drop instruction on failure.
//
// AssembleFilter returns an error if matches is empty, or if any Match's
// Size is invalid (see LoadAbsolute).
func AssembleFilter(matches []Match) ([]syscall.SockFilter, error) {
	return nil, errors.New("not implemented")
}

// AttachFilter attaches prog to fd via SO_ATTACH_FILTER (SOL_SOCKET level):
// from then on, the kernel only delivers packets/datagrams matching prog to
// fd. AttachFilter returns an error if prog is empty, or if the underlying
// setsockopt(2) call fails (e.g. fd is not a valid socket).
func AttachFilter(fd int, prog []syscall.SockFilter) error {
	return errors.New("not implemented")
}
