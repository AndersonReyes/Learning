package minihttp

import (
	"bufio"
	"bytes"
	"io"
	"net"
	"net/http"
	"reflect"
	"strings"
	"testing"
	"time"
)

// pipeTimeout bounds how long a test waits on a net.Pipe operation. It's
// generous enough for a correct implementation (everything here is
// in-memory) but keeps a not-yet-implemented stub that never reads or
// writes its conn from hanging the test run.
const pipeTimeout = 200 * time.Millisecond

func TestReadRequestLine(t *testing.T) {
	tests := []struct {
		name        string
		input       string
		wantMethod  string
		wantTarget  string
		wantVersion string
		wantErr     bool
	}{
		{"GET request", "GET /index.html HTTP/1.1\r\n", "GET", "/index.html", "HTTP/1.1", false},
		{"POST request", "POST /submit HTTP/1.1\r\n", "POST", "/submit", "HTTP/1.1", false},
		{"too few fields", "GET /\r\n", "", "", "", true},
		{"empty input", "", "", "", "", true},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			r := bufio.NewReader(strings.NewReader(tt.input))
			method, target, version, err := ReadRequestLine(r)

			if tt.wantErr {
				if err == nil {
					t.Fatalf("ReadRequestLine() error = nil, want non-nil")
				}
				return
			}
			if err != nil {
				t.Fatalf("ReadRequestLine() error = %v", err)
			}
			if method != tt.wantMethod || target != tt.wantTarget || version != tt.wantVersion {
				t.Errorf("ReadRequestLine() = (%q, %q, %q), want (%q, %q, %q)",
					method, target, version, tt.wantMethod, tt.wantTarget, tt.wantVersion)
			}
		})
	}
}

func TestReadHeaders(t *testing.T) {
	tests := []struct {
		name    string
		input   string
		want    http.Header
		wantErr bool
	}{
		{
			name:  "multiple headers",
			input: "Host: example.com\r\nContent-Type: text/plain\r\n\r\n",
			want:  http.Header{"Host": {"example.com"}, "Content-Type": {"text/plain"}},
		},
		{
			name:  "no headers",
			input: "\r\n",
			want:  http.Header{},
		},
		{
			name:  "repeated header name",
			input: "X-Custom: a\r\nX-Custom: b\r\n\r\n",
			want:  http.Header{"X-Custom": {"a", "b"}},
		},
		{
			name:  "lowercase header name is canonicalized",
			input: "content-type: text/plain\r\n\r\n",
			want:  http.Header{"Content-Type": {"text/plain"}},
		},
		{
			name:    "missing colon is an error",
			input:   "BadHeaderLine\r\n\r\n",
			wantErr: true,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			r := bufio.NewReader(strings.NewReader(tt.input))
			got, err := ReadHeaders(r)

			if tt.wantErr {
				if err == nil {
					t.Fatalf("ReadHeaders() error = nil, want non-nil")
				}
				return
			}
			if err != nil {
				t.Fatalf("ReadHeaders() error = %v", err)
			}
			if !reflect.DeepEqual(got, tt.want) {
				t.Errorf("ReadHeaders() = %v, want %v", got, tt.want)
			}
		})
	}
}

func TestReadRequest(t *testing.T) {
	t.Run("GET request without body", func(t *testing.T) {
		input := "GET /index.html HTTP/1.1\r\nHost: example.com\r\n\r\n"
		r := bufio.NewReader(strings.NewReader(input))

		got, err := ReadRequest(r)
		if err != nil {
			t.Fatalf("ReadRequest() error = %v", err)
		}

		want := &Request{
			Method:  "GET",
			Target:  "/index.html",
			Version: "HTTP/1.1",
			Headers: http.Header{"Host": {"example.com"}},
			Body:    nil,
		}
		if got.Method != want.Method || got.Target != want.Target || got.Version != want.Version {
			t.Errorf("ReadRequest() line = (%q, %q, %q), want (%q, %q, %q)",
				got.Method, got.Target, got.Version, want.Method, want.Target, want.Version)
		}
		if !reflect.DeepEqual(got.Headers, want.Headers) {
			t.Errorf("ReadRequest() headers = %v, want %v", got.Headers, want.Headers)
		}
		if len(got.Body) != 0 {
			t.Errorf("ReadRequest() body = %q, want empty", got.Body)
		}
	})

	t.Run("POST request with body", func(t *testing.T) {
		input := "POST /submit HTTP/1.1\r\nHost: example.com\r\nContent-Length: 11\r\n\r\nhello world"
		r := bufio.NewReader(strings.NewReader(input))

		got, err := ReadRequest(r)
		if err != nil {
			t.Fatalf("ReadRequest() error = %v", err)
		}

		if string(got.Body) != "hello world" {
			t.Errorf("ReadRequest() body = %q, want %q", got.Body, "hello world")
		}
	})

	t.Run("body shorter than Content-Length is an error", func(t *testing.T) {
		input := "POST /submit HTTP/1.1\r\nContent-Length: 20\r\n\r\nhello"
		r := bufio.NewReader(strings.NewReader(input))

		if _, err := ReadRequest(r); err == nil {
			t.Error("ReadRequest() error = nil, want non-nil")
		}
	})
}

