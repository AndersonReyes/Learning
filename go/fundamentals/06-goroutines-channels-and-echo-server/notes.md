# 06. Goroutines & Channels

## Goroutines

`go f(args)` starts `f` running concurrently in a new goroutine — a
lightweight thread managed by the Go runtime, not the OS. The call returns
immediately; `f` runs independently.

```go
go process(data) // runs concurrently
fmt.Println("started") // may print before or after process starts
```

**Gotcha:** when `main` returns, the program exits immediately — any
goroutines still running are killed mid-execution, with no cleanup. A
program that starts goroutines must have some way to wait for them (a
channel, in this topic; `sync.WaitGroup`/`context` come in topic 7).

## Channels

A channel is a typed conduit for communication between goroutines.

```go
ch := make(chan int)    // unbuffered
ch := make(chan int, 5) // buffered, capacity 5

ch <- 42        // send
v := <-ch       // receive
v, ok := <-ch   // ok == false if ch is closed AND drained
close(ch)       // only the sender should close a channel
```

- **Unbuffered** (`make(chan T)`): a send blocks until another goroutine
  receives, and vice versa — this is a synchronization point ("rendezvous"),
  not just a queue.
- **Buffered** (`make(chan T, n)`): a send blocks only when the buffer is
  full (`n` elements already queued); a receive blocks only when the buffer
  is empty.
- **`range ch`** receives values until `ch` is closed and drained, then
  exits the loop automatically:
  ```go
  for v := range ch {
      fmt.Println(v)
  }
  ```

### Closing rules (panics if violated)

- Sending on a closed channel **panics**.
- Closing an already-closed channel **panics**.
- Closing a `nil` channel **panics**.
- Receiving from a closed channel returns the zero value immediately
  (`v, ok := <-ch` gives `ok == false`); receiving from a closed *and
  empty* channel never blocks.
- Receiving from (or sending to) a `nil` channel blocks **forever** — this
  is a common source of accidental deadlocks, not a panic.

### Directional channel types

Function signatures should narrow a channel parameter to the direction
it's actually used in:

```go
func produce(out chan<- int)   // send-only: this func may only send to out
func consume(in <-chan int)    // receive-only: this func may only receive from in
```

A bidirectional `chan T` value can be passed where `chan<- T` or `<-chan T`
is expected (implicit conversion), but not the reverse. This documents
intent and lets the compiler catch misuse (e.g. accidentally closing a
channel a function should only read from).

## Deadlocks

If every goroutine is blocked on a channel operation that nothing will ever
unblock, the Go runtime detects it and crashes with `fatal error: all
goroutines are asleep - deadlock!`. Common causes:

- Sending on an unbuffered channel with no corresponding receiver (and
  vice versa).
- Forgetting to `close` a channel that something else `range`s over —
  the range loop blocks forever waiting for either a value or a close.

## Concurrency patterns used in this topic's exercise

- **One goroutine per connection**: spawn a goroutine to handle each
  independent unit of work (here, a simulated connection); they run
  concurrently and don't block each other.
- **Fan-in**: merge several channels into one by spawning a forwarding
  goroutine per input channel, all writing to a shared output channel.
- **Fan-out / worker pool**: spawn N goroutines that all read from the
  same channel of jobs, so work is distributed across them.
- **Pipeline**: chain stages together, each stage a goroutine reading
  from one channel and writing to the next; data flows through the chain
  while every stage runs concurrently.
- **Completion signaling without `sync`** (preview of topic 7, done with
  plain channels here): a worker goroutine sends a value (or closes a
  channel) when it finishes, and a coordinator goroutine receives one such
  signal per worker before proceeding.

---

## Networking: the "one goroutine per connection" server model

[RFC 862](https://www.rfc-editor.org/rfc/rfc862) defines the **Echo
Protocol**: a server that sends back to the client exactly the data it
receives, byte for byte, until the connection closes.

The idiomatic Go TCP server structure (covered fully with real sockets in
topic 8) is:

```go
for {
    conn, err := listener.Accept()
    if err != nil {
        // handle error
        continue
    }
    go handleConn(conn) // one goroutine per connection
}
```

Each `handleConn` runs independently and concurrently with every other
connection's handler — a slow or stuck client doesn't block any other
client. This topic's exercise models that shape using channels in place of
`net.Conn`: a simulated connection is a pair of channels (`In` for data
arriving from the client, `Out` for data the handler writes back), and an
"echo handler" goroutine reads from `In` and writes the same bytes to
`Out` until `In` is closed — exactly RFC 862's behavior, minus the
sockets.

## Further Reading

- [Tour: Goroutines](https://go.dev/tour/concurrency/1)
- [Tour: Channels](https://go.dev/tour/concurrency/2)
- [Tour: Buffered channels](https://go.dev/tour/concurrency/3)
- [Tour: Range and close](https://go.dev/tour/concurrency/4)
- [Effective Go: Concurrency](https://go.dev/doc/effective_go#concurrency)
- [RFC 862 (Echo Protocol)](https://www.rfc-editor.org/rfc/rfc862)
