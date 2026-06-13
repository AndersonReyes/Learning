# 10. `net/http` Internals + HTTP/1.1 From Scratch

## HTTP/1.1 message format (RFC 9112)

Both requests and responses share the same shape: a start line, header
fields, a blank line, then an optional body.

```
START-LINE\r\n
Header-Name: header value\r\n
Another-Header: another value\r\n
\r\n
optional body bytes...
```

- **Request** start line: `METHOD target HTTP-version` —
  e.g. `GET /index.html HTTP/1.1`.
- **Response** start line (status line): `HTTP-version status-code
  reason-phrase` — e.g. `HTTP/1.1 404 Not Found`.
- Every line (including the start line and each header) ends with `\r\n`
  (CRLF). The blank line `\r\n` separates headers from the body.
- Header field names are case-insensitive. Go's [`http.Header`](https://pkg.go.dev/net/http#Header)
  is a `map[string][]string` whose `Add`/`Set`/`Get`/`Values` methods
  canonicalize the key (e.g. `content-type` → `Content-Type`) so lookups
  don't depend on the wire casing. A header name can repeat — `Add`
  appends another value under the same canonical key.

## Framing the body: `Content-Length`

Topic 9 built a custom length-prefixed binary framing. HTTP/1.1 does the
same thing with a header instead of a fixed binary prefix: the
`Content-Length` header gives the body's size in bytes, and the reader
calls `io.ReadFull` for exactly that many bytes after the blank line — the
same EOF-vs-`io.ErrUnexpectedEOF` reasoning from topic 9 applies if the
body is shorter than advertised. (HTTP/1.1 also supports `Transfer-Encoding:
chunked` for bodies whose length isn't known up front, which this topic
doesn't implement.)

## Parsing with `bufio.Reader`

`bufio.Reader.ReadString('\n')` (topic 9) is the natural tool for reading
CRLF-terminated lines one at a time:

```go
line, err := r.ReadString('\n')
line = strings.TrimRight(line, "\r\n")
```

A request line splits into exactly three whitespace-separated fields
(`strings.Fields`); a header line splits on the first `:` into a name and
a value (`strings.SplitN(line, ":", 2)`, then `strings.TrimSpace` the
value). The blank line terminating the headers is just `line == ""` after
trimming.

## `net/http`: the idiomatic version

Everything above is what `net/http` does internally, exposed through a much
higher-level API:

```go
mux := http.NewServeMux()
mux.HandleFunc("/ping", func(w http.ResponseWriter, r *http.Request) {
    w.Header().Set("Content-Type", "text/plain")
    w.WriteHeader(http.StatusOK)
    w.Write([]byte("pong"))
})

srv := &http.Server{Addr: "127.0.0.1:8080", Handler: mux}
err := srv.ListenAndServe()
```

- `http.HandlerFunc` adapts a `func(http.ResponseWriter, *http.Request)`
  to the `http.Handler` interface (one method: `ServeHTTP`).
- `*http.Request` already has `Method`, `URL` (parsed target), `Header`
  (an `http.Header`, same type used above), and `Body` (an `io.ReadCloser`
  — read it with `io.ReadAll` or `json.NewDecoder`, topic 11).
- `http.ResponseWriter`: call `Header().Set(...)` to set response headers
  *before* `WriteHeader(statusCode)` — headers are flushed to the
  connection as soon as the first `Write` or explicit `WriteHeader` call
  happens, so setting headers afterward has no effect.
- `net.Conn` (topic 8), `bufio` (topic 9), and `Content-Length` framing all
  still apply underneath — `net/http` just builds and parses the messages
  for you, handles `Transfer-Encoding: chunked`, keep-alive connections,
  pipelining edge cases, and timeouts.

---

## Networking: HTTP/1.1 over TCP

HTTP/1.1 ([RFC 9112](https://www.rfc-editor.org/rfc/rfc9112)) is a
text-based, request/response protocol layered on top of TCP (topic 8): a
client opens a connection, sends a request message, and the server sends
back exactly one response message before (with `Connection: keep-alive`,
the default in HTTP/1.1) the same connection can be reused for further
requests. The message framing covered above — start line, headers,
`Content-Length`-delimited body — is what lets both ends agree on where one
message ends and the next begins on the same byte stream, exactly the
problem topic 9 introduced framing to solve.

This topic's exercise builds: a request-line parser, a header parser, a
full request reader (using `Content-Length` for the body), a response
writer (computing `Content-Length` from the body and using
`http.StatusText` for the reason phrase), and a `ServeOnce` function that
reads one request from a `net.Conn`, dispatches it to a handler, and writes
the response — the core loop `net/http` runs for you.

## Further Reading

- [RFC 9112 (HTTP/1.1)](https://www.rfc-editor.org/rfc/rfc9112)
- [`net/http`](https://pkg.go.dev/net/http)
- [`http.Header`](https://pkg.go.dev/net/http#Header), [`http.StatusText`](https://pkg.go.dev/net/http#StatusText)
- [`http.Server`](https://pkg.go.dev/net/http#Server), [`http.HandlerFunc`](https://pkg.go.dev/net/http#HandlerFunc)
- [`bufio.Reader.ReadString`](https://pkg.go.dev/bufio#Reader.ReadString)
