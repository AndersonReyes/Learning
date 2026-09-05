# Go Roadmap

A self-directed research roadmap for an experienced senior engineer. It focuses
on Go language semantics, runtime behavior, standard-library contracts, tooling,
and idioms. Networking protocols, distributed algorithms, and system design
belong in capstones or dedicated learning modules.

The implementation baseline is the module's current `go 1.24` directive. The
living Go specification may document newer language versions; check its version
markers and the relevant release notes before using newer features.

## Phase 1 — Language Semantics

| # | Topic | Research target |
| --- | --- | --- |
| 1 | Declarations, variables, and zero values | Identifiers, declarations, initialization, short declarations, blank identifiers, scope, shadowing, zero values, addressability, and initialization dependencies. |
| 2 | Constants, numeric types, and conversions | Untyped constants, representability, default types, integer and floating-point behavior, complex numbers, overflow, shifts, explicit conversions, and architecture-dependent integer sizes. |
| 3 | Assignment, pointers, and value semantics | Assignment evaluation order, copying, aliasing, pointers, addressability, `new`, composite values, parameter passing, multiple assignment, and the absence of pointer arithmetic. |
| 4 | Control flow, functions, and closures | `if`, `for`, `range`, `switch`, type switches, labels, function values, closures, multiple returns, named results, variadic functions, and evaluation order. |
| 5 | Strings, bytes, runes, and UTF-8 | String immutability, byte sequences, UTF-8, rune values, indexing, ranging, invalid encodings, conversions, builders, readers, normalization boundaries, and formatting. |
| 6 | `defer`, `panic`, and `recover` | Argument evaluation, LIFO execution, named-result interaction, cleanup, panic propagation, recovery boundaries, runtime panics, and appropriate library behavior. |
| 7 | Errors | Error values, wrapping, `errors.Is`, `errors.As`, `errors.Join`, sentinel and typed errors, opaque errors, context, partial results, and API compatibility. |
| 8 | Packages, initialization, and visibility | Package clauses, imports, exported identifiers, `init`, dependency initialization order, import cycles, internal packages, commands versus libraries, and package-level state. |

### Phase 1 completion criteria

- Predict initialization, assignment, copying, aliasing, and evaluation order.
- Explain Go's numeric, string, closure, and pointer semantics.
- Design error and cleanup behavior without using panic for ordinary failures.
- Organize packages with explicit visibility and minimal initialization effects.

### Authoritative sources

