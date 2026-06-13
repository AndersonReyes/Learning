package dns

import (
	"bytes"
	"net"
	"strings"
	"testing"
	"time"
)

// dialTimeout bounds how long a test waits on network operations. It's
// generous enough for a correct implementation (everything here is
// loopback) but keeps a not-yet-implemented stub from hanging the test
// run.
const dialTimeout = 2 * time.Second

// exampleComEncoded is "example.com" encoded per RFC 1035 §4.1.2:
// 7 e x a m p l e 3 c o m 0
var exampleComEncoded = []byte{7, 'e', 'x', 'a', 'm', 'p', 'l', 'e', 3, 'c', 'o', 'm', 0}

// mustListenUDP opens a UDP socket on an ephemeral loopback port.
func mustListenUDP(t *testing.T) *net.UDPConn {
	t.Helper()
	addr, err := net.ResolveUDPAddr("udp", "127.0.0.1:0")
	if err != nil {
		t.Fatalf("net.ResolveUDPAddr() error = %v", err)
	}
	conn, err := net.ListenUDP("udp", addr)
	if err != nil {
		t.Fatalf("net.ListenUDP() error = %v", err)
	}
	return conn
}

// buildAnswerRecord encodes one type-A answer record whose NAME is a
// compression pointer back to the question's QNAME at offset 12.
func buildAnswerRecord(ip net.IP) []byte {
	rec := []byte{0xC0, 0x0C}                 // NAME: pointer to offset 12
	rec = append(rec, 0x00, 0x01)             // TYPE = A
	rec = append(rec, 0x00, 0x01)             // CLASS = IN
	rec = append(rec, 0x00, 0x00, 0x01, 0x2C) // TTL = 300
	rec = append(rec, 0x00, 0x04)             // RDLENGTH = 4
	rec = append(rec, ip.To4()...)
	return rec
}

func TestEncodeName(t *testing.T) {
	tests := []struct {
		name    string
		input   string
		want    []byte
		wantErr bool
	}{
		{
			name:  "two labels",
			input: "example.com",
			want:  []byte{7, 'e', 'x', 'a', 'm', 'p', 'l', 'e', 3, 'c', 'o', 'm', 0},
		},
		{
			name:  "root",
			input: "",
			want:  []byte{0},
		},
		{
			name:  "three single-character labels",
			input: "a.b.c",
			want:  []byte{1, 'a', 1, 'b', 1, 'c', 0},
		},
		{
			name:    "label too long",
			input:   strings.Repeat("a", 64) + ".com",
			wantErr: true,
		},
		{
			name:    "empty label",
			input:   "a..b",
			wantErr: true,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got, err := EncodeName(tt.input)
			if tt.wantErr {
				if err == nil {
					t.Errorf("EncodeName(%q) error = nil, want non-nil", tt.input)
				}
				return
			}
			if err != nil {
				t.Fatalf("EncodeName(%q) error = %v", tt.input, err)
			}
			if !bytes.Equal(got, tt.want) {
				t.Errorf("EncodeName(%q) = %v, want %v", tt.input, got, tt.want)
			}
		})
	}
}

func TestDecodeName(t *testing.T) {
	data := []byte{
		// offset 0: 12-byte header placeholder
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		// offset 12: "example.com"
		7, 'e', 'x', 'a', 'm', 'p', 'l', 'e', 3, 'c', 'o', 'm', 0,
		// offset 25: pointer to offset 12 ("example.com")
		0xC0, 0x0C,
		// offset 27: "www" label, then pointer to offset 12
		3, 'w', 'w', 'w', 0xC0, 0x0C,
		// offset 33: root (zero-length label)
		0,
	}

	tests := []struct {
		name     string
		offset   int
		wantName string
		wantNext int
	}{
		{"plain labels", 12, "example.com", 25},
		{"pointer only", 25, "example.com", 27},
		{"label then pointer", 27, "www.example.com", 33},
		{"root", 33, "", 34},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			name, next, err := DecodeName(data, tt.offset)
			if err != nil {
				t.Fatalf("DecodeName() error = %v", err)
			}
			if name != tt.wantName {
				t.Errorf("DecodeName() name = %q, want %q", name, tt.wantName)
			}
			if next != tt.wantNext {
				t.Errorf("DecodeName() next = %d, want %d", next, tt.wantNext)
			}
		})
	}
}

