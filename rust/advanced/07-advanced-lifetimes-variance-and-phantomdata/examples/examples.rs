//! Run with: `cargo run --example examples -p advanced-07-advanced-lifetimes-variance-and-phantomdata`

use std::marker::PhantomData;

// ---------------------------------------------------------------------------
// Lifetime in structs
// ---------------------------------------------------------------------------

struct Excerpt<'a> {
    part: &'a str,
}

impl<'a> Excerpt<'a> {
    fn display(&self) -> &str {
        self.part
    }
}

// ---------------------------------------------------------------------------
// Two lifetime parameters
// ---------------------------------------------------------------------------

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() { x } else { y }
}

// ---------------------------------------------------------------------------
// PhantomData branding
// ---------------------------------------------------------------------------

#[derive(Debug, Copy, Clone)]
struct UserId;
#[derive(Debug, Copy, Clone)]
struct OrderId;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Id<Brand> {
    value: u64,
    _brand: PhantomData<Brand>,
}

impl<Brand> Id<Brand> {
    fn new(v: u64) -> Self {
        Id { value: v, _brand: PhantomData }
    }
}

// ---------------------------------------------------------------------------
// Splitting borrows
// ---------------------------------------------------------------------------

struct Config {
    host: String,
    port: String,
}

fn split_config(c: &mut Config) -> (&mut String, &mut String) {
    (&mut c.host, &mut c.port)
}

// ---------------------------------------------------------------------------
// HRTB
// ---------------------------------------------------------------------------

fn transform_each<T, F>(items: &[T], f: F) -> Vec<&T>
where
    F: for<'a> Fn(&'a T) -> &'a T,
{
    items.iter().map(|x| f(x)).collect()
}

fn main() {
    // --- lifetimes in structs ------------------------------------------------
    println!("-- lifetime in struct --");
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence;
    {
        let i = novel.find('.').unwrap_or(novel.len());
        first_sentence = &novel[..i];
    }
    let e = Excerpt { part: first_sentence };
    println!("excerpt: {:?}", e.display());

    // --- two lifetime parameters ---------------------------------------------
    println!("\n-- two lifetime params --");
    let s1 = String::from("long string");
    let result;
    {
        let s2 = String::from("xy");
        result = longest(s1.as_str(), s2.as_str());
        println!("longest: {}", result);
    }

    // --- PhantomData branding ------------------------------------------------
    println!("\n-- PhantomData branding --");
    let uid: Id<UserId>  = Id::new(1);
    let oid: Id<OrderId> = Id::new(1);
    println!("user id:  {:?}", uid);
    println!("order id: {:?}", oid);
    // uid == oid would be a type error — they're different types

    // --- splitting borrows ---------------------------------------------------
    println!("\n-- splitting borrows --");
    let mut cfg = Config { host: "localhost".into(), port: "8080".into() };
    let (host, port) = split_config(&mut cfg);
    host.push_str(":extra");
    port.push_str("-dev");
    println!("host: {}, port: {}", cfg.host, cfg.port);

    // --- HRTB ---------------------------------------------------------------
    println!("\n-- HRTB for<'a> --");
    let nums = vec![10_i32, 20, 30];
    let refs = transform_each(&nums, |x| x);
    for r in refs {
        println!("  {}", r);
    }
}
