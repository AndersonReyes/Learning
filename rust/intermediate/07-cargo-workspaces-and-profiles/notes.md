# Cargo Workspaces, Profiles & Iterator Performance

Book ch. 13.4 (loops vs. iterators) and ch. 14 (more about Cargo). The
exercises below are entirely ch. 13.4 — workspaces/profiles/publishing aren't
things a unit test can assert on, so this topic's `src/lib.rs` instead
practices writing **iterator-chain implementations** of problems that would
otherwise be hand-rolled loops. `rust/Cargo.toml` (this whole track) *is* a
live workspace example for the Cargo-side material below.

## ch. 13.4 — Loops vs. Iterators: zero-cost abstractions

The book's running example: a `search` function written two ways.

```rust
// Loop version
fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

// Iterator-chain version
fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}
```

Both compile to **roughly the same assembly**. Rust's iterators are a
*zero-cost abstraction*: the compiler unrolls loops, inlines closures passed
to `map`/`filter`/etc., and in some cases eliminates bounds checks that the
loop version couldn't (the iterator never indexes with `[i]`, so there's
nothing to check). Iterator chains aren't "slower but nicer" — they're
typically **at least as fast**, and often clearer about *what* is being
computed rather than *how*.

This topic's exercises are all "translate a stateful loop into an iterator
chain" problems. The patterns below are the toolbox.

### `.windows(n)` — sliding-window views

A `&[T]` method (not an `Iterator` adaptor) returning overlapping slices of
length `n`. Empty if `n > slice.len()`; panics if `n == 0`.

```rust
let v = [1, 2, 3, 4];
let windows: Vec<&[i32]> = v.windows(2).collect();
// [[1,2], [2,3], [3,4]]
```

Pairs naturally with `.fold()` (carry running state across windows) or
`.filter()`/`.map()` (stateless per-window computation).

### `.fold(init, f)` — the general-purpose "loop with an accumulator"

```rust
let total = [1, 2, 3].iter().fold(0, |acc, x| acc + x); // 6
```

Any loop of the shape `let mut acc = init; for x in iter { acc = f(acc, x); }
acc` is a `.fold()`. The accumulator can be a tuple to track multiple running
values at once (e.g. `(longest_so_far, current_run_length)`).

**Floats and min/max**: `f64` doesn't implement `Ord` (NaN breaks total
ordering), so `.min()`/`.max()`/`.iter().max()` don't work on `f64` iterators.
Use `.fold()` with `f64::min`/`f64::max` (which define NaN handling
explicitly):

```rust
let min = data.iter().copied().fold(f64::INFINITY, f64::min);
let max = data.iter().copied().fold(f64::NEG_INFINITY, f64::max);
```

### `.scan(init_state, f)` — stateful `.map()`

`.scan()` is `.map()` where the closure also gets `&mut` access to a running
state, and returning `None` stops iteration early (like `.take_while()` fused
with `.map()`).

```rust
// Running totals (cumulative sum)
let cumsum: Vec<i32> = [1, 2, 3, 4]
    .iter()
    .scan(0, |total, &x| {
        *total += x;
        Some(*total)
    })
    .collect();
// [1, 3, 6, 10]
```

A loop computing `output[i] = f(output[i-1], input[i])` (e.g. an exponential
moving average) is a `.scan()` whose first element needs special-casing —
`.scan()` always calls the closure on the *first* input element too, so seed
the state with the first element and `.chain()` it back in via
`std::iter::once`:

```rust
let mut it = data.iter();
let first = *it.next().unwrap();
let rest = it.scan(first, |state, &x| {
    *state = /* combine *state and x */ x;
    Some(*state)
});
let result: Vec<f64> = std::iter::once(first).chain(rest).collect();
```

### `.zip()`, `.flat_map()`, `.chain()` — combining sequences

- `.zip()` stops at the **shorter** of the two iterators — leftover elements
  from the longer one are dropped unless handled separately.
- `.flat_map(f)` is `.map(f).flatten()` — `f` returns an iterator/array per
  element, and all of them are concatenated.
- `.chain()` appends one iterator after another; **both must have the same
  `Item` type**.

```rust
// Interleave two equal-length slices: [1,3,5] + [2,4,6] -> [1,2,3,4,5,6]
let interleaved: Vec<i32> = a.iter().zip(b.iter())
    .flat_map(|(&x, &y)| [x, y])
    .collect();
```