- [The Go Programming Language Specification](https://go.dev/ref/spec)
- [Go 1 compatibility guarantee](https://go.dev/doc/go1compat)
- [Effective Go](https://go.dev/doc/effective_go)
- [Built-in package documentation](https://pkg.go.dev/builtin)
- [Package initialization model](https://go.dev/ref/spec#Program_initialization_and_execution)
- [Go blog: Errors are values](https://go.dev/blog/errors-are-values)
- [Go blog: Defer, Panic, and Recover](https://go.dev/blog/defer-panic-and-recover)

## Phase 2 — Types, Data, and Generics

| # | Topic | Research target |
| --- | --- | --- |
| 9 | Arrays and slices | Array value semantics, slice descriptors, length and capacity, slicing expressions, aliasing, append growth, copy, clear, nil versus empty slices, retention, bounds, and multidimensional layouts. |
| 10 | Maps | Key comparability, zero-value reads, comma-ok, nil maps, insertion and deletion, iteration order, mutation during iteration, references held by entries, equality limitations, and concurrent access rules. |
| 11 | Structs and composite literals | Field layout, tags, literals, comparability, anonymous fields, promoted members, copying, padding, alignment, empty structs, and serialization-facing design. |
| 12 | Defined types, aliases, and underlying types | Type definitions, aliases, identity, assignability, conversion, underlying types, predeclared aliases, generic aliases at the target language version, and API migration. |
| 13 | Methods, receivers, and method sets | Value and pointer receivers, automatic address/dereference in calls, method expressions, method values, method sets, interface satisfaction, copying, nil receivers, and receiver consistency. |
| 14 | Embedding and composition | Embedded fields and interfaces, promotion, selector depth, ambiguity, overriding through explicit methods, initialization, representation exposure, and composition without inheritance. |
| 15 | Interfaces and implicit satisfaction | Interface values, dynamic type and value, implicit implementation, method sets, interface embedding, empty interfaces, comparability, type identity, and small consumer-owned interfaces. |
| 16 | Nil interfaces, assertions, and type switches | Nil interface values versus typed nils, equality, type assertions, comma-ok, type switches, exhaustive assumptions, pointer/value method sets, and failure modes. |
| 17 | Generics, constraints, and inference | Type parameters, constraint interfaces, type sets, unions, approximation elements, `comparable`, inference, instantiation, generic types and functions, operations permitted by constraints, and Go 1.24 limitations. |
| 18 | Iterators and range functions | Built-in range forms, per-iteration variables, integer ranges, iterator function signatures, `iter.Seq`/`Seq2`, early termination, pull iterators, ownership, and version requirements. |

### Phase 2 completion criteria

- Explain the runtime-relevant representation and aliasing of arrays, slices,
  maps, structs, and interfaces.
- Derive method sets and interface satisfaction for value and pointer types.
- Recognize typed-nil and promotion traps.
- Design constraints that expose exactly the operations generic code requires.

### Authoritative sources

- [Go specification: Types](https://go.dev/ref/spec#Types)
- [Go specification: Properties of types and values](https://go.dev/ref/spec#Properties_of_types_and_values)
- [Go specification: Method sets](https://go.dev/ref/spec#Method_sets)
- [Go specification: Type parameters](https://go.dev/ref/spec#Type_parameter_declarations)
- [Go blog: Go slices—usage and internals](https://go.dev/blog/slices-intro)
- [Go blog: Maps in action](https://go.dev/blog/maps)
- [Go blog: The laws of reflection](https://go.dev/blog/laws-of-reflection)
- [Go Wiki: Range over function iterators](https://go.dev/wiki/RangefuncExperiment)

## Phase 3 — Core Standard Library

| # | Topic | Research target |
| --- | --- | --- |
| 19 | `io` abstractions and resource ownership | `Reader`, `Writer`, `Closer`, composition helpers, partial reads and writes, EOF, buffering, streaming, copying, limits, pipes, ownership, and close/error ordering. |
| 20 | Files, paths, and filesystems | `os`, `io/fs`, `path`, `filepath`, `embed`, permissions, temporary files, atomic replacement, directory walking, symlinks, platform differences, and cleanup. |
| 21 | Encoding and serialization APIs | `encoding/json`, `encoding/binary`, `encoding/text`, struct tags, exported fields, zero and omitted values, custom marshalers, streaming decoders, limits, unknown data, compatibility, and unsafe input. |
| 22 | Time | `time.Time`, monotonic readings, locations, durations, parsing, formatting, timers, tickers, deadlines, equality, serialization, daylight-saving transitions, and testable clocks. |
| 23 | Networking primitives | `net.Conn`, listeners, packet connections, addresses, DNS resolution, deadlines, cancellation, half-close behavior, temporary failures, resource limits, and ownership. |
| 24 | HTTP clients and servers | `net/http`, request contexts, transports, connection reuse, body ownership, timeouts, redirects, handlers, middleware, server shutdown, streaming, testing, and protocol selection. |
| 25 | Database access | `database/sql`, pools, contexts, transactions, prepared statements, scanning, null values, connection lifetime, retries, isolation boundaries, cleanup, and driver contracts. |
| 26 | Logging | `log/slog`, structured records, handlers, levels, attributes, groups, context, source information, redaction, performance, and library-versus-application ownership. |
| 27 | Reflection | `reflect.Type`, `Value`, kinds, addressability, settability, conversions, method lookup, tags, generic interaction, panic conditions, performance, and when static alternatives are preferable. |

### Phase 3 completion criteria

- Compose standard-library interfaces without losing errors or ownership.
- Bound I/O, decoding, HTTP, and database operations with context and deadlines.
- Distinguish bytes, text, paths, times, and serialized representations.
- Use reflection and structured logging behind narrow, documented boundaries.

### Authoritative sources

- [Go standard library](https://pkg.go.dev/std)
- [`io`](https://pkg.go.dev/io)
- [`io/fs`](https://pkg.go.dev/io/fs)
- [`encoding/json`](https://pkg.go.dev/encoding/json)
- [`time`](https://pkg.go.dev/time)
- [`net`](https://pkg.go.dev/net)
- [`net/http`](https://pkg.go.dev/net/http)
- [`database/sql`](https://pkg.go.dev/database/sql)
- [`log/slog`](https://pkg.go.dev/log/slog)
- [`reflect`](https://pkg.go.dev/reflect)

## Phase 4 — Concurrency

| # | Topic | Research target |
| --- | --- | --- |
| 28 | Goroutines and lifecycle | Creation, initial stacks, scheduling, blocking, preemption, closure capture, ownership, completion, panic boundaries, process exit, cost model, and leak prevention. |
| 29 | Channels and ownership | Unbuffered and buffered channels, send/receive synchronization, direction types, close ownership, zero values, nil channels, closed-channel behavior, ranging, capacity, and backpressure. |
| 30 | `select`, timers, and timeouts | Ready-case selection, default cases, nil-channel disabling, timer and ticker lifecycle, cancellation, timeout composition, fairness assumptions, and busy loops. |
| 31 | `context` propagation | Cancellation trees, deadlines, causes, values, API placement, request scoping, cleanup functions, detached work, misuse as optional parameters, and interoperability with blocking APIs. |
| 32 | Go memory model and data races | Sequenced-before, synchronized-before, happens-before, channel and lock synchronization, initialization, atomics, ordinary reads and writes, race-free guarantees, and permitted racy behavior. |
| 33 | `sync` primitives | `Mutex`, `RWMutex`, `WaitGroup`, `Once`, `Cond`, `Map`, `Pool`, locker discipline, copying restrictions, memory-model edges, contention, and selection criteria. |
| 34 | Atomics | Typed atomic values, compare-and-swap, swaps, read-modify-write, memory ordering provided by Go, alignment, atomic flags and snapshots, contention, and multi-variable invariants. |
| 35 | Concurrency ownership and cancellation | Goroutine ownership, channel-closing responsibility, bounded fan-out, cancellation propagation, cleanup, error collection, partial failure, shutdown ordering, and blocked senders/receivers. |
| 36 | Pipelines and bounded concurrency | Pipelines, fan-out/fan-in, worker groups, semaphores, rate versus concurrency limits, ordering, buffering, backpressure, early termination, and avoiding goroutine-per-item explosions. |
| 37 | Deadlocks, livelocks, starvation, and leaks | Wait cycles, nil channels, missing senders or receivers, lock ordering, callbacks under locks, scheduler dependence, unfair progress, leaked timers, leaked goroutines, and runtime diagnosis. |

### Phase 4 completion criteria

- Prove synchronization and visibility with the Go memory model.
- Assign ownership for every goroutine, channel, timer, context, and shutdown path.
- Select channels, locks, atomics, or confinement from the invariant.
- Bound concurrency independently from request or input volume.
- Diagnose races, deadlocks, starvation, and goroutine leaks.

### Authoritative sources

- [The Go Memory Model](https://go.dev/ref/mem)
- [`context`](https://pkg.go.dev/context)
- [`sync`](https://pkg.go.dev/sync)
- [`sync/atomic`](https://pkg.go.dev/sync/atomic)
- [Go blog: Pipelines and cancellation](https://go.dev/blog/pipelines)
- [Go blog: Context](https://go.dev/blog/context)
- [Data race detector](https://go.dev/doc/articles/race_detector)

## Phase 5 — Runtime, Testing, and Tooling

| # | Topic | Research target |
| --- | --- | --- |
| 38 | Modules, workspaces, and dependency selection | `go.mod`, `go.sum`, module paths, semantic import versioning, minimum version selection, replacement, retraction, vendoring, workspaces, proxies, checksum database, private modules, and reproducibility. |
| 39 | Build system and portability | `go` commands, packages and targets, build cache, build constraints, file suffixes, `GOOS`/`GOARCH`, cross-compilation, cgo interaction, linking, embedding, generation, and reproducible build metadata. |
| 40 | Allocation and escape analysis | Stack and heap decisions, escape analysis, inlining interaction, slice and map allocation, interface boxing, closures, allocation diagnostics, object lifetime, and optimization instability. |
| 41 | Scheduler and goroutine stacks | G-M-P scheduling, work stealing, network poller, syscalls, blocking, preemption, `GOMAXPROCS`, stack growth and shrinking, cgo transitions, and scheduler tracing. |
| 42 | Garbage collection | Tracing, concurrent mark and sweep, write barriers, pacing, roots, heap goals, memory limits, finalizers, cleanup APIs for the target release, pools, retention, and latency/throughput tradeoffs. |
| 43 | Unit tests, subtests, and examples | `testing`, table-driven tests, subtests, parallel tests, cleanup, helpers, examples, test caching, package boundaries, temporary resources, determinism, and testable API design. |
| 44 | Fuzzing and benchmarks | Native fuzz targets, seed corpora, minimization, supported inputs, persistence, benchmarks, sub-benchmarks, setup exclusion, allocation reporting, compiler effects, and representative workloads. |
| 45 | Race detection and static analysis | `-race`, coverage limits, overhead, `go vet`, analyzers, build tags, false confidence, platform support, and combining dynamic and static evidence. |
| 46 | Profiling, tracing, and diagnostics | `pprof`, execution traces, runtime metrics, block and mutex profiles, goroutine dumps, CPU and heap profiles, labels, production collection cost, and evidence-driven optimization. |
| 47 | `unsafe` and cgo boundaries | `unsafe.Pointer`, `uintptr`, layout, alignment, `unsafe.Add`/`Slice`/`String`, pointer-passing rules, `runtime.KeepAlive`, cgo calls, callbacks, pinning, ownership, and portability. |

### Phase 5 completion criteria

- Explain module selection, builds, portability, and reproducibility.
- Relate source code to escape decisions, scheduling, stack growth, and GC.
- Design deterministic tests and representative fuzz and benchmark targets.
- Use race, vet, profile, and trace evidence within their documented limits.
- Isolate unsafe and foreign-code boundaries behind explicit ownership contracts.

### Authoritative sources

- [Go Modules Reference](https://go.dev/ref/mod)
- [Workspace tutorial](https://go.dev/doc/tutorial/workspaces)
- [Command `go`](https://pkg.go.dev/cmd/go)
- [Go build constraints](https://pkg.go.dev/cmd/go#hdr-Build_constraints)
- [Go GC guide](https://go.dev/doc/gc-guide)
- [Diagnostics](https://go.dev/doc/diagnostics)
- [`testing`](https://pkg.go.dev/testing)
- [Go fuzzing](https://go.dev/doc/security/fuzz/)
- [Data race detector](https://go.dev/doc/articles/race_detector)
- [`runtime/pprof`](https://pkg.go.dev/runtime/pprof)
- [`runtime/trace`](https://pkg.go.dev/runtime/trace)
- [cgo command documentation](https://pkg.go.dev/cmd/cgo)
- [`unsafe`](https://pkg.go.dev/unsafe)

## Phase 6 — Idiomatic Go Patterns

Study this phase after the language, library, concurrency, and tooling phases.
Focus on patterns that arise from Go's zero values, structural interfaces,
composition, functions, explicit errors, and concurrency model.

| # | Topic | Research target |
| --- | --- | --- |
| 48 | Zero-value APIs, constructors, and functional options | Useful zero values, validation, unexported fields, constructor necessity, option functions, defaults, option errors, configuration structs, discoverability, compatibility, and avoiding needless builders. |
| 49 | Small interfaces and composition roots | Consumer-owned interfaces, capability interfaces, implicit satisfaction, concrete returns, manual dependency injection, composition roots, test fakes, lifecycle ownership, and avoiding service locators or interface pollution. |
| 50 | Embedding, adapters, decorators, and middleware | Structural adaptation, promoted methods, explicit forwarding, wrappers, HTTP middleware, ordering, identity, optional capabilities, error behavior, and avoiding inheritance-style embedding. |
| 51 | Error API design | Stable error contracts, wrapping, matching, typed details, operation and resource context, partial success, retries, logging ownership, transport translation, and compatibility. |
| 52 | Context and concurrent API design | Context-first signatures, cancellation ownership, synchronous versus asynchronous APIs, goroutine lifetime, channel ownership, result and error delivery, backpressure, and shutdown contracts. |
| 53 | Generics, interfaces, reflection, or generation | Choosing compile-time constraints, runtime polymorphism, metadata-driven behavior, or generated code based on operations, type safety, performance, binary size, tooling, and maintenance. |
| 54 | Go-specific anti-patterns | Typed-nil errors, copied mutexes, goroutine leaks, unbounded concurrency, forgotten cancellation, premature interfaces, package globals, panic for routine errors, ignored close failures, overuse of reflection or `unsafe`, and Java-style API design. |

### Phase 6 completion criteria

- Design APIs that are useful at their zero value or require construction for a
  stated invariant.
- Keep interfaces small, consumer-owned, and behavior-oriented.
- Make error, context, goroutine, channel, and resource ownership explicit.
- Choose generics, interfaces, reflection, and generation deliberately.
- Recognize non-idiomatic patterns imported from class-oriented languages.

### Authoritative sources

- [Effective Go](https://go.dev/doc/effective_go)
- [Go Code Review Comments](https://go.dev/wiki/CodeReviewComments)
- [Go blog: Package names](https://go.dev/blog/package-names)
- [Go blog: Organizing Go code](https://go.dev/blog/organizing-go-code)
- [Go blog: JSON and Go](https://go.dev/blog/json)
- [Go blog: First-class functions in Go](https://go.dev/blog/functions-codewalk)

## Capstone-driven learning

Capstones are not graduation projects that must wait until the roadmap is
complete. Start one after the first phase and evolve it while researching later
phases.

The current capstones are:

- [Distributed message queue](./capstone-message-queue/)
- [Linux container runtime](./capstone-container-runtime/)

Use the phases incrementally:

1. Apply language semantics to domain types, errors, configuration, and package
   boundaries.
2. Apply data and generic concepts to storage, indexing, messages, and reusable
   components.
3. Add standard-library I/O, encoding, networking, persistence, and observability.
4. Add explicit concurrency ownership, cancellation, bounds, and shutdown.
5. Test, fuzz, race-check, profile, trace, and inspect runtime behavior.
6. Refactor toward idiomatic APIs only after concrete usage reveals the needed
   abstractions.

Keep system-design decisions in the capstone documentation or the system-design
module. Keep the language roadmap focused on how Go expresses and implements
those decisions.

## Suggested research method

For each topic:

1. Start with the Go specification or standard-library contract.
2. Check the module's language version and relevant Go release notes.
3. Write the smallest experiment needed to probe a language or API boundary.
4. Apply the concept to the active capstone when it naturally belongs there.
5. Capture conclusions, invariants, and tradeoffs in your own words.
