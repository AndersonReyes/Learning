package ipaddr

import "testing"

func TestParseIPv4(t *testing.T) {
	tests := []struct {
		name    string
		in      string
		want    uint32
		wantErr bool
	}{
		{"basic", "192.168.1.1", 3232235777, false},
		{"ten net", "10.0.0.1", 167772161, false},
		{"all zero", "0.0.0.0", 0, false},
		{"all ones", "255.255.255.255", 4294967295, false},
		{"octet too big", "256.0.0.1", 0, true},
		{"too few octets", "1.2.3", 0, true},
		{"too many octets", "1.2.3.4.5", 0, true},
		{"non numeric octet", "1.2.3.a", 0, true},
		{"negative octet", "1.2.3.-1", 0, true},
		{"empty string", "", 0, true},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got, err := ParseIPv4(tt.in)
			if (err != nil) != tt.wantErr {
				t.Fatalf("ParseIPv4(%q) error = %v, wantErr %v", tt.in, err, tt.wantErr)
			}
			if !tt.wantErr && got != tt.want {
				t.Errorf("ParseIPv4(%q) = %d, want %d", tt.in, got, tt.want)
			}
		})
	}
}

func TestIPv4ToString(t *testing.T) {
	tests := []struct {
		name string
		in   uint32
		want string
	}{
		{"basic", 3232235777, "192.168.1.1"},
		{"ten net", 167772161, "10.0.0.1"},
		{"all zero", 0, "0.0.0.0"},
		{"all ones", 4294967295, "255.255.255.255"},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := IPv4ToString(tt.in); got != tt.want {
				t.Errorf("IPv4ToString(%d) = %q, want %q", tt.in, got, tt.want)
			}
		})
	}
}

func TestParseAndFormatRoundTrip(t *testing.T) {
	for _, s := range []string{
		"192.168.1.1", "10.0.0.1", "0.0.0.0", "255.255.255.255", "172.16.254.1",
	} {
		ip, err := ParseIPv4(s)
		if err != nil {
			t.Fatalf("ParseIPv4(%q) unexpected error: %v", s, err)
		}
		if got := IPv4ToString(ip); got != s {
			t.Errorf("round trip %q -> %d -> %q, want %q", s, ip, got, s)
		}
	}
}

func TestNetworkAddress(t *testing.T) {
	const (
		ip192_168_1_130 uint32 = 3232235906 // 192.168.1.130
		ip10_20_30_40   uint32 = 169090600  // 10.20.30.40
	)
	tests := []struct {
		name      string
		ip        uint32
		prefixLen int
		want      uint32
		wantErr   bool
	}{
		{"/24", ip192_168_1_130, 24, 3232235776, false}, // 192.168.1.0
		{"/26", ip192_168_1_130, 26, 3232235904, false}, // 192.168.1.128
		{"/8", ip10_20_30_40, 8, 167772160, false},      // 10.0.0.0
		{"/32 is identity", ip192_168_1_130, 32, ip192_168_1_130, false},
		{"/0 is zero", ip192_168_1_130, 0, 0, false},
		{"negative prefix", ip192_168_1_130, -1, 0, true},
		{"prefix too large", ip192_168_1_130, 33, 0, true},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got, err := NetworkAddress(tt.ip, tt.prefixLen)
			if (err != nil) != tt.wantErr {
				t.Fatalf("NetworkAddress(%d, %d) error = %v, wantErr %v", tt.ip, tt.prefixLen, err, tt.wantErr)
			}
			if !tt.wantErr && got != tt.want {
				t.Errorf("NetworkAddress(%d, %d) = %d, want %d", tt.ip, tt.prefixLen, got, tt.want)
			}
		})
	}
}

func TestBroadcastAddress(t *testing.T) {
	const ip192_168_1_130 uint32 = 3232235906 // 192.168.1.130
	tests := []struct {
		name      string
		ip        uint32
		prefixLen int
		want      uint32
		wantErr   bool
	}{
		{"/24", ip192_168_1_130, 24, 3232236031, false}, // 192.168.1.255
		{"/26", ip192_168_1_130, 26, 3232235967, false}, // 192.168.1.191
		{"/32 is identity", ip192_168_1_130, 32, ip192_168_1_130, false},
		{"/0 is all ones", ip192_168_1_130, 0, 4294967295, false}, // 255.255.255.255
		{"negative prefix", ip192_168_1_130, -1, 0, true},
		{"prefix too large", ip192_168_1_130, 33, 0, true},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got, err := BroadcastAddress(tt.ip, tt.prefixLen)
			if (err != nil) != tt.wantErr {
				t.Fatalf("BroadcastAddress(%d, %d) error = %v, wantErr %v", tt.ip, tt.prefixLen, err, tt.wantErr)
			}
			if !tt.wantErr && got != tt.want {
				t.Errorf("BroadcastAddress(%d, %d) = %d, want %d", tt.ip, tt.prefixLen, got, tt.want)
			}
		})
	}
}

func TestUsableHostCount(t *testing.T) {
	tests := []struct {
		name      string
		prefixLen int
		want      uint64
		wantErr   bool
	}{
		{"/24", 24, 254, false},
		{"/30", 30, 2, false},
		{"/31 point-to-point", 31, 2, false},
		{"/32 single host", 32, 1, false},
		{"/0", 0, 4294967294, false},
		{"/1", 1, 2147483646, false},
		{"negative prefix", -1, 0, true},
		{"prefix too large", 33, 0, true},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got, err := UsableHostCount(tt.prefixLen)
			if (err != nil) != tt.wantErr {
				t.Fatalf("UsableHostCount(%d) error = %v, wantErr %v", tt.prefixLen, err, tt.wantErr)
			}
			if !tt.wantErr && got != tt.want {
				t.Errorf("UsableHostCount(%d) = %d, want %d", tt.prefixLen, got, tt.want)
			}
		})
	}
}