For unequal lengths, `.zip()` the common prefix, then `.chain()` the
remainder of whichever slice is longer. Both branches of an `if`/`else` must
produce the *same iterator type* — `a[n..].iter().copied()` and
`b[n..].iter().copied()` are both `Copied<slice::Iter<i32>>`, so an `if`/
`else` choosing between them type-checks.

### Gotchas

- `.windows(0)` **panics** — guard `n == 0` before calling it.
- `.windows(n)` with `n > slice.len()` yields **zero** windows, not an error
  — don't forget the empty case downstream.
- `.scan()`'s closure runs on **every** element including the first; if your
  recurrence needs the first output to equal the first input unchanged,
  special-case it (see above) rather than baking it into the closure.
- `f64: !Ord` — `.max()`/`.min()`/`Iterator::max` don't compile on `f64`
  iterators. Use `.fold()` with `f64::max`/`f64::min`, or `.partial_cmp()`.
- `.zip()` silently truncates to the shorter iterator — a common source of
  "missing last element" bugs when lengths can differ.

## ch. 14 — More about Cargo (conceptual; this track's `Cargo.toml` is a live example)

### 14.1 — Release profiles

`[profile.dev]` (used by `cargo build`/`cargo run`) and `[profile.release]`
(used with `--release`) control optimization level (`opt-level`, 0-3),
debug info, etc. Override defaults in `Cargo.toml`:

```toml
[profile.release]
opt-level = 3
debug = false

[profile.dev]
opt-level = 1  # speed up debug builds of hot code
```

### 14.2 — Publishing to crates.io

- `//!` doc comments document the *containing item* (module/crate root);
  `///` document the *following item*. `cargo doc --open` renders them.
- Doc comments support runnable examples in ` ```rust ` fences — `cargo test`
  runs these as doctests (this track uses ` ```ignore ` for examples that
  reference `todo!()`-stub functions, since those would panic).
- `pub use some_module::Thing;` re-exports `Thing` at the crate root —
  useful when your internal module layout differs from the public API you
  want consumers to `use`.
- Crate metadata (`description`, `license`, `version`) goes in
  `[package]`. `cargo publish` uploads to crates.io; versions are immutable
  once published — `cargo yank` hides a version from new dependents without
  deleting it (existing `Cargo.lock` files can still use it).

### 14.3 — Workspaces

A `[workspace]` table (in a *root* `Cargo.toml` with no `[package]` of its
own) groups multiple packages that share one `Cargo.lock` and one `target/`
directory:

```toml
# rust/Cargo.toml
[workspace]
members = ["fundamentals/*", "intermediate/*", "advanced/*"]
```

Every topic in this track (`fundamentals-NN-*`, `intermediate-NN-*`, ...) is
a workspace member — a separate package with its own `Cargo.toml`, but built
and tested together. `cargo build`/`cargo test` from the workspace root
(`rust/`) operates on **all** members; `-p <name>` targets one. Workspace
members can depend on each other via `path = "../other-member"`.

### 14.4 — `cargo install`

`cargo install <crate>` builds a crate's binary target(s) in release mode
and copies them to `~/.cargo/bin` (must be on `PATH`). Only installs binaries
(`src/bin/`, `[[bin]]`), not libraries.

### 14.5 — Custom subcommands

Any executable named `cargo-foo` on `PATH` is invokable as `cargo foo`. This
is how tools like `cargo-watch` or `cargo-edit` integrate — no special Cargo
configuration needed, just the naming convention.

## Further Reading (Book)

- [Ch. 13.4 — Comparing Performance: Loops vs. Iterators](https://doc.rust-lang.org/book/ch13-04-performance.html)
- [Ch. 14.1 — Customizing Builds with Release Profiles](https://doc.rust-lang.org/book/ch14-01-release-profiles.html)
- [Ch. 14.2 — Publishing a Crate to Crates.io](https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html)
- [Ch. 14.3 — Cargo Workspaces](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html)
- [Ch. 14.4 — Installing Binaries with `cargo install`](https://doc.rust-lang.org/book/ch14-04-installing-binaries.html)
- [Ch. 14.5 — Extending Cargo with Custom Commands](https://doc.rust-lang.org/book/ch14-05-extending-cargo.html)
- [`slice::windows`](https://doc.rust-lang.org/std/primitive.slice.html#method.windows)
- [`Iterator::fold`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold)
- [`Iterator::scan`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.scan)
- [`Iterator::flat_map`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.flat_map)
- [`Iterator::chain`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.chain)
- [`std::iter::once`](https://doc.rust-lang.org/std/iter/fn.once.html)
