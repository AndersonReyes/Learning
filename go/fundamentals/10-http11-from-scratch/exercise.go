// Package minihttp implements a minimal HTTP/1.1 request/response parser
// and writer from scratch (RFC 9112), using bufio and Content-Length
// framing from topic 9 over a net.Conn from topic 8.
package minihttp

import (
	"bufio"
	"bytes"
	"errors"
	"fmt"
	"io"
	"net"
	"net/http"
	"sort"
	"strconv"
	"strings"
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
	line, err := r.ReadString('\n')
	if err != nil {
		return "", "", "", err
	}

	line = strings.Trim(line, "\r\n")
	splits := strings.SplitN(line, " ", 3)

	if len(splits) != 3 {
		return "", "", "", errors.New("Invalid requrest line")
	}
	return splits[0], splits[1], splits[2], nil
}

// ReadHeaders reads zero or more "Name: value\r\n" header lines from r,
// canonicalizing names and supporting repeated header names, stopping at
// and consuming the terminating blank line ("\r\n"). It returns an error
// if a non-blank line has no ":" separator.
func ReadHeaders(r *bufio.Reader) (http.Header, error) {
	header := make(http.Header)
	for {
		line, err := r.ReadString('\n')

		if err == io.EOF {
			break
		}

		if err != nil {
			return nil, err
		}

		line = strings.Trim(line, "\r\n")
		if line == "" {
			// reached blank line
			break
		}

		splits := strings.SplitN(line, ": ", 2)

		if len(splits) != 2 {
			return nil, fmt.Errorf("missing ':' separator in line [%s]\n", line)
		}
		header.Add(splits[0], splits[1])
	}
	return header, nil
}

// ReadRequest reads a full HTTP/1.1 request from r: the request line (via
// ReadRequestLine), headers (via ReadHeaders), and — if the Content-Length
// header is present and positive — exactly that many body bytes.
func ReadRequest(r *bufio.Reader) (*Request, error) {

	method, target, version, err := ReadRequestLine(r)

	req := Request{
		Method:  method,
		Target:  target,
		Version: version,
	}
	if err == io.EOF {
		return &req, nil

	}

	if err != nil {
		return nil, err
	}

	header, err := ReadHeaders(r)
	req.Headers = header
	if err == io.EOF {
		return &req, nil
	}

	if err != nil {
		return nil, err
	}

	//allbytes, err := io.ReadAll(r)
	//fmt.Printf("allBytes as string: [%s]", string(allbytes))
	//return &req, nil

	var contentLength int64 = 0
	if cl := header.Get("Content-Length"); cl != "" {
		l, err := strconv.ParseInt(cl, 10, 64)
		if err != nil {
			return nil, err
		}
		contentLength = l
	}

	if contentLength > 0 {
		req.Body = make([]byte, contentLength)
		_, err := io.ReadFull(r, req.Body)
		if err != nil && err != io.EOF {
			return nil, err
		}
	}

	return &req, nil
}

// WriteResponse writes resp to w as an HTTP/1.1 response: a status line
// (using http.StatusText for the reason phrase), resp.Headers plus a
// Content-Length computed from len(resp.Body) written in sorted key
// order, a blank line, and the body.
func WriteResponse(w io.Writer, resp *Response) error {
	var buf bytes.Buffer
	_, err := fmt.Fprintf(&buf, "HTTP/1.1 %d %s\r\n", resp.StatusCode, http.StatusText(resp.StatusCode))

	if err != nil {
		return err
	}

	keys := []string{"Content-Length"}

	for k := range resp.Headers {
		keys = append(keys, k)
	}

	sort.Strings(keys)

	for _, name := range keys {

		values := resp.Headers.Values(name)
		if name == "Content-Length" {
			s := strconv.Itoa(len(resp.Body))
			values = []string{s}
		}

		for _, v := range values {
			_, err = fmt.Fprintf(&buf, "%s: %s\r\n", name, v)
			if err != nil {
				return err
			}
		}
	}
	buf.WriteString("\r\n")
	buf.Write(resp.Body)

	_, err = w.Write(buf.Bytes())
	if err != nil {
		return err
	}

	return nil
}

// ServeOnce reads a single request from conn (via ReadRequest), passes it
// to handler, writes handler's returned Response back to conn (via
// WriteResponse), and closes conn.
func ServeOnce(conn net.Conn, handler func(*Request) *Response) error {
	reader := bufio.NewReader(conn)
	req, err := ReadRequest(reader)
	if err != nil {
		return err
	}

	resp := handler(req)

	writer := bufio.NewWriter(conn)
	err = WriteResponse(writer, resp)
	if err != nil {
		return nil
	}
	err = writer.Flush()
	if err != nil {
		return nil
	}

	err = conn.Close()
	if err != nil {
		return nil
	}
	return nil
}
