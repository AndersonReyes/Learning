# Generics, Traits & Lifetimes

## Generic data types

A type parameter `<T>` lets one definition work over many concrete types,
monomorphized (specialized per concrete type) at compile time — zero
runtime cost.

```rust
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

// A method only available for a specific concrete type:
impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```

- `<T: PartialOrd + Copy>` on `largest` is a **trait bound** — see below.
  Without `PartialOrd`, `>` doesn't compile; without `Copy` (or using `&T`),
  assigning `list[0]` to `largest` would try to move out of the slice.
- A struct/enum can use multiple type parameters (`struct Pair<T, U>`).
- Recursive generic types (e.g. a generic tree/list node) need `Box<T>` —
  same reason as `fundamentals/07`'s recursive `Expr` enum: the type's size
  must be known at compile time, and `Option<Box<Node<T>>>` is a
  pointer-sized field regardless of `T`.

## Traits: shared behavior

A trait declares method signatures that implementing types must provide.

```rust
trait Summary {
    fn summarize_author(&self) -> String;

    // default implementation -- can be overridden per type
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
```

A default method can call other (non-default) methods of the same trait —
implementors only need to provide those. Overriding a default method is
just defining it in the `impl` block.

### Trait bounds

```rust
// `impl Trait` syntax -- sugar for a trait-bound generic
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// equivalent, explicit generic with a trait bound
fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// multiple bounds with `+`
fn notify_and_log<T: Summary + std::fmt::Debug>(item: &T) { /* ... */ }

// `where` clause -- clearer for several bounds/params
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: std::fmt::Display + Clone,
    U: Clone + std::fmt::Debug,
{
    42
}
```

- `&impl Trait` / `T: Trait` accepts **any single concrete type** that
  implements `Trait` — monomorphized per call site, *not* dynamic dispatch.
  (Trait *objects*, `&dyn Trait` / `Box<dyn Trait>`, for runtime
  polymorphism over heterogeneous types, are covered in `advanced/01`.)
- **Returning `impl Trait`**: `fn make_summary() -> impl Summary { ... }`
  returns *some* concrete type implementing `Summary`, without naming it —
  useful for closures and iterators whose concrete types are unwieldy or
  unnameable. The function must return a *single* concrete type (no `if`
  branches returning different types).
- **Blanket impls**: implement a trait for every type satisfying a bound,
  e.g. the standard library's
  `impl<T: Display> ToString for T { ... }` — every `Display` type gets
  `.to_string()` for free.

### Operator overloading via traits

Standard operators are themselves traits in `std::ops` — e.g. `+` is
`std::ops::Add`:

```rust
use std::ops::Add;

#[derive(Clone, Copy)]
struct Millimeters(u32);

impl Add for Millimeters {
    type Output = Millimeters;
    fn add(self, other: Millimeters) -> Millimeters {
        Millimeters(self.0 + other.0)
    }
}
```

A generic function can require `T: Add<Output = T>` to use `+` on `T`. Full
operator-trait coverage (including `Deref`/`Index`) is in `advanced/03`.

## Lifetimes

Every reference has a **lifetime** — the scope for which it's valid. Most of
the time this is inferred; lifetime *annotations* are needed when the
compiler can't determine how multiple references relate, most commonly: a
function takes multiple reference parameters and returns a reference, and
the compiler can't tell which input the output borrows from.

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

`'a` doesn't change how long anything lives — it's a *constraint*: "the
returned reference is valid for as long as **both** `x` and `y` are valid."
The borrow checker then rejects any use of the result that outlives either
input.

### Lifetime elision rules

The compiler infers lifetimes for common patterns without annotations:

1. Each reference parameter gets its own lifetime parameter.
2. If there's exactly **one** input lifetime, it's assigned to all output
   lifetimes.
3. If one parameter is `&self`/`&mut self`, its lifetime is assigned to all
   output lifetimes.

`fn first_word(s: &str) -> &str` compiles without annotations (rule 2) —
it's sugar for `fn first_word<'a>(s: &'a str) -> &'a str`. `longest` above
needs an explicit annotation because it has *two* input reference
parameters and rule 2/3 don't apply.

### Lifetimes in struct definitions

A struct holding a reference needs a lifetime parameter, tying the struct's
validity to the reference's:

```rust
struct Excerpt<'a> {
    part: &'a str,
}

impl<'a> Excerpt<'a> {
    fn part(&self) -> &str { // elided: &'a str, via rule 3
        self.part
    }
}
```

An `Excerpt<'a>` instance cannot outlive the string it borrows from `'a`.

### `'static`

`&'static str` (and `&'static T` generally) lives for the entire program —
string literals are `&'static str`. Don't reach for `'static` to "fix" a
lifetime error unless the data genuinely lives forever (e.g. baked into the
binary); usually the real fix is restructuring ownership.

## Further Reading (Rust Book)

- [Ch. 10 — Generic Types, Traits, and Lifetimes](https://doc.rust-lang.org/book/ch10-00-generics.html)
- [Ch. 10.1 — Generic Data Types](https://doc.rust-lang.org/book/ch10-01-syntax.html)
- [Ch. 10.2 — Traits: Defining Shared Behavior](https://doc.rust-lang.org/book/ch10-02-traits.html)
- [Ch. 10.3 — Validating References with Lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)
- [`std::ops::Add`](https://doc.rust-lang.org/std/ops/trait.Add.html)
- [Reference — Lifetime elision](https://doc.rust-lang.org/reference/lifetime-elision.html)
