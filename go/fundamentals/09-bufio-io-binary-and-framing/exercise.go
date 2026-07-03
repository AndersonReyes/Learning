// Package wire applies bufio, io, and encoding/binary to a custom
// length-prefixed binary wire protocol: each message is a 4-byte
// big-endian length header followed by that many payload bytes.
package wire

import (
	"bufio"
	"encoding/binary"
	"errors"
	"io"
)

// ErrMessageTooLarge is returned by ReadMessageLimit when a message's
// length header exceeds the configured limit.
var ErrMessageTooLarge = errors.New("wire: message exceeds size limit")

// WriteMessage writes payload to w as a single length-prefixed message: a
// 4-byte big-endian header containing len(payload), followed by payload
// itself.
func WriteMessage(w io.Writer, payload []byte) error {

	var header [4]byte
	binary.BigEndian.PutUint32(header[:], uint32(len(payload)))

	bytesWritten, err := w.Write(header[:])

	if err != nil {
		return err
	}

	if bytesWritten != 4 {
		return errors.New("did not write header correctly")
	}

	bytesWritten, err = w.Write(payload[:])

	if err != nil {
		return err
	}

	if bytesWritten != len(payload) {
		return errors.New("did not write payload correctly")
	}

	return nil
}

// ReadMessage reads one length-prefixed message from r and returns its
// payload. If r ends before any bytes of the header are read,
// ReadMessage returns io.EOF (a clean end of stream). Any other
// short read (a truncated header or payload) is reported as an error
// wrapping io.ErrUnexpectedEOF.
func ReadMessage(r io.Reader) ([]byte, error) {
	header := make([]byte, 4)
	nRead, err := r.Read(header)

	if err != nil {
		return nil, err
	}

	if nRead != 4 {
		return nil, errors.New("did not read header correctly")
	}

	payloadSize := binary.BigEndian.Uint32(header)
	payload := make([]byte, payloadSize)

	nRead, err = r.Read(payload)

	if err != nil {
		return nil, err
	}
	if nRead != int(payloadSize) {
		return nil, errors.New("did not read payload correctly")
	}

	return payload, nil
}

// ReadMessageLimit behaves like ReadMessage, but first checks the decoded
// length header against maxSize. If the header's length exceeds maxSize,
// ReadMessageLimit returns an error wrapping ErrMessageTooLarge without
// attempting to read (or allocate a buffer for) the payload.
func ReadMessageLimit(r io.Reader, maxSize uint32) ([]byte, error) {
	header := make([]byte, 4)
	nRead, err := r.Read(header)

	if err != nil {
		return nil, err
	}

	if nRead != 4 {
		return nil, errors.New("did not read header correctly")
	}

	payloadSize := binary.BigEndian.Uint32(header)

	if payloadSize >= maxSize {
		return nil, ErrMessageTooLarge
	}

	payload := make([]byte, payloadSize)

	nRead, err = r.Read(payload)

	if err != nil {
		return nil, err
	}
	if nRead != int(payloadSize) {
		return nil, errors.New("did not read payload correctly")
	}
	return payload, nil
}

// WriteMessages writes each element of payloads to w as a separate
// length-prefixed message (via WriteMessage), then flushes w so all
// messages reach the underlying writer.
func WriteMessages(w *bufio.Writer, payloads [][]byte) error {
	for _, p := range payloads {
		err := WriteMessage(w, p)
		if err != nil {
			return err
		}
	}

	return w.Flush()
}

// ReadAllMessages reads length-prefixed messages from r (via ReadMessage)
// until a clean io.EOF, and returns their payloads in order. A clean EOF
// at a message boundary is not an error; any other error from ReadMessage
// is returned to the caller.
func ReadAllMessages(r *bufio.Reader) ([][]byte, error) {
	var out [][]byte
	for {
		payload, err := ReadMessage(r)

		if err != nil {
			if err == io.EOF {
				break
			}
			return nil, err
		}

		out = append(out, payload)

	}
	return out, nil
}
