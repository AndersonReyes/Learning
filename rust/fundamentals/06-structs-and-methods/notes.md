# Structs & Methods

## Defining and instantiating structs

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

let user1 = User {
    active: true,
    username: String::from("alice"),
    email: String::from("alice@example.com"),
    sign_in_count: 1,
};
```

- Field order in the literal doesn't have to match the definition order.
- The whole instance must be declared `mut` to mutate any field — Rust has
  no per-field mutability.
- Struct fields own their data by default (`String`, not `&str`). Storing
  references requires lifetime annotations (ch. 10) — out of scope until
  then.

### Field init shorthand

When a variable name matches a field name, you can drop the `field: field`
repetition:

```rust
fn build_user(email: String, username: String) -> User {
    User { email, username, active: true, sign_in_count: 1 }
}
```

### Struct update syntax

`..base` fills in any fields not explicitly listed, copying/moving them from
`base`:

```rust
let user2 = User {
    email: String::from("bob@example.com"),
    ..user1
};
```

**Gotcha**: this *moves* non-`Copy` fields out of `user1` (here,
`username`). After this, `user1` as a whole is no longer usable (it's
"partially moved"), though `user1.active` and `user1.sign_in_count` (both
`Copy`) are still individually readable — only the moved-out fields and any
use of the *whole* struct are rejected by the borrow checker.

## Tuple structs and unit-like structs

```rust
struct Point(i32, i32, i32);
struct Color(i32, i32, i32);

let origin = Point(0, 0, 0);
println!("{}", origin.0); // access by index: .0, .1, .2

struct AlwaysEqual; // no fields — "unit-like struct"
```

- `Point` and `Color` are different types even though both wrap
  `(i32, i32, i32)` — no implicit conversion between them.
- Unit-like structs are useful when you need a type to implement a trait but
  have no data to store (common with marker types).

## Methods

Defined in an `impl` block. The first parameter is always some form of
`self`:

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    fn into_square(self) -> Rectangle {
        let side = self.width.max(self.height);
        Rectangle { width: side, height: side }
    }
}
```

- `&self` — borrows the instance immutably. Most methods use this.
- `&mut self` — borrows mutably; required to modify fields.
- `self` (by value) — takes ownership; used when the method consumes the
  instance and produces something new (rare, but `into_square` above is a
  realistic example — the old `Rectangle` is gone afterward).
- `&self`/`&mut self`/`self` are shorthand for `self: &Self` /
  `self: &mut Self` / `self: Self`, where `Self` is the type the `impl`
  block is for.

### Automatic referencing and dereferencing

`object.method()` automatically becomes `(&object).method()`,
`(&mut object).method()`, or `(*object).method()` — whichever matches the
method's `self` type. You never write `(&rect).area()`; `rect.area()` is
enough even though `area` takes `&self`. This is one of the few places Rust
inserts implicit references, specifically to make method-call ergonomics
work with the ownership system.

### Associated functions (no `self`)

Functions in an `impl` block that don't take any form of `self` are called
on the *type* itself, via `::`:

```rust
impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }

    fn square(size: u32) -> Self {
        Self { width: size, height: size }
    }
}

let r = Rectangle::new(10, 20);
```

- `Self` (capital S) inside an `impl Rectangle` block means `Rectangle` —
  useful if the type gets renamed later.
- `new` has no special meaning to the compiler; it's a naming convention for
  constructors. `String::from` and `Vec::new` are associated functions too.

### Multiple `impl` blocks

A type can have more than one `impl` block; the compiler treats them as one.
Rarely needed for plain structs, but common once generics/traits are
involved (ch. 10+).

## `Debug` and printing

`{}` (`Display`) is not implemented for structs by default. Derive `Debug`
to use `{:?}` / `{:#?}` (pretty-printed):

```rust
#[derive(Debug)]
struct Rectangle { width: u32, height: u32 }

let r = Rectangle::new(3, 4);
println!("{:?}", r);   // Rectangle { width: 3, height: 4 }
println!("{:#?}", r);  // multi-line pretty version
```

`dbg!(expr)` prints `file:line` plus the value to stderr *and* returns
ownership of the value (so `let r = dbg!(Rectangle::new(3, 4));` both prints
and binds).

`#[derive(Clone, PartialEq)]` are likewise common: `Clone` for `.clone()`,
`PartialEq` for `==`/`assert_eq!` between two instances (compares all
fields).

## Further Reading

- [The Book, ch. 5 — Using Structs to Structure Related Data](https://doc.rust-lang.org/book/ch05-00-structs.html)
- [ch. 5.1 — Defining and Instantiating Structs](https://doc.rust-lang.org/book/ch05-01-defining-structs.html)
- [ch. 5.3 — Method Syntax](https://doc.rust-lang.org/book/ch05-03-method-syntax.html)
- [Appendix C — Derivable Traits](https://doc.rust-lang.org/book/appendix-03-derivable-traits.html)
- [Reference — Struct types](https://doc.rust-lang.org/reference/types/struct.html)
- [`std::fmt::Debug`](https://doc.rust-lang.org/std/fmt/trait.Debug.html)
