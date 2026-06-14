// Package miniquic implements wire-format primitives from RFC 9000 (QUIC):
// variable-length integers (§16) and CRYPTO frames (§19.6) — and drives the
// TLS 1.3 handshake QUIC uses to establish a connection (RFC 9001) via the
// crypto/tls package's QUICConn API. Together these are the pieces a QUIC
// implementation needs to perform its handshake: encode/decode the integers
// that appear throughout the wire format, frame the TLS handshake bytes
// that flow over CRYPTO frames, and drive crypto/tls's QUIC-aware TLS state
// machine to a completed connection.
package miniquic

import (
	"crypto/tls"
	"errors"
)

// cryptoFrameType is the QUIC frame type for a CRYPTO frame (RFC 9000
// §19.6).
const cryptoFrameType = 0x06

// maxVarint is the largest value a QUIC variable-length integer can encode
// (2^62 - 1, RFC 9000 §16).
const maxVarint = (1 << 62) - 1

// EncodeVarint encodes v as a QUIC variable-length integer (RFC 9000 §16):
// 1, 2, 4, or 8 bytes, network byte order, where the two most-significant
// bits of the first byte encode log2 of the length (00 = 1 byte, 01 = 2,
// 10 = 4, 11 = 8) and the remaining bits hold v. EncodeVarint returns an
// error if v > maxVarint (2^62 - 1), the largest value a QUIC varint can
// represent.
//
//	EncodeVarint(37)    -> []byte{0x25}, nil
//	EncodeVarint(15293) -> []byte{0x7b, 0xbd}, nil
//	EncodeVarint(1<<62) -> nil, error
func EncodeVarint(v uint64) ([]byte, error) {
	return nil, errors.New("not implemented")
}

// DecodeVarint decodes a QUIC variable-length integer (RFC 9000 §16) from
// the start of data, returning the decoded value and the number of bytes
// consumed (1, 2, 4, or 8 — never more than len(data)). DecodeVarint returns
// an error if data is empty, or shorter than the length indicated by the two
// most-significant bits of data[0].
//
//	DecodeVarint([]byte{0x25})       -> 37, 1, nil
//	DecodeVarint([]byte{0x7b, 0xbd}) -> 15293, 2, nil
//	DecodeVarint([]byte{0x40})       -> 0, 0, error (2-byte encoding, only 1 byte present)
//	DecodeVarint([]byte{})           -> 0, 0, error
func DecodeVarint(data []byte) (uint64, int, error) {
	return 0, 0, errors.New("not implemented")
}

// EncodeCryptoFrame returns a QUIC CRYPTO frame (RFC 9000 §19.6): the 1-byte
// frame type 0x06, followed by offset and len(data), each encoded as a QUIC
// variable-length integer, followed by data itself.
func EncodeCryptoFrame(offset uint64, data []byte) []byte {
	return nil
}

// DecodeCryptoFrame parses a QUIC CRYPTO frame (RFC 9000 §19.6) from frame,
// returning its offset and data. DecodeCryptoFrame returns an error if
// frame's type byte is not cryptoFrameType, frame is too short to contain
// its offset and length fields, or frame has fewer bytes remaining than its
// declared data length.
func DecodeCryptoFrame(frame []byte) (offset uint64, data []byte, err error) {
	return 0, nil, errors.New("not implemented")
}

// RunHandshake drives conn's TLS 1.3 handshake (RFC 9001: Using TLS to
// Secure QUIC) to completion, exchanging CRYPTO frame data with a peer
// running RunHandshake on the other end of send/recv. conn must already be
// configured via tls.QUICClient or tls.QUICServer.
//
// RunHandshake calls conn.Start, returning its error if any, and then, until
// the handshake completes, repeatedly inspects conn's next event:
//
//   - QUICWriteData: send the event's data to the peer, encoded as
//     append([]byte{byte(level)}, EncodeCryptoFrame(offset, data)...), where
//     level and data come from the event and offset is the running total of
//     bytes previously sent at that encryption level (starting at 0).
//   - QUICTransportParametersRequired: call conn.SetTransportParameters with
//     an empty parameter list.
//   - QUICNoEvent (conn needs more input): receive the next message from
//     recv, split it into its 1-byte encryption level and an
//     EncodeCryptoFrame-encoded remainder, decode that remainder with
//     DecodeCryptoFrame, and pass its data to conn.HandleData at the decoded
//     level, returning conn.HandleData's error if any.
//   - QUICHandshakeDone: return nil.
//   - any other event kind: ignore it and continue.
func RunHandshake(conn *tls.QUICConn, send chan<- []byte, recv <-chan []byte) error {
	return errors.New("not implemented")
}
