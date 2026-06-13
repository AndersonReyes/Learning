# 07. `select`, `sync` & `context`

## `select`

`select` waits on multiple channel operations at once and proceeds with
whichever is ready first:

```go
select {
case v := <-ch1:
    fmt.Println("from ch1:", v)
case ch2 <- 42:
    fmt.Println("sent to ch2")
case <-ctx.Done():
    fmt.Println("canceled")
default:
    fmt.Println("nothing ready right now")
}
```

- If multiple cases are ready simultaneously, `select` picks one **at
  random** — don't rely on case order for priority.
- `default` makes the whole `select` non-blocking: if no other case is
  ready immediately, `default` runs instead of waiting.
- With no `default` and no case ever becoming ready, `select` blocks
  forever (same deadlock rules as a plain channel op).
- `select {}` (no cases) blocks forever — sometimes used deliberately to
  park `main` while goroutines run.

### Timeouts with `select` + `time.After`

```go
select {
case result := <-resultCh:
    fmt.Println("got result:", result)
case <-time.After(2 * time.Second):
    fmt.Println("timed out")
}
```

`time.After(d)` returns a channel that receives once, after `d` elapses.
Racing it against a result channel is the standard way to bound how long
an operation can take.

## `sync.Mutex` / `sync.RWMutex`

A `sync.Mutex` protects shared state from concurrent access. Its zero
value is an unlocked mutex — no initialization needed.

```go
type Stats struct {
    mu     sync.Mutex
    counts map[string]int
}

func (s *Stats) Record(key string) {
    s.mu.Lock()
    defer s.mu.Unlock()
    s.counts[key]++
}
```

- `Lock`/`Unlock` must be paired; `defer s.mu.Unlock()` right after `Lock`
  ensures it happens even on early return or panic.
- Without the mutex, concurrent map writes (`s.counts[key]++` from
  multiple goroutines) are a **data race** — undefined behavior, caught by
  `go test -race`.
- `sync.RWMutex` adds `RLock`/`RUnlock` for concurrent readers when no
  writer holds the lock — use it when reads vastly outnumber writes.
- A struct embedding a mutex should generally not be copied after first
  use (copying duplicates the lock state).

## `sync.WaitGroup`

Waits for a collection of goroutines to finish — the `sync`-based
alternative to topic 6's channel-counting pattern:

```go
var wg sync.WaitGroup
for _, task := range tasks {
    wg.Add(1)
    go func(task func()) {
        defer wg.Done()
        task()
    }(task)
}
wg.Wait() // blocks until every Done() has been called
```

- Call `Add` **before** starting the goroutine (not inside it) — otherwise
  `Wait` can race past a goroutine that hasn't called `Add` yet.
- `Done` is `Add(-1)`; mismatched counts (`Wait` returns before all `Done`s,
  or more `Done`s than `Add`s) panic or deadlock.
- A `sync.WaitGroup`'s zero value is ready to use.

## `context.Context`

`context.Context` carries cancellation signals (and deadlines, and
request-scoped values) across API boundaries and between goroutines.

```go
ctx, cancel := context.WithCancel(context.Background())
defer cancel() // always call cancel to release resources, even on success

ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
defer cancel()
```

- `ctx.Done()` returns a channel that's closed when the context is
  canceled or its deadline passes.
- `ctx.Err()` returns `nil` while `ctx` is active, `context.Canceled` after
  `cancel()`, or `context.DeadlineExceeded` after a timeout/deadline.
- Canceling a parent context cancels all contexts derived from it.
- Convention: `ctx` is the **first parameter** of a function
  (`func F(ctx context.Context, ...)`), never stored in a struct field.

### Combining `select` + `context` + channels

```go
select {
case v, ok := <-ch:
    if !ok {
        return // ch closed
    }
    use(v)
case <-ctx.Done():
    return ctx.Err() // canceled or timed out
}
```

This is the core pattern for making any channel-based operation
cancelable: race the real operation against `ctx.Done()`.

---

## Networking: connection timeouts & cancellation

[`net.Conn`](https://pkg.go.dev/net#Conn) (topic 5) has three deadline
methods:

```go
SetDeadline(t time.Time) error
SetReadDeadline(t time.Time) error
SetWriteDeadline(t time.Time) error
```

Once a deadline passes, **any** in-progress or future `Read`/`Write` on
that connection fails with a timeout error (one whose `Timeout() bool`
method returns `true`) — the connection itself remains usable for a fresh
deadline. This is the socket-level mechanism for "don't let this
connection's I/O block forever."

`context.Context` is the higher-level, idiomatic mechanism layered on top:
`net.Dialer.DialContext`, `http.NewRequestWithContext`, and most modern
networking APIs accept a `ctx` so a caller can cancel an in-flight
connection attempt or request — e.g. because the overall request's
deadline passed, or the user navigated away. This topic's exercise applies
`select`, `sync`, and `context` to model both: racing an operation against
a timeout (`SetDeadline`-style), and propagating cancellation through a
fan-in of channels (`context`-style).

## Further Reading

- [Tour: select](https://go.dev/tour/concurrency/5)
- [Tour: sync.Mutex](https://go.dev/tour/concurrency/9)
- [`sync.WaitGroup`](https://pkg.go.dev/sync#WaitGroup)
- [`sync.Mutex`](https://pkg.go.dev/sync#Mutex), [`sync.RWMutex`](https://pkg.go.dev/sync#RWMutex)
- [`context`](https://pkg.go.dev/context)
- [`net.Conn.SetDeadline`](https://pkg.go.dev/net#Conn.SetDeadline)
