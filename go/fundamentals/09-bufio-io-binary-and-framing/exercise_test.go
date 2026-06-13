package wire

import (
	"bufio"
	"bytes"
	"encoding/binary"
	"errors"
	"io"
	"testing"
)

func TestWriteMessageAndReadMessage(t *testing.T) {
	tests := []struct {
		name    string
		payload []byte
	}{
		{"empty payload", []byte{}},
		{"nil payload", nil},
		{"small payload", []byte("hello")},
		{"large payload", bytes.Repeat([]byte("x"), 100000)},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			var buf bytes.Buffer
			if err := WriteMessage(&buf, tt.payload); err != nil {
				t.Fatalf("WriteMessage() error = %v", err)
			}

			got, err := ReadMessage(&buf)
			if err != nil {
				t.Fatalf("ReadMessage() error = %v", err)
			}
			if !bytes.Equal(got, tt.payload) {
				t.Errorf("ReadMessage() = %v, want %v", got, tt.payload)
			}
		})
	}
}

func TestWriteMessageFraming(t *testing.T) {
	var buf bytes.Buffer
	if err := WriteMessage(&buf, []byte("hi")); err != nil {
		t.Fatalf("WriteMessage() error = %v", err)
	}

	if buf.Len() != 4+2 {
		t.Fatalf("WriteMessage() wrote %d bytes, want %d", buf.Len(), 6)
	}

	gotLen := binary.BigEndian.Uint32(buf.Bytes()[:4])
	if gotLen != 2 {
		t.Errorf("header length = %d, want %d", gotLen, 2)
	}
	if string(buf.Bytes()[4:]) != "hi" {
		t.Errorf("payload = %q, want %q", buf.Bytes()[4:], "hi")
	}
}

func TestReadMessageEOF(t *testing.T) {
	t.Run("empty reader returns clean EOF", func(t *testing.T) {
		_, err := ReadMessage(bytes.NewReader(nil))
		if !errors.Is(err, io.EOF) {
			t.Errorf("ReadMessage() error = %v, want io.EOF", err)
		}
	})

	t.Run("partial header returns a non-EOF error", func(t *testing.T) {
		_, err := ReadMessage(bytes.NewReader([]byte{0, 0}))
		if err == nil {
			t.Error("ReadMessage() error = nil, want non-nil")
		}
		if errors.Is(err, io.EOF) {
			t.Errorf("ReadMessage() error = %v, want non-EOF error for a partial header", err)
		}
	})

	t.Run("truncated payload returns an error", func(t *testing.T) {
		var buf bytes.Buffer
		var header [4]byte
		binary.BigEndian.PutUint32(header[:], 10)
		buf.Write(header[:])
		buf.WriteString("short")

		if _, err := ReadMessage(&buf); err == nil {
			t.Error("ReadMessage() error = nil, want non-nil")
		}
	})
}

func TestReadMessageLimit(t *testing.T) {
	t.Run("message within limit is read normally", func(t *testing.T) {
		var buf bytes.Buffer
		if err := WriteMessage(&buf, []byte("hello")); err != nil {
			t.Fatalf("WriteMessage() error = %v", err)
		}

		got, err := ReadMessageLimit(&buf, 10)
		if err != nil {
			t.Fatalf("ReadMessageLimit() error = %v", err)
		}
		if !bytes.Equal(got, []byte("hello")) {
			t.Errorf("ReadMessageLimit() = %q, want %q", got, "hello")
		}
	})

	t.Run("message exceeding limit is rejected", func(t *testing.T) {
		var buf bytes.Buffer
		if err := WriteMessage(&buf, []byte("hello world")); err != nil {
			t.Fatalf("WriteMessage() error = %v", err)
		}

		_, err := ReadMessageLimit(&buf, 5)
		if !errors.Is(err, ErrMessageTooLarge) {
			t.Errorf("ReadMessageLimit() error = %v, want ErrMessageTooLarge", err)
		}
	})

	t.Run("clean EOF at message boundary", func(t *testing.T) {
		_, err := ReadMessageLimit(bytes.NewReader(nil), 10)
		if !errors.Is(err, io.EOF) {
			t.Errorf("ReadMessageLimit() error = %v, want io.EOF", err)
		}
	})
}

func TestWriteMessagesAndReadAllMessages(t *testing.T) {
	t.Run("round trip preserves order", func(t *testing.T) {
		payloads := [][]byte{[]byte("one"), []byte("two"), []byte("three"), {}}

		var buf bytes.Buffer
		bw := bufio.NewWriter(&buf)
		if err := WriteMessages(bw, payloads); err != nil {
			t.Fatalf("WriteMessages() error = %v", err)
		}

		br := bufio.NewReader(&buf)
		got, err := ReadAllMessages(br)
		if err != nil {
			t.Fatalf("ReadAllMessages() error = %v", err)
		}

		if len(got) != len(payloads) {
			t.Fatalf("ReadAllMessages() returned %d messages, want %d", len(got), len(payloads))
		}
		for i := range payloads {
			if !bytes.Equal(got[i], payloads[i]) {
				t.Errorf("message %d = %q, want %q", i, got[i], payloads[i])
			}
		}
	})

	t.Run("empty input produces no messages", func(t *testing.T) {
		br := bufio.NewReader(bytes.NewReader(nil))
		got, err := ReadAllMessages(br)
		if err != nil {
			t.Fatalf("ReadAllMessages() error = %v", err)
		}
		if len(got) != 0 {
			t.Errorf("ReadAllMessages() = %v, want empty", got)
		}
	})
}
