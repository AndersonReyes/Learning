//! Run with: `cargo run --example examples -p fundamentals-09-common-collections`

use std::collections::{HashMap, HashSet};

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    // --- Vec<T> ---------------------------------------------------------

    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    let v2 = vec![10, 20, 30];
    println!("v = {v:?}, v2 = {v2:?}");

    // Indexing vs. get
    let third = v2[2];
    let maybe_fourth = v2.get(3);
    println!("v2[2] = {third}, v2.get(3) = {maybe_fourth:?}");

    // Iterating by reference (read-only)
    let mut sum = 0;
    for x in &v {
        sum += x;
    }
    println!("sum of v by &v = {sum}");

    // Iterating by mutable reference (mutate in place)
    let mut counters = vec![1, 2, 3];
    for x in &mut counters {
        *x *= 10;
    }
    println!("counters after &mut iteration = {counters:?}");

    // Iterating by value (consumes the Vec, yields owned elements)
    let owned_strings = vec![String::from("a"), String::from("b")];
    let mut joined = String::new();
    for s in owned_strings {
        joined.push_str(&s);
    }
    println!("joined owned strings = {joined}");

    // Ownership gotcha: you can't hold &v[0] across a push that may
    // reallocate. This would fail to compile:
    //
    // let first = &v[0];
    // v.push(4); // error: cannot borrow `v` as mutable while borrowed
    // println!("{first}");
    //
    // Fix: finish using the reference before mutating, e.g. clone/copy it,
    // or push first and borrow after.

    // Enums let a Vec hold "different types" via one enum
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    for cell in &row {
        match cell {
            SpreadsheetCell::Int(n) => println!("cell = {cell:?} (int value: {n})"),
            SpreadsheetCell::Float(f) => println!("cell = {cell:?} (float value: {f})"),
            SpreadsheetCell::Text(t) => println!("cell = {cell:?} (text value: {t})"),
        }
    }

    // --- String ----------------------------------------------------------

    let mut s = String::new();
    s.push_str("foo");
    s.push('!');
    println!("s = {s}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 is moved here; s2 is only borrowed
    println!("s3 = {s3}");
    // s1 is no longer usable; s2 still is:
    println!("s2 still usable = {s2}");

    let s4 = format!("{s3} again");
    println!("s4 = {s4}");

    // No indexing: s4[0] would not compile. Use slicing, .chars(), etc.
    let first_word = &s4[0..5];
    println!("first 5 bytes of s4 = {first_word}");

    // --- HashMap<K, V> -----------------------------------------------------

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let blue_score = scores.get("Blue");
    println!("Blue score (Option<&i32>) = {blue_score:?}");

    let red_score = scores.get("Red").copied().unwrap_or(0);
    println!("Red score (default 0) = {red_score}");

    // Overwrite vs. insert-only-if-absent
    scores.insert(String::from("Blue"), 25); // last insert wins
    scores.entry(String::from("Blue")).or_insert(100); // no-op, key exists
    scores.entry(String::from("Green")).or_insert(100); // inserted
    println!("Blue after overwrite+entry = {:?}", scores.get("Blue"));
    println!("Green after entry = {:?}", scores.get("Green"));

    // Iteration order is unspecified -- sort keys for stable output.
    let mut entries: Vec<(&String, &i32)> = scores.iter().collect();
    entries.sort_by_key(|(k, _)| (*k).clone());
    for (key, value) in entries {
        println!("{key}: {value}");
    }

    // Classic word-counter idiom with entry().or_insert()
    let text = "the quick brown fox jumps over the lazy dog the fox";
    let mut word_counts: HashMap<&str, i32> = HashMap::new();
    for word in text.split_whitespace() {
        *word_counts.entry(word).or_insert(0) += 1;
    }
    println!("'the' count = {}", word_counts["the"]);
    println!("'fox' count = {}", word_counts["fox"]);

    // --- HashSet<T> --------------------------------------------------------

    let mut seen = HashSet::new();
    for word in text.split_whitespace() {
        if seen.insert(word) {
            println!("first time seeing: {word}");
        }
    }
    println!("distinct words = {}", seen.len());
    println!("contains 'fox'? {}", seen.contains("fox"));
}
