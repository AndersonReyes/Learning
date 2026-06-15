//! Run with: `cargo run --example examples -p fundamentals-04-ownership-and-borrowing`
//!
//! Demonstrates `notes.md`'s move semantics, `Clone`/`Copy`, `&T`/`&mut T`
//! borrowing, and the borrow-checker's non-lexical-lifetime rules.
//! Self-contained: doesn't call into this package's exercises (those are
//! unimplemented `todo!()` stubs until you finish them).

fn main() {
    // --- Move semantics ---

    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved into s2
    // println!("{s1}"); // would not compile: "value borrowed here after move"
    println!("s2 = {s2}");

    // --- `Clone` — explicit deep copy ---

    let s3 = s2.clone();
    println!("s2 = {s2}, s3 = {s3}"); // both valid: separate heap buffers

    // --- `Copy` — simple scalar types don't move ---

    let x = 5;
    let y = x; // copy, not move
    println!("x = {x}, y = {y}"); // both valid: i32 is Copy

    // --- Ownership and functions ---

    let owned = String::from("consume me");
    let len = takes_ownership(owned);
    // println!("{owned}"); // would not compile: owned was moved
    println!("len = {len}");

    // --- `&T` — shared/immutable borrow ---

    let s = String::from("hello");
    let len = calculate_length(&s);
    println!("'{s}' has length {len}"); // s still valid: only borrowed

    // --- `&mut T` — exclusive/mutable borrow ---

    let mut s = String::from("hello");
    append_world(&mut s);
    println!("{s}");

    // --- Non-lexical lifetimes: a borrow ends at its last use ---

    let mut v = vec![1, 2, 3];
    let first = v[0];
    println!("first = {first}"); // last use of `first` (a copy, since i32: Copy)
    v.push(4); // OK: nothing still borrows v immutably
    println!("v = {v:?}");

    // --- Mutating through `&mut`: reassigning via `*` ---

    let mut numbers = vec![1, 2, 3];
    replace_with_doubles(&mut numbers);
    println!("numbers = {numbers:?}");
}

/// Takes ownership of `s` — `s` is dropped at the end of this function.
fn takes_ownership(s: String) -> usize {
    s.len()
}

/// Borrows `s` — caller retains ownership.
fn calculate_length(s: &String) -> usize {
    s.len()
}

/// Mutably borrows `s` and appends to it in place.
fn append_world(s: &mut String) {
    s.push_str(" world");
}

/// Replaces the contents of `values` with each element doubled, by building
/// a new `Vec` and assigning through the `&mut` reference with `*`.
fn replace_with_doubles(values: &mut Vec<i32>) {
    let doubled: Vec<i32> = values.iter().map(|v| v * 2).collect();
    *values = doubled;
}