func TestEncodeQuery(t *testing.T) {
	got, err := EncodeQuery(0x1234, "example.com", TypeA)
	if err != nil {
		t.Fatalf("EncodeQuery() error = %v", err)
	}

	want := []byte{
		0x12, 0x34, // ID
		0x01, 0x00, // flags (RD set)
		0x00, 0x01, // QDCOUNT
		0x00, 0x00, // ANCOUNT
		0x00, 0x00, // NSCOUNT
		0x00, 0x00, // ARCOUNT
		7, 'e', 'x', 'a', 'm', 'p', 'l', 'e', 3, 'c', 'o', 'm', 0, // QNAME
		0x00, 0x01, // QTYPE = A
		0x00, 0x01, // QCLASS = IN
	}

	if !bytes.Equal(got, want) {
		t.Errorf("EncodeQuery() = %v, want %v", got, want)
	}
}

func TestParseResponse(t *testing.T) {
	question := append([]byte{}, exampleComEncoded...)
	question = append(question, 0x00, 0x01, 0x00, 0x01) // QTYPE=A, QCLASS=IN

	t.Run("single A answer", func(t *testing.T) {
		data := []byte{
			0x12, 0x34, // ID
			0x81, 0x80, // flags, RCODE = 0
			0x00, 0x01, // QDCOUNT
			0x00, 0x01, // ANCOUNT
			0x00, 0x00, // NSCOUNT
			0x00, 0x00, // ARCOUNT
		}
		data = append(data, question...)
		data = append(data, buildAnswerRecord(net.IPv4(192, 0, 2, 1))...)

		resp, err := ParseResponse(data)
		if err != nil {
			t.Fatalf("ParseResponse() error = %v", err)
		}
		if resp.ID != 0x1234 {
			t.Errorf("ID = %#x, want %#x", resp.ID, 0x1234)
		}
		if resp.RCODE != 0 {
			t.Errorf("RCODE = %d, want 0", resp.RCODE)
		}
		if len(resp.Answers) != 1 || !resp.Answers[0].Equal(net.IPv4(192, 0, 2, 1)) {
			t.Errorf("Answers = %v, want [192.0.2.1]", resp.Answers)
		}
	})

	t.Run("multiple answers", func(t *testing.T) {
		data := []byte{
			0x12, 0x34, // ID
			0x81, 0x80, // flags, RCODE = 0
			0x00, 0x01, // QDCOUNT
			0x00, 0x02, // ANCOUNT
			0x00, 0x00, // NSCOUNT
			0x00, 0x00, // ARCOUNT
		}
		data = append(data, question...)
		data = append(data, buildAnswerRecord(net.IPv4(192, 0, 2, 1))...)
		data = append(data, buildAnswerRecord(net.IPv4(192, 0, 2, 2))...)

		resp, err := ParseResponse(data)
		if err != nil {
			t.Fatalf("ParseResponse() error = %v", err)
		}
		want := []net.IP{net.IPv4(192, 0, 2, 1), net.IPv4(192, 0, 2, 2)}
		if len(resp.Answers) != len(want) {
			t.Fatalf("len(Answers) = %d, want %d", len(resp.Answers), len(want))
		}
		for i := range want {
			if !resp.Answers[i].Equal(want[i]) {
				t.Errorf("Answers[%d] = %v, want %v", i, resp.Answers[i], want[i])
			}
		}
	})

	t.Run("non-zero RCODE has no answers", func(t *testing.T) {
		data := []byte{
			0x12, 0x34, // ID
			0x81, 0x83, // flags, RCODE = 3 (NXDOMAIN)
			0x00, 0x01, // QDCOUNT
			0x00, 0x00, // ANCOUNT
			0x00, 0x00, // NSCOUNT
			0x00, 0x00, // ARCOUNT
		}
		data = append(data, question...)

		resp, err := ParseResponse(data)
		if err != nil {
			t.Fatalf("ParseResponse() error = %v", err)
		}
		if resp.RCODE != 3 {
			t.Errorf("RCODE = %d, want 3", resp.RCODE)
		}
		if len(resp.Answers) != 0 {
			t.Errorf("Answers = %v, want empty", resp.Answers)
		}
	})
}

