# Java Roadmap

A self-directed research roadmap for an experienced senior engineer. Start with
the Java Language Specification, Java Virtual Machine Specification, Java SE API
documentation, final OpenJDK JEPs, and documentation for the JVM you run.

## Phase 1 — Language and Core APIs

| # | Topic | Research target |
| --- | --- | --- |
| 1 | Java execution and value model | Compilation, bytecode, static and runtime types, primitive values, references, assignment, argument passing, aliasing, field and array defaults, and definite assignment. |
| 2 | Numeric types and conversions | Integral and floating-point types, promotion, narrowing, overflow, division, NaN, signed zero, boxing, unboxing, wrapper caches, and `BigInteger`/`BigDecimal`. |
| 3 | Identity, equality, and hashing | Reference identity, primitive equality, `Object.equals`, `hashCode`, arrays, records, inheritance hazards, nulls, and mutable hash keys. |
| 4 | Strings and Unicode | String immutability and pooling, UTF-16 code units, code points, normalization, locale-sensitive operations, text blocks, formatting, and regular expressions. |
| 5 | Classes, construction, `final`, and `static` | Object creation, constructors, initialization order, instance and class members, constants, effective finality, final classes and methods, class initialization, and class-loader scope. |
| 6 | Nested, inner, local, and anonymous classes | Static nested classes, enclosing instances, captured state, local classes, anonymous classes, shadowing, initialization, generated access paths, and lifecycle implications. |
| 7 | Overloading, overriding, and dispatch | Compile-time overload resolution, runtime instance dispatch, static hiding, field hiding, covariant returns, bridge methods, varargs, and ambiguous calls. |
| 8 | Inheritance, interfaces, and composition | Java superclass rules, interface inheritance, abstract classes, default methods, substitutability, delegation, protected members, and fragile base classes. |
| 9 | Exceptions and resource management | Checked and unchecked exceptions, `Error`, propagation, translation, causes, suppressed exceptions, multi-catch, `AutoCloseable`, and `try`-with-resources. |
| 10 | Null contracts | The null type, dereferencing, API null policies, `Objects` utilities, nullness annotations, collection null rules, and boundary validation. |
| 11 | Enums | Enum identity, generated members, constructors, constant-specific behavior, switching, serialization semantics, `EnumSet`, and `EnumMap`. |
| 12 | Packages, access control, and modules | Package membership, imports, public/protected/package/private access, qualified access, modules, readability, exports, opens, services, classpath, and module path. |
| 13 | Annotations and reflection | Annotation targets and retention, repeatable and inherited annotations, type annotations, reflection, method handles, annotation processing, encapsulation, and runtime cost. |
| 14 | Date and time API | `Instant`, `Duration`, `Period`, local and zoned types, clocks, zones, daylight-saving transitions, parsing, formatting, and legacy interoperation. |
| 15 | I/O and NIO | Bytes versus characters, streams, readers and writers, charsets, buffering, paths, files, channels, byte buffers, selectors, memory mapping, asynchronous I/O, and resource ownership. |

### Phase 1 completion criteria

- Explain assignment, argument passing, initialization, dispatch, and equality
  from language rules.
- Predict numeric, string, exception, null, and nested-class edge cases.
- Design Java types with explicit construction, ownership, access, and lifecycle
  contracts.
- Use date/time and I/O APIs without conflating representations or resource
  lifetimes.

### Authoritative sources

