# Writing Tests & Project Organization

## `#[test]` and assertion macros

```rust
#[test]
fn it_adds_two() {
    assert_eq!(2 + 2, 4);
}
```

- `assert!(expr)` — fails (panics) if `expr` is `false`.
- `assert_eq!(a, b)` / `assert_ne!(a, b)` — fail with a diff-style message
  showing both values. Requires `a`/`b` to implement `PartialEq` (to
  compare) and `Debug` (to print on failure) — `#[derive(Debug, PartialEq)]`
  on custom types.
- Custom failure message: extra args are a `format!`-style message, e.g.
  `assert!(result > 0, "expected positive, got {result}")`.
- `cargo test` runs every `#[test]` fn in the crate (lib unit tests,
  `tests/*.rs` integration tests, and doc-tests), reporting
  pass/FAILED/ignored per test.

## `#[should_panic]`

Marks a test as passing *only if* the function panics:

```rust
#[test]
#[should_panic]
fn rejects_zero() {
    divide(10, 0);
}

#[test]
#[should_panic(expected = "divisor must be nonzero")]
fn rejects_zero_with_message() {
    divide(10, 0);
}
```

`expected = "..."` checks the panic message *contains* this substring —
without it, the test passes on *any* panic, even one from an unrelated bug.
Prefer `expected` so a wrong-reason panic still fails the test.

## Tests returning `Result<(), E>`

```rust
#[test]
fn it_works() -> Result<(), String> {
    if 2 + 2 == 4 {
        Ok(())
    } else {
        Err("math is broken".to_string())
    }
}
```

Lets you use `?` inside a test to bail out on the first `Err`. **Cannot**
combine with `#[should_panic]` — returning `Err` doesn't panic, so the two
attributes are mutually exclusive.

## Controlling how tests run

```text
cargo test                       # run everything
cargo test it_adds                # only tests whose name contains "it_adds"
cargo test -- --test-threads=1    # run serially (default: parallel threads)
cargo test -- --nocapture         # show println! output even for passing tests
cargo test -- --ignored           # run only #[ignore]'d tests
```

Args after `--` go to the test binary itself, not `cargo test`. Tests run in
parallel by default and share no state — tests that touch shared resources
(files, env vars, global state) need `--test-threads=1` or their own
isolation.

```rust
#[test]
#[ignore] // excluded from the default run (e.g. slow/expensive)
fn expensive_computation() {
    // ...
}
```

## Test organization

### Unit tests: `#[cfg(test)] mod tests`

Live in the same file as the code, in a `tests` submodule gated by
`#[cfg(test)]` (only compiled when testing) with `use super::*` to access
**everything** in the parent module — including private items:

```rust
fn internal_helper(x: i32) -> i32 {
    x * 2
}

pub fn public_api(x: i32) -> i32 {
    internal_helper(x) + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal_helper_doubles() {
        assert_eq!(internal_helper(3), 6); // testing a PRIVATE function
    }
}
```

This is the main reason Rust's unit tests live alongside the code: testing
implementation details that integration tests (below) can't see.

### Integration tests: `tests/*.rs`

Each file directly under `tests/` is compiled as its **own separate crate**,
and can only call the library's `pub` API via `use <crate_name>::...` —
exactly like an external user of the crate. This is why
`tests/exercise_test.rs` throughout this repo only exercises `pub fn`s.

```rust
// tests/exercise_test.rs
use my_crate::public_api;

#[test]
fn public_api_works() {
    assert_eq!(public_api(3), 7);
}
```

Shared test helpers go in `tests/common/mod.rs` (the `mod.rs` name keeps
`cargo test` from treating `common` as its own test file) and are pulled in
with `mod common;` at the top of each integration test file that needs them.

Binary crates (`src/main.rs` with no `src/lib.rs`) **can't** have
integration tests, because there's no library API for `tests/*.rs` to
import — another reason nontrivial logic belongs in `src/lib.rs` with a
thin `main.rs` wrapper.

## Further Reading (Rust Book)

- [Ch. 11 — Writing Automated Tests](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Ch. 11.1 — How to Write Tests](https://doc.rust-lang.org/book/ch11-01-writing-tests.html)
- [Ch. 11.2 — Controlling How Tests Are Run](https://doc.rust-lang.org/book/ch11-02-running-tests.html)
- [Ch. 11.3 — Test Organization](https://doc.rust-lang.org/book/ch11-03-test-organization.html)
