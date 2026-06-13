// Package minihttp implements a minimal HTTP/1.1 request/response parser
// and writer from scratch (RFC 9112), using bufio and Content-Length
// framing from topic 9 over a net.Conn from topic 8.
package minihttp

import (
	"bufio"
	"errors"
	"io"
	"net"
	"net/http"
)

// Request is a parsed HTTP/1.1 request.
type Request struct {
	Method  string
	Target  string
	Version string
	Headers http.Header
	Body    []byte
}

// Response is an HTTP/1.1 response to be written to a client.
type Response struct {
	StatusCode int
	Headers    http.Header
	Body       []byte
}

// ReadRequestLine reads and parses an HTTP/1.1 request line
// ("METHOD target HTTP-version\r\n") from r, returning its three
// whitespace-separated fields. It returns an error if the line cannot be
// read or does not have exactly three fields.
func ReadRequestLine(r *bufio.Reader) (method, target, version string, err error) {
	return "", "", "", errors.New("not implemented")
}

// ReadHeaders reads zero or more "Name: value\r\n" header lines from r,
// canonicalizing names and supporting repeated header names, stopping at
// and consuming the terminating blank line ("\r\n"). It returns an error
// if a non-blank line has no ":" separator.
func ReadHeaders(r *bufio.Reader) (http.Header, error) {
	return nil, errors.New("not implemented")
}

// ReadRequest reads a full HTTP/1.1 request from r: the request line (via
// ReadRequestLine), headers (via ReadHeaders), and — if the Content-Length
// header is present and positive — exactly that many body bytes.
func ReadRequest(r *bufio.Reader) (*Request, error) {
	return nil, errors.New("not implemented")
}

// WriteResponse writes resp to w as an HTTP/1.1 response: a status line
// (using http.StatusText for the reason phrase), resp.Headers plus a
// Content-Length computed from len(resp.Body) written in sorted key
// order, a blank line, and the body.
func WriteResponse(w io.Writer, resp *Response) error {
	return errors.New("not implemented")
}

// ServeOnce reads a single request from conn (via ReadRequest), passes it
// to handler, writes handler's returned Response back to conn (via
// WriteResponse), and closes conn.
func ServeOnce(conn net.Conn, handler func(*Request) *Response) error {
	return errors.New("not implemented")
}
