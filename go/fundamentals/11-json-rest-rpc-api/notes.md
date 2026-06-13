# 11. `encoding/json` + JSON REST/RPC API

## `json.Marshal`/`json.Unmarshal` and struct tags

```go
type Item struct {
    Name  string `json:"name"`
    Price int    `json:"price,omitempty"`
}

data, err := json.Marshal(Item{Name: "widget", Price: 0})
// data = `{"name":"widget"}` — omitempty drops the zero-value Price

var got Item
err = json.Unmarshal(data, &got)
```

- A struct tag `json:"name"` controls the field's JSON key; `,omitempty`
  drops the field entirely when it holds its zero value; `json:"-"` skips
  the field entirely.
- `json.Marshal` only encodes **exported** fields (capitalized names) —
  unexported fields are silently skipped.
- `json.Unmarshal` ignores JSON object keys with no matching struct field
  by default; `json.NewDecoder(r).DisallowUnknownFields()` makes that an
  error instead.

## `json.RawMessage`: deferred/raw JSON

`json.RawMessage` is `type RawMessage []byte` that implements
`json.Marshaler`/`json.Unmarshaler` as a **no-op** — it stores the exact
JSON bytes without parsing them:

```go
type Envelope struct {
    Kind string          `json:"kind"`
    Data json.RawMessage `json:"data"`
}
```

This is the standard way to handle JSON whose *shape* depends on another
field (`Kind` here) — decode the envelope first, then `json.Unmarshal` the
`Data` bytes into the right type once you know `Kind`. It's also useful for
"store and forward" cases: a server that holds arbitrary JSON values
without ever needing to know their structure (this topic's key/value
store).

`json.Valid(data []byte) bool` checks whether `data` is well-formed JSON
without fully parsing it — useful for validating a `json.RawMessage` before
storing it.

## `json.NewEncoder`/`json.NewDecoder`: streaming JSON over `io`

Topics 9 and 10 read/wrote raw bytes from `io.Reader`/`io.Writer`.
`encoding/json` has streaming equivalents that work directly with HTTP
bodies (`http.Request.Body`, `http.ResponseWriter`):

```go
var v MyType
if err := json.NewDecoder(r.Body).Decode(&v); err != nil {
    http.Error(w, err.Error(), http.StatusBadRequest)
    return
}

w.Header().Set("Content-Type", "application/json")
json.NewEncoder(w).Encode(result)
```

`Decode`/`Encode` avoid buffering the entire body in memory as a
`[]byte` the way `json.Unmarshal`/`json.Marshal` do — relevant for large
request/response bodies.

---

## Networking: JSON REST and RPC APIs over HTTP

[RFC 8259](https://www.rfc-editor.org/rfc/rfc8259) defines the JSON text
format itself; it says nothing about HTTP. Two common ways to expose JSON
over the HTTP server from topic 10:

**REST**: resources are identified by URL path, and the HTTP method
encodes the operation:

| Method | Path | Meaning | Typical status |
|---|---|---|---|
| `GET` | `/items/{key}` | fetch | `200` + JSON body, or `404` |
| `PUT` | `/items/{key}` | create/replace | `204` (no body), or `400` for invalid JSON |
| `DELETE` | `/items/{key}` | remove | `204`, or `404` |
| other | `/items/{key}` | — | `405 Method Not Allowed` |

**RPC**: every request goes to one endpoint with a method name and
parameters in the body — e.g. a JSON-RPC-style envelope:

```json
{"method": "add", "params": [1, 2, 3], "id": 1}
```

with a response:

```json
{"result": 6, "id": 1}
```

or, on error, `{"error": "...", "id": 1}` instead of `result`. Because
different methods take different `params` shapes, `params` (and `result`)
are naturally `json.RawMessage`: the dispatcher decodes the envelope, looks
up the method by name, and only that method's handler knows how to decode
its own `params`.

This topic's exercise builds both: a concurrency-safe `Store` of
`json.RawMessage` values (mutex pattern from topic 7) exposed as a REST API
via `ServeHTTP` (an `http.Handler`, topic 10), and a `HandleRPC` dispatcher
for the JSON-RPC-style pattern above.

## Further Reading

- [`encoding/json`](https://pkg.go.dev/encoding/json)
- [`json.Marshal`](https://pkg.go.dev/encoding/json#Marshal), [`json.Unmarshal`](https://pkg.go.dev/encoding/json#Unmarshal)
- [`json.RawMessage`](https://pkg.go.dev/encoding/json#RawMessage), [`json.Valid`](https://pkg.go.dev/encoding/json#Valid)
- [`json.NewEncoder`](https://pkg.go.dev/encoding/json#NewEncoder), [`json.NewDecoder`](https://pkg.go.dev/encoding/json#NewDecoder)
- [RFC 8259 (JSON)](https://www.rfc-editor.org/rfc/rfc8259)