func TestWriteResponse(t *testing.T) {
	tests := []struct {
		name string
		resp *Response
		want string
	}{
		{
			name: "200 with body",
			resp: &Response{StatusCode: 200, Headers: http.Header{}, Body: []byte("hello")},
			want: "HTTP/1.1 200 OK\r\nContent-Length: 5\r\n\r\nhello",
		},
		{
			name: "404 with no body",
			resp: &Response{StatusCode: 404, Headers: http.Header{}, Body: nil},
			want: "HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\n\r\n",
		},
		{
			name: "extra headers are sorted",
			resp: &Response{StatusCode: 200, Headers: http.Header{"Content-Type": {"text/plain"}}, Body: []byte("hi")},
			want: "HTTP/1.1 200 OK\r\nContent-Length: 2\r\nContent-Type: text/plain\r\n\r\nhi",
		},
		{
			name: "nil headers map",
			resp: &Response{StatusCode: 200, Headers: nil, Body: []byte("ok")},
			want: "HTTP/1.1 200 OK\r\nContent-Length: 2\r\n\r\nok",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			var buf bytes.Buffer
			if err := WriteResponse(&buf, tt.resp); err != nil {
				t.Fatalf("WriteResponse() error = %v", err)
			}
			if buf.String() != tt.want {
				t.Errorf("WriteResponse() wrote %q, want %q", buf.String(), tt.want)
			}
		})
	}
}

func TestServeOnce(t *testing.T) {
	t.Run("GET request gets a 200 response", func(t *testing.T) {
		client, server := net.Pipe()
		defer client.Close()

		var gotReq *Request
		serveDone := make(chan error, 1)
		go func() {
			serveDone <- ServeOnce(server, func(req *Request) *Response {
				gotReq = req
				return &Response{StatusCode: 200, Headers: http.Header{}, Body: []byte("pong")}
			})
		}()

		client.SetDeadline(time.Now().Add(pipeTimeout))

		if _, err := client.Write([]byte("GET /ping HTTP/1.1\r\nHost: example.com\r\n\r\n")); err != nil {
			t.Fatalf("client.Write() error = %v", err)
		}

		respBytes, err := io.ReadAll(client)
		if err != nil {
			t.Fatalf("io.ReadAll() error = %v", err)
		}

		want := "HTTP/1.1 200 OK\r\nContent-Length: 4\r\n\r\npong"
		if string(respBytes) != want {
			t.Errorf("response = %q, want %q", respBytes, want)
		}

		select {
		case err := <-serveDone:
			if err != nil {
				t.Errorf("ServeOnce() = %v, want nil", err)
			}
		case <-time.After(pipeTimeout):
			t.Fatal("ServeOnce did not return")
		}

		if gotReq == nil {
			t.Fatal("handler was not called")
		}
		if gotReq.Method != "GET" || gotReq.Target != "/ping" {
			t.Errorf("handler request = (%q, %q), want (%q, %q)", gotReq.Method, gotReq.Target, "GET", "/ping")
		}
		if got := gotReq.Headers.Get("Host"); got != "example.com" {
			t.Errorf("handler request Host header = %q, want %q", got, "example.com")
		}
	})

	t.Run("POST request body reaches the handler", func(t *testing.T) {
		client, server := net.Pipe()
		defer client.Close()

		var gotBody []byte
		serveDone := make(chan error, 1)
		go func() {
			serveDone <- ServeOnce(server, func(req *Request) *Response {
				gotBody = req.Body
				return &Response{StatusCode: 201, Headers: http.Header{}, Body: nil}
			})
		}()

		client.SetDeadline(time.Now().Add(pipeTimeout))

		req := "POST /items HTTP/1.1\r\nContent-Length: 4\r\n\r\ndata"
		if _, err := client.Write([]byte(req)); err != nil {
			t.Fatalf("client.Write() error = %v", err)
		}

		respBytes, err := io.ReadAll(client)
		if err != nil {
			t.Fatalf("io.ReadAll() error = %v", err)
		}

		want := "HTTP/1.1 201 Created\r\nContent-Length: 0\r\n\r\n"
		if string(respBytes) != want {
			t.Errorf("response = %q, want %q", respBytes, want)
		}

		select {
		case err := <-serveDone:
			if err != nil {
				t.Errorf("ServeOnce() = %v, want nil", err)
			}
		case <-time.After(pipeTimeout):
			t.Fatal("ServeOnce did not return")
		}

		if string(gotBody) != "data" {
			t.Errorf("handler request body = %q, want %q", gotBody, "data")
		}
	})
}
