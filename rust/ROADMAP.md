# Rust Roadmap

A self-directed research roadmap for an experienced senior engineer. Use the
Rust Reference and standard-library API as the primary contracts, supported by
the Book, Edition Guide, Cargo Book, rustc documentation, Rustonomicon, and
other primary Rust project documentation.

The baseline is Rust Edition 2024. Current documentation may describe behavior
from newer compiler releases, so distinguish stable language guarantees,
edition-specific behavior, library contracts, compiler behavior, and ecosystem
conventions.

## Phase 1 — Language, Ownership, and Data Model

| # | Topic | Research target |
| --- | --- | --- |
| 1 | Toolchain, editions, and compilation model | Roles of `rustup`, `rustc`, and Cargo; stable, beta, and nightly channels; editions versus compiler versions; crate roots and targets; the prelude; feature stability; edition migration; and cross-edition interoperability. |
| 2 | Bindings, mutability, types, and inference | `let`, patterns, shadowing, mutability, scalar and compound types, inference, coercions, casts, numeric overflow, unit, never, diverging expressions, and type ascription boundaries. |
| 3 | Expressions, control flow, and patterns | Expression-oriented blocks, statements and semicolons, `if`, loops, labels, `match`, guards, or-patterns, destructuring, refutability, exhaustiveness, `if let`, `while let`, and `let else`. |
| 4 | Ownership, moves, copying, cloning, and destruction | Places and values, move paths, partial moves, `Copy`, `Clone`, destructors, drop order, temporaries, assignment, argument passing, return values, and closure capture consequences. |
| 5 | Borrowing, references, and reborrowing | Shared and mutable references, exclusivity, reborrowing, non-lexical lifetimes, temporary lifetime extension, dereference coercion, two-phase borrows, disjoint fields, indexing, and common borrow-checker diagnostics. |
| 6 | Slices, strings, and borrowed views | Arrays and slices, `str` and `String`, UTF-8, bytes versus Unicode scalar values and grapheme clusters, indexing restrictions, ranges, `OsStr`/`OsString`, `Path`/`PathBuf`, and ownership choices at API boundaries. |
| 7 | Structs, enums, methods, and associated items | Product and sum types, field and tuple syntax, inherent implementations, associated functions and constants, constructors by convention, newtypes, recursive types, `Option`, discriminants, and layout guarantees versus optimizations. |
| 8 | Functions, closures, and callable types | Function items and pointers, closure capture modes, `move`, `Fn`/`FnMut`/`FnOnce`, inference, generic callables, returned closures, higher-order APIs, ABI distinctions, and lifetime effects. |
| 9 | Errors, `Option`, `Result`, and panics | Recoverable versus unrecoverable failure, combinators, `?`, residual conversion, `From`/`TryFrom`, error sources, context, panic hooks, unwinding versus abort, double panics, and failure contracts. |
| 10 | Modules, crates, packages, and visibility | Module trees, paths, `use`, re-exports, privacy, `pub(crate)` and restricted visibility, library and binary crates, packages, target discovery, namespaces, conditional compilation, and public API boundaries. |
| 11 | Constants, statics, and initialization | `const` versus `static`, promotion, constant evaluation, mutable statics, thread-safe lazy initialization, `OnceLock`/`LazyLock`, initialization order, destructors at process exit, and edition-sensitive rules. |

### Phase 1 completion criteria

- Predict moves, borrows, drop order, closure capture, coercions, and pattern
  behavior from language rules.
- Design ownership and failure contracts without defaulting to cloning,
  reference counting, or panics.
- Separate Unicode text, OS strings, paths, bytes, and borrowed views.
- Explain how editions, crates, modules, packages, and compiler channels differ.

### Authoritative sources

