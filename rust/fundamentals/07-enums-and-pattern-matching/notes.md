# Enums & Pattern Matching

## Defining enums

An enum's variants can carry no data, a tuple of data, or named fields —
all variants share one type:

```rust
enum Shape {
    Point,                          // unit variant — no data
    Circle(f64),                    // tuple variant — radius
    Rectangle { w: f64, h: f64 },   // struct-like variant — named fields
}

let c = Shape::Circle(2.0);
let r = Shape::Rectangle { w: 3.0, h: 4.0 };
```

### Recursive enums need `Box`

A variant can't directly contain another value of the same enum — that
would make the type infinitely large. Indirection via `Box<T>` (a heap
pointer, fixed size regardless of what it points to) breaks the cycle:

```rust
enum Expr {
    Num(f64),
    Add(Box<Expr>, Box<Expr>), // Box<Expr> has a known fixed size
}

let e = Expr::Add(Box::new(Expr::Num(1.0)), Box::new(Expr::Num(2.0)));
```

## `Option<T>`

Rust has no `null`. Instead, anything that might be absent is
`Option<T>`, defined as:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

`Option<T>` and its variants are in the prelude — write `Some(x)` / `None`
directly, no `Option::` prefix needed. The compiler forces you to handle
the `None` case before you can use the inner value, eliminating null-pointer
bugs at compile time.

## `match`

`match` compares a value against patterns top-to-bottom and runs the first
arm that matches. **It must be exhaustive** — the compiler rejects a `match`
that doesn't cover every possible value (use `_` as a catch-all).

```rust
let n = 5;
let label = match n {
    0 => "zero",
    1 | 2 | 3 => "small",       // multiple patterns with `|`
    4..=9 => "medium",          // inclusive range pattern
    _ => "large",               // catch-all (required for exhaustiveness)
};
```

### Binding values out of variants

```rust
let maybe_age: Option<u32> = Some(42);
match maybe_age {
    Some(age) if age >= 18 => println!("adult, age {age}"), // match guard
    Some(age) => println!("minor, age {age}"),
    None => println!("unknown"),
}
```

- `Some(age)` binds the inner value to `age` for that arm only.
- `if age >= 18` is a **match guard** — extra `bool` condition. Guards don't
  count toward exhaustiveness (the compiler still requires a plain `Some(age)`
  fallback, as above).

### Matching on references — match ergonomics

When you `match` on a `&T` (e.g. iterating `for x in &vec`, or a method
taking `&self`), you can write the *unreferenced* patterns directly —
the compiler automatically adjusts bindings to be references:

```rust
fn describe(dir: &Direction) -> &'static str {
    match dir {
        Direction::North => "up",   // not `&Direction::North`
        Direction::South => "down",
        _ => "sideways",
    }
}
```

### Tuple patterns

Matching a tuple of values is the idiomatic way to branch on a combination
of conditions:

```rust
match (a == b, b == c) {
    (true, true) => "all equal",
    (true, false) | (false, true) => "two equal",
    (false, false) => "all different",
}
```

## `if let` and `while let`

`if let PATTERN = value { ... }` is sugar for a `match` that only cares
about one pattern, with everything else falling through (optionally to
`else`):

```rust
let config: Option<u32> = Some(8);
if let Some(max) = config {
    println!("max = {max}");
} else {
    println!("no config");
}
```

`while let PATTERN = value { ... }` loops as long as the pattern keeps
matching — e.g. draining a `Vec` with `.pop()`:

```rust
let mut stack = vec![1, 2, 3];
while let Some(top) = stack.pop() {
    println!("{top}");
}
```

## `matches!`

`matches!(value, PATTERN)` returns `true`/`false` for a quick pattern check
without a full `match`:

```rust
let is_north = matches!(dir, Direction::North);
```

## Bonus: the `?` operator with `Option`

`?` isn't just for `Result` (ch. 9) — in a function returning `Option<T>`,
`expr?` unwraps `Some(x)` to `x`, or returns `None` from the *whole
function* immediately if `expr` is `None`:

```rust
fn add(a: Option<i32>, b: Option<i32>) -> Option<i32> {
    Some(a? + b?) // if either is None, the function returns None right away
}
```

This is the cleanest way to propagate "missing/invalid" through a chain of
sub-computations that each return `Option`.

## Deriving traits on enums

```rust
#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction { North, South, East, West }
```

- `Debug` — `{:?}` printing.
- `PartialEq` — `==` and `assert_eq!` between variants (and their data, if
  any — all fields must themselves be `PartialEq`).
- `Clone`/`Copy` — only derivable if every variant's data is itself
  `Clone`/`Copy` (unit variants always qualify).

## Further Reading

- [The Book, ch. 6 — Enums and Pattern Matching](https://doc.rust-lang.org/book/ch06-00-enums.html)
- [ch. 6.1 — Defining an Enum](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html)
- [ch. 6.2 — The `match` Control Flow Construct](https://doc.rust-lang.org/book/ch06-02-match.html)
- [ch. 6.3 — Concise Control Flow with `if let`](https://doc.rust-lang.org/book/ch06-03-if-let.html)
- [`std::option::Option`](https://doc.rust-lang.org/std/option/enum.Option.html)
- [Reference — Match expressions and patterns](https://doc.rust-lang.org/reference/expressions/match-expr.html)
