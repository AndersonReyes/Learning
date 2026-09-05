# Go

A self-directed Go research track for experienced engineers. The roadmap focuses
on Go's language semantics, runtime, standard library, concurrency model,
toolchain, and idioms. Capstones provide the implementation environment while
you research.

The module currently targets Go 1.24. The living Go specification may describe
newer language versions, so check version markers and release notes before using
newer features.

## How it is organized

```text
go/
├── README.md
├── ROADMAP.md
├── go.mod
├── capstone-container-runtime/
└── capstone-message-queue/
```

- [`ROADMAP.md`](./ROADMAP.md) defines ordered research phases, topic
  boundaries, completion criteria, and authoritative sources.
- Capstones are long-running integration projects. They may be started after the
  first phase and expanded throughout the roadmap.
- There are no per-topic exercises, generated solutions, or required lesson
  implementations.

## How to use the roadmap

1. Choose the next topic in the current phase.
2. Begin with the linked Go specification, standard-library documentation, or
   official Go project material.
3. Create a minimal experiment only when it helps resolve a language or API
   boundary.
4. Apply the concept to an active capstone when it fits naturally.
5. Record conclusions and tradeoffs in your own words.
6. Use the phase completion criteria to decide when to continue.

The roadmap is not coupled to a networking curriculum. Networking protocols,
storage engines, distributed algorithms, and operating-system mechanisms belong
inside capstones or dedicated subject modules. Go-specific APIs such as
`net/http`, `context`, `io`, and `database/sql` remain part of the
language track.

## Capstone-driven learning

Capstones replace isolated topic exercises. They integrate language features,
standard-library contracts, concurrency, runtime behavior, tooling, and system
design in one evolving codebase.

Current capstones:

- [Distributed message queue](./capstone-message-queue/)
- [Linux container runtime](./capstone-container-runtime/)

A capstone can evolve in parallel with the roadmap:

- Early phases establish types, errors, packages, and ownership.
- Standard-library phases add I/O, encoding, networking, and persistence.
- Concurrency phases add cancellation, bounded work, and shutdown.
- Runtime and tooling phases add testing, fuzzing, race detection, profiling,
  and diagnostics.
- The idioms phase refactors abstractions after concrete use reveals what is
  needed.

Capstone implementation is learner-owned. Research and implement incrementally;
do not wait until every roadmap topic is complete.

## Scope

This track covers Go-specific behavior and APIs. It does not teach networking
protocols, routing algorithms, consensus, caching architecture, or other general
system-design topics as language lessons. Those subjects may still appear as
capstone requirements.

See [`ROADMAP.md`](./ROADMAP.md) to begin.
