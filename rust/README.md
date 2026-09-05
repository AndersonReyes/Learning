# Rust

A self-directed Rust research track for experienced engineers. The roadmap
focuses on Rust's ownership and borrowing model, type system, standard library,
concurrency and async model, unsafe boundary, compilation model, tooling, and
idiomatic API design.

This roadmap targets Rust Edition 2024. It does not pin a compiler release;
check stabilization and version markers in current documentation before
relying on newer behavior.

## How it is organized

```text
rust/
├── README.md
└── ROADMAP.md
```

- [`ROADMAP.md`](./ROADMAP.md) defines ordered research phases, topic
  boundaries, completion criteria, authoritative sources, and capstone ideas.
- There are no generated notes, per-topic packages, exercises, or answer keys.
- Capstones replace isolated exercises and may evolve while related topics are
  being researched.

## Suggested workflow

1. Choose the next topic whose prerequisites you understand.
2. Begin with the Rust Reference or standard-library contract, then use the
   Book, Edition Guide, Cargo Book, rustc documentation, or Rustonomicon for
   context.
3. Write minimal probes for ownership, inference, layout, concurrency, or
   compiler behavior when reading alone is insufficient.
4. Record the rules, guarantees, unspecified behavior, tradeoffs, and failure
   modes in your own words.
5. Apply the topic to an active capstone when it belongs there naturally.

## Track boundaries

This track covers Rust-specific language semantics, libraries, runtime
behavior, tooling, and idioms. General networking, distributed-systems,
embedded-systems, and compiler theory belong in dedicated modules or
capstones. Their Rust-specific consequences remain in scope.

## Capstone-driven learning

Candidate capstones are described in the roadmap:

- Embedded Rust on the BBC micro:bit v2
- A user-space TCP/IP stack
- A small interpreted or compiled language implemented in Rust

Capstones are learner-owned. Start with a narrow vertical slice, extend it as
the roadmap unlocks new techniques, and document ownership, failure,
concurrency, unsafe invariants, and verification decisions as they emerge.
