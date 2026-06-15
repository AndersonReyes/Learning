//! Run with: `cargo run --example examples -p fundamentals-03-control-flow`
//!
//! Demonstrates `notes.md`'s `if`/`else` as an expression, `loop`/`while`/
//! `for`, `break`/`continue`, and loop labels for nested loops.
//! Self-contained: doesn't call into this package's exercises (those are
//! unimplemented `todo!()` stubs until you finish them).

fn main() {
    // --- `if`/`else` as an expression ---

    let n = 7;
    let parity = if n % 2 == 0 { "even" } else { "odd" };
    println!("{n} is {parity}");

    // `else if` chain.
    let score = 82;
    let grade = if score >= 90 {
        "A"
    } else if score >= 80 {
        "B"
    } else if score >= 70 {
        "C"
    } else {
        "F"
    };
    println!("score {score} -> grade {grade}");

    // --- `loop` + `break value` ---

    let mut i = 0;
    let first_multiple_of_7_over_50 = loop {
        i += 7;
        if i > 50 {
            break i;
        }
    };
    println!("first multiple of 7 over 50: {first_multiple_of_7_over_50}");

    // --- `while` ---

    let mut countdown = 3;
    while countdown > 0 {
        println!("countdown: {countdown}");
        countdown -= 1;
    }

    // --- `for` over ranges, and array iteration by value vs reference ---

    for i in 0..3 {
        println!("0..3 -> {i}");
    }
    for i in 0..=3 {
        println!("0..=3 -> {i}");
    }

    let arr = [10, 20, 30];
    for x in arr {
        println!("by value: {x}");
    }
    println!("arr still usable: {arr:?}");
    for x in &arr {
        println!("by reference: {x}");
    }

    // --- `continue` ---

    print!("odd numbers below 10:");
    for i in 0..10 {
        if i % 2 == 0 {
            continue;
        }
        print!(" {i}");
    }
    println!();

    // --- Loop labels for nested loops ---

    let mut found = None;
    'outer: for i in 0..5 {
        for j in 0..5 {
            if i * j == 6 {
                found = Some((i, j));
                break 'outer;
            }
        }
    }
    println!("first (i, j) with i*j == 6: {found:?}");
}
