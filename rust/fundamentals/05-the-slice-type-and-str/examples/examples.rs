//! Run with: `cargo run --example examples -p fundamentals-05-the-slice-type-and-str`
//!
//! Demonstrates `notes.md`'s slice range syntax, UTF-8 byte-vs-char
//! distinctions, `str` vs `String` and deref coercion, and general `&[T]`
//! slices. Self-contained: doesn't call into this package's exercises
//! (those are unimplemented `todo!()` stubs until you finish them).

fn main() {
    // --- Range syntax on a Vec ---

    let v = vec![1, 2, 3, 4, 5];
    println!("&v[1..3] = {:?}", &v[1..3]); // [2, 3]
    println!("&v[..2]  = {:?}", &v[..2]); // [1, 2]
    println!("&v[3..]  = {:?}", &v[3..]); // [4, 5]
    println!("&v[..]   = {:?}", &v[..]); // [1, 2, 3, 4, 5]

    // --- &str is a byte slice; .len() counts bytes, not chars ---

    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("hello = {hello:?}, world = {world:?}");

    let accented = "héllo";
    println!(
        "'{accented}'.len() = {} bytes, but {} chars",
        accented.len(),
        accented.chars().count()
    );

    // &accented[0..2] would panic: byte 2 is mid-character ('é' spans bytes 1-2).
    let he = &accented[0..3]; // OK: byte 3 is right after 'é'
    println!("&accented[0..3] = {he:?}");

    // --- .char_indices() for finding valid boundaries ---

    for (i, c) in accented.char_indices() {
        println!("byte {i}: {c}");
    }

    // --- str vs String, and deref coercion ---

    print_it(&accented.to_string()); // &String -> &str via deref coercion
    print_it("a literal &str");

    // --- General slices &[T] ---

    let pair = first_two(&v);
    println!("first_two(&v) = {pair:?}");

    // --- Mutable slices ---

    let mut numbers = vec![3, 1, 2];
    numbers[..].sort();
    println!("sorted: {numbers:?}");
}

/// Takes `&str` — accepts both `&String` (via deref coercion) and `&str`.
fn print_it(s: &str) {
    println!("print_it: {s}");
}

/// Returns a slice borrowing the first two elements of `s`.
fn first_two(s: &[i32]) -> &[i32] {
    &s[..2]
}
