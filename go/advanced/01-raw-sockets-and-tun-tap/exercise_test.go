package rawnet

import (
	"bytes"
	"encoding/hex"
	"net"
	"os"
	"os/exec"
	"reflect"
	"testing"
)

func mustDecodeHex(t *testing.T, s string) []byte {
	b, err := hex.DecodeString(s)
	if err != nil {
		t.Fatalf("hex.DecodeString(%q): %v", s, err)
	}
	return b
}

func TestChecksum(t *testing.T) {
	headerZeroCksum := mustDecodeHex(t, "45000020abcd000040010000c0a80101c0a80102")
	if got := Checksum(headerZeroCksum); got != 0x4bbc {
		t.Errorf("Checksum(header, cksum field zeroed) = %#04x, want 0x4bbc", got)
	}

	headerFinal := mustDecodeHex(t, "45000020abcd000040014bbcc0a80101c0a80102")
	if got := Checksum(headerFinal); got != 0 {
		t.Errorf("Checksum(header with correct checksum) = %#04x, want 0", got)
	}

	// Odd-length input: the trailing 0x02 is treated as the high byte of a
	// zero-padded word (0x0200). sum = 0x0001 + 0x0200 = 0x0201, checksum =
	// ^0x0201 = 0xFDFE.
	if got := Checksum([]byte{0x00, 0x01, 0x02}); got != 0xFDFE {
		t.Errorf("Checksum(odd-length) = %#04x, want 0xfdfe", got)
	}
}

func TestBuildIPv4Packet(t *testing.T) {
	icmpMsg := mustDecodeHex(t, "0800192d0001000170696e67")
	wantPacket := mustDecodeHex(t, "45000020abcd000040014bbcc0a80101c0a801020800192d0001000170696e67")

	tests := []struct {
		name    string
		src     net.IP
		dst     net.IP
		wantErr bool
	}{
		{"valid", net.IPv4(192, 168, 1, 1), net.IPv4(192, 168, 1, 2), false},
		{"src not IPv4", net.ParseIP("::1"), net.IPv4(192, 168, 1, 2), true},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got, err := BuildIPv4Packet(tt.src, tt.dst, ProtocolICMP, icmpMsg, 64, 0xABCD)
			if (err != nil) != tt.wantErr {
				t.Fatalf("BuildIPv4Packet() error = %v, wantErr %v", err, tt.wantErr)
			}
			if tt.wantErr {
				return
			}
			if !bytes.Equal(got, wantPacket) {
				t.Errorf("BuildIPv4Packet() = %x, want %x", got, wantPacket)
			}
		})
	}
}

func TestParseIPv4Packet(t *testing.T) {
	icmpMsg := mustDecodeHex(t, "0800192d0001000170696e67")
	fullPacket := mustDecodeHex(t, "45000020abcd000040014bbcc0a80101c0a801020800192d0001000170696e67")

	badChecksum := append([]byte{}, fullPacket...)
	badChecksum[10], badChecksum[11] = 0xFF, 0xFF

	tests := []struct {
		name        string
		data        []byte
		wantHdr     IPv4Header
		wantPayload []byte
		wantErr     bool
	}{
		{
			name: "valid packet",
			data: fullPacket,
			wantHdr: IPv4Header{
				TotalLen: 32,
				ID:       0xABCD,
				TTL:      64,
				Protocol: ProtocolICMP,
				Checksum: 0x4bbc,
				Src:      net.IPv4(192, 168, 1, 1).To4(),
				Dst:      net.IPv4(192, 168, 1, 2).To4(),
			},
			wantPayload: icmpMsg,
		},
		{"too short", fullPacket[:10], IPv4Header{}, nil, true},
		{"bad checksum", badChecksum, IPv4Header{}, nil, true},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			hdr, payload, err := ParseIPv4Packet(tt.data)
			if (err != nil) != tt.wantErr {
				t.Fatalf("ParseIPv4Packet() error = %v, wantErr %v", err, tt.wantErr)
			}
			if tt.wantErr {
				return
			}
			if !reflect.DeepEqual(hdr, tt.wantHdr) {
				t.Errorf("ParseIPv4Packet() header = %#v, want %#v", hdr, tt.wantHdr)
			}
			if !bytes.Equal(payload, tt.wantPayload) {
				t.Errorf("ParseIPv4Packet() payload = %x, want %x", payload, tt.wantPayload)
			}
		})
	}
}

