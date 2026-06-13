// Command main demonstrates bufio, io, and encoding/binary concepts used
// in this topic's exercise: io.ReadFull's EOF vs ErrUnexpectedEOF
// distinction, bufio.Scanner for line-oriented framing, and binary.Write/
// binary.Read for a fixed-size struct header — applied to small examples
// that are deliberately *not* the exercise (WriteMessage/ReadMessage/
// ReadMessageLimit/WriteMessages/ReadAllMessages in exercise.go).
package main

import (
	"bufio"
	"bytes"
	"encoding/binary"
	"errors"
	"fmt"
	"io"
	"strings"
)

// header is a fixed-size struct binary.Write/binary.Read can encode and
// decode directly, without manual PutUint32/Uint32 calls.
type header struct {
	Version uint8
	Flags   uint8
	Length  uint16
}

func main() {
	// io.ReadFull: clean EOF vs ErrUnexpectedEOF.
	full := bytes.NewReader([]byte{1, 2, 3, 4})
	buf := make([]byte, 4)
	if _, err := io.ReadFull(full, buf); err != nil {
		fmt.Println("unexpected error:", err)
	} else {
		fmt.Println("read full buffer:", buf)
	}

	empty := bytes.NewReader(nil)
	if _, err := io.ReadFull(empty, buf); errors.Is(err, io.EOF) {
		fmt.Println("empty reader: clean io.EOF")
	}

	short := bytes.NewReader([]byte{1, 2})
	if _, err := io.ReadFull(short, buf); errors.Is(err, io.ErrUnexpectedEOF) {
		fmt.Println("short reader: io.ErrUnexpectedEOF")
	}

	// bufio.Scanner: line-oriented framing (an alternative to this
	// topic's length-prefixed framing, used by text protocols).
	text := "first line\nsecond line\nthird line\n"
	scanner := bufio.NewScanner(strings.NewReader(text))
	for scanner.Scan() {
		fmt.Println("line:", scanner.Text())
	}

	// binary.Write/binary.Read: encode and decode a fixed-size struct
	// header in one call, using BigEndian byte order.
	var out bytes.Buffer
	h := header{Version: 1, Flags: 0x02, Length: 1024}
	if err := binary.Write(&out, binary.BigEndian, h); err != nil {
		fmt.Println("binary.Write error:", err)
		return
	}
	fmt.Printf("encoded header bytes: %v\n", out.Bytes())

	var decoded header
	if err := binary.Read(&out, binary.BigEndian, &decoded); err != nil {
		fmt.Println("binary.Read error:", err)
		return
	}
	fmt.Printf("decoded header: %+v\n", decoded)
}
