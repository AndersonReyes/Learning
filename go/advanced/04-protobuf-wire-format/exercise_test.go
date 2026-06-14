package minipb

import (
	"bytes"
	"math"
	"reflect"
	"testing"
)

func TestEncodeVarint(t *testing.T) {
	tests := []struct {
		name string
		v    uint64
		want []byte
	}{
		{name: "zero", v: 0, want: []byte{0x00}},
		{name: "one", v: 1, want: []byte{0x01}},
		{name: "largest single byte", v: 127, want: []byte{0x7f}},
		{name: "smallest two bytes", v: 128, want: []byte{0x80, 0x01}},
		{name: "protobuf spec example (150)", v: 150, want: []byte{0x96, 0x01}},
		{name: "three hundred", v: 300, want: []byte{0xac, 0x02}},
		{
			name: "max uint64",
			v:    math.MaxUint64,
			want: []byte{0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := EncodeVarint(tt.v)
			if !bytes.Equal(got, tt.want) {
				t.Errorf("EncodeVarint(%d) = % x, want % x", tt.v, got, tt.want)
			}
		})
	}
}

func TestDecodeVarint(t *testing.T) {
	tests := []struct {
		name    string
		data    []byte
		want    uint64
		wantN   int
		wantErr bool
	}{
		{name: "zero", data: []byte{0x00}, want: 0, wantN: 1},
		{name: "one", data: []byte{0x01}, want: 1, wantN: 1},
		{name: "protobuf spec example (150)", data: []byte{0x96, 0x01}, want: 150, wantN: 2},
		{name: "three hundred", data: []byte{0xac, 0x02}, want: 300, wantN: 2},
		{
			name:  "max uint64",
			data:  []byte{0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01},
			want:  math.MaxUint64,
			wantN: 10,
		},
		{
			name: "ignores trailing bytes",
			data: []byte{0x96, 0x01, 0xde, 0xad},
			want: 150, wantN: 2,
		},
		{name: "empty input", data: []byte{}, wantErr: true},
		{name: "truncated (continuation bit with no more bytes)", data: []byte{0x80}, wantErr: true},
		{
			name:    "too long (11th byte would be needed)",
			data:    []byte{0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff},
			wantErr: true,
		},
		{
			name:    "overflows uint64 (10th byte > 1)",
			data:    []byte{0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x02},
			wantErr: true,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got, n, err := DecodeVarint(tt.data)
			if tt.wantErr {
				if err == nil {
					t.Fatalf("DecodeVarint(% x) error = nil, want error", tt.data)
				}
				return
			}
			if err != nil {
				t.Fatalf("DecodeVarint(% x) unexpected error: %v", tt.data, err)
			}
			if got != tt.want || n != tt.wantN {
				t.Errorf("DecodeVarint(% x) = (%d, %d), want (%d, %d)", tt.data, got, n, tt.want, tt.wantN)
			}
		})
	}
}

func TestEncodeMessage(t *testing.T) {
	tests := []struct {
		name    string
		fields  map[int]any
		want    []byte
		wantErr bool
	}{
		{
			name:   "single varint field",
			fields: map[int]any{1: uint64(150)},
			want:   []byte{0x08, 0x96, 0x01},
		},
		{
			name:   "single length-delimited field",
			fields: map[int]any{2: []byte("testing")},
			want:   []byte{0x12, 0x07, 0x74, 0x65, 0x73, 0x74, 0x69, 0x6e, 0x67},
		},
		{
			name: "both fields, in field-number order",
			fields: map[int]any{
				2: []byte("testing"),
				1: uint64(150),
			},
			want: []byte{0x08, 0x96, 0x01, 0x12, 0x07, 0x74, 0x65, 0x73, 0x74, 0x69, 0x6e, 0x67},
		},
		{
			name:   "empty message",
			fields: map[int]any{},
			want:   []byte{},
		},
		{
			name:    "field number zero is invalid",
			fields:  map[int]any{0: uint64(1)},
			wantErr: true,
		},
		{
			name:    "negative field number is invalid",
			fields:  map[int]any{-1: uint64(1)},
			wantErr: true,
		},
		{
			name:    "unsupported value type",
			fields:  map[int]any{1: "not a uint64 or []byte"},
			wantErr: true,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got, err := EncodeMessage(tt.fields)
			if tt.wantErr {
				if err == nil {
					t.Fatalf("EncodeMessage(%+v) error = nil, want error", tt.fields)
				}
				return
			}
			if err != nil {
				t.Fatalf("EncodeMessage(%+v) unexpected error: %v", tt.fields, err)
			}
			if !bytes.Equal(got, tt.want) {
				t.Errorf("EncodeMessage(%+v) = % x, want % x", tt.fields, got, tt.want)
			}
		})
	}
}

