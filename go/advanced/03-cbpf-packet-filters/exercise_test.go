package cbpf

import (
	"reflect"
	"syscall"
	"testing"
)

// Real opcode values, for readability in expected-output tables below
// (Documentation/networking/filter.txt):
//
//	BPF_LD|BPF_ABS|BPF_W = 0x00 | 0x20 | 0x00 = 0x20
//	BPF_LD|BPF_ABS|BPF_H = 0x00 | 0x20 | 0x08 = 0x28
//	BPF_LD|BPF_ABS|BPF_B = 0x00 | 0x20 | 0x10 = 0x30
//	BPF_JMP|BPF_JEQ|BPF_K = 0x05 | 0x10 | 0x00 = 0x15
//	BPF_RET|BPF_K = 0x06 | 0x00 = 0x06
const (
	codeLoadW = 0x20
	codeLoadH = 0x28
	codeLoadB = 0x30
	codeJEQ   = 0x15
	codeRET   = 0x06
)

func TestLoadAbsolute(t *testing.T) {
	tests := []struct {
		name    string
		size    uint16
		offset  uint32
		want    syscall.SockFilter
		wantErr bool
	}{
		{
			name:   "4-byte load",
			size:   syscall.BPF_W,
			offset: 0,
			want:   syscall.SockFilter{Code: codeLoadW, K: 0},
		},
		{
			name:   "2-byte load at EtherType offset",
			size:   syscall.BPF_H,
			offset: 12,
			want:   syscall.SockFilter{Code: codeLoadH, K: 12},
		},
		{
			name:   "1-byte load at IP protocol offset",
			size:   syscall.BPF_B,
			offset: 23,
			want:   syscall.SockFilter{Code: codeLoadB, K: 23},
		},
		{
			name:    "invalid size",
			size:    0x99,
			offset:  0,
			wantErr: true,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got, err := LoadAbsolute(tt.size, tt.offset)
			if tt.wantErr {
				if err == nil {
					t.Fatalf("LoadAbsolute(%#x, %d) error = nil, want error", tt.size, tt.offset)
				}
				return
			}
			if err != nil {
				t.Fatalf("LoadAbsolute(%#x, %d) unexpected error: %v", tt.size, tt.offset, err)
			}
			if got != tt.want {
				t.Errorf("LoadAbsolute(%#x, %d) = %+v, want %+v", tt.size, tt.offset, got, tt.want)
			}
		})
	}
}

func TestJumpIfEqual(t *testing.T) {
	tests := []struct {
		name   string
		value  uint32
		jt, jf uint8
		want   syscall.SockFilter
	}{
		{
			name:  "fall through on match, jump on mismatch",
			value: 0x0800,
			jt:    0,
			jf:    2,
			want:  syscall.SockFilter{Code: codeJEQ, Jt: 0, Jf: 2, K: 0x0800},
		},
		{
			name:  "jump on match, fall through on mismatch",
			value: 80,
			jt:    1,
			jf:    0,
			want:  syscall.SockFilter{Code: codeJEQ, Jt: 1, Jf: 0, K: 80},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := JumpIfEqual(tt.value, tt.jt, tt.jf)
			if got != tt.want {
				t.Errorf("JumpIfEqual(%d, %d, %d) = %+v, want %+v", tt.value, tt.jt, tt.jf, got, tt.want)
			}
		})
	}
}

func TestReturn(t *testing.T) {
	tests := []struct {
		name  string
		value uint32
		want  syscall.SockFilter
	}{
		{name: "drop", value: 0, want: syscall.SockFilter{Code: codeRET, K: 0}},
		{name: "accept whole packet", value: 0xffff, want: syscall.SockFilter{Code: codeRET, K: 0xffff}},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := Return(tt.value)
			if got != tt.want {
				t.Errorf("Return(%#x) = %+v, want %+v", tt.value, got, tt.want)
			}
		})
	}
}

