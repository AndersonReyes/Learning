# Rust Roadmap

**Status: planning.** Curriculum and reference are settled (below). 29
topics total: 10 Fundamentals + 9 Intermediate + 10 Advanced, plus two
capstones. Fundamentals topic 1,
`fundamentals/01-toolchain-cargo-and-hello-world`, is built — its 5
exercises are "first real Rust programs" algorithmic problems
(`collatz_steps`, `is_prime`, `longest_run`, `caesar_cipher`,
`matrix_transpose`) that only need variables, loops, `if`/`match`, and basic
types, while `notes.md` covers the toolchain/Cargo/Hello-World/Guessing-Game
material conceptually. Everything else is `planned`.

## Reference

Primary references, in the order they're pulled from:

- [**The Rust Programming Language**](https://doc.rust-lang.org/book/) ("the
  Book") — covers Fundamentals (ch. 1-9) and most of Intermediate/Advanced
  (ch. 10-21). Each topic's "Further Reading" links the matching chapter(s)
  at `https://doc.rust-lang.org/book/chXX-...`.
- [**The Rustonomicon**](https://doc.rust-lang.org/nomicon/) ("the Nomicon")
  — unsafe Rust, data layout, advanced ownership/lifetimes, and the
  from-scratch `Vec`/`Arc`/`Mutex` implementations that anchor the back half
  of Advanced.
- [**The Embedonomicon**](https://doc.rust-lang.org/embedonomicon/) and
  [**Discovery (micro:bit v2 edition)**](https://docs.rust-embedded.org/discovery-mb2/)
  — embedded Rust, used for Capstone A.
- [`rust-lang/rust`](https://github.com/rust-lang/rust) — the compiler and
  standard library source itself. Not used as teaching material directly,
  but called out where it's useful to compare a Nomicon's-eye pedagogical
  implementation (e.g. `Vec`, `Arc`) against the real `alloc`/`core` source.
- [`std` docs](https://doc.rust-lang.org/std/) and
  [the Reference](https://doc.rust-lang.org/reference/) — linked per-topic
  for API/grammar details the Book and Nomicon don't cover exhaustively.

Two Nomicon chapters — **FFI** and **Beneath `std`** (`#[panic_handler]`) —
aren't given standalone topics. They're covered hands-on in Capstone A: every
`#![no_std]` program needs a `#[panic_handler]`, and the embedded HALs lean
on `extern "C"` FFI conventions. Likewise Nomicon's **Uninitialized Memory**
and **Ownership Based Resource Management** chapters don't get their own
topics — they're load-bearing material for `advanced/10-implementing-vec-and-arc`,
which *is* an exercise in managing uninitialized memory and resource
lifetimes by hand.

## Per-topic structure

Each topic is its own Cargo package, e.g.
`fundamentals/04-ownership-and-borrowing/`, named
`<tier>-<NN>-<slug>` (e.g. `fundamentals-04-ownership-and-borrowing`) so
`cargo test -p <name>` is unambiguous. Every topic has:

- **`notes.md`** — concept explanation, terse like the other tracks: syntax,
  rules, gotchas, short snippets. Self-contained — covers what you need to do
  the exercises without leaving the page. Ends with a "Further Reading"
  section linking the matching Book/Nomicon chapter(s) and relevant
  `std`/Reference pages.
- **`Cargo.toml`** — `edition = "2021"`, no dependencies unless the topic's
  concept genuinely requires one (async topics need a runtime; the message
  queue capstone needs `tokio`). Fundamentals through most of Intermediate
  are dependency-free.
- **`src/lib.rs`** — 5 exported function/type stubs with full `///` rustdoc
  comments (signature, behavior, hand-verified example I/O). Every stub body
  is `todo!()`.
- **`examples/examples.rs`** — Cargo's built-in examples convention, run via
  `cargo run --example examples -p <name>`. Demonstrates `notes.md`'s
  concepts with `println!`. No exercises here.
- **`tests/exercise_test.rs`** — integration tests (`#[test]` fns +
  `assert_eq!`/`assert!`). This **is** the spec/answer key — there are no
  separate solution files. Imports from the package under its `<name>` (or
  via `<crate_name as snake_case>` — see topic 1 for the exact form).

## Testing strategy

Unlike the C++ track, there's no framework to build: `cargo test` works from
day one. `todo!()` panics, and `cargo test` reports a panicking test as a
clean `FAILED` — so every stub fails its test out of the box, with no
bootstrapping topic needed. This is the big simplification relative to
`cpp/testing.h`.

## Exercise difficulty

Same bar as the other tracks: **hard, challenging algorithmic problems**,
hand-verified before writing `tests/exercise_test.rs` (temporarily implement
a reference solution in `src/lib.rs`, run `cargo test -p <name>` and confirm
every test passes, then revert every function body to `todo!()` and confirm
every test now fails). Rust-specific topics (ownership, lifetimes, traits,
unsafe) should additionally exercise the *language rules* — borrow-checker
edge cases, trait resolution, `Send`/`Sync` boundaries, alignment/layout —
not just "happy path" I/O.

## Adapted topics

A couple of topics narrow `src/lib.rs`'s scope because their subject doesn't
map onto plain testable functions:

- **`intermediate/07-cargo-workspaces-and-profiles`** — workspaces,
  publishing to crates.io, release profiles, and `cargo install` aren't
  things a unit test can assert on. `notes.md` covers all of it
  conceptually (and this track's own `rust/Cargo.toml` *is* a live workspace
  example); `src/lib.rs`'s 5 exercises instead cover ch. 13.4's
  iterator-vs-loop performance material — writing iterator-chain
  implementations and proving them equivalent to hand-rolled loops.
- **`advanced/09-async-await-and-futures`** — there's no async runtime
  anywhere else in the curriculum yet. This topic adds a minimal
  dependency (a small async runtime, e.g. `pollster` or a hand-rolled
  block-on) scoped to just this package, so `src/lib.rs` can have real
  `async fn` exercises without pulling `tokio` into the whole workspace.
  (The message-queue capstone is where `tokio` shows up for real.)

## Adding a new topic

1. Pick the next `planned` topic from the tables below, in order
   (Fundamentals → Intermediate → Advanced), noting its Book/Nomicon
   chapter(s).
2. Write, in this order: `notes.md` → `Cargo.toml` → `src/lib.rs` →
   `tests/exercise_test.rs` → `examples/examples.rs`.
   - If this is the **first topic in a tier** (`intermediate/` or
     `advanced/`), add that tier's glob (e.g. `"intermediate/*"`) to
     `members` in `rust/Cargo.toml` — a glob matching zero directories makes
     `cargo` error out, so each tier's glob can only be added once it has at
     least one member.
3. Verify, from `rust/`:
   - `cargo build -p <name>` succeeds.
   - `cargo test -p <name>` — **every test currently fails** (`todo!()`
     panics), the expected starting state for a learner.
   - `cargo run --example examples -p <name>` runs cleanly and prints
     sensible demonstration output.
   - `cargo test` (no args, from `rust/`) discovers the new package.
4. Update this file: mark the topic `done` and turn its folder cell into a
   link, with a one-paragraph summary of what the 5 exercises cover (see the
   style of `cpp/ROADMAP.md`'s status banner).

## Fundamentals

Covers Book ch. 1-9: the language core, ownership, and the standard
library's basic data structures.

| # | Topic | Folder | Reference | Status |
|---|-------|--------|-----------|--------|
| 1 | Toolchain, Cargo & Hello World | [`fundamentals/01-toolchain-cargo-and-hello-world`](./fundamentals/01-toolchain-cargo-and-hello-world) | Book ch. 1-2 | done |
| 2 | Variables, Data Types & Functions | `fundamentals/02-variables-data-types-and-functions` | Book ch. 3.1-3.3 | planned |
| 3 | Control Flow | `fundamentals/03-control-flow` | Book ch. 3.4-3.5 | planned |
| 4 | Ownership & Borrowing | `fundamentals/04-ownership-and-borrowing` | Book ch. 4.1-4.2 | planned |
| 5 | The Slice Type & `&str` | `fundamentals/05-the-slice-type-and-str` | Book ch. 4.3 | planned |
| 6 | Structs & Methods | `fundamentals/06-structs-and-methods` | Book ch. 5 | planned |
| 7 | Enums & Pattern Matching | `fundamentals/07-enums-and-pattern-matching` | Book ch. 6 | planned |
| 8 | Packages, Crates & Modules | `fundamentals/08-packages-crates-and-modules` | Book ch. 7 | planned |
| 9 | Common Collections (`Vec`, `String`, `HashMap`) | `fundamentals/09-common-collections` | Book ch. 8 | planned |
| 10 | Error Handling (`panic!`, `Result`, `?`) | `fundamentals/10-error-handling` | Book ch. 9 | planned |

## Intermediate

Covers Book ch. 10-16: generics/traits/lifetimes, testing, a real CLI
project, the iterator/closure system in depth, the Cargo ecosystem, smart
pointers, and concurrency.

| # | Topic | Folder | Reference | Status |
|---|-------|--------|-----------|--------|
| 1 | Generics, Traits & Lifetimes | `intermediate/01-generics-traits-and-lifetimes` | Book ch. 10 | planned |
| 2 | Writing Tests & Project Organization | `intermediate/02-testing-and-project-organization` | Book ch. 11 | planned |
| 3 | CLI I/O Project (args, files, env, stderr) | `intermediate/03-cli-io-project` | Book ch. 12 | planned |
| 4 | Closures & Iterators | `intermediate/04-closures-and-iterators` | Book ch. 13.1-13.2 | planned |
| 5 | Custom Iterators & Adapters | `intermediate/05-custom-iterators-and-adapters` | Book ch. 13.2 (deepening) | planned |
| 6 | Error Handling Deep Dive (`From`, `Box<dyn Error>`) | `intermediate/06-error-handling-deep-dive` | Book ch. 9 (deepening) | planned |
| 7 | Cargo Workspaces, Profiles & Performance | `intermediate/07-cargo-workspaces-and-profiles` | Book ch. 13.4, 14 (adapted) | planned |
| 8 | Smart Pointers (`Box`, `Deref`, `Drop`, `Rc`, `RefCell`) | `intermediate/08-smart-pointers` | Book ch. 15 | planned |
| 9 | Fearless Concurrency (threads, channels, `Mutex`/`Arc`) | `intermediate/09-fearless-concurrency` | Book ch. 16 | planned |

## Advanced

Covers Book ch. 17-20 and the Nomicon: trait objects, pattern matching depth,
advanced traits/types/macros, unsafe Rust, data layout, advanced
lifetimes/variance, low-level concurrency primitives, async/await, and a
from-scratch `Vec`/`Arc`/`Mutex`.

| # | Topic | Folder | Reference | Status |
|---|-------|--------|-----------|--------|
| 1 | Trait Objects, Dynamic Dispatch & OOP Patterns | `advanced/01-trait-objects-and-oop-patterns` | Book ch. 18 | planned |
| 2 | Patterns & Matching Deep Dive | `advanced/02-patterns-and-matching` | Book ch. 19 | planned |
| 3 | Advanced Traits & Types | `advanced/03-advanced-traits-and-types` | Book ch. 20.2-20.3 | planned |
| 4 | Advanced Functions, Closures & Macros | `advanced/04-advanced-functions-and-macros` | Book ch. 20.4-20.5 | planned |
| 5 | Unsafe Rust Foundations | `advanced/05-unsafe-rust-foundations` | Book ch. 20.1; Nomicon "Meet Safe and Unsafe", "Working with Unsafe" | planned |
| 6 | Data Layout & Type Conversions | `advanced/06-data-layout-and-type-conversions` | Nomicon "Data Layout", "Type Conversions" | planned |
| 7 | Advanced Lifetimes, Variance & `PhantomData` | `advanced/07-advanced-lifetimes-variance-and-phantomdata` | Nomicon "Ownership" (subtyping, HRTB, `PhantomData`, splitting borrows) | planned |
| 8 | Concurrency Internals: `Send`, `Sync` & Atomics | `advanced/08-concurrency-internals` | Nomicon "Concurrency" (races, `Send`/`Sync`, atomics) | planned |
| 9 | Async/Await & Futures | `advanced/09-async-await-and-futures` | Book ch. 17 (adapted) | planned |
| 10 | Implementing `Vec` and `Arc` from Scratch | `advanced/10-implementing-vec-and-arc` | Nomicon "Implementing Vec", "Implementing Arc and Mutex" | planned |

## Capstones

Both capstones are unlocked once enough of the core curriculum is built (the
embedded capstone's Phase 1 only needs Fundamentals + a little Advanced
unsafe; the message-queue capstone needs Intermediate's concurrency topic at
minimum, and Advanced's async topic for its later phases). Like the C++ ray
tracer and the JS capstone, these are "build it and verify by running"
projects — no pre-written test suite to satisfy (light integration tests
where useful, but the deliverable is a working program).

### Capstone A: Embedded Rust (`rust/capstone-embedded/`)

Targets the **BBC micro:bit v2** (Nordic nRF52833, Cortex-M4F,
`thumbv7em-none-eabihf`) on **real hardware via [`probe-rs`](https://probe.rs/)
end to end — no QEMU/emulation**. Code is written and cross-compiled in this
repo's sandbox (`cargo build --target thumbv7em-none-eabihf` needs no
attached device — already verified to install via `rustup target add
thumbv7em-none-eabihf`); flashing, running, and debugging (`probe-rs
run`/`probe-rs attach`, RTT logging) happens on a machine with the board
plugged in. "Done" for each phase means it builds here *and* runs correctly
on the board, confirmed by the user.

**Phase 1 — `phase1-from-scratch/`**: apply the Embedonomicon's "build a
`#![no_std]` `#![no_main]` program from scratch" approach directly to the
nRF52833 (instead of QEMU's `lm3s6965evb`):

- A custom linker script for the nRF52833's real flash/RAM memory map, a
  hand-written `.vector_table` and reset handler — the smallest possible
  program, blinking an LED via raw memory-mapped GPIO register writes (no
  HAL yet).
- Exception handling: the exception vector table and a default exception
  handler.
- Inline assembly on stable (`asm!`) for the reset/startup sequence.
- A `#[panic_handler]` (Nomicon's "Beneath `std`") and logging over RTT
  (`probe-rs`'s real-time transfer — the on-hardware equivalent of
  Embedonomicon's QEMU semihosting).
- Global singletons (`Mutex<RefCell<Option<T>>>` + `cortex_m::interrupt`),
  the pattern every later interrupt-driven exercise builds on.
- DMA, conceptually (Embedonomicon's DMA chapter).

By the end of Phase 1 you've hand-built the scaffolding that `cortex-m-rt`
and the `microbit-v2` board support crate provide for you from Phase 2 on —
so none of it is "magic".

**Phase 2 — `phase2-discovery/`**: Discovery (micro:bit v2 edition) on real
hardware via `probe-rs`, using `cortex-m-rt` + the `microbit-v2` board
support crate:

- Hello World: toggle an LED via PAC/HAL GPIO registers; spin-wait and timer
  delays; the board support crate.
- LED roulette (the book's first real challenge).
- Polling-based input (buttons) → a "turn signaller".
- Registers: reading the RTRM, type-safe register manipulation, the
  "spooky action at a distance" / volatile-access lessons.
- UART: send/receive a byte, an echo server, reverse-a-string over serial.
- I2C: talk to the LSM303AGR accelerometer/magnetometer; build an LED
  compass and a "punch-o-meter".
- Interrupts: NVIC priorities, debouncing, sharing data with globals
  (building on Phase 1's singleton pattern), the MB2 speaker, PWM.
- Final assembly: the Snake game (LED matrix display, button controls,
  non-blocking rendering).

This phase deliberately follows discovery-mb2's existing
challenge-then-solution structure rather than inventing new exercises — it's
already a well-designed progression for this exact board.

### Capstone B: Distributed Message Queue (`rust/capstone-message-queue/`)

A "mini-Kafka": a single growing project, built in phases, each phase a
runnable milestone (and a thin integration-test layer where it helps, but
the deliverable is the running broker + a client driving it).

1. **Storage engine** — append-only log segments on disk; length-prefixed
   message framing; offset index files; a `Log` type with `append`/`read`.
   Pure Fundamentals/Intermediate Rust: structs, `Result`-based error
   handling, file I/O.
2. **Topics & partitions** — a topic/partition registry on top of the
   storage engine; a producer API (with a partitioning strategy) and a
   consumer API (sequential read by offset). Uses generics, collections,
   `Rc`/`RefCell` or `Arc` for shared registry state.
3. **Concurrency** — multiple producer/consumer threads against shared
   partitions (`Arc<Mutex<...>>`/`RwLock`), a background flush thread.
   Directly applies `intermediate/09-fearless-concurrency` and
   `advanced/08-concurrency-internals`.
4. **Network protocol & async server** — a small binary wire protocol
   (length-prefixed frames) over TCP; an async server (`tokio`) handling
   concurrent produce/fetch/metadata requests. Applies
   `advanced/09-async-await-and-futures`.
5. **Consumer groups & offsets** — consumer group membership, per-group
   committed offsets, basic partition assignment/rebalancing.
6. **Replication & clustering (stretch)** — leader/follower replication
   between broker instances (primary-backup, or a minimal Raft-style leader
   election), plus log compaction.

### Future: Capstone C (cross-referenced from `go/ROADMAP.md`)

`go/ROADMAP.md`'s Capstones section already roadmaps a third future project:
a **TCP/IP stack from scratch in Rust** (CS144/`smoltcp`-style — IP, ARP, TCP
handshake, retransmission, flow control over a TUN device). Not part of this
track's two active capstones, but recorded here as the natural "go all the
way to advanced" follow-on once both capstones above are done.