func TestDecodeMessage(t *testing.T) {
	tests := []struct {
		name    string
		data    []byte
		want    map[int]any
		wantErr bool
	}{
		{
			name: "single varint field",
			data: []byte{0x08, 0x96, 0x01},
			want: map[int]any{1: uint64(150)},
		},
		{
			name: "single length-delimited field",
			data: []byte{0x12, 0x07, 0x74, 0x65, 0x73, 0x74, 0x69, 0x6e, 0x67},
			want: map[int]any{2: []byte("testing")},
		},
		{
			name: "both fields",
			data: []byte{0x08, 0x96, 0x01, 0x12, 0x07, 0x74, 0x65, 0x73, 0x74, 0x69, 0x6e, 0x67},
			want: map[int]any{1: uint64(150), 2: []byte("testing")},
		},
		{
			name: "empty message",
			data: []byte{},
			want: map[int]any{},
		},
		{name: "truncated tag varint", data: []byte{0x80}, wantErr: true},
		{name: "truncated length varint", data: []byte{0x12, 0x80}, wantErr: true},
		{
			name:    "length exceeds remaining data",
			data:    []byte{0x12, 0x05, 0x41},
			wantErr: true,
		},
		{
			name:    "unsupported wire type (5, fixed32)",
			data:    []byte{0x0d, 0x00, 0x00, 0x00, 0x00},
			wantErr: true,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got, err := DecodeMessage(tt.data)
			if tt.wantErr {
				if err == nil {
					t.Fatalf("DecodeMessage(% x) error = nil, want error", tt.data)
				}
				return
			}
			if err != nil {
				t.Fatalf("DecodeMessage(% x) unexpected error: %v", tt.data, err)
			}
			if !reflect.DeepEqual(got, tt.want) {
				t.Errorf("DecodeMessage(% x) = %+v, want %+v", tt.data, got, tt.want)
			}
		})
	}
}

func TestDecodeNestedMessage(t *testing.T) {
	// outer's field 3 (LENGTH_DELIMITED) = tag(3,LEN) + len(2) +
	// EncodeMessage({1: uint64(5)}) = tag(1,VARINT) + varint(5)
	outerField3 := []byte{0x1a, 0x02, 0x08, 0x05}
	// full = EncodeMessage({1: 150, 2: "testing"}) ++ outerField3
	full := append([]byte{0x08, 0x96, 0x01, 0x12, 0x07, 0x74, 0x65, 0x73, 0x74, 0x69, 0x6e, 0x67}, outerField3...)
	// malformed: field 3 marked as a message, but its bytes are not a valid message (truncated varint)
	malformedNested := []byte{0x1a, 0x02, 0x08, 0x80}

	tests := []struct {
		name          string
		data          []byte
		messageFields map[int]bool
		want          map[int]any
		wantErr       bool
	}{
		{
			name:          "length-delimited field decoded as nested message",
			data:          outerField3,
			messageFields: map[int]bool{3: true},
			want:          map[int]any{3: map[int]any{1: uint64(5)}},
		},
		{
			name:          "same field, not marked as a message, stays raw bytes",
			data:          outerField3,
			messageFields: nil,
			want:          map[int]any{3: []byte{0x08, 0x05}},
		},
		{
			name:          "mixed message: varint, bytes, and nested message fields",
			data:          full,
			messageFields: map[int]bool{3: true},
			want: map[int]any{
				1: uint64(150),
				2: []byte("testing"),
				3: map[int]any{1: uint64(5)},
			},
		},
		{
			name:          "field marked as a message but not a valid message",
			data:          malformedNested,
			messageFields: map[int]bool{3: true},
			wantErr:       true,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got, err := DecodeNestedMessage(tt.data, tt.messageFields)
			if tt.wantErr {
				if err == nil {
					t.Fatalf("DecodeNestedMessage(% x, %v) error = nil, want error", tt.data, tt.messageFields)
				}
				return
			}
			if err != nil {
				t.Fatalf("DecodeNestedMessage(% x, %v) unexpected error: %v", tt.data, tt.messageFields, err)
			}
			if !reflect.DeepEqual(got, tt.want) {
				t.Errorf("DecodeNestedMessage(% x, %v) =\n%+v\nwant\n%+v", tt.data, tt.messageFields, got, tt.want)
			}
		})
	}
}
