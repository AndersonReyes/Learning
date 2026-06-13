# Go

This directory is a self-contained Go learning track, taught **through
computer networking**: notes, runnable examples, and test-driven exercises.
Every topic pairs a Go language concept with a networking concept, and the
exercises build toward real protocol/network tooling. No external
dependencies — everything uses the Go standard library.

## How it's organized

```
go/
  ROADMAP.md             # full curriculum: Go + networking, fundamentals -> advanced
  go.mod
  fundamentals/
    01-go-basics-and-ip-addressing/
      notes.md           # Go concepts + networking concepts, with go.dev/RFC links
      exercise.go        # function stubs for you to implement
      exercise_test.go   # tests that define the expected behavior
      examples/
        main.go          # runnable demo code
    02-.../
    ...
```

Each topic folder follows the same pattern.

## How to work through a topic

1. Read `notes.md` for the concept explanation and gotchas.
2. Run the examples to see the concepts in action:
   ```
   go run ./fundamentals/01-go-basics-and-ip-addressing/examples
   ```
3. Open `exercise.go`. Each exported function has a doc comment describing
   what it should do, and a stub body that returns a zero value / a
   `"not implemented"` error.
4. Open `exercise_test.go` to see the test cases — these define exactly what
   each function must do. There are no separate solution files; the tests
   *are* the spec.
5. Implement the functions in `exercise.go` until the tests pass:
   ```
   go test ./fundamentals/01-go-basics-and-ip-addressing/...
   ```
   Or run every topic's tests at once from this directory:
   ```
   go test ./...
   ```

All exercises start failing — that's expected. Work through them in order,
since later topics build on earlier ones.

## Roadmap

See [`ROADMAP.md`](./ROADMAP.md) for the full curriculum, including
intermediate and advanced topics planned for later, and the future Rust
TCP/IP-stack capstone.
