# Rust

**Status: planning.** Curriculum is settled (see [`ROADMAP.md`](./ROADMAP.md)).
`fundamentals/01-toolchain-cargo-and-hello-world` is built; see `ROADMAP.md`
for status of the rest.

## How this track works

Same shape as `javascript/`, `go/`, and `cpp/`: numbered topic folders under
`fundamentals/`, `intermediate/`, and `advanced/`. Each topic is its own
Cargo package with `notes.md`, `src/lib.rs` (5 stubbed exercises),
`tests/exercise_test.rs`, and `examples/examples.rs`. `tests/exercise_test.rs`
is the spec/answer key — there are no separate solution files. See
[`ROADMAP.md`](./ROADMAP.md) for the full curriculum, per-topic file pattern,
and "Adding a new topic" workflow.

The curriculum is built around
[**The Rust Programming Language**](https://doc.rust-lang.org/book/) (the
Book) for Fundamentals through most of Intermediate/Advanced, and
[**The Rustonomicon**](https://doc.rust-lang.org/nomicon/) for the unsafe,
data-layout, and from-scratch `Vec`/`Arc`/`Mutex` topics at the end of
Advanced. Each topic's "Further Reading" links the matching chapter(s).

## Building and testing

This is a Cargo workspace — `rust/Cargo.toml` lists every topic as a
workspace member. From `rust/`:

```sh
# Build one topic
cargo build -p fundamentals-01-toolchain-cargo-and-hello-world

# Run a topic's examples
cargo run --example examples -p fundamentals-01-toolchain-cargo-and-hello-world

# Run a topic's exercise tests (spec)
cargo test -p fundamentals-01-toolchain-cargo-and-hello-world

# Run every topic's tests
cargo test
```

No external dependencies for Fundamentals and most of Intermediate — `cargo
test` works out of the box, no framework to build first (unlike the C++
track's `cpp/testing.h`). A handful of later topics add a small,
topic-scoped dependency where the concept genuinely needs one (an async
runtime for `advanced/09`, `tokio` for the message-queue capstone) — see
"Adapted topics" in `ROADMAP.md`.

## Capstones

Two capstones, unlocked progressively as the curriculum builds out — see the
"Capstones" section of [`ROADMAP.md`](./ROADMAP.md):

- **`rust/capstone-embedded/`** — embedded Rust on the BBC micro:bit v2,
  following the Embedonomicon (QEMU-buildable now) and Discovery
  (micro:bit v2 edition, once physical hardware is available).
- **`rust/capstone-message-queue/`** — a "mini-Kafka": a concurrent,
  networked, distributed message queue built in phases (storage engine →
  topics/partitions → concurrency → async network protocol → consumer
  groups → replication).
