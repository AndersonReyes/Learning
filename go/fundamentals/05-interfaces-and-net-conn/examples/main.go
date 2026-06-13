// Command main demonstrates interfaces, type switches, io.Reader, and
// errors.Join concepts used in this topic's exercise: implicit interface
// satisfaction, the fmt.Stringer interface, type switches over a custom
// interface, manual io.Reader loops, and combining multiple errors —
// applied to small examples that are deliberately *not* the exercise
// (Conn/Addr abstractions) in exercise.go.
package main

import (
	"errors"
	"fmt"
	"io"
	"strings"
)

// Message is implemented by different kinds of protocol messages.
type Message interface {
	Kind() string
}

// PingMessage is a liveness check with a sequence number.
type PingMessage struct{ Seq int }

func (m PingMessage) Kind() string { return "ping" }

// DataMessage carries a payload.
type DataMessage struct{ Payload []byte }

func (m DataMessage) Kind() string { return "data" }

// String implements fmt.Stringer, so fmt.Sprintf("%s", m) and
// fmt.Println(m) print this instead of the struct's default form.
func (m DataMessage) String() string {
	return fmt.Sprintf("DataMessage(%d bytes)", len(m.Payload))
}

// describe uses a type switch to format each kind of Message differently.
func describe(m Message) string {
	switch v := m.(type) {
	case PingMessage:
		return fmt.Sprintf("ping seq=%d", v.Seq)
	case DataMessage:
		return fmt.Sprintf("data: %s", v)
	default:
		return fmt.Sprintf("unknown message kind %q", m.Kind())
	}
}

// readAll demonstrates the manual io.Reader loop: Read returns (n, nil)
// while data remains and (0, io.EOF) once exhausted.
func readAll(r io.Reader) ([]byte, error) {
	var out []byte
	buf := make([]byte, 4)
	for {
		n, err := r.Read(buf)
		out = append(out, buf[:n]...)
		if err == io.EOF {
			return out, nil
		}
		if err != nil {
			return out, err
		}
	}
}

var (
	errMissingHost = errors.New("missing host")
	errMissingPort = errors.New("missing port")
)

// validateConfig demonstrates errors.Join: collect every problem instead
// of stopping at the first.
func validateConfig(host string, port int) error {
	var errs []error
	if host == "" {
		errs = append(errs, errMissingHost)
	}
	if port == 0 {
		errs = append(errs, errMissingPort)
	}
	return errors.Join(errs...)
}

func main() {
	fmt.Println(describe(PingMessage{Seq: 7}))
	fmt.Println(describe(DataMessage{Payload: []byte("hello")}))

	data, err := readAll(strings.NewReader("hello, reader"))
	if err != nil {
		fmt.Println("readAll error:", err)
	}
	fmt.Printf("readAll: %q (%d bytes)\n", data, len(data))

	if err := validateConfig("", 0); err != nil {
		fmt.Println("config errors:", err)
		fmt.Println("missing host?", errors.Is(err, errMissingHost))
		fmt.Println("missing port?", errors.Is(err, errMissingPort))
	}

	if err := validateConfig("example.com", 443); err != nil {
		fmt.Println("unexpected error:", err)
	} else {
		fmt.Println("config valid")
	}
}