func TestBuildICMPEchoRequest(t *testing.T) {
	want := mustDecodeHex(t, "0800192d0001000170696e67")

	got := BuildICMPEchoRequest(1, 1, []byte("ping"))
	if !bytes.Equal(got, want) {
		t.Errorf("BuildICMPEchoRequest() = %x, want %x", got, want)
	}
}

// TestOpenTUN exercises OpenTUN end-to-end: it creates a TUN interface,
// configures an address on it with the "ip" command, then writes a
// hand-built ICMP Echo Request into the device addressed to the interface's
// own IP, as if it had arrived from the network. The kernel's IP stack
// answers its own Echo Request with an Echo Reply and routes that reply
// back out the TUN device, where this test reads it and checks it with
// ParseIPv4Packet — exercising BuildIPv4Packet, BuildICMPEchoRequest,
// Checksum, and ParseIPv4Packet together.
//
// This requires CAP_NET_ADMIN (typically root) and a kernel with
// /dev/net/tun.
func TestOpenTUN(t *testing.T) {
	if os.Geteuid() != 0 {
		t.Skip("OpenTUN requires CAP_NET_ADMIN (run as root)")
	}

	const (
		ifName   = "tuntest0"
		localIP  = "10.200.0.1"
		remoteIP = "10.200.0.2"
	)

	tun, err := OpenTUN(ifName)
	if err != nil {
		t.Fatalf("OpenTUN(%q) error = %v", ifName, err)
	}
	defer tun.Close()

	if out, err := exec.Command("ip", "addr", "add", localIP+"/24", "dev", ifName).CombinedOutput(); err != nil {
		t.Fatalf("ip addr add: %v: %s", err, out)
	}
	if out, err := exec.Command("ip", "link", "set", ifName, "up").CombinedOutput(); err != nil {
		t.Fatalf("ip link set up: %v: %s", err, out)
	}

	// A ping "from" remoteIP "to" localIP, as if it arrived on the wire.
	echoReq := BuildICMPEchoRequest(0x1234, 1, []byte("ping"))
	pkt, err := BuildIPv4Packet(net.ParseIP(remoteIP), net.ParseIP(localIP), ProtocolICMP, echoReq, 64, 1)
	if err != nil {
		t.Fatalf("BuildIPv4Packet() error = %v", err)
	}
	if _, err := tun.Write(pkt); err != nil {
		t.Fatalf("write request to tun: %v", err)
	}

	buf := make([]byte, 1500)
	n, err := tun.Read(buf)
	if err != nil {
		t.Fatalf("read reply from tun: %v", err)
	}

	hdr, payload, err := ParseIPv4Packet(buf[:n])
	if err != nil {
		t.Fatalf("ParseIPv4Packet(reply) error = %v", err)
	}
	if hdr.Protocol != ProtocolICMP {
		t.Errorf("reply protocol = %d, want %d (ICMP)", hdr.Protocol, ProtocolICMP)
	}
	if !hdr.Src.Equal(net.ParseIP(localIP)) || !hdr.Dst.Equal(net.ParseIP(remoteIP)) {
		t.Errorf("reply src/dst = %s/%s, want %s/%s", hdr.Src, hdr.Dst, localIP, remoteIP)
	}
	if len(payload) < icmpHeaderLen || payload[0] != ICMPTypeEchoReply {
		t.Fatalf("reply ICMP payload = % x, want type %d (echo reply)", payload, ICMPTypeEchoReply)
	}
	if Checksum(payload) != 0 {
		t.Errorf("reply ICMP checksum invalid: % x", payload)
	}
	if id := uint16(payload[4])<<8 | uint16(payload[5]); id != 0x1234 {
		t.Errorf("reply ICMP id = %#04x, want 0x1234", id)
	}
}
