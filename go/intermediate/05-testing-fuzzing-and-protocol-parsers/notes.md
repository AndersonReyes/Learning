# Intermediate 5. Testing in Go: Table-Driven Tests, `httptest` & Fuzzing

## Table-driven tests, formalized

Every exercise so far has used this shape:

```go
tests := []struct {
    name    string
    in      InputType
    want    OutputType
    wantErr bool
}{
    {"descriptive case name", input1, want1, false},
    {"error case", badInput, zeroValue, true},
}

for _, tt := range tests {
    t.Run(tt.name, func(t *testing.T) {
        got, err := FunctionUnderTest(tt.in)
        if (err != nil) != tt.wantErr {
            t.Fatalf("error = %v, wantErr %v", err, tt.wantErr)
        }
        if !tt.wantErr && got != tt.want {
            t.Errorf("got %v, want %v", got, tt.want)
        }
    })
}
```

`t.Run` gives each case its own named subtest (`go test -run TestX/case_name`
to isolate one), and a failure in one case doesn't stop the others from
running. This is *the* idiomatic Go testing pattern — not specific to any
one topic.

## `net/http/httptest`, recap

Topics 2 and 4 already used `httptest.NewServer` /
`httptest.NewTLSServer` to spin up a real `net/http` server on an
ephemeral port for testing handlers, proxies, and clients without binding
to a fixed address. Nothing new here — listed because it's part of Go's
core testing toolkit alongside the two techniques below.

## New: fuzzing with `testing.F`

A fuzz test repeatedly calls your code with **mutated inputs** derived from
a seed corpus, looking for inputs that cause a panic, an infinite loop, or
a failed assertion — exactly the kind of bug a hand-written table of cases
is likely to miss, and exactly the risk in a function that parses
**untrusted bytes from the network**.

```go
func FuzzParseValue(f *testing.F) {
    // Seed corpus: known-good and known-tricky inputs.
    f.Add([]byte("+OK\r\n"))
    f.Add([]byte("$-1\r\n"))
    f.Add([]byte("*999999999999\r\n")) // implausible length

    f.Fuzz(func(t *testing.T, data []byte) {
        v, err := ParseValue(bufio.NewReader(bytes.NewReader(data)))
        if err != nil {
            return // a parse error is fine; a panic is not
        }
        // ...assert some invariant about a successfully-parsed v...
    })
}
```

- `go test` runs the seed corpus (from `f.Add`) plus any saved corpus
  entries as regular test cases — no special flag needed.
- `go test -fuzz=FuzzParseValue -fuzztime=30s` runs the **fuzzing engine**:
  it mutates the corpus and looks for crashes for 30 seconds.
- A crashing input is written to `testdata/fuzz/FuzzParseValue/<hash>` and
  becomes a permanent regression test — `go test` will replay it forever
  after.

The single most important property a fuzz test can check for a parser:
**it never panics**, regardless of input. The next section covers the most
common way a length-prefixed parser violates that.

### The classic parser bug fuzzing finds: unbounded allocation

A length-prefixed format (this topic's RESP bulk strings, topic 9's framing,
topic 12's DNS records) reads a length field, then does
`make([]byte, length)`. If `length` comes straight from untrusted input with
no upper bound, an attacker (or the fuzzer) sends `$999999999999999\r\n` and
the allocation either fails immediately (`panic: runtime: makeslice: cap
out of range`) or, on a number just small enough to allocate, exhausts
memory. **Always validate a wire-supplied length against a sane maximum
before allocating** — this exercise's `ParseValue` rejects bulk strings over
512 MiB and arrays over 2^20 elements, matching Redis's own
`proto-max-bulk-len` default.

---

## Networking: RESP (REdis Serialization Protocol)

RESP is the wire protocol Redis (and Redis-compatible servers/proxies)
speak. It's a small, recursive, text-prefixed-with-binary-payloads format —
simple enough to fully implement in one exercise, real enough that this
*is*, in miniature, a Redis client/server codec.

Every value starts with a one-byte type, and ends with `\r\n`:

| Type | Byte | Format | Example |
|---|---|---|---|
| Simple String | `+` | `+<string>\r\n` | `+OK\r\n` |
| Error | `-` | `-<message>\r\n` | `-WRONGTYPE wrong kind of value\r\n` |
| Integer | `:` | `:<number>\r\n` | `:1000\r\n` |
| Bulk String | `$` | `$<len>\r\n<len bytes>\r\n` | `$6\r\nfoobar\r\n` |
| Null Bulk String | `$` | `$-1\r\n` | `$-1\r\n` |
| Array | `*` | `*<count>\r\n<count values>` | `*2\r\n$3\r\nfoo\r\n$3\r\nbar\r\n` |
| Null Array | `*` | `*-1\r\n` | `*-1\r\n` |

**Bulk strings are length-prefixed, not delimiter-terminated** — the
payload can contain arbitrary bytes including `\r\n`, which is why Redis
values can hold binary data. **Arrays are recursive**: each of the `count`
elements is itself a RESP value of any type (including nested arrays),
parsed the same way `ParseValue` parses the top-level value.

### The command format

A RESP client sends every command as an **array of bulk strings** — e.g.
`SET key value` is sent as
`*3\r\n$3\r\nSET\r\n$3\r\nkey\r\n$5\r\nvalue\r\n`. `EncodeCommand` /
`ParseCommand` produce and consume exactly this shape, which is how a real
Redis server reads client requests from its `net.Conn` (topic 8).

## Further Reading

- [`testing`](https://pkg.go.dev/testing) — `T.Run`, `F.Add`, `F.Fuzz`
- [Go Fuzzing tutorial](https://go.dev/doc/tutorial/fuzz)
- [`net/http/httptest`](https://pkg.go.dev/net/http/httptest)
- [RESP protocol specification](https://redis.io/docs/latest/develop/reference/protocol-spec/)
