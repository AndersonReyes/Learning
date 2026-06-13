package packet

import (
	"reflect"
	"testing"
)

// exampleHeader is the canonical IPv4 header example from notes.md (20
// bytes, no options): version 4, IHL 5, total length 52, ID 0x1c46,
// flags=DF, fragment offset 0, TTL 64, protocol TCP, src 192.168.1.1, dst
// 192.168.1.2.
var exampleHeader = []byte{
	0x45, 0x00, 0x00, 0x34, 0x1c, 0x46, 0x40, 0x00,
	0x40, 0x06, 0xb1, 0xe6, 0xc0, 0xa8, 0x01, 0x01,
	0xc0, 0xa8, 0x01, 0x02,
}

// fragmentedHeader is the same header with flags=MF (more fragments) and a
// fragment offset of 185 (in 8-byte units).
var fragmentedHeader = []byte{
	0x45, 0x00, 0x00, 0x34, 0x1c, 0x46, 0x20, 0xb9,
	0x40, 0x06, 0xb1, 0xe6, 0xc0, 0xa8, 0x01, 0x01,
	0xc0, 0xa8, 0x01, 0x02,
}

// withByte0 returns a copy of data with its first byte replaced by b.
func withByte0(data []byte, b byte) []byte {
	out := make([]byte, len(data))
	copy(out, data)
	out[0] = b
	return out
}

func TestParseIPv4Header(t *testing.T) {
	tests := []struct {
		name    string
		data    []byte
		want    *IPv4Header
		wantErr bool
	}{
		{
			"example header",
			exampleHeader,
			&IPv4Header{
				Version: 4, IHL: 5, TOS: 0,
				TotalLength: 52, ID: 7238,
				Flags: 2, FragmentOffset: 0,
				TTL: 64, Protocol: 6,
				Checksum: 45542,
				SrcIP:    3232235777,
				DstIP:    3232235778,
			},
			false,
		},
		{
			"fragmented header",
			fragmentedHeader,
			&IPv4Header{
				Version: 4, IHL: 5, TOS: 0,
				TotalLength: 52, ID: 7238,
				Flags: 1, FragmentOffset: 185,
				TTL: 64, Protocol: 6,
				Checksum: 45542,
				SrcIP:    3232235777,
				DstIP:    3232235778,
			},
			false,
		},
		{"too short", exampleHeader[:19], nil, true},
		{"wrong version", withByte0(exampleHeader, 0x65), nil, true},    // version 6
		{"IHL too small", withByte0(exampleHeader, 0x44), nil, true},    // IHL 4
		{"IHL exceeds data", withByte0(exampleHeader, 0x46), nil, true}, // IHL 6 -> needs 24 bytes
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got, err := ParseIPv4Header(tt.data)
			if (err != nil) != tt.wantErr {
				t.Fatalf("ParseIPv4Header() error = %v, wantErr %v", err, tt.wantErr)
			}
			if tt.wantErr {
				return
			}
			if !reflect.DeepEqual(got, tt.want) {
				t.Errorf("ParseIPv4Header() = %+v, want %+v", got, tt.want)
			}
		})
	}
}

func TestHeaderLength(t *testing.T) {
	h, err := ParseIPv4Header(exampleHeader)
	if err != nil {
		t.Fatalf("ParseIPv4Header() error = %v", err)
	}
	if got := h.HeaderLength(); got != 20 {
		t.Errorf("HeaderLength() = %d, want 20", got)
	}
}

func TestPayloadLength(t *testing.T) {
	h, err := ParseIPv4Header(exampleHeader)
	if err != nil {
		t.Fatalf("ParseIPv4Header() error = %v", err)
	}
	if got := h.PayloadLength(); got != 32 {
		t.Errorf("PayloadLength() = %d, want 32", got)
	}
}

func TestDecrementTTL(t *testing.T) {
	t.Run("decrements", func(t *testing.T) {
		h := &IPv4Header{TTL: 64}
		if err := h.DecrementTTL(); err != nil {
			t.Fatalf("DecrementTTL() error = %v", err)
		}
		if h.TTL != 63 {
			t.Errorf("TTL = %d, want 63", h.TTL)
		}
	})
	t.Run("zero TTL errors", func(t *testing.T) {
		h := &IPv4Header{TTL: 0}
		if err := h.DecrementTTL(); err == nil {
			t.Fatal("DecrementTTL() error = nil, want error")
		}
		if h.TTL != 0 {
			t.Errorf("TTL = %d, want unchanged 0", h.TTL)
		}
	})
}

func TestMarshalBinary(t *testing.T) {
	tests := []struct {
		name string
		h    *IPv4Header
		want []byte
	}{
		{
			"example header",
			&IPv4Header{
				Version: 4, IHL: 5, TOS: 0,
				TotalLength: 52, ID: 7238,
				Flags: 2, FragmentOffset: 0,
				TTL: 64, Protocol: 6,
				Checksum: 45542,
				SrcIP:    3232235777,
				DstIP:    3232235778,
			},
			exampleHeader,
		},
		{
			"fragmented header",
			&IPv4Header{
				Version: 4, IHL: 5, TOS: 0,
				TotalLength: 52, ID: 7238,
				Flags: 1, FragmentOffset: 185,
				TTL: 64, Protocol: 6,
				Checksum: 45542,
				SrcIP:    3232235777,
				DstIP:    3232235778,
			},
			fragmentedHeader,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := tt.h.MarshalBinary()
			if !reflect.DeepEqual(got, tt.want) {
				t.Errorf("MarshalBinary() = % x, want % x", got, tt.want)
			}
		})
	}
}

func TestParseMarshalRoundTrip(t *testing.T) {
	h, err := ParseIPv4Header(exampleHeader)
	if err != nil {
		t.Fatalf("ParseIPv4Header() error = %v", err)
	}
	got := h.MarshalBinary()
	if !reflect.DeepEqual(got, exampleHeader) {
		t.Errorf("round trip = % x, want % x", got, exampleHeader)
	}
}
