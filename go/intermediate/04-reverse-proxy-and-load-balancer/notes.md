# Intermediate 4. Reverse Proxy / Load Balancer

## `sync/atomic`: lock-free counters and flags

Topic 7 used `sync.Mutex` to protect shared state. For simple values — a
counter or a boolean flag — `sync/atomic`'s typed wrappers (`atomic.Bool`,
`atomic.Uint64`, `atomic.Int64`, ...) give atomic reads/writes without a
mutex:

```go
var alive atomic.Bool
alive.Store(true)       // write
ok := alive.Load()      // read

var counter atomic.Uint64
n := counter.Add(1)     // increment and return the new value
```

These types are zero-value ready (no constructor needed) and safe for
concurrent use. `Add` returns the *new* value, which is the standard way to
turn a shared counter into a round-robin index: `(counter.Add(1) - 1) % n`
gives 0, 1, 2, ..., n-1, 0, 1, ... across concurrent callers.

A mutex is still the right tool when multiple related fields must change
together atomically; `sync/atomic` is for a single independent value.

## New stdlib: `net/http/httputil.ReverseProxy`

[`httputil.NewSingleHostReverseProxy(target)`](https://pkg.go.dev/net/http/httputil#NewSingleHostReverseProxy)
returns an `http.Handler` (topic 5's interface, topic 10's `net/http`) that
forwards every request it receives to `target`, copying the response back
to the original client. Two fields customize its behavior:

- **`Director func(*http.Request)`** — mutates the outgoing request before
  it's sent (rewriting the URL, adding headers like `X-Forwarded-For`).
  `NewSingleHostReverseProxy` sets a default `Director` that rewrites the
  scheme/host/path to `target`'s.
- **`ErrorHandler func(http.ResponseWriter, *http.Request, error)`** — called
  if the round trip to the backend fails (connection refused, timeout).
  Without it, a failed backend produces a generic `502 Bad Gateway` with no
  control over the response; setting it lets you customize the status/body
  (or, in a load balancer, retry against a different backend).

---

## Networking: reverse proxies and load balancing

A **reverse proxy** sits in front of one or more backend servers and
forwards client requests to them, returning the backend's response as if it
came from the proxy itself. Clients only ever talk to the proxy — the
backends' addresses are hidden. This is the opposite of a *forward* proxy
(which sits in front of clients, forwarding their requests to the wider
internet).

### Why put a proxy in front of your servers?

- **Load balancing**: spread requests across multiple backend instances.
- **TLS termination**: the proxy holds the certificate (topic 2's
  `crypto/tls`); backends speak plain HTTP on a private network.
- **Health checking**: stop sending traffic to backends that are down,
  without clients noticing.

### Round-robin load balancing

The simplest load-balancing algorithm: maintain a list of backends and an
ever-incrementing counter; each request goes to
`backends[counter % len(backends)]`. Using `atomic.Uint64` for the counter
(above) makes this safe under concurrent requests without a mutex. A
production load balancer also skips backends marked unhealthy — this
exercise's `NextBackend` does both: round-robin *and* skip dead backends.

### Active health checks

A load balancer periodically probes each backend (e.g. `GET /health`) on a
timer (topic 7's `time.Ticker` pattern) and marks it alive/dead based on the
response. Requests are then only routed to backends currently marked alive.
This exercise's `HealthCheck` performs one probe; `RunHealthChecks` runs it
for every backend and updates each `Backend`'s `atomic.Bool` alive flag —
the same flag `NextBackend` reads when choosing a backend, with no lock
needed on either side.

## Further Reading

- [`sync/atomic`](https://pkg.go.dev/sync/atomic)
- [`net/http/httputil`](https://pkg.go.dev/net/http/httputil)
- [`net/http/httputil.ReverseProxy`](https://pkg.go.dev/net/http/httputil#ReverseProxy)
- [`net/http/httptest`](https://pkg.go.dev/net/http/httptest)
