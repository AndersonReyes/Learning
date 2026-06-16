# Rust Roadmap

**Status: in progress.** Curriculum and reference are settled (below). 29
topics total: 10 Fundamentals + 9 Intermediate + 10 Advanced, plus two
capstones. Fundamentals (all 10), Intermediate (all 9), and Advanced topics 1–4
are built (see "Build Log" below for what each one's exercises cover);
everything else is `planned`.

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

## Build Log

Short summary of what each built topic's 5 exercises cover, for picking up
across sessions without re-reading every file.

- **01 — Toolchain, Cargo & Hello World**: first real Rust programs —
  `collatz_steps`, `is_prime`, `longest_run`, `caesar_cipher`,
  `matrix_transpose`.
- **02 — Variables, Data Types & Functions**: scalar/compound types and `as`
  cast rules — `rotate_array_left`, `pack_rgb`/`unpack_rgb`,
  `overflowing_factorial`, `fixed_point_divide`.
- **03 — Control Flow**: `if`/`else` as an expression, `loop`/`while`/`for`,
  loop labels — `find_in_grid`, `is_armstrong_number`,
  `sum_of_multiples_below`, `digital_root`, `count_steps_to_reach`.
- **04 — Ownership & Borrowing**: move/`Clone`/`Copy` and `&T`/`&mut T`
  borrowing — `partition_in_place`, `merge_sorted_into`,
  `take_ownership_and_split`, `drain_below_threshold`,
  `longest_common_prefix_owned`.
- **05 — The Slice Type & `&str`**: `&[T]`/`&str` slicing, range syntax,
  UTF-8 byte-vs-char — `split_on_whitespace_runs`, `first_n_chars`,
  `longest_palindromic_substring_slice`, `max_subarray_slice`,
  `chunk_slices`.
- **06 — Structs & Methods**: a single `Polynomial` struct (canonical
  `Vec<f64>` coefficient form, trailing zeros trimmed) with an associated
  `new` constructor and `&self` methods — `evaluate` (Horner's method),
  `derivative`, `add`, `multiply` (convolution).
- **07 — Enums & Pattern Matching**: a recursive `Expr` AST (`Box`-based)
  with `eval -> Option<f64>` (`?` on `Option`, division-by-zero ->
  `None`); `TriangleKind` enum + `classify_triangle` (tuple-pattern
  matching with `|`); `Direction` enum + `from_token -> Option<Direction>`
  and `walk` (exhaustive match over variants); `first_non_repeating_char ->
  Option<char>`.
- **08 — Packages, Crates & Modules**: the module system itself is the
  exercise — `src/geometry.rs` (`polygon_area` via the shoelace formula,
  `closest_pair_distance` brute force) and `src/stats.rs` (`median`,
  `standard_deviation`, `mode`), both declared as `pub mod` in `lib.rs`,
  with `stats::median` re-exported at the crate root via `pub use`.
- **09 — Common Collections**: `HashMap`/`HashSet`/`String` idioms —
  `word_frequency` (entry-API counting with case-folding and punctuation
  trimming), `group_anagrams` (sorted-chars key, order-preserving groups),
  `top_k_frequent` (count, sort by frequency desc / value asc),
  `dedup_preserve_order` (`HashSet` membership tracking),
  `run_length_encode` (`String` building over `.chars()`, UTF-8 safe).
- **10 — Error Handling**: custom error enums, `?`, `.map_err()`, and error
  accumulation — `eval_rpn` (RPN calculator over `CalcError`, stack
  underflow/extra-operands/unknown-operator/div-by-zero), `parse_csv_row`
  (`RowError` with column-count and per-column number checks),
  `checked_transfer` (`TransferError`, mutable `HashMap`, all-or-nothing
  balance updates), `parse_all_or_first_error` (`(usize, ParseIntError)`,
  first-failure index), `validate_password` (`Result<(), Vec<String>>`,
  reports *every* violated rule).
- **Intermediate 01 — Generics, Traits & Lifetimes**: a generic `Bst<T:
  Ord>` (`insert` ignoring duplicates, `contains`, `in_order` traversal,
  recursive `Box`-based nodes); `sum_all<T: Add<Output=T> + Copy + Default>`
  over both primitives and a custom `Money` type; `Tokenizer<'a>::next_token`
  (lifetime-bound `&'a str` slices, ASCII-alphanumeric lexing over UTF-8
  input).
- **Intermediate 02 — Writing Tests & Project Organization**: standard
  hard-algorithm exercises chosen so `tests/exercise_test.rs` itself
  demonstrates ch.11's testing techniques — `binary_search<T: Ord>` (generic
  over `&str` too), `kth_smallest` (panics with a specific message, exercised
  via `#[should_panic(expected = "...")]`), `merge_intervals` (sort + merge
  overlapping/touching ranges, with one test using the `-> Result<(), String>`
  pattern), `longest_increasing_subsequence` (O(n²) DP), `min_coins` (coin
  change DP, `Option<u32>`).
- **Intermediate 03 — CLI I/O Project**: pieces of a `minigrep`-style tool as
  pure, testable functions — `parse_args` (flag/positional parsing into a
  `Config`, with `-i`/`--ignore-case`, `-n`/`--line-numbers`, and exact error
  messages for too-few/too-many args and unknown flags), `search_lines`
  (ch.12.4's TDD'd line search, case-(in)sensitive), `highlight_matches`
  (non-overlapping `**match**` wrapping, case-folded matching with
  original-case output, UTF-8-safe), `grep_report` (combines the above into a
  formatted report with line numbers and singular/plural match-count
  summary), `resolve_ignore_case` (ch.12.5's CLI-flag-vs-env-var precedence).
- **Intermediate 04 — Closures & Iterators**: `Memoizer<F: Fn(u64) -> u64>`
  (generalized `Cacher` keyed by a `HashMap`, `new` provided, `value` caches
  per-`arg`), `compose` (boxed function composition, `Box<dyn Fn(A) -> C>`),
  `retry` (`FnMut`-based retry-until-`Ok`, `"no attempts allowed"` for
  `max_attempts == 0`), `top_n_by<T, K: Ord>` (generic stable top-N-by-key
  sort, `Reverse` for bottom-N), `running_stats` (`.scan()`-based running
  min/max per prefix).
- **Intermediate 05 — Custom Iterators & Adapters**: `Fibonacci` (infinite
  `Iterator<Item = u64>`, `0,1,1,2,3,...`), `Pairwise<I>` (adapter yielding
  consecutive `(prev, curr)` pairs), `RunLength<I>` (adapter doing run-length
  encoding via a `peeked: Option<I::Item>` lookahead field),
  `ChunksIterator<I>` (adapter yielding fixed-size `Vec<I::Item>` chunks,
  final chunk possibly shorter, `size == 0` yields nothing), `Grid` +
  `GridIntoIter` (custom `IntoIterator` impl flattening `Vec<Vec<i32>>` in
  row-major order, skipping empty rows).
- **Intermediate 06 — Error Handling Deep Dive**: `parse_duration_ms`
  (`<digits><unit>` segment parser summing to total ms, `ParseDurationError`
  with `From<ParseIntError>` for `?`-based conversion and a `source()` impl),
  `error_chain_messages` (walks an `&dyn Error`'s `source()` chain collecting
  `Display` messages outermost-first, over a `WrappedError` wrapper type),
  `describe_error` (`downcast_ref` dispatch over `&(dyn Error + 'static)` for
  `NotFoundError`/`PermissionError`/fallback), `process_record` (`"name:age"`
  parser returning `Box<dyn Error>`, combining a custom `RecordError` enum
  with a propagated `ParseIntError`), `first_valid_port` (`Box<dyn Error>`
  port picker distinguishing "no candidate in range" from "no candidate
  parsed" via downcasting to `NoValidPortError` vs `ParseIntError`).
- **Intermediate 07 — Cargo Workspaces, Profiles & Iterator Performance**
  (adapted; `notes.md` covers ch. 13.4 + ch. 14 conceptually, exercises are
  all ch. 13.4 "loop -> iterator chain" translations): `longest_increasing_run`
  (`.windows(2)` + `.fold()` tracking `(longest, current)`), `moving_average`
  (`.windows(window)` + `.map()`/`.sum()`, guarding `window == 0` and
  `window > len`), `zigzag_merge` (`.zip()` + `.flat_map()` + `.chain()` to
  interleave two slices and append the longer one's remainder),
  `count_local_maxima` (`.windows(3)` + `.filter()` + `.count()` for interior
  elements, direct comparisons for the two endpoints), `exponential_moving_average`
  (`.scan()` carrying the running EMA, seeded via `std::iter::once` +
  `.chain()`).
- **Intermediate 08 — Smart Pointers**: `LruCache::get`/`put` (`Rc<RefCell<VecDeque<(K,V)>>>`-style
  interior mutability for a fixed-capacity LRU cache, ordering entries
  most- to least-recently-used), `tree_depth`/`lowest_common_ancestor` (a
  `Node` with owning `Rc` links down to children and a non-owning `Weak` link
  up to its parent; depth via repeated `.parent.borrow().upgrade()`, LCA by
  collecting one node's ancestor chain and walking the other's, comparing
  with `Rc::ptr_eq`), `CountedRef`'s `Deref`/`DerefMut` (a smart pointer that
  counts reads via `Cell` and writes via a plain field, demonstrating deref
  coercion vs. explicit `*x`), `Pool`/`PoolGuard` (an RAII object pool:
  `acquire` pops an item into a guard, `Drop` pushes it back via
  `RefCell::borrow_mut`).
- **Intermediate 09 — Fearless Concurrency**: `sum_with_threads` (splits a
  `Vec<i64>` into `ceil(len/num_threads)`-sized chunks, sums each on its own
  `thread::spawn`, joins and adds the partial sums), `merge_sort_parallel`
  (recursive merge sort that spawns a thread for the right half while
  `max_depth > 0`, sorts the left half on the current thread, joins and
  merges), `collect_messages` (spawns `num_producers` threads each sending
  `"producer-{p}-msg-{m}"` over a cloned `mpsc::Sender`, drops the original
  sender so the receiver's iterator terminates, then sorts the collected
  messages), `concurrent_word_count` (each thread builds a local `HashMap` for
  its chunk, then merges into a shared `Arc<Mutex<HashMap<String, usize>>>`),
  `run_in_parallel` (runs a `Vec<Box<dyn FnOnce() -> T + Send + 'static>>` on
  one thread each via `thread::spawn`, joins in order to preserve result
  order regardless of completion order).
- **Advanced 01 — Trait Objects, Dynamic Dispatch & OOP Patterns**: `Expr`
  trait + `Num`/`Add`/`Sub`/`Mul`/`Div`/`Neg` node types + `eval_all`
  (recursive `Box<dyn Expr>` tree evaluator returning `Result<f64, String>`,
  `?`-propagating a `"division by zero"` error from any subtree),
  `TurnstileState` trait + `Locked`/`Unlocked` + `run_turnstile` (State
  pattern using `self: Box<Self>` transitions, feeding `Event`s through a
  turnstile and recording the state name after each), `Task` trait +
  `run_tasks_in_priority_order` (stable-sorts a heterogeneous
  `Vec<Box<dyn Task>>` by `priority()`, ties keep insertion order, then runs
  each), `Shape: Any` trait + `Circle`/`Square`/`Rectangle` + `count_by_type`
  (`dyn Any::downcast_ref` to tally concrete types in a
  `Vec<Box<dyn Shape>>`), `Notifier` trait + `EmailNotifier`/`SmsDecorator`/
  `LoggingDecorator` (Decorator pattern: nested `Box<dyn Notifier>` each
  prepending their own log line before delegating to the wrapped notifier).
- **Advanced 02 — Patterns & Matching Deep Dive**: `simplify(expr: Expr) ->
  Expr` (algebraic simplifier over a recursive `#[derive(PartialEq,Clone)]`
  enum using nested arm patterns, `|` alternatives that bind the same
  variable from different positions `(e, Num(0)) | (Num(0), e)`, and a match
  guard for `Sub(x, x) → Num(0)`), `parse_ipv4(s: &str) -> Option<[u8; 4]>`
  (slice pattern `[a, b, c, d]` on the collected dot-separated parts),
  `balanced_brackets(s: &str) -> bool` (stack-based checker using a single
  `match (stack.last().copied(), c)` with `|` across `(Some('('), ')') |
  (Some('['), ']') | (Some('{'), '}')` and or-patterns in the opening arm),
  `longest_run<T: PartialEq>(slice: &[T]) -> Option<(&T, usize)>` (`[prev,
  next]` slice pattern over `.windows(2)` with a `if next == prev` match
  guard), `classify_triangle(a: u64, b: u64, c: u64) -> &'static str` (array
  destructuring `let [s, m, l] = sorted;` + a match guard for the
  Pythagorean theorem + `|` pattern for isosceles detection).
- **Advanced 04 — Advanced Functions, Closures & Macros**: `apply_all` (dispatch table of `fn` pointers zipped with values), `make_pipeline` (left-to-right closure composition returning `Box<dyn Fn(i32) -> i32>`, identity for empty input), `call_with_one` (generic `F: Fn(i32) -> i32` — accepts both fn pointers and capturing closures), `sum_of_squares!` (`macro_rules!` accepting one-or-more `$expr` arguments, summing their squares as `i64` via `$($x as i64 * $x as i64 +)+ 0`), `fold_with<T,B,F>` (standalone `FnMut`-based left fold over `&[T]`).
- **Advanced 03 — Advanced Traits & Types**: `Magnitude` trait with associated
  `Output` type implemented for `i64` (unsigned_abs) and `(f64,f64)` (Euclidean
  length), plus `max_magnitude<T: Magnitude>` using `.map().reduce()`; `Matrix2x2`
  with `std::ops::Add` (elementwise), `std::ops::Mul<f64>` (scalar), and
  `mat_mul` (standard 2×2 product); `WordCloud` newtype wrapping
  `HashMap<String, usize>` with a `Display` impl sorted descending-by-count /
  ascending-alpha-on-ties and a `top_n` extractor; `Summarize: Display`
  supertrait with a default `headline()` method implemented on `NewsArticle`,
  plus a `headlines<T: Summarize>` collector; `Greet` and `Farewell` traits
  both declaring `fn message(&self) -> String` on `Person`, disambiguated via
  `<Person as Greet>::message(p)` / `<Person as Farewell>::message(p)` in
  `greet_and_farewell`.

## Fundamentals

Covers Book ch. 1-9: the language core, ownership, and the standard
library's basic data structures.

| # | Topic | Folder | Reference | Status |
|---|-------|--------|-----------|--------|
| 1 | Toolchain, Cargo & Hello World | [`fundamentals/01-toolchain-cargo-and-hello-world`](./fundamentals/01-toolchain-cargo-and-hello-world) | Book ch. 1-2 | done |
| 2 | Variables, Data Types & Functions | [`fundamentals/02-variables-data-types-and-functions`](./fundamentals/02-variables-data-types-and-functions) | Book ch. 3.1-3.3 | done |
| 3 | Control Flow | [`fundamentals/03-control-flow`](./fundamentals/03-control-flow) | Book ch. 3.4-3.5 | done |
| 4 | Ownership & Borrowing | [`fundamentals/04-ownership-and-borrowing`](./fundamentals/04-ownership-and-borrowing) | Book ch. 4.1-4.2 | done |
| 5 | The Slice Type & `&str` | [`fundamentals/05-the-slice-type-and-str`](./fundamentals/05-the-slice-type-and-str) | Book ch. 4.3 | done |
| 6 | Structs & Methods | [`fundamentals/06-structs-and-methods`](./fundamentals/06-structs-and-methods) | Book ch. 5 | done |
| 7 | Enums & Pattern Matching | [`fundamentals/07-enums-and-pattern-matching`](./fundamentals/07-enums-and-pattern-matching) | Book ch. 6 | done |
| 8 | Packages, Crates & Modules | [`fundamentals/08-packages-crates-and-modules`](./fundamentals/08-packages-crates-and-modules) | Book ch. 7 | done |
| 9 | Common Collections (`Vec`, `String`, `HashMap`) | [`fundamentals/09-common-collections`](./fundamentals/09-common-collections) | Book ch. 8 | done |
| 10 | Error Handling (`panic!`, `Result`, `?`) | [`fundamentals/10-error-handling`](./fundamentals/10-error-handling) | Book ch. 9 | done |

## Intermediate

Covers Book ch. 10-16: generics/traits/lifetimes, testing, a real CLI
project, the iterator/closure system in depth, the Cargo ecosystem, smart
pointers, and concurrency.

| # | Topic | Folder | Reference | Status |
|---|-------|--------|-----------|--------|
| 1 | Generics, Traits & Lifetimes | [`intermediate/01-generics-traits-and-lifetimes`](./intermediate/01-generics-traits-and-lifetimes) | Book ch. 10 | done |
| 2 | Writing Tests & Project Organization | [`intermediate/02-testing-and-project-organization`](./intermediate/02-testing-and-project-organization) | Book ch. 11 | done |
| 3 | CLI I/O Project (args, files, env, stderr) | [`intermediate/03-cli-io-project`](./intermediate/03-cli-io-project) | Book ch. 12 | done |
| 4 | Closures & Iterators | [`intermediate/04-closures-and-iterators`](./intermediate/04-closures-and-iterators) | Book ch. 13.1-13.2 | done |
| 5 | Custom Iterators & Adapters | [`intermediate/05-custom-iterators-and-adapters`](./intermediate/05-custom-iterators-and-adapters) | Book ch. 13.2 (deepening) | done |
| 6 | Error Handling Deep Dive (`From`, `Box<dyn Error>`) | [`intermediate/06-error-handling-deep-dive`](./intermediate/06-error-handling-deep-dive) | Book ch. 9 (deepening) | done |
| 7 | Cargo Workspaces, Profiles & Performance | [`intermediate/07-cargo-workspaces-and-profiles`](./intermediate/07-cargo-workspaces-and-profiles) | Book ch. 13.4, 14 (adapted) | done |
| 8 | Smart Pointers (`Box`, `Deref`, `Drop`, `Rc`, `RefCell`) | [`intermediate/08-smart-pointers`](./intermediate/08-smart-pointers) | Book ch. 15 | done |
| 9 | Fearless Concurrency (threads, channels, `Mutex`/`Arc`) | [`intermediate/09-fearless-concurrency`](./intermediate/09-fearless-concurrency) | Book ch. 16 | done |

## Advanced

Covers Book ch. 17-20 and the Nomicon: trait objects, pattern matching depth,
advanced traits/types/macros, unsafe Rust, data layout, advanced
lifetimes/variance, low-level concurrency primitives, async/await, and a
from-scratch `Vec`/`Arc`/`Mutex`.

| # | Topic | Folder | Reference | Status |
|---|-------|--------|-----------|--------|
| 1 | Trait Objects, Dynamic Dispatch & OOP Patterns | [`advanced/01-trait-objects-and-oop-patterns`](./advanced/01-trait-objects-and-oop-patterns) | Book ch. 18 | done |
| 2 | Patterns & Matching Deep Dive | [`advanced/02-patterns-and-matching`](./advanced/02-patterns-and-matching) | Book ch. 19 | done |
| 3 | Advanced Traits & Types | [`advanced/03-advanced-traits-and-types`](./advanced/03-advanced-traits-and-types) | Book ch. 20.2-20.3 | done |
| 4 | Advanced Functions, Closures & Macros | [`advanced/04-advanced-functions-and-macros`](./advanced/04-advanced-functions-and-macros) | Book ch. 20.4-20.5 | done |
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
plugged in. "Done" means it builds here *and* runs correctly on the board,
confirmed by the user.

Built from two guides, worked through in order:

- [**The Embedonomicon**](https://doc.rust-lang.org/embedonomicon/) —
  building a `#![no_std]`/`#![no_main]` program from scratch, applied to the
  nRF52833 instead of its QEMU target.
- [**Discovery (micro:bit v2 edition)**](https://docs.rust-embedded.org/discovery-mb2/)
  — the full board curriculum (Hello World through the Snake game) on real
  hardware via `probe-rs`.

Detailed phase/exercise breakdown is deferred until the capstone is actually
started — no need to scope it out this far ahead.

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
