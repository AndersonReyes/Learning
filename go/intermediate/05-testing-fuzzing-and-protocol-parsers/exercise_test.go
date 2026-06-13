package resp

import (
	"bufio"
	"bytes"
	"reflect"
	"strings"
	"testing"
)

func TestParseValue(t *testing.T) {
	tests := []struct {
		name    string
		input   string
		want    Value
		wantErr bool
	}{
		{"simple string", "+OK\r\n", Value{Type: SimpleString, Str: "OK"}, false},
		{"error", "-Error message\r\n", Value{Type: Error, Str: "Error message"}, false},
		{"integer", ":1000\r\n", Value{Type: Integer, Int: 1000}, false},
		{"negative integer", ":-1\r\n", Value{Type: Integer, Int: -1}, false},
		{"bulk string", "$6\r\nfoobar\r\n", Value{Type: BulkString, Str: "foobar"}, false},
		{"empty bulk string", "$0\r\n\r\n", Value{Type: BulkString, Str: ""}, false},
		{"null bulk string", "$-1\r\n", Value{Type: BulkString, Null: true}, false},
		{"array", "*2\r\n$3\r\nfoo\r\n$3\r\nbar\r\n", Value{Type: Array, Array: []Value{
			{Type: BulkString, Str: "foo"},
			{Type: BulkString, Str: "bar"},
		}}, false},
		{"empty array", "*0\r\n", Value{Type: Array, Array: []Value{}}, false},
		{"null array", "*-1\r\n", Value{Type: Array, Null: true}, false},
		{"nested array", "*1\r\n*1\r\n+OK\r\n", Value{Type: Array, Array: []Value{
			{Type: Array, Array: []Value{
				{Type: SimpleString, Str: "OK"},
			}},
		}}, false},
		{"unknown type byte", "X1\r\n", Value{}, true},
		{"bulk length too large", "$999999999999\r\n", Value{}, true},
		{"array length too large", "*999999999999\r\n", Value{}, true},
		{"negative bulk length other than -1", "$-2\r\n", Value{}, true},
		{"truncated bulk string", "$6\r\nfoo\r\n", Value{}, true},
		{"empty input", "", Value{}, true},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got, err := ParseValue(bufio.NewReader(strings.NewReader(tt.input)))
			if (err != nil) != tt.wantErr {
				t.Fatalf("ParseValue(%q) error = %v, wantErr %v", tt.input, err, tt.wantErr)
			}
			if tt.wantErr {
				return
			}
			if !reflect.DeepEqual(got, tt.want) {
				t.Errorf("ParseValue(%q) = %#v, want %#v", tt.input, got, tt.want)
			}
		})
	}
}

func TestEncode(t *testing.T) {
	tests := []struct {
		name string
		in   Value
		want string
	}{
		{"simple string", Value{Type: SimpleString, Str: "OK"}, "+OK\r\n"},
		{"error", Value{Type: Error, Str: "Error message"}, "-Error message\r\n"},
		{"integer", Value{Type: Integer, Int: 1000}, ":1000\r\n"},
		{"negative integer", Value{Type: Integer, Int: -1}, ":-1\r\n"},
		{"bulk string", Value{Type: BulkString, Str: "foobar"}, "$6\r\nfoobar\r\n"},
		{"empty bulk string", Value{Type: BulkString, Str: ""}, "$0\r\n\r\n"},
		{"null bulk string", Value{Type: BulkString, Null: true}, "$-1\r\n"},
		{"array", Value{Type: Array, Array: []Value{
			{Type: BulkString, Str: "foo"},
			{Type: BulkString, Str: "bar"},
		}}, "*2\r\n$3\r\nfoo\r\n$3\r\nbar\r\n"},
		{"empty array", Value{Type: Array, Array: []Value{}}, "*0\r\n"},
		{"null array", Value{Type: Array, Null: true}, "*-1\r\n"},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := string(tt.in.Encode()); got != tt.want {
				t.Errorf("Encode() = %q, want %q", got, tt.want)
			}
		})
	}
}

