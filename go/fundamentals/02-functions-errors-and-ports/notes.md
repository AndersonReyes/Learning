# 02. Functions, Errors & Ports

## Multiple & named return values

```go
func divmod(a, b int) (int, int) {
	return a / b, a % b
}
q, r := divmod(17, 5) // q=3, r=2

// named returns: declared in the signature, "naked" return uses their
// current values. Useful for documenting what each return value means.
func parsePort(s string) (port uint16, err error) {
	// ...
	return // returns current values of port and err
}
```

Named returns are most useful for documentation and for `defer`-based
cleanup (later topics). Don't overuse them — for short functions, explicit
`return x, err` is clearer.

## Variadic functions

```go
func sum(nums ...int) int { // nums is []int inside the function
	total := 0
	for _, n := range nums {
		total += n
	}
	return total
}

sum(1, 2, 3)       // 6
sum()              // 0 — zero args is valid
nums := []int{1, 2, 3}
sum(nums...)       // spread a slice into variadic args
```

A variadic parameter must be **last**, and there can only be one.

## Custom error types

`error` is just an interface (`Error() string`). Define your own type when
callers need to extract structured info (not just a message):

```go
type OutOfRangeError struct {
	Value int64
}

func (e *OutOfRangeError) Error() string {
	return fmt.Sprintf("value %d out of range", e.Value)
}

// usage:
return 0, &OutOfRangeError{Value: v} // pointer receiver -> return a pointer
```

## Sentinel errors, wrapping, `errors.Is` / `errors.As`

```go
var ErrUnknownService = errors.New("unknown service")

// wrap with %w to preserve the original error for errors.Is/As, while
// adding context:
return Service{}, fmt.Errorf("lookup %q: %w", name, ErrUnknownService)

// caller:
if errors.Is(err, ErrUnknownService) { /* ... */ }

var rangeErr *OutOfRangeError
if errors.As(err, &rangeErr) {
	fmt.Println(rangeErr.Value) // structured access
}
```

- `errors.Is(err, target)` — "does this error chain contain `target`?"
  (sentinel comparison, follows `%w` wrapping).
- `errors.As(err, &target)` — "does this error chain contain a value
  assignable to `*target`'s type?" (type assertion through wrapping).
- A plain `fmt.Errorf("...: %v", err)` does NOT preserve the chain — use
  `%w` when callers might need `errors.Is`/`errors.As`.

## `sort.Slice` and maps (used in this topic's exercises)

```go
sort.Slice(s, func(i, j int) bool {
	return s[i][0] < s[j][0] // sort by first element
})

services := map[string]Service{"http": {Port: 80}}
strings.ToLower("HTTP") // "http" — case-insensitive map keys
```

---

## Networking: transport layer & ports

**TCP** ([RFC 793](https://www.rfc-editor.org/rfc/rfc793)) — connection-oriented,
reliable, ordered byte stream (handshake, retransmission, flow control).
**UDP** ([RFC 768](https://www.rfc-editor.org/rfc/rfc768)) — connectionless,
best-effort datagrams, minimal header overhead (8 bytes vs TCP's 20+). Same
port-number space is used by both, but a "TCP port 53" and "UDP port 53" are
independent.

### Ports: a 16-bit namespace (0-65535)

[RFC 6335](https://www.rfc-editor.org/rfc/rfc6335) divides the port space:

| Range | Name |
|-------|------|
| 0-1023 | Well-known (system) ports |
| 1024-49151 | Registered ports |
| 49152-65535 | Dynamic/private (ephemeral) ports |

A **socket** is the tuple `(IP address, port, protocol)` — what a connection
is actually identified by.

### Well-known services (used by `LookupService`)

| Service | Port | Transport |
|---------|------|-----------|
| ftp | 21 | tcp |
| ssh | 22 | tcp |
| telnet | 23 | tcp |
| smtp | 25 | tcp |
| dns | 53 | udp |
| http | 80 | tcp |
| ntp | 123 | udp |
| https | 443 | tcp |

## Further Reading

- [Effective Go: Multiple returns](https://go.dev/doc/effective_go#multiple-returns)
- [Effective Go: Named results](https://go.dev/doc/effective_go#named-results)
- [Effective Go: Errors](https://go.dev/doc/effective_go#errors)
- [`errors` package](https://pkg.go.dev/errors)
- [`sort` package](https://pkg.go.dev/sort)
- [RFC 793 — Transmission Control Protocol](https://www.rfc-editor.org/rfc/rfc793)
- [RFC 768 — User Datagram Protocol](https://www.rfc-editor.org/rfc/rfc768)
- [RFC 6335 — IANA Port Number Registry Procedures](https://www.rfc-editor.org/rfc/rfc6335)
