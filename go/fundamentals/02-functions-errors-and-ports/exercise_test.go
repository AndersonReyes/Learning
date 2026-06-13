package ports

import (
	"errors"
	"reflect"
	"testing"
)

func TestParsePort(t *testing.T) {
	tests := []struct {
		name           string
		in             string
		want           uint16
		wantErr        bool
		wantOutOfRange bool
		wantValue      int64
	}{
		{"well-known", "80", 80, false, false, 0},
		{"min", "0", 0, false, false, 0},
		{"max", "65535", 65535, false, false, 0},
		{"too large", "65536", 0, true, true, 65536},
		{"negative", "-1", 0, true, true, -1},
		{"non numeric", "abc", 0, true, false, 0},
		{"empty", "", 0, true, false, 0},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got, err := ParsePort(tt.in)
			if (err != nil) != tt.wantErr {
				t.Fatalf("ParsePort(%q) error = %v, wantErr %v", tt.in, err, tt.wantErr)
			}
			if !tt.wantErr {
				if got != tt.want {
					t.Errorf("ParsePort(%q) = %d, want %d", tt.in, got, tt.want)
				}
				return
			}
			var rangeErr *OutOfRangeError
			isRangeErr := errors.As(err, &rangeErr)
			if isRangeErr != tt.wantOutOfRange {
				t.Errorf("ParsePort(%q): errors.As(err, *OutOfRangeError) = %v, want %v (err = %v)", tt.in, isRangeErr, tt.wantOutOfRange, err)
				return
			}
			if tt.wantOutOfRange && rangeErr.Value != tt.wantValue {
				t.Errorf("ParsePort(%q): OutOfRangeError.Value = %d, want %d", tt.in, rangeErr.Value, tt.wantValue)
			}
		})
	}
}

func TestClassifyPort(t *testing.T) {
	tests := []struct {
		name string
		in   uint16
		want string
	}{
		{"zero", 0, "well-known"},
		{"top of well-known", 1023, "well-known"},
		{"start of registered", 1024, "registered"},
		{"top of registered", 49151, "registered"},
		{"start of dynamic", 49152, "dynamic"},
		{"max", 65535, "dynamic"},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := ClassifyPort(tt.in); got != tt.want {
				t.Errorf("ClassifyPort(%d) = %q, want %q", tt.in, got, tt.want)
			}
		})
	}
}

func TestLookupService(t *testing.T) {
	tests := []struct {
		name    string
		in      string
		want    Service
		wantErr bool
	}{
		{"http", "http", Service{"http", 80, "tcp"}, false},
		{"https uppercase", "HTTPS", Service{"https", 443, "tcp"}, false},
		{"ssh mixed case", "Ssh", Service{"ssh", 22, "tcp"}, false},
		{"dns is udp", "dns", Service{"dns", 53, "udp"}, false},
		{"ftp", "ftp", Service{"ftp", 21, "tcp"}, false},
		{"telnet", "telnet", Service{"telnet", 23, "tcp"}, false},
		{"smtp", "smtp", Service{"smtp", 25, "tcp"}, false},
		{"ntp is udp", "ntp", Service{"ntp", 123, "udp"}, false},
		{"unknown service", "carrier-pigeon", Service{}, true},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got, err := LookupService(tt.in)
			if tt.wantErr {
				if !errors.Is(err, ErrUnknownService) {
					t.Fatalf("LookupService(%q): errors.Is(err, ErrUnknownService) = false, err = %v", tt.in, err)
				}
				if got != (Service{}) {
					t.Errorf("LookupService(%q) = %+v, want zero value", tt.in, got)
				}
				return
			}
			if err != nil {
				t.Fatalf("LookupService(%q) unexpected error: %v", tt.in, err)
			}
			if got != tt.want {
				t.Errorf("LookupService(%q) = %+v, want %+v", tt.in, got, tt.want)
			}
		})
	}
}

func TestPortRangeSize(t *testing.T) {
	tests := []struct {
		name    string
		ranges  [][2]uint16
		want    uint64
		wantErr bool
	}{
		{"single port", [][2]uint16{{80, 80}}, 1, false},
		{"range", [][2]uint16{{1024, 2048}}, 1025, false},
		{"full range", [][2]uint16{{0, 65535}}, 65536, false},
		{"multiple ranges", [][2]uint16{{80, 80}, {443, 443}}, 2, false},
		{"no ranges", nil, 0, false},
		{"invalid single range", [][2]uint16{{100, 50}}, 0, true},
		{"invalid second range", [][2]uint16{{80, 80}, {50, 10}}, 0, true},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got, err := PortRangeSize(tt.ranges...)
			if (err != nil) != tt.wantErr {
				t.Fatalf("PortRangeSize(%v) error = %v, wantErr %v", tt.ranges, err, tt.wantErr)
			}
			if !tt.wantErr && got != tt.want {
				t.Errorf("PortRangeSize(%v) = %d, want %d", tt.ranges, got, tt.want)
			}
		})
	}
}

func TestMergePortRanges(t *testing.T) {
	tests := []struct {
		name    string
		in      [][2]uint16
		want    [][2]uint16
		wantErr bool
	}{
		{"overlapping", [][2]uint16{{80, 100}, {90, 120}}, [][2]uint16{{80, 120}}, false},
		{"adjacent", [][2]uint16{{80, 100}, {101, 120}}, [][2]uint16{{80, 120}}, false},
		{"disjoint", [][2]uint16{{80, 100}, {200, 300}}, [][2]uint16{{80, 100}, {200, 300}}, false},
		{"empty input", nil, nil, false},
		{
			"unsorted multiple ranges",
			[][2]uint16{{500, 600}, {100, 200}, {150, 180}, {700, 750}, {600, 650}},
			[][2]uint16{{100, 200}, {500, 650}, {700, 750}},
			false,
		},
		{"invalid range", [][2]uint16{{100, 50}}, nil, true},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got, err := MergePortRanges(tt.in)
			if (err != nil) != tt.wantErr {
				t.Fatalf("MergePortRanges(%v) error = %v, wantErr %v", tt.in, err, tt.wantErr)
			}
			if tt.wantErr {
				return
			}
			if len(tt.want) == 0 {
				if len(got) != 0 {
					t.Errorf("MergePortRanges(%v) = %v, want empty", tt.in, got)
				}
				return
			}
			if !reflect.DeepEqual(got, tt.want) {
				t.Errorf("MergePortRanges(%v) = %v, want %v", tt.in, got, tt.want)
			}
		})
	}
}