func TestString(t *testing.T) {
	tests := []struct {
		name string
		in   Value
		want string
	}{
		{"simple string", Value{Type: SimpleString, Str: "OK"}, "OK"},
		{"error", Value{Type: Error, Str: "Error message"}, "(error) Error message"},
		{"integer", Value{Type: Integer, Int: 1000}, "(integer) 1000"},
		{"bulk string", Value{Type: BulkString, Str: "foobar"}, `"foobar"`},
		{"null bulk string", Value{Type: BulkString, Null: true}, "(nil)"},
		{"array", Value{Type: Array, Array: []Value{
			{Type: BulkString, Str: "foo"},
			{Type: BulkString, Str: "bar"},
		}}, "1) \"foo\"\n2) \"bar\""},
		{"empty array", Value{Type: Array, Array: []Value{}}, "(empty array)"},
		{"null array", Value{Type: Array, Null: true}, "(nil)"},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := tt.in.String(); got != tt.want {
				t.Errorf("String() = %q, want %q", got, tt.want)
			}
		})
	}
}

func TestEncodeCommand(t *testing.T) {
	got := string(EncodeCommand("SET", "key", "value"))
	want := "*3\r\n$3\r\nSET\r\n$3\r\nkey\r\n$5\r\nvalue\r\n"
	if got != want {
		t.Errorf("EncodeCommand() = %q, want %q", got, want)
	}
}

func TestParseCommand(t *testing.T) {
	tests := []struct {
		name    string
		input   string
		want    []string
		wantErr bool
	}{
		{"simple command", "*3\r\n$3\r\nSET\r\n$3\r\nkey\r\n$5\r\nvalue\r\n", []string{"SET", "key", "value"}, false},
		{"single command", "*1\r\n$4\r\nPING\r\n", []string{"PING"}, false},
		{"not an array", "+OK\r\n", nil, true},
		{"null array", "*-1\r\n", nil, true},
		{"non-bulk-string element", "*1\r\n:1\r\n", nil, true},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got, err := ParseCommand(bufio.NewReader(strings.NewReader(tt.input)))
			if (err != nil) != tt.wantErr {
				t.Fatalf("ParseCommand(%q) error = %v, wantErr %v", tt.input, err, tt.wantErr)
			}
			if tt.wantErr {
				return
			}
			if !reflect.DeepEqual(got, tt.want) {
				t.Errorf("ParseCommand(%q) = %v, want %v", tt.input, got, tt.want)
			}
		})
	}
}

// FuzzParseValue checks the most important property of a parser fed
// untrusted bytes: it never panics, and any successfully-parsed value
// round-trips through Encode and back to an identical Value.
func FuzzParseValue(f *testing.F) {
	seeds := []string{
		"+OK\r\n",
		"-Error message\r\n",
		":1000\r\n",
		"$6\r\nfoobar\r\n",
		"$0\r\n\r\n",
		"$-1\r\n",
		"*2\r\n$3\r\nfoo\r\n$3\r\nbar\r\n",
		"*0\r\n",
		"*-1\r\n",
		"*1\r\n*1\r\n+OK\r\n",
		"$999999999999\r\n",
		"*999999999999\r\n",
		"$-2\r\n",
		"X1\r\n",
		"",
		"\r\n",
		"$6\r\nfoo\r\n",
	}
	for _, seed := range seeds {
		f.Add([]byte(seed))
	}

	f.Fuzz(func(t *testing.T, data []byte) {
		v, err := ParseValue(bufio.NewReader(bytes.NewReader(data)))
		if err != nil {
			return
		}

		encoded := v.Encode()
		v2, err := ParseValue(bufio.NewReader(bytes.NewReader(encoded)))
		if err != nil {
			t.Fatalf("ParseValue(Encode(v)) error = %v; v = %#v, encoded = %q", err, v, encoded)
		}
		if !reflect.DeepEqual(v, v2) {
			t.Fatalf("round trip mismatch: v = %#v, v2 = %#v", v, v2)
		}
	})
}
