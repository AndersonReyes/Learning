//! Run with: `cargo run --example examples -p intermediate-02-testing-and-project-organization`
//!
//! `notes.md` is about *how `cargo test` works*, which isn't something a
//! `cargo run` binary can demonstrate directly. Instead, this file
//! demonstrates the underlying mechanisms `cargo test` relies on:
//! `assert!`/`assert_eq!`/`assert_ne!` as ordinary macros, `panic!` +
//! `catch_unwind` (what `#[should_panic]` checks under the hood), the
//! private-helper-plus-public-API shape that `#[cfg(test)] mod tests {
//! use super::*; }` can reach into, and the `Result<(), E>`-returning test
//! pattern from ch.11.1.

use std::panic;

// --- Private helper + public API -------------------------------------------
//
// A `#[cfg(test)] mod tests { use super::*; ... }` in this file could call
// `normalize` directly, even though it isn't `pub` -- exactly like
// `internal_helper` in notes.md. An integration test in `tests/` could not;
// it would only see `greet`, if this were a library.

/// Not `pub` -- only visible within this crate/binary.
fn normalize(name: &str) -> String {
    name.trim().to_lowercase()
}

/// Public-facing API, built on top of the private helper.
fn greet(name: &str) -> String {
    format!("Hello, {}!", normalize(name))
}

// --- assert! / assert_eq! / assert_ne! as ordinary macros -------------------

fn demo_assertions() {
    let sum = 2 + 2;
    assert_eq!(sum, 4); // would panic with a diff-style message if this failed
    assert!(sum > 0, "expected positive, got {sum}"); // custom failure message
    assert_ne!(sum, 5);
    println!("assert_eq!/assert!/assert_ne! all passed for sum = {sum}");
}

// --- panic! + catch_unwind: the mechanism behind #[should_panic] ------------

fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("divisor must be nonzero");
    }
    a / b
}

fn demo_should_panic_mechanism() {
    // `#[should_panic(expected = "...")]` runs the test body and checks that
    // it panicked with a message *containing* `expected`. `catch_unwind` lets
    // us perform that same check at runtime instead of via a test attribute.
    //
    // Suppress the default "thread panicked at ..." printout so this demo's
    // output stays clean -- `catch_unwind` already gives us the payload.
    let previous_hook = panic::take_hook();
    panic::set_hook(Box::new(|_| {}));
    let result = panic::catch_unwind(|| divide(10, 0));
    panic::set_hook(previous_hook);

    match result {
        Ok(value) => println!("divide(10, 0) returned {value} (unexpected!)"),
        Err(payload) => {
            let message = payload
                .downcast_ref::<&str>()
                .copied()
                .unwrap_or("<non-string panic payload>");
            println!("divide(10, 0) panicked with: {message:?}");
            assert!(message.contains("divisor must be nonzero"));
            println!(
                "  -> message contains the expected substring, as #[should_panic(expected = ..)] would check"
            );
        }
    }
}

// --- Tests returning Result<(), E> -------------------------------------------

/// Mirrors a `#[test] fn ... -> Result<(), String>` body (ch.11.1): `Ok(())`
/// on success, `Err(message)` to report a failure without panicking. Inside
/// a real test this would let you use `?` to bail out on the first problem.
fn check_no_duplicates(values: &[i32]) -> Result<(), String> {
    for i in 0..values.len() {
        for j in (i + 1)..values.len() {
            if values[i] == values[j] {
                return Err(format!(
                    "duplicate value {} at indices {i} and {j}",
                    values[i]
                ));
            }
        }
    }
    Ok(())
}

fn main() {
    // --- private helper + public API ---
    println!("greet(\"  Ferris \") = {:?}", greet("  Ferris "));
    println!("normalize(\"  Ferris \") = {:?}", normalize("  Ferris "));

    // --- assert! family ---
    demo_assertions();

    // --- panic! + catch_unwind ---
    demo_should_panic_mechanism();

    // --- Result-returning check pattern ---
    match check_no_duplicates(&[1, 2, 3]) {
        Ok(()) => println!("check_no_duplicates(&[1, 2, 3]) -> Ok(())"),
        Err(e) => println!("check_no_duplicates(&[1, 2, 3]) -> Err({e:?})"),
    }
    match check_no_duplicates(&[1, 2, 2]) {
        Ok(()) => println!("check_no_duplicates(&[1, 2, 2]) -> Ok(())"),
        Err(e) => println!("check_no_duplicates(&[1, 2, 2]) -> Err({e:?})"),
    }
}