func TestAssembleFilter(t *testing.T) {
	const (
		etherTypeOffset = 12
		etherTypeIPv4   = 0x0800
		ipProtoOffset   = 23
		ipProtoUDP      = 17
		ipProtoTCP      = 6
		tcpDstPort      = 36
	)

	tests := []struct {
		name    string
		matches []Match
		want    []syscall.SockFilter
		wantErr bool
	}{
		{
			name:    "no matches",
			matches: nil,
			wantErr: true,
		},
		{
			name: "invalid size in a match",
			matches: []Match{
				{Offset: 0, Size: 0x99, Value: 0},
			},
			wantErr: true,
		},
		{
			name: "single match: EtherType == IPv4",
			matches: []Match{
				{Offset: etherTypeOffset, Size: syscall.BPF_H, Value: etherTypeIPv4},
			},
			want: []syscall.SockFilter{
				{Code: codeLoadH, K: etherTypeOffset},
				{Code: codeJEQ, Jt: 1, Jf: 0, K: etherTypeIPv4},
				{Code: codeRET, K: 0},
				{Code: codeRET, K: 0xffff},
			},
		},
		{
			name: "two matches: EtherType == IPv4 && IPProto == UDP",
			matches: []Match{
				{Offset: etherTypeOffset, Size: syscall.BPF_H, Value: etherTypeIPv4},
				{Offset: ipProtoOffset, Size: syscall.BPF_B, Value: ipProtoUDP},
			},
			want: []syscall.SockFilter{
				{Code: codeLoadH, K: etherTypeOffset},
				{Code: codeJEQ, Jt: 0, Jf: 2, K: etherTypeIPv4},
				{Code: codeLoadB, K: ipProtoOffset},
				{Code: codeJEQ, Jt: 1, Jf: 0, K: ipProtoUDP},
				{Code: codeRET, K: 0},
				{Code: codeRET, K: 0xffff},
			},
		},
		{
			name: "three matches: EtherType == IPv4 && IPProto == TCP && DstPort == 80",
			matches: []Match{
				{Offset: etherTypeOffset, Size: syscall.BPF_H, Value: etherTypeIPv4},
				{Offset: ipProtoOffset, Size: syscall.BPF_B, Value: ipProtoTCP},
				{Offset: tcpDstPort, Size: syscall.BPF_H, Value: 80},
			},
			want: []syscall.SockFilter{
				{Code: codeLoadH, K: etherTypeOffset},
				{Code: codeJEQ, Jt: 0, Jf: 4, K: etherTypeIPv4},
				{Code: codeLoadB, K: ipProtoOffset},
				{Code: codeJEQ, Jt: 0, Jf: 2, K: ipProtoTCP},
				{Code: codeLoadH, K: tcpDstPort},
				{Code: codeJEQ, Jt: 1, Jf: 0, K: 80},
				{Code: codeRET, K: 0},
				{Code: codeRET, K: 0xffff},
			},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got, err := AssembleFilter(tt.matches)
			if tt.wantErr {
				if err == nil {
					t.Fatalf("AssembleFilter(%+v) error = nil, want error", tt.matches)
				}
				return
			}
			if err != nil {
				t.Fatalf("AssembleFilter(%+v) unexpected error: %v", tt.matches, err)
			}
			if !reflect.DeepEqual(got, tt.want) {
				t.Errorf("AssembleFilter(%+v) =\n%+v\nwant\n%+v", tt.matches, got, tt.want)
			}
		})
	}
}

func TestAttachFilter(t *testing.T) {
	t.Run("empty program is an error", func(t *testing.T) {
		if err := AttachFilter(0, nil); err == nil {
			t.Fatal("AttachFilter(0, nil) error = nil, want error")
		}
	})

	t.Run("invalid fd is an error", func(t *testing.T) {
		prog, err := AssembleFilter([]Match{{Offset: 12, Size: syscall.BPF_H, Value: 0x0800}})
		if err != nil {
			t.Fatalf("AssembleFilter: %v", err)
		}
		if err := AttachFilter(-1, prog); err == nil {
			t.Fatal("AttachFilter(-1, prog) error = nil, want error")
		}
	})

	t.Run("attaches to a real socket", func(t *testing.T) {
		fd, err := syscall.Socket(syscall.AF_INET, syscall.SOCK_DGRAM, 0)
		if err != nil {
			t.Fatalf("syscall.Socket: %v", err)
		}
		defer syscall.Close(fd)

		prog, err := AssembleFilter([]Match{{Offset: 12, Size: syscall.BPF_H, Value: 0x0800}})
		if err != nil {
			t.Fatalf("AssembleFilter: %v", err)
		}
		if err := AttachFilter(fd, prog); err != nil {
			t.Fatalf("AttachFilter(fd, prog) = %v, want nil", err)
		}
	})
}
