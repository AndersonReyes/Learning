//! Run with: `cargo run --example examples -p fundamentals-07-enums-and-pattern-matching`
//!
//! Demonstrates `notes.md`'s enum/pattern-matching concepts: enum variants
//! (unit, tuple, struct-like), recursive enums via `Box`, `Option<T>`,
//! `match` (ranges, `|`, guards, tuple patterns, match ergonomics on
//! references), `if let`/`while let`, `matches!`, and the `?` operator on
//! `Option`. Self-contained: doesn't call into this package's exercises
//! (those are unimplemented `todo!()` stubs until you finish them).

// --- Enum with mixed variant kinds ---

#[derive(Debug)]
enum Shape {
    Point,
    Circle(f64),
    Rectangle { w: f64, h: f64 },
}

fn area(shape: &Shape) -> f64 {
    match shape {
        Shape::Point => 0.0,
        Shape::Circle(r) => std::f64::consts::PI * r * r,
        Shape::Rectangle { w, h } => w * h,
    }
}

// --- Recursive enum via Box ---

enum Tree {
    Leaf(i32),
    Node(Box<Tree>, Box<Tree>),
}

fn sum_tree(t: &Tree) -> i32 {
    match t {
        Tree::Leaf(v) => *v,
        Tree::Node(l, r) => sum_tree(l) + sum_tree(r),
    }
}

// --- Compass direction, for match-ergonomics + matches! demos ---

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

// `dir: &Direction` — patterns are written without `&`, thanks to match
// ergonomics.
fn describe(dir: &Direction) -> &'static str {
    match dir {
        Direction::North => "up",
        Direction::South => "down",
        _ => "sideways",
    }
}

// --- ? on Option ---

fn add_options(a: Option<i32>, b: Option<i32>) -> Option<i32> {
    Some(a? + b?)
}

fn main() {
    // --- match over mixed enum variants ---

    let shapes = [
        Shape::Point,
        Shape::Circle(2.0),
        Shape::Rectangle { w: 3.0, h: 4.0 },
    ];
    for shape in &shapes {
        println!("{shape:?} has area {:.2}", area(shape));
    }

    // --- range patterns and `|` ---

    for n in [-5, 0, 2, 7, 42] {
        let label = match n {
            i32::MIN..=-1 => "negative",
            0 => "zero",
            1 | 2 | 3 => "small",
            4..=9 => "medium",
            _ => "large",
        };
        println!("{n} is {label}");
    }

    // --- Option, binding, match guards ---

    let ages: [Option<u32>; 3] = [Some(8), Some(42), None];
    for age in ages {
        match age {
            Some(a) if a >= 18 => println!("adult, age {a}"),
            Some(a) => println!("minor, age {a}"),
            None => println!("unknown age"),
        }
    }

    // --- if let / else ---

    let config: Option<u32> = Some(8);
    if let Some(max) = config {
        println!("max = {max}");
    } else {
        println!("no config");
    }

    // --- while let, draining a Vec ---

    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        println!("popped {top}");
    }

    // --- Recursive enum via Box ---

    let tree = Tree::Node(
        Box::new(Tree::Leaf(1)),
        Box::new(Tree::Node(Box::new(Tree::Leaf(2)), Box::new(Tree::Leaf(3)))),
    );
    println!("sum_tree = {}", sum_tree(&tree));

    // --- match ergonomics on references, matches! ---

    for dir in [Direction::North, Direction::South, Direction::East, Direction::West] {
        println!(
            "describe(&{dir:?}) = {}, matches!(dir, North) = {}",
            describe(&dir),
            matches!(dir, Direction::North)
        );
    }

    // --- ? operator on Option ---

    println!("add_options(Some(2), Some(3)) = {:?}", add_options(Some(2), Some(3)));
    println!("add_options(Some(2), None) = {:?}", add_options(Some(2), None));
}
