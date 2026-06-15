# Toolchain, Cargo & Hello World

## Installing and managing Rust

- Install via **rustup** (the toolchain manager), not your OS package
  manager — lets you switch toolchains/targets per-project.
- `rustc` is the compiler. `cargo` is the build tool/package manager — almost
  everything in this track goes through `cargo`, not `rustc` directly.
- `rustup show` — active toolchain and installed targets.
- `rustup target add <triple>` — install a cross-compilation target (used
  later for the embedded capstone, e.g. `thumbv7m-none-eabi`).
- `rustc --version`, `cargo --version` — sanity check the install.
- `rustup update` — update the toolchain. `rustup component add clippy
  rustfmt` — linter and formatter, both included by default with `rustup`.

## Cargo basics

Cargo is a build system, package manager, test runner, and doc generator in
one.

| Command | What it does |
|---|---|
| `cargo new <name>` | scaffold a new package (binary by default; `--lib` for a library) |
| `cargo build` | compile (debug profile, `target/debug/`) |
| `cargo build --release` | compile with optimizations (`target/release/`) |
| `cargo run` | build + run the binary |
| `cargo check` | type-check without producing a binary — much faster, use while iterating |
| `cargo test` | build and run all `#[test]` functions and `tests/*.rs` integration tests |
| `cargo doc --open` | build and open HTML docs (including for your dependencies) |
| `cargo run --example <name>` | run a file under `examples/<name>.rs` |

### `Cargo.toml` vs `Cargo.lock`

- `Cargo.toml` — the manifest: package metadata, edition, dependencies
  (`[dependencies]`), dev-only dependencies (`[dev-dependencies]`).
- `Cargo.lock` — exact resolved dependency versions, for reproducible
  builds. Commit it for binaries/applications (including these exercise
  packages); libraries published to crates.io often `.gitignore` it instead.
- Dependencies come from [crates.io](https://crates.io) by default. Adding
  one is just adding a line under `[dependencies]`, e.g. `rand = "0.8"`, then
  building — Cargo fetches and compiles it (and everything it depends on)
  automatically.

### Workspaces

A `[workspace]` in a root `Cargo.toml` groups multiple packages so they share
one `Cargo.lock` and `target/` directory, and `cargo test`/`cargo build` from
the workspace root operate on all members. `rust/Cargo.toml` (one level up
from this file) is exactly this — every topic in this track is a workspace
member, hence `cargo test -p <package-name>` to target just one.

## Hello, World!

```rust
fn main() {
    println!("Hello, world!");
}
```

- `fn main()` is the entry point for a binary crate.
- `println!` is a **macro** (note the `!`), not a function — macros can take
  a variable number of arguments and do compile-time checking of the format
  string against the arguments. `format!`, `vec!`, `assert_eq!`, `panic!`,
  `todo!` are all macros for the same reason.
- No semicolon needed after `fn main() { ... }` — it's a block, not a
  statement.

## The Guessing Game (Book ch. 2) — concepts preview

The Book's second chapter builds a number-guessing game and, along the way,
previews almost every concept this track covers properly later. Skim these
now; each gets its own topic:

- **`let` and `let mut`** — bindings are immutable by default
  (`fundamentals/02`).
- **`String::new()`, `&mut`** — owned strings and mutable references
  (`fundamentals/04`, `fundamentals/05`).
- **`std::io::stdin().read_line(&mut guess)`** — reading input returns a
  `Result`; `.expect("message")` unwraps it or panics with that message
  (`fundamentals/10`).
- **Shadowing**: `let guess: u32 = guess.trim().parse().expect(...)` rebinds
  `guess` to a new type, parsing the string into a number
  (`fundamentals/02`).
- **`match` and `Ordering`**: `match guess.cmp(&secret) { Less => ..., Greater
  => ..., Equal => ... }` (`fundamentals/07`).
- **`loop` / `break`** — `fundamentals/03`.
- **External crates**: adding `rand = "0.8"` to `Cargo.toml` and calling
  `rand::thread_rng().gen_range(1..=100)`.

## This track's package layout

Every topic from here on is one Cargo package:

```
fundamentals/NN-topic-name/
  Cargo.toml          # edition = "2021", package name = fundamentals-NN-topic-name
  notes.md
  src/lib.rs          # 5 exported fns, doc comments, todo!() bodies
  examples/examples.rs  # cargo run --example examples -p <name>
  tests/exercise_test.rs  # cargo test -p <name> — the spec
```

`todo!()` is a macro that panics with `"not yet implemented"` — `cargo test`
reports a panicking test as `FAILED`, so every exercise starts red with zero
extra setup.

## Further Reading (Rust Book)

- [Ch. 1 — Getting Started](https://doc.rust-lang.org/book/ch01-00-getting-started.html)
  (Installation, Hello World, Hello Cargo)
- [Ch. 2 — Programming a Guessing Game](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html)
- [The Cargo Book — Workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html)
- [The Cargo Book — Specifying Dependencies](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html)
