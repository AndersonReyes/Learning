//! Run with: `cargo run --example examples -p intermediate-01-generics-traits-and-lifetimes`

use std::ops::Add;

// --- Generic data types --------------------------------------------------

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// A method only available when T = f64
impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// --- Traits: default methods + trait bounds ------------------------------

trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

struct Tweet {
    username: String,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    // uses the default `summarize`
}

struct Article {
    author: String,
    headline: String,
}

impl Summary for Article {
    fn summarize_author(&self) -> String {
        self.author.clone()
    }

    // overrides the default
    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.summarize_author())
    }
}

// `impl Trait` parameter -- accepts any single type implementing Summary
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// equivalent, explicit generic + trait bound
fn notify_generic<T: Summary>(item: &T) {
    println!("(generic) {}", item.summarize());
}

// `impl Trait` return type -- caller doesn't need to name the concrete type
fn make_tweet(username: &str) -> impl Summary {
    Tweet {
        username: username.to_string(),
    }
}

// --- Operator overloading via std::ops::Add ------------------------------

#[derive(Clone, Copy, Debug)]
struct Millimeters(u32);

impl Add for Millimeters {
    type Output = Millimeters;
    fn add(self, other: Millimeters) -> Millimeters {
        Millimeters(self.0 + other.0)
    }
}

// --- Lifetimes ------------------------------------------------------------

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct Excerpt<'a> {
    part: &'a str,
}

impl<'a> Excerpt<'a> {
    fn part(&self) -> &str {
        // elided: &'a str, via lifetime elision rule 3 (&self -> output)
        self.part
    }
}

fn main() {
    // --- Generic data types ---
    let numbers = vec![34, 50, 25, 100, 65];
    println!("largest number = {}", largest(&numbers));

    let floats = vec![3.4, 50.2, 25.9];
    println!("largest float = {}", largest(&floats));

    let p = Point { x: 3.0, y: 4.0 };
    println!("p.x() = {}, distance = {}", p.x(), p.distance_from_origin());

    // --- Traits ---
    let tweet = Tweet {
        username: "rustlang".to_string(),
    };
    let article = Article {
        author: "Jane Doe".to_string(),
        headline: "Rust 2.0 Announced".to_string(),
    };

    notify(&tweet);
    notify(&article);
    notify_generic(&tweet);

    let made = make_tweet("ferris");
    println!("made tweet: {}", made.summarize());

    // --- Operator overloading ---
    let total = Millimeters(10) + Millimeters(25);
    println!("total = {total:?}");

    // --- Lifetimes ---
    let s1 = String::from("long string is long");
    let s2 = String::from("short");
    println!("longest = {}", longest(&s1, &s2));

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let excerpt = Excerpt {
        part: first_sentence,
    };
    println!("excerpt.part() = {}", excerpt.part());

    // 'static -- string literals live for the whole program
    let s: &'static str = "I live forever";
    println!("{s}");
}
