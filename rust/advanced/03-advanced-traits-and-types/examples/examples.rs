//! Run with: `cargo run --example examples -p advanced-03-advanced-traits-and-types`

use std::collections::HashMap;

// ---------------------------------------------------------------------------
// Associated types
// ---------------------------------------------------------------------------

trait Distance {
    type Output: std::fmt::Display;
    fn distance_from_origin(&self) -> Self::Output;
}

struct Point2D(f64, f64);
struct GridPoint(i32, i32);

impl Distance for Point2D {
    type Output = f64;
    fn distance_from_origin(&self) -> f64 {
        (self.0 * self.0 + self.1 * self.1).sqrt()
    }
}

impl Distance for GridPoint {
    type Output = u32;
    fn distance_from_origin(&self) -> u32 {
        self.0.unsigned_abs() + self.1.unsigned_abs()
    }
}

fn print_distance<T: Distance>(t: &T) {
    println!("distance: {}", t.distance_from_origin());
}

// ---------------------------------------------------------------------------
// Operator overloading
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug)]
struct Vec2(f64, f64);

impl std::ops::Add for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl std::ops::Mul<f64> for Vec2 {
    type Output = Vec2;
    fn mul(self, s: f64) -> Vec2 {
        Vec2(self.0 * s, self.1 * s)
    }
}

impl std::fmt::Display for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

// ---------------------------------------------------------------------------
// Newtype pattern + Display
// ---------------------------------------------------------------------------

struct Meters(f64);

impl std::fmt::Display for Meters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}m", self.0)
    }
}

impl std::ops::Add for Meters {
    type Output = Meters;
    fn add(self, rhs: Meters) -> Meters {
        Meters(self.0 + rhs.0)
    }
}

// ---------------------------------------------------------------------------
// Supertraits
// ---------------------------------------------------------------------------

trait Animal: std::fmt::Display {
    fn sound(&self) -> &str;
    fn describe(&self) -> String {
        format!("{} says '{}'", self, self.sound())
    }
}

struct Dog { name: String }

impl std::fmt::Display for Dog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Dog({})", self.name)
    }
}

impl Animal for Dog {
    fn sound(&self) -> &str { "woof" }
}

// ---------------------------------------------------------------------------
// Fully qualified syntax
// ---------------------------------------------------------------------------

trait Printer {
    fn print(&self);
}

trait Logger {
    fn print(&self);
}

struct Server;

impl Printer for Server {
    fn print(&self) { println!("[Printer] Server output"); }
}

impl Logger for Server {
    fn print(&self) { println!("[Logger] Server log"); }
}

fn main() {
    println!("-- associated types --");
    print_distance(&Point2D(3.0, 4.0));  // 5.0
    print_distance(&GridPoint(3, 4));    // 7 (Manhattan)

    println!("\n-- operator overloading --");
    let a = Vec2(1.0, 2.0);
    let b = Vec2(3.0, 4.0);
    println!("{} + {} = {}", a, b, a + b);
    println!("{} * 2 = {}", a, a * 2.0);

    println!("\n-- newtype pattern --");
    let d1 = Meters(5.0);
    let d2 = Meters(3.0);
    let sum = Meters(d1.0 + d2.0);
    println!("{}m + {}m = {}", d1.0, d2.0, sum);

    println!("\n-- supertraits --");
    let dog = Dog { name: "Rex".into() };
    println!("{}", dog.describe());

    println!("\n-- fully qualified syntax --");
    let s = Server;
    <Server as Printer>::print(&s);
    <Server as Logger>::print(&s);

    println!("\n-- newtype wrapping HashMap --");
    let mut freq: HashMap<String, usize> = HashMap::new();
    for word in ["rust", "go", "rust", "python", "rust", "go"] {
        *freq.entry(word.to_string()).or_insert(0) += 1;
    }
    let mut entries: Vec<_> = freq.iter().collect();
    entries.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
    for (w, c) in &entries {
        println!("{}: {}", w, c);
    }
}
