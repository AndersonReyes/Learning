//! Run with: `cargo run --example examples -p fundamentals-01-toolchain-cargo-and-hello-world`
//!
//! Demonstrates Hello World plus the std::io / shadowing / match / loop
//! ideas previewed in `notes.md` via the Guessing Game — without needing
//! the `rand` crate or interactive input. Self-contained: doesn't call into
//! this package's exercises (those are unimplemented `todo!()` stubs until
//! you finish them).

use std::cmp::Ordering;
use std::io::BufRead;
use std::io::Cursor;

fn main() {
    // The classic.
    println!("Hello, world!");

    // `println!` is a macro: it checks the format string against its
    // arguments at compile time.
    let name = "Rustacean";
    let topic = 1;
    println!("Hello, {name}! This is fundamentals topic {topic}.");

    // `let mut` + reading a line of input. `Cursor` lets us feed
    // `std::io::BufRead::read_line` from an in-memory buffer instead of
    // stdin, so this example runs the same with no terminal attached.
    let mut input = Cursor::new(b"42\n");
    let mut guess = String::new();
    input.read_line(&mut guess).expect("failed to read line");

    // Shadowing: `guess` goes from `String` to `u32`.
    let guess: u32 = guess.trim().parse().expect("not a number");
    println!("Parsed guess: {guess}");

    // `match` on `Ordering`, exactly like the Guessing Game's comparison
    // against the secret number.
    let secret = 50;
    match guess.cmp(&secret) {
        Ordering::Less => println!("{guess} is less than {secret}"),
        Ordering::Greater => println!("{guess} is greater than {secret}"),
        Ordering::Equal => println!("{guess} equals {secret}"),
    }

    // `loop` + `break` with a value.
    let mut exponent = 0;
    let first_power_of_two_over_100 = loop {
        exponent += 1;
        let value = 2_u32.pow(exponent);
        if value > 100 {
            break value;
        }
    };
    println!("First power of two over 100: {first_power_of_two_over_100}");
}
