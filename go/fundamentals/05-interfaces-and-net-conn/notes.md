# 05. Interfaces & Error Wrapping

## Interfaces: implicit satisfaction

An interface type is a set of method signatures. A concrete type
satisfies an interface simply by having those methods — **no `implements`
keyword**, no declared relationship:

```go
type Addr interface {
	Network() string
	String() string
}

type TCPAddr struct{ IP string; Port int }

func (a TCPAddr) Network() string { return "tcp" }
func (a TCPAddr) String() string  { return fmt.Sprintf("%s:%d", a.IP, a.Port) }

// TCPAddr satisfies Addr automatically — no extra declaration needed.
var a Addr = TCPAddr{IP: "192.0.2.1", Port: 80}
```

A type can satisfy many interfaces at once just by having the union of
their methods. This is why a single concrete connection type can be used
anywhere an `io.Reader`, `io.Writer`, or `net.Conn` is expected.

## Interface values

An interface value is a `(type, value)` pair. The zero value of an
interface is `nil` (both type and value are nil). Calling a method on a
nil interface value panics — but a non-nil interface holding a nil
*concrete* pointer is NOT a nil interface (a classic gotcha, avoided here
by storing fields with the interface type directly rather than wrapping
concrete pointers).

## Type assertions & type switches

```go
var a Addr = TCPAddr{IP: "192.0.2.1", Port: 80}

t, ok := a.(TCPAddr) // type assertion with ok-check: no panic if it fails
_ = t

switch v := a.(type) {
case TCPAddr:
	fmt.Println("tcp", v.Port)
case UDPAddr:
	fmt.Println("udp", v.Port)
default:
	fmt.Println("unknown addr type")
}
```

## `io.Reader` / `io.Writer` conventions

```go
type Reader interface { Read(p []byte) (n int, err error) }
type Writer interface { Write(p []byte) (n int, err error) }
```

- `Read` returns `n > 0` bytes read and `nil` error when data is
  available — even if that's the last of the data.
- `Read` returns `(0, io.EOF)` once there is nothing left to read.
  `io.EOF` is a sentinel `error` value, checked with `err == io.EOF` or
  `errors.Is(err, io.EOF)`.
- `Write` returns `(n, nil)` with `n == len(p)` on success. A short write
  (`n < len(p)` with `err == nil`) is itself an error condition
  (`io.ErrShortWrite`).

## `errors.Join`: combining multiple errors

`fmt.Errorf("...: %w", err)` (topic 2) wraps **one** error. `errors.Join`
(Go 1.20+) combines **multiple** errors into one:

```go
err := errors.Join(err1, err2, nil, err3) // nils are dropped
errors.Join()        // -> nil
errors.Join(nil, nil) // -> nil

errors.Is(err, err1) // true — checks ALL joined errors, recursively
errors.Is(err, err2) // true
```

`errors.Is`/`errors.As` traverse a joined error's tree, so a caller can
check for any specific sentinel among many collected failures — useful
for "validate everything, report everything" functions.

---

## Networking: abstracting transports with `net.Conn`

[`net.Conn`](https://pkg.go.dev/net#Conn) is an interface, not a concrete
type:

```go
type Conn interface {
	Read(b []byte) (n int, err error)
	Write(b []byte) (n int, err error)
	Close() error
	LocalAddr() Addr
	RemoteAddr() Addr
	SetDeadline(t time.Time) error
	SetReadDeadline(t time.Time) error
	SetWriteDeadline(t time.Time) error
}
```

TCP connections, UDP connections, TLS connections, and Unix-domain socket
connections are all *different concrete types* that satisfy `net.Conn`.
Code written against the `net.Conn` interface (or even just `io.Reader`
and `io.Writer`) works with all of them — and with in-memory fakes used in
tests, so you can exercise protocol logic without opening real sockets.

[`net.Addr`](https://pkg.go.dev/net#Addr) is similarly minimal:

```go
type Addr interface {
	Network() string // "tcp", "udp"
	String() string   // "192.0.2.1:80"
}
```

## Further Reading

- [Tour: Interfaces](https://go.dev/tour/methods/9)
- [Tour: Interface values](https://go.dev/tour/methods/11)
- [Tour: Type assertions](https://go.dev/tour/methods/15)
- [Tour: Type switches](https://go.dev/tour/methods/16)
- [Tour: Errors](https://go.dev/tour/methods/18)
- [`errors.Join`](https://pkg.go.dev/errors#Join)
- [`io` package](https://pkg.go.dev/io) (`io.Reader`, `io.Writer`, `io.EOF`)
- [`net.Conn`](https://pkg.go.dev/net#Conn), [`net.Addr`](https://pkg.go.dev/net#Addr)