- [Java Language Specification, Java SE 26](https://docs.oracle.com/en/java/javase/26/docs/specs/jls/index.html)
- [Java SE 26 API specification](https://docs.oracle.com/en/java/javase/26/docs/api/)
- [JLS Chapter 7: Packages and Modules](https://docs.oracle.com/en/java/javase/26/docs/specs/jls/jls-7.html)
- [JLS Chapter 8: Classes](https://docs.oracle.com/en/java/javase/26/docs/specs/jls/jls-8.html)
- [JLS Chapter 11: Exceptions](https://docs.oracle.com/en/java/javase/26/docs/specs/jls/jls-11.html)
- [Java I/O APIs](https://docs.oracle.com/en/java/javase/26/docs/api/java.base/java/io/package-summary.html)
- [Java NIO APIs](https://docs.oracle.com/en/java/javase/26/docs/api/java.base/java/nio/package-summary.html)
- [Date and time APIs](https://docs.oracle.com/en/java/javase/26/docs/api/java.base/java/time/package-summary.html)

## Phase 2 — Generics and Collections

| # | Topic | Research target |
| --- | --- | --- |
| 16 | Generics, inference, and type erasure | Generic types and methods, invariance, bounds, invocation and diamond inference, local type inference, raw types, erasure, reifiable types, inserted casts, bridge methods, heap pollution, restrictions, reflection, and type tokens. |
| 17 | Wildcards and PECS | Upper, lower, and unbounded wildcards; variance; capture; recursive bounds; safe reads and writes; and wildcard API design. |
| 18 | Collection contracts, specialized collections, and complexity | The hierarchy, iterators, spliterators, encounter order, sequenced collections, mutation, backed views, immutable factories, unmodifiable wrappers, identity and weak maps, fail-fast behavior, null policies, expected and worst-case costs, memory, and concurrency. |
| 19 | Lists and deques | `ArrayList`, `LinkedList`, `Deque`, and `ArrayDeque`; representation, indexing, iteration, end operations, circular arrays, allocation, locality, and legacy replacements. |
| 20 | Hash-based maps and sets | `HashMap`, `HashSet`, linked variants, `EnumMap`, and `EnumSet`; hashing, buckets, collisions, equality, capacity, load factor, resizing, tree bins, mutable keys, ordering, and implementation details. |
| 21 | Sorted and navigable collections | `TreeMap`, `TreeSet`, `SortedMap`, `NavigableMap`, comparators, natural ordering, uniqueness, tree costs, range views, neighbor queries, and consistency with equality. |
| 22 | Priority queues | Heap ordering, head semantics, comparators, insertion and removal costs, arbitrary lookup, iteration order, ties, top-K algorithms, and concurrent alternatives. |

### Phase 2 completion criteria

- Explain erasure, bounds, inference, invariance, capture, and PECS.
- Select collection interfaces and implementations from required semantics.
- Distinguish ordering, equality, mutability, view, snapshot, and specialized
  collection contracts.
- State documented complexity without confusing it with implementation folklore.

### Authoritative sources

- [JLS Chapter 4: Types, Values, and Variables](https://docs.oracle.com/en/java/javase/26/docs/specs/jls/jls-4.html)
- [JLS Chapter 18: Type Inference](https://docs.oracle.com/en/java/javase/26/docs/specs/jls/jls-18.html)
- [Generics tutorial](https://dev.java/learn/generics/)
- [Collections Framework tutorial](https://dev.java/learn/api/collections-framework/)
- [Java Collections Framework API](https://docs.oracle.com/en/java/javase/26/docs/api/java.base/java/util/doc-files/coll-overview.html)

## Phase 3 — Functional and Modern Java

| # | Topic | Research target |
| --- | --- | --- |
| 23 | Lambdas, functional interfaces, method references, and inference | Functional-interface rules, target typing, capture, effective finality, `this`, checked exceptions, `java.util.function`, bound and unbound references, constructor references, receivers, overloads, and `var` in lambda parameters. |
| 24 | Streams and collectors | Pipeline creation, laziness, non-interference, encounter order, primitive streams, mapping, flattening, reduction, collectors, duplicate keys, short-circuiting, parallel execution, and loop tradeoffs. |
| 25 | `Optional` | Creation, mapping, flattening, filtering, eager and lazy fallback, absence handling, primitive variants, streams, and appropriate API positions. |
| 26 | Records and immutability | Components, generated members, canonical and compact constructors, validation, defensive copying, shallow and deep immutability, value semantics, interfaces, serialization, and safe construction. |
| 27 | Sealed hierarchies, patterns, and modern `switch` | Permitted subtypes, subtype modifiers, type and record patterns, flow scoping, nested deconstruction, guards, dominance, null handling, exhaustiveness, switch expressions, arrow cases, and `yield`. |

### Phase 3 completion criteria

- Choose intentionally among loops, streams, lambdas, and method references.
- Model absence and immutable values without ambiguous contracts.
- Use records, sealed hierarchies, patterns, and switch expressions together.
- Explain how target typing and inference affect modern Java constructs.

### Authoritative sources

- [JLS §15.13: Method Reference Expressions](https://docs.oracle.com/en/java/javase/26/docs/specs/jls/jls-15.html#jls-15.13)
- [JLS §15.27: Lambda Expressions](https://docs.oracle.com/en/java/javase/26/docs/specs/jls/jls-15.html#jls-15.27)
- [Stream API package](https://docs.oracle.com/en/java/javase/26/docs/api/java.base/java/util/stream/package-summary.html)
- [`Optional` API](https://docs.oracle.com/en/java/javase/26/docs/api/java.base/java/util/Optional.html)
- [JEP 395: Records](https://openjdk.org/jeps/395)
- [JEP 409: Sealed Classes](https://openjdk.org/jeps/409)
- [JEP 440: Record Patterns](https://openjdk.org/jeps/440)
- [JEP 441: Pattern Matching for `switch`](https://openjdk.org/jeps/441)

## Phase 4 — JVM Internals

| # | Topic | Research target |
| --- | --- | --- |
| 28 | Runtime memory and allocation | JVM runtime data areas, stacks, frames, local variables, operand stacks, heap, method area, object creation, TLABs, escape analysis, scalar replacement, and native-resource lifetime. |
| 29 | Garbage collection and reference types | Reachability, roots, tracing, generations, regions, marking, copying, compaction, concurrent phases, pauses, strong/soft/weak/phantom references, cleaners, finalization, and retention analysis. |
| 30 | Bytecode and JIT compilation | Class-file structure, bytecode, interpretation, tiered compilation, profiling, inlining, devirtualization, speculation, deoptimization, escape analysis, on-stack replacement, and benchmark traps. |
| 31 | Class loading and initialization | Loading, verification, preparation, resolution, initialization, delegation, defining versus initiating loaders, runtime type identity, module layers, initialization locks, and loader leaks. |
| 32 | JVM diagnostics and performance | JFR, `jcmd`, thread dumps, heap dumps, class histograms, GC logs, native memory tracking, async profiling, JMH, warmup, and evidence-driven tuning. |

### Phase 4 completion criteria

- Map source behavior to JVM runtime areas and lifecycle stages.
- Separate allocation, reachability, reclamation, and resource ownership.
- Explain bytecode execution, optimization, deoptimization, and warmup.
- Gather and interpret basic JFR, heap, thread, GC, and JIT evidence.

### Authoritative sources

- [Java Virtual Machine Specification, Java SE 26](https://docs.oracle.com/javase/specs/jvms/se26/html/)
- [JVMS §2.5: Run-Time Data Areas](https://docs.oracle.com/javase/specs/jvms/se26/html/jvms-2.html#jvms-2.5)
- [JVMS Chapter 5: Loading, Linking, and Initializing](https://docs.oracle.com/javase/specs/jvms/se26/html/jvms-5.html)
- [Java HotSpot VM documentation](https://docs.oracle.com/en/java/javase/26/vm/)
- [Java Flight Recorder API](https://docs.oracle.com/en/java/javase/26/docs/api/jdk.jfr/jdk/jfr/package-summary.html)
- [OpenJDK JMH](https://github.com/openjdk/jmh)

## Phase 5 — Concurrency

| # | Topic | Research target |
| --- | --- | --- |
| 33 | Java Memory Model and safe publication | Inter-thread actions, data races, happens-before, synchronization order, visibility, ordering, final-field semantics, static initialization, immutable publication, and legal executions. |
| 34 | Threads and interruption | Platform-thread lifecycle, creation, scheduling, states, joining, interruption, blocking methods, daemon behavior, uncaught exceptions, and confinement. |
| 35 | Executors, task lifecycle, and `Future` | `ExecutorService`, pool sizing, queues, rejection, thread factories, task failure, blocking and timed retrieval, cancellation, deadlines, shutdown, memory consistency, and overload. |
| 36 | `CompletableFuture` | Stage composition, combining, recovery, observation, synchronous and asynchronous continuations, executor choice, timeouts, cancellation, graph completion, and starvation. |
| 37 | Virtual threads and structured task design | Virtual and platform threads, per-task execution, scheduling, mounting, pinning for the target JDK, thread locals, scoped values, structured concurrency status, observability, CPU work, and downstream limits. |
| 38 | Monitors, locks, and conditions | `synchronized`, intrinsic monitors, reentrancy, wait sets, guarded invariants, `Lock`, conditions, timed and interruptible acquisition, fairness, read-write locks, stamped locks, and guaranteed unlocking. |
| 39 | Volatile state and atomics | Volatile ordering and visibility, compare-and-set, atomic variables, update loops, ABA, linearization points, field updaters, `LongAdder`, contention, immutable state replacement, and multi-field invariants. |
| 40 | Concurrent collections | Concurrent maps and navigable maps, copy-on-write collections, concurrent queues, atomic compound APIs, weakly consistent iteration, null policies, and snapshots. |
| 41 | `BlockingQueue` and producer/consumer | Queue operation families, bounded capacity, blocking and timed calls, implementations, handoff, interruption, backpressure, consumer parallelism, ordering, shutdown, and overload policies. |
| 42 | Coordination primitives | `Semaphore`, `CountDownLatch`, `CyclicBarrier`, `Phaser`, `Exchanger`, permits, one-shot versus reusable coordination, failure, interruption, and concurrency limiting. |
| 43 | Race conditions and thread safety | Data races, check-then-act, read-modify-write, unsafe publication, invariants, linearization, immutability, confinement, locking, atomic transitions, composition, callbacks, and thread-safety contracts. |
| 44 | Deadlocks and liveness | Circular wait, lock ordering, nested locks, external calls, timed acquisition, thread-dump diagnosis, livelock, starvation, fairness, and progress guarantees. |

### Phase 5 completion criteria

- Define ownership for tasks, threads, executors, cancellation, and shutdown.
- Prove visibility and atomicity from happens-before and linearization points.
- Select monitors, locks, atomics, coordination primitives, confinement, or
  concurrent collections from the invariant.
- Diagnose races, deadlocks, starvation, and queue saturation.
- Apply virtual threads without removing downstream limits or safety controls.

### Authoritative sources

- [JLS Chapter 17: Threads and Locks](https://docs.oracle.com/en/java/javase/26/docs/specs/jls/jls-17.html)
- [`java.util.concurrent` package](https://docs.oracle.com/en/java/javase/26/docs/api/java.base/java/util/concurrent/package-summary.html)
- [`java.util.concurrent.locks` package](https://docs.oracle.com/en/java/javase/26/docs/api/java.base/java/util/concurrent/locks/package-summary.html)
- [`java.util.concurrent.atomic` package](https://docs.oracle.com/en/java/javase/26/docs/api/java.base/java/util/concurrent/atomic/package-summary.html)
- [JEP 444: Virtual Threads](https://openjdk.org/jeps/444)
- [JEP 506: Scoped Values](https://openjdk.org/jeps/506)
- [OpenJDK Project Loom](https://openjdk.org/projects/loom/)

## Phase 6 — Java Patterns (Later Deep Dive)

Study this phase after the language, runtime, and concurrency phases. Focus on
how Java's type system, lambdas, annotations, reflection, proxies, and resource
lifecycle affect each pattern.

| # | Topic | Research target |
| --- | --- | --- |
| 45 | Dependency injection and composition roots | Constructor injection, dependency inversion, manual composition, containers, scopes, qualifiers, cycles, lifecycle ownership, field and method injection, testing, and service locators. |
| 46 | Factories and builders | Static factories, factory methods, abstract factories, constructor visibility, implementation selection, caching, validation, staged builders, fluent APIs, and builder tradeoffs. |
| 47 | Strategy and template method | Interfaces, lambdas, method references, enum strategies, inheritance hooks, composition, state, and testability. |
| 48 | Adapter and decorator | Interface translation, object adapters, delegation, transparent wrapping, ordering, identity, equality, exception behavior, and wrapper composition. |
| 49 | Proxies and framework interception | Static proxies, `java.lang.reflect.Proxy`, invocation handlers, generated subclasses, method handles, annotations, interception boundaries, self-invocation, equality, and exception wrapping. |
| 50 | Observer and Java event models | Listener interfaces, event objects, registration, lifetime, weak listeners, ordering, reentrancy, synchronous and asynchronous delivery, `Flow`, and error handling. |
| 51 | Java-specific anti-patterns | Service locators, mutable statics, inheritance misuse, reflection overuse, raw types, swallowed interruption, unsafe publication, broken equality, serialization hazards, and unnecessary pattern machinery. |

### Phase 6 completion criteria

- Explain the Java mechanisms supporting each pattern.
- State ownership, lifecycle, failure, and concurrency behavior explicitly.
- Prefer direct construction or composition when a named pattern adds no value.
- Recognize framework-generated proxies and reflection boundaries during
  debugging.

### Authoritative sources

- [Jakarta CDI specification](https://jakarta.ee/specifications/cdi/)
- [`ServiceLoader` API](https://docs.oracle.com/en/java/javase/26/docs/api/java.base/java/util/ServiceLoader.html)
- [`Proxy` API](https://docs.oracle.com/en/java/javase/26/docs/api/java.base/java/lang/reflect/Proxy.html)
- [`Flow` API](https://docs.oracle.com/en/java/javase/26/docs/api/java.base/java/util/concurrent/Flow.html)
- [Method handles API](https://docs.oracle.com/en/java/javase/26/docs/api/java.base/java/lang/invoke/package-summary.html)

## Suggested research method

For each topic:

1. Start with the Java SE API contract or relevant JLS/JVMS section.
2. Read the final OpenJDK JEP when the topic originated as a language or runtime
   feature.
3. Write a minimal program that probes specification boundary cases.
4. Inspect bytecode, JFR data, GC logs, or thread dumps when runtime behavior is
   involved.
5. Summarize the invariant, tradeoffs, and production failure modes in your own
   words.
