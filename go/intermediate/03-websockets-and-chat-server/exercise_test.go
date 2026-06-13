package wschat

import (
	"bytes"
	"testing"
	"time"
)

// chanTimeout bounds how long a test waits on a channel operation. It's
// generous enough for a correct implementation but keeps a
// not-yet-implemented stub from hanging the test run.
const chanTimeout = 200 * time.Millisecond

func TestComputeAcceptKey(t *testing.T) {
	// RFC 6455 §1.3 worked example.
	got := ComputeAcceptKey("dGhlIHNhbXBsZSBub25jZQ==")
	want := "s3pPLMBiTxaQ9kYGzzhZRbK+xOo="
	if got != want {
		t.Errorf("ComputeAcceptKey() = %q, want %q", got, want)
	}
}

func TestMaskPayload(t *testing.T) {
	t.Run("matches RFC 6455 example", func(t *testing.T) {
		// RFC 6455 §5.7: masked "Hello" with key 37 fa 21 3d.
		key := [4]byte{0x37, 0xfa, 0x21, 0x3d}
		got := MaskPayload([]byte("Hello"), key)
		want := []byte{0x7f, 0x9f, 0x4d, 0x51, 0x58}
		if !bytes.Equal(got, want) {
			t.Errorf("MaskPayload() = % x, want % x", got, want)
		}
	})

	t.Run("is its own inverse", func(t *testing.T) {
		key := [4]byte{0x01, 0x02, 0x03, 0x04}
		data := []byte("round trip me")

		masked := MaskPayload(data, key)
		unmasked := MaskPayload(masked, key)
		if !bytes.Equal(unmasked, data) {
			t.Errorf("MaskPayload(MaskPayload(data, key), key) = %q, want %q", unmasked, data)
		}
	})
}

func TestReadFrameRFCExamples(t *testing.T) {
	t.Run(`unmasked text "Hello"`, func(t *testing.T) {
		// RFC 6455 §5.7.
		data := []byte{0x81, 0x05, 0x48, 0x65, 0x6c, 0x6c, 0x6f}

		opcode, payload, err := ReadFrame(bytes.NewReader(data))
		if err != nil {
			t.Fatalf("ReadFrame() error = %v", err)
		}
		if opcode != OpcodeText {
			t.Errorf("opcode = %#x, want %#x", opcode, OpcodeText)
		}
		if string(payload) != "Hello" {
			t.Errorf("payload = %q, want %q", payload, "Hello")
		}
	})

	t.Run(`masked text "Hello"`, func(t *testing.T) {
		// RFC 6455 §5.7.
		data := []byte{0x81, 0x85, 0x37, 0xfa, 0x21, 0x3d, 0x7f, 0x9f, 0x4d, 0x51, 0x58}

		opcode, payload, err := ReadFrame(bytes.NewReader(data))
		if err != nil {
			t.Fatalf("ReadFrame() error = %v", err)
		}
		if opcode != OpcodeText {
			t.Errorf("opcode = %#x, want %#x", opcode, OpcodeText)
		}
		if string(payload) != "Hello" {
			t.Errorf("payload = %q, want %q", payload, "Hello")
		}
	})
}

func TestWriteFrameAndReadFrame(t *testing.T) {
	t.Run("unmasked round trip", func(t *testing.T) {
		var buf bytes.Buffer
		if err := WriteFrame(&buf, OpcodeText, []byte("Hello"), false); err != nil {
			t.Fatalf("WriteFrame() error = %v", err)
		}

		opcode, payload, err := ReadFrame(&buf)
		if err != nil {
			t.Fatalf("ReadFrame() error = %v", err)
		}
		if opcode != OpcodeText {
			t.Errorf("opcode = %#x, want %#x", opcode, OpcodeText)
		}
		if string(payload) != "Hello" {
			t.Errorf("payload = %q, want %q", payload, "Hello")
		}
	})

	t.Run("masked round trip", func(t *testing.T) {
		var buf bytes.Buffer
		if err := WriteFrame(&buf, OpcodeBinary, []byte("masked payload"), true); err != nil {
			t.Fatalf("WriteFrame() error = %v", err)
		}

		opcode, payload, err := ReadFrame(&buf)
		if err != nil {
			t.Fatalf("ReadFrame() error = %v", err)
		}
		if opcode != OpcodeBinary {
			t.Errorf("opcode = %#x, want %#x", opcode, OpcodeBinary)
		}
		if string(payload) != "masked payload" {
			t.Errorf("payload = %q, want %q", payload, "masked payload")
		}
	})

	t.Run("16-bit extended length", func(t *testing.T) {
		want := bytes.Repeat([]byte("x"), 200)

		var buf bytes.Buffer
		if err := WriteFrame(&buf, OpcodeBinary, want, false); err != nil {
			t.Fatalf("WriteFrame() error = %v", err)
		}

		_, payload, err := ReadFrame(&buf)
		if err != nil {
			t.Fatalf("ReadFrame() error = %v", err)
		}
		if !bytes.Equal(payload, want) {
			t.Errorf("payload length = %d, want %d", len(payload), len(want))
		}
	})

	t.Run("64-bit extended length", func(t *testing.T) {
		want := bytes.Repeat([]byte("y"), 70000)

		var buf bytes.Buffer
		if err := WriteFrame(&buf, OpcodeBinary, want, false); err != nil {
			t.Fatalf("WriteFrame() error = %v", err)
		}

		_, payload, err := ReadFrame(&buf)
		if err != nil {
			t.Fatalf("ReadFrame() error = %v", err)
		}
		if !bytes.Equal(payload, want) {
			t.Errorf("payload length = %d, want %d", len(payload), len(want))
		}
	})
}

func TestHub(t *testing.T) {
	t.Run("Broadcast delivers to all registered clients", func(t *testing.T) {
		h := NewHub()
		a := make(chan []byte, 1)
		b := make(chan []byte, 1)
		h.Register(a)
		h.Register(b)

		h.Broadcast([]byte("hi"))

		for name, ch := range map[string]chan []byte{"a": a, "b": b} {
			select {
			case msg := <-ch:
				if string(msg) != "hi" {
					t.Errorf("%s received %q, want %q", name, msg, "hi")
				}
			case <-time.After(chanTimeout):
				t.Errorf("%s did not receive the broadcast message", name)
			}
		}
	})

	t.Run("Broadcast does not block on a full client", func(t *testing.T) {
		h := NewHub()
		full := make(chan []byte, 1)
		full <- []byte("already full")
		h.Register(full)

		done := make(chan struct{})
		go func() {
			h.Broadcast([]byte("dropped"))
			close(done)
		}()

		select {
		case <-done:
		case <-time.After(chanTimeout):
			t.Fatal("Broadcast blocked on a full client channel")
		}
	})

	t.Run("Unregister stops further broadcasts and closes the channel", func(t *testing.T) {
		h := NewHub()
		client := make(chan []byte, 1)
		h.Register(client)
		h.Unregister(client)

		h.Broadcast([]byte("after unregister"))

		if _, ok := <-client; ok {
			t.Error("client channel ok = true after Unregister, want closed channel")
		}
	})
}