// buildResponseTemplate builds a complete DNS response message for
// "example.com" with a placeholder ID of 0x0000. A test server patches
// bytes [0:2] with the query's ID before sending the response back.
func buildResponseTemplate(rcode uint8, answers []net.IP) []byte {
	flags := uint16(0x8180) | uint16(rcode)
	data := []byte{
		0x00, 0x00, // ID placeholder, patched by the server
		byte(flags >> 8), byte(flags),
		0x00, 0x01, // QDCOUNT
		0x00, byte(len(answers)), // ANCOUNT
		0x00, 0x00, // NSCOUNT
		0x00, 0x00, // ARCOUNT
	}
	data = append(data, exampleComEncoded...)
	data = append(data, 0x00, 0x01, 0x00, 0x01) // QTYPE=A, QCLASS=IN

	for _, ip := range answers {
		data = append(data, buildAnswerRecord(ip)...)
	}
	return data
}

// respondOnce reads one query datagram from conn and replies with a copy
// of template whose first two bytes (the ID) are replaced by the query's
// ID.
func respondOnce(conn *net.UDPConn, template []byte) {
	buf := make([]byte, 512)
	n, from, err := conn.ReadFromUDP(buf)
	if err != nil || n < 2 {
		return
	}
	resp := append([]byte{}, template...)
	resp[0], resp[1] = buf[0], buf[1]
	conn.WriteToUDP(resp, from)
}

func TestResolve(t *testing.T) {
	t.Run("returns A records from a successful response", func(t *testing.T) {
		conn := mustListenUDP(t)
		defer conn.Close()

		template := buildResponseTemplate(0, []net.IP{net.IPv4(192, 0, 2, 1), net.IPv4(192, 0, 2, 2)})
		go respondOnce(conn, template)

		ips, err := Resolve(conn.LocalAddr().String(), "example.com", dialTimeout)
		if err != nil {
			t.Fatalf("Resolve() error = %v", err)
		}

		want := []net.IP{net.IPv4(192, 0, 2, 1), net.IPv4(192, 0, 2, 2)}
		if len(ips) != len(want) {
			t.Fatalf("len(ips) = %d, want %d", len(ips), len(want))
		}
		for i := range want {
			if !ips[i].Equal(want[i]) {
				t.Errorf("ips[%d] = %v, want %v", i, ips[i], want[i])
			}
		}
	})

	t.Run("returns an error for non-zero RCODE", func(t *testing.T) {
		conn := mustListenUDP(t)
		defer conn.Close()

		template := buildResponseTemplate(3, nil) // NXDOMAIN
		go respondOnce(conn, template)

		_, err := Resolve(conn.LocalAddr().String(), "example.com", dialTimeout)
		if err == nil {
			t.Error("Resolve() error = nil, want non-nil")
		}
	})

	t.Run("returns an error when the server doesn't respond", func(t *testing.T) {
		conn := mustListenUDP(t)
		addr := conn.LocalAddr().String()
		conn.Close()

		_, err := Resolve(addr, "example.com", 50*time.Millisecond)
		if err == nil {
			t.Error("Resolve() error = nil, want non-nil")
		}
	})
}