- [The Rust Reference](https://doc.rust-lang.org/reference/)
- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [Rust Edition Guide](https://doc.rust-lang.org/edition-guide/)
- [Rust 2024](https://doc.rust-lang.org/edition-guide/rust-2024/index.html)
- [Rust standard library](https://doc.rust-lang.org/std/)
- [Reference: types](https://doc.rust-lang.org/reference/types.html)
- [Reference: expressions](https://doc.rust-lang.org/reference/expressions.html)
- [Reference: patterns](https://doc.rust-lang.org/reference/patterns.html)
- [Reference: destructors](https://doc.rust-lang.org/reference/destructors.html)

## Phase 2 — Traits, Generics, and API Design

| # | Topic | Research target |
| --- | --- | --- |
| 12 | Generics, bounds, inference, and monomorphization | Generic functions and types, type and const parameters, bounds, `where` clauses, turbofish syntax, inference limits, const generics, monomorphization, code-size costs, and when dynamic dispatch is preferable. |
| 13 | Traits, associated items, and coherence | Trait declarations and implementations, default methods, associated types and constants, supertraits, blanket implementations, coherence, orphan rules, overlap, negative reasoning limits, sealed traits, and semver consequences. |
| 14 | Trait objects and dynamic dispatch | `dyn Trait`, object safety/dyn compatibility, fat pointers, vtables, lifetime defaults, auto traits, dispatch cost, downcasting boundaries, heterogeneous collections, and alternatives such as enums or generics. |
| 15 | Lifetimes, variance, and higher-ranked bounds | Lifetime parameters, elision, outlives relations, early and late binding, higher-ranked trait bounds, subtyping, variance, drop checking, `PhantomData`, generic associated types, and API signatures that overconstrain callers. |
| 16 | Opaque types and return-position abstraction | Argument- and return-position `impl Trait`, opaque type identity, capture rules, return-position `impl Trait` in traits, async functions in traits, public API evolution, and boxing or associated-type alternatives. |
| 17 | Smart pointers and interior mutability | `Box`, `Rc`, `Weak`, `Arc`, `Cell`, `RefCell`, `UnsafeCell`, `Deref`, `Drop`, reference cycles, runtime borrow checks, thread-safety boundaries, ownership graphs, and explicit cleanup. |
| 18 | Conversions, borrowing traits, and owned/borrowed APIs | `From`/`Into`, `TryFrom`/`TryInto`, `AsRef`/`AsMut`, `Borrow`, `ToOwned`, `Cow`, deref coercion, blanket implementations, conversion cost, allocation visibility, and coherent generic signatures. |
| 19 | Iterators and ownership-aware pipelines | `Iterator`, `IntoIterator`, `iter`/`iter_mut`/`into_iter`, laziness, adapters, consumers, `FromIterator`, `Extend`, borrowing across iteration, custom iterators, fused behavior, fallible iteration, and loop tradeoffs. |
| 20 | Declarative and procedural macros | `macro_rules!`, fragment specifiers, repetition, hygiene, name resolution, expansion order, derive and attribute macros, token streams, spans and diagnostics, edition behavior, debugging expansion, and when functions or generics are clearer. |
| 21 | Idiomatic public API design | Newtypes, builders, typestate, common traits, naming, visibility, non-exhaustive types, sealed extension points, ownership flexibility, error and panic documentation, thread-safety promises, semver, and avoiding clever type machinery. |

### Phase 2 completion criteria

- Choose among generics, associated types, opaque types, trait objects, and
  enums from API and runtime constraints.
- Explain coherence, object safety, variance, higher-ranked bounds, and
  monomorphization.
- Design owned and borrowed APIs with visible allocation and lifetime costs.
- Use macros only when syntax or code generation provides material value.

### Authoritative sources

- [Reference: generic parameters](https://doc.rust-lang.org/reference/items/generics.html)
- [Reference: traits](https://doc.rust-lang.org/reference/items/traits.html)
- [Reference: trait implementations](https://doc.rust-lang.org/reference/items/implementations.html)
- [Reference: trait objects](https://doc.rust-lang.org/reference/types/trait-object.html)
- [Reference: lifetime elision](https://doc.rust-lang.org/reference/lifetime-elision.html)
- [Reference: `impl Trait`](https://doc.rust-lang.org/reference/types/impl-trait.html)
- [Standard conversion traits](https://doc.rust-lang.org/std/convert/)
- [Iterator API](https://doc.rust-lang.org/std/iter/trait.Iterator.html)
- [Reference: macros](https://doc.rust-lang.org/reference/macros.html)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)

## Phase 3 — Standard Library and Application Boundaries

| # | Topic | Research target |
| --- | --- | --- |
| 22 | Collection contracts and complexity | `Vec`, `VecDeque`, linked lists, hash and tree maps/sets, `BinaryHeap`, capacity, allocation, iteration order, entry APIs, hashing, ordering, key mutation, indexing, documented complexity, locality, and collection selection. |
| 23 | Formatting, parsing, and binary data | `Display`, `Debug`, formatting machinery, `FromStr`, byte slices, arrays, endian conversion, cursors, UTF-8 validation, zero-copy parsing boundaries, and explicit serialization formats. |
| 24 | I/O traits, buffering, files, and paths | `Read`, `Write`, `BufRead`, seeking, buffering, partial operations, `io::Error`, files, metadata, directories, canonicalization, platform path behavior, resource ownership, and durability boundaries. |
| 25 | Networking APIs | TCP and UDP types, address resolution, blocking behavior, timeouts, cloning handles, shutdown, partial reads and writes, framing boundaries, platform differences, and what the standard library intentionally omits. |
| 26 | Time and measurement | `Duration`, `Instant`, `SystemTime`, monotonic versus wall time, elapsed measurement, overflow, sleeping, deadlines, clock changes, and the boundary between `std` and calendar/time-zone crates. |
| 27 | Processes, environment, and platform interaction | Arguments, environment variables, child processes, stdio ownership, exit status, signals and terminals outside `std`, platform extensions, non-Unicode data, and injection or quoting hazards. |
| 28 | Resource and lifecycle design | Scope guards, `Drop`, explicit close/shutdown methods, destructor failure, blocking destructors, cancellation cleanup, leaked values, cycles, global state, and ownership of external resources. |
| 29 | Library testing and documentation contracts | Unit, integration, and documentation tests; examples; compile-fail behavior; test isolation; filesystem, time, and concurrency seams; rustdoc links; safety sections; and public contract verification. |

### Phase 3 completion criteria

- Select standard collections and I/O abstractions from contracts rather than
  habit.
- Preserve partial-I/O, path, time, process, and external-resource semantics at
  system boundaries.
- Make allocation, encoding, blocking, cleanup, and portability visible in API
  design.
- Treat documentation and tests as public-contract evidence, not topic drills.

### Authoritative sources

- [Standard collections](https://doc.rust-lang.org/std/collections/)
- [`std::fmt`](https://doc.rust-lang.org/std/fmt/)
- [`std::io`](https://doc.rust-lang.org/std/io/)
- [`std::fs`](https://doc.rust-lang.org/std/fs/)
- [`std::path`](https://doc.rust-lang.org/std/path/)
- [`std::net`](https://doc.rust-lang.org/std/net/)
- [`std::time`](https://doc.rust-lang.org/std/time/)
- [`std::process`](https://doc.rust-lang.org/std/process/)
- [Rustdoc Book](https://doc.rust-lang.org/rustdoc/)

## Phase 4 — Concurrency and Async Rust

| # | Topic | Research target |
| --- | --- | --- |
| 30 | Threads, scoped threads, and task ownership | Spawning and joining, `move` closures, `'static` bounds, scoped threads, thread-local state, panics, parking, naming, stack size, shutdown, cancellation limits, and ownership of background work. |
| 31 | `Send`, `Sync`, auto traits, and publication | Auto-trait derivation, negative implementations, raw pointers and interior mutability, safe publication, ownership transfer, shared references, trait-object bounds, unsafe implementations, and soundness contracts. |
| 32 | Locks, condition variables, and shared state | `Mutex`, `RwLock`, guards, poisoning, lock scope, invariant restoration, condition variables, spurious wakeups, one-time initialization, barriers, deadlocks, fairness, priority inversion, and external calls under locks. |
| 33 | Atomics and the memory model | Atomic types, relaxed/acquire/release/AcqRel/SeqCst ordering, compare-exchange loops, fences, modification order, happens-before reasoning, linearization points, ABA, false sharing, and when a lock is the safer abstraction. |
| 34 | Channels and ownership transfer | Standard channels, synchronous versus unbounded behavior, multi-producer ownership, disconnection, shutdown protocols, backpressure, fairness, selection outside `std`, and message passing versus shared state. |
| 35 | Futures, polling, waking, and pinning | `Future`, state-machine lowering, `Poll`, `Context`, `Waker`, executor responsibilities, cooperative progress, `Pin`/`Unpin`, self-referential state, wake contracts, and why futures do nothing until polled. |
| 36 | Async composition, cancellation, and lifetimes | `async` blocks and functions, joining and racing, cancellation by dropping, cancellation safety, borrowing across suspension, async closures, streams, timeouts, structured task ownership, and cleanup after partial progress. |
| 37 | Executors, async I/O, and runtime boundaries | Runtime scheduling, work stealing, reactor/executor roles, blocking in async contexts, `Send` futures, local tasks, spawn boundaries, bridging sync and async code, runtime shutdown, and downstream concurrency limits. |
| 38 | Concurrency correctness and verification | Data races versus race conditions, deadlock, livelock, starvation, deterministic tests, model checking, stress testing, tracing, lock contention, queue saturation, and defining progress guarantees. |

### Phase 4 completion criteria

- Prove thread transfer and sharing properties using ownership, `Send`, and
  `Sync`.
- Select channels, locks, atomics, or confinement from the invariant and
  progress requirements.
- Explain futures as state machines driven by an executor rather than threads.
- Define task ownership, cancellation, shutdown, backpressure, and blocking
  boundaries explicitly.

### Authoritative sources

- [`std::thread`](https://doc.rust-lang.org/std/thread/)
- [`std::sync`](https://doc.rust-lang.org/std/sync/)
- [`std::sync::atomic`](https://doc.rust-lang.org/std/sync/atomic/)
- [`std::future::Future`](https://doc.rust-lang.org/std/future/trait.Future.html)
- [`std::task`](https://doc.rust-lang.org/std/task/)
- [`std::pin`](https://doc.rust-lang.org/std/pin/)
- [Async Programming in Rust](https://rust-lang.github.io/async-book/)
- [Rustonomicon: concurrency](https://doc.rust-lang.org/nomicon/concurrency.html)
- [Tokio documentation](https://docs.rs/tokio/latest/tokio/)

## Phase 5 — Unsafe Rust, Runtime Behavior, and Performance

| # | Topic | Research target |
| --- | --- | --- |
| 39 | Unsafe boundaries and sound abstractions | Unsafe operations, unsafe functions and traits, safety invariants, caller versus implementer obligations, `unsafe_op_in_unsafe_fn`, minimizing unsafe scope, documenting `# Safety`, reviewing unsafe code, and proving safe callers cannot trigger undefined behavior. |
| 40 | Raw pointers, provenance, aliasing, and validity | Raw-pointer creation and access, null and dangling pointers, alignment, provenance, aliasing, reference validity, pointer arithmetic, integer round trips, strict provenance APIs, exposed provenance, and areas not fully specified. |
| 41 | Layout, representations, and dynamically sized types | Size, alignment, padding, `repr(Rust)` guarantees, `repr(C)`, transparent representations, enums and niches, slices and trait-object metadata, fat pointers, zero-sized types, uninhabited types, and layout-dependent API hazards. |
| 42 | Allocation, initialization, and manual destruction | `alloc`, `Layout`, `NonNull`, `MaybeUninit`, `ManuallyDrop`, `mem::forget`, ownership transfer, partial initialization, drop flags, panic safety, allocator contracts, and avoiding double drop, leaks, or use-after-free. |
| 43 | Pinning and address-sensitive values | `Pin`, `Unpin`, structural pinning, projection, drop guarantees, self-referential state, intrusive structures, pinned futures, unsafe pin construction, and when pinning should remain hidden behind an abstraction. |
| 44 | FFI, ABI, and cross-language ownership | `extern` blocks and functions, calling conventions, symbol linkage, `repr(C)`, strings and buffers, callbacks, opaque handles, allocation ownership, thread and unwind boundaries, bindgen-style generation, and safety wrappers. |
| 45 | Panic and unwind safety | `UnwindSafe`, `RefUnwindSafe`, cleanup during unwinding, partially updated invariants, destructors that panic, `catch_unwind` boundaries, abort configurations, FFI unwind rules, and exception safety in unsafe abstractions. |
| 46 | Rust compilation pipeline and generated code | Parsing and expansion, name resolution, type checking, HIR, MIR, borrow checking, monomorphization, LLVM/code generation, vtables, drop glue, incremental compilation, codegen units, linking, and useful inspection outputs. |
| 47 | Performance analysis and optimization | Debug versus release behavior, profiles, benchmarking traps, allocation and cloning, layout and locality, bounds-check elimination, iterator optimization, inlining, vectorization, dynamic dispatch, LTO, PGO, compile-time costs, and evidence-driven tuning. |
| 48 | `no_std`, targets, and embedded constraints | `core`, `alloc`, panic handlers, allocators, target specifications, cross-compilation, linker scripts, startup, interrupts, volatile access, atomics by target, peripheral ownership, and which language guarantees remain unchanged. |

### Phase 5 completion criteria

- State an unsafe abstraction's invariants and justify every unsafe operation.
- Distinguish stable layout, validity, provenance, and ABI guarantees from
  compiler implementation details.
- Trace representative Rust constructs through MIR, code generation, linking,
  and runtime behavior.
- Measure performance before changing ownership, dispatch, allocation, or
  safety boundaries.

### Authoritative sources

- [Reference: unsafe keyword](https://doc.rust-lang.org/reference/unsafe-keyword.html)
- [The Rustonomicon](https://doc.rust-lang.org/nomicon/)
- [Unsafe Code Guidelines Reference](https://rust-lang.github.io/unsafe-code-guidelines/)
- [`std::ptr`](https://doc.rust-lang.org/std/ptr/)
- [`std::mem`](https://doc.rust-lang.org/std/mem/)
- [`std::alloc`](https://doc.rust-lang.org/std/alloc/)
- [Reference: type layout](https://doc.rust-lang.org/reference/type-layout.html)
- [Reference: external blocks](https://doc.rust-lang.org/reference/items/external-blocks.html)
- [rustc-dev-guide](https://rustc-dev-guide.rust-lang.org/)
- [The Embedonomicon](https://docs.rust-embedded.org/embedonomicon/)

## Phase 6 — Cargo, Tooling, and Ecosystem Stewardship

| # | Topic | Research target |
| --- | --- | --- |
| 49 | Cargo manifests, targets, and workspaces | Manifest fields, automatic and explicit targets, workspaces, package selection, resolver versions, lockfiles, metadata inheritance, virtual manifests, command scope, and reproducible workspace structure. |
| 50 | Dependencies, features, and compatibility | Registry, git, and path dependencies; source replacement; feature additivity and unification; optional dependencies; target-specific dependencies; resolver behavior; semver; MSRV; duplicate versions; and public dependency exposure. |
| 51 | Profiles, build scripts, configuration, and cross-compilation | Dev, release, test, and custom profiles; optimization and debug settings; build-script lifecycle and inputs; environment/config precedence; target configuration; linkers; build caching; reproducibility; and build-time diagnosis. |
| 52 | Diagnostics, formatting, linting, and documentation | Reading compiler diagnostics, lint levels and groups, future-incompatible lints, `rustfmt`, Clippy, rustdoc, doctests, intra-doc links, generated documentation, unsafe documentation, CI policy, and toolchain-version drift. |
| 53 | Verification and dynamic analysis | Unit/integration/doc tests, compile-fail tests, benchmarks, property testing, fuzzing, Miri, sanitizers, coverage, model checking, unsupported configurations, false confidence, and selecting tools from the suspected failure mode. |
| 54 | Publishing, API evolution, and supply-chain boundaries | Package metadata, crates.io publishing, yanking, ownership, semver compatibility, feature evolution, deprecation, MSRV policy, auditing dependencies, licenses, build-script risk, unsafe dependency surface, and release reproducibility. |

### Phase 6 completion criteria

- Explain Cargo resolution, feature unification, workspaces, profiles, and build
  scripts without treating them as opaque automation.
- Maintain stable APIs across semver, edition, and MSRV constraints.
- Select compiler, lint, documentation, testing, fuzzing, interpreter, and
  sanitizer tools from concrete risks.
- Evaluate dependency and release decisions as part of the system's trust
  boundary.

### Authoritative sources

- [The Cargo Book](https://doc.rust-lang.org/cargo/)
- [Cargo Reference](https://doc.rust-lang.org/cargo/reference/)
- [Cargo: specifying dependencies](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html)
- [Cargo: features](https://doc.rust-lang.org/cargo/reference/features.html)
- [Cargo: profiles](https://doc.rust-lang.org/cargo/reference/profiles.html)
- [Cargo: build scripts](https://doc.rust-lang.org/cargo/reference/build-scripts.html)
- [The rustc Book](https://doc.rust-lang.org/rustc/)
- [The rustdoc Book](https://doc.rust-lang.org/rustdoc/)
- [Clippy documentation](https://doc.rust-lang.org/clippy/)
- [Miri](https://github.com/rust-lang/miri)
- [Rust Forge: releases](https://forge.rust-lang.org/release/)

## Capstone-driven learning

Capstones are integration environments, not graduation projects. Start them
when the required concepts become relevant and evolve them alongside research.
Keep general domain theory in capstone or dedicated-module documentation; use
this roadmap for the Rust-specific consequences.

### Embedded Rust on the BBC micro:bit v2

Explore `no_std`, target configuration, linker behavior, startup, panic
handling, interrupts, volatile device access, concurrency, peripheral
ownership, and hardware debugging. Use the
[Discovery book](https://docs.rust-embedded.org/discovery-mb2/) and
[Embedonomicon](https://docs.rust-embedded.org/embedonomicon/) as primary
project references. Hardware behavior, not compilation alone, defines success.

### User-space TCP/IP stack

Use Rust ownership, byte-oriented parsing, state machines, timers, concurrency,
and unsafe boundaries while implementing progressively deeper protocol layers.
Keep protocol and network architecture in the capstone; document buffer
ownership, partial I/O, cancellation, lifecycle, and verification as Rust
design decisions.

### Small language implementation

Use enums and patterns for syntax trees, traits or generics for phases, owned
and borrowed representations, diagnostics, testing, and performance analysis
while evolving a small language from interpretation toward bytecode or native
compilation. Keep general language and compiler theory in its dedicated
learning module or wishlist; use the capstone to examine Rust implementation
tradeoffs.

## Suggested research method

For each topic:

1. Begin with the relevant Reference or standard-library contract.
2. Check edition and stabilization notes before relying on current syntax or
   behavior under the Edition 2024 baseline.
3. Write a minimal compiler probe when ownership, inference, layout, or
   diagnostics are unclear.
4. Inspect expanded code, MIR, generated assembly, Miri output, or runtime
   traces when the abstraction hides relevant behavior.
5. Record guarantees, non-guarantees, invariants, tradeoffs, and production
   failure modes in your own words.
6. Apply the topic to a capstone when it naturally advances a vertical slice.
