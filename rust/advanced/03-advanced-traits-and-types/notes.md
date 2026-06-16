# Advanced Traits & Types

Book ch. 20.2-20.3.

---

## Associated types vs generic type parameters

Associated types bind a placeholder type *inside* a trait — each implementor
picks exactly one concrete type. Generic type parameters let the same type
implement the trait multiple times with different type arguments.

```rust
// Associated type — Output is fixed per implementor
trait Add {
    type Output;
    fn add(self, rhs: Self) -> Self::Output;
}

// Generic — could impl Converter<i32> and Converter<f64> on the same type
trait Converter<T> {
    fn convert(&self) -> T;
}
```

Use associated types when a type should have exactly one "natural" result type
for a trait (e.g. `Iterator::Item`, `Add::Output`). Use generics when multiple
implementations make sense.

---

## Default generic type parameters

A type parameter can have a default: `<T = DefaultType>`. If the caller omits
`T`, the default is used. The standard `Add` trait uses this:

```rust
pub trait Add<Rhs = Self> {
    type Output;
    fn add(self, rhs: Rhs) -> Self::Output;
}
```

Implementing `Add` for a type adds the same type by default. Override `Rhs` to
add different types.

---

## Fully qualified syntax

When a type implements two traits with the same method name, Rust needs
disambiguation:

```rust
trait Pilot { fn fly(&self); }
trait Wizard { fn fly(&self); }
struct Human;

impl Pilot for Human { fn fly(&self) { ... } }
impl Wizard for Human { fn fly(&self) { ... } }

let h = Human;
Pilot::fly(&h);   // calls Pilot's fly
Wizard::fly(&h);  // calls Wizard's fly
```

For associated functions (no `self`), use the fully qualified form:

```rust
<Type as Trait>::function(args)
```

---

## Supertraits

`trait Foo: Bar` means "to implement `Foo`, you must also implement `Bar`".
`Foo`'s methods can call `Bar`'s methods on `self`.

```rust
use std::fmt;

trait PrintSelf: fmt::Display {
    fn print(&self) {
        println!("{}", self); // works because Display is guaranteed
    }
}
```

---

## Newtype pattern

Wrap an external type in a tuple struct to implement external traits on it.
The wrapper is your own type, so the orphan rule is satisfied.

```rust
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}
```

Gotcha: the wrapper doesn't automatically expose the inner type's methods.
Implement `Deref` to forward calls, or delegate manually.

---

## Type aliases

`type Km = i32;` — creates an alias, not a new type. The alias and `i32` are
interchangeable everywhere; the compiler treats them as the same type.

```rust
type Thunk = Box<dyn Fn() -> String>;
fn returns_closure() -> Thunk { Box::new(|| String::from("hi")) }
```

Common use: shorten long generic types (e.g. `Result<T, std::io::Error>`) to
avoid repetition.

---

## Never type `!`

`!` is the "never" type — a type with no values. Functions that never return
have return type `!` (called *diverging*):

```rust
fn diverges() -> ! {
    panic!("always panics");
}
```

Expressions of type `!` can be coerced to any type:

```rust
let x: u32 = loop { break 42; };  // loop: !, break gives u32
let y = match some_option {
    Some(v) => v,
    None => panic!("gone"), // panic!: !, coerced to match arm type
};
```

`continue` also has type `!` in loop contexts.

---

## Dynamically Sized Types (DSTs)

DSTs are types whose size is not known at compile time: `str`, `[T]`,
`dyn Trait`. You can only use them behind a pointer (`&str`, `&[T]`,
`Box<dyn Trait>`), which is a *fat pointer* carrying the value address
plus metadata (length or vtable pointer).

```rust
let s: &str = "hello";        // fat ptr: addr + length
let t: &[i32] = &[1, 2, 3];  // fat ptr: addr + length
let b: Box<dyn Display> = Box::new(42); // fat ptr: addr + vtable
```

---

## `Sized` and `?Sized`

Every generic type parameter implicitly has a `Sized` bound:

```rust
fn foo<T>(t: T) {}          // same as fn foo<T: Sized>(t: T) {}
fn bar<T: ?Sized>(t: &T) {} // relaxes: T may or may not be Sized
```

`?Sized` is the only "maybe" bound syntax in Rust. Use it when you want a
function or struct to accept both sized types and DSTs.

---

## Further Reading

- Book ch. 20.2 — Advanced Traits:
  <https://doc.rust-lang.org/book/ch20-02-advanced-traits.html>
- Book ch. 20.3 — Advanced Types:
  <https://doc.rust-lang.org/book/ch20-03-advanced-types.html>
- `std::ops::Add` (default type parameter example):
  <https://doc.rust-lang.org/std/ops/trait.Add.html>
- `std::fmt::Display`:
  <https://doc.rust-lang.org/std/fmt/trait.Display.html>
- `std::marker::Sized`:
  <https://doc.rust-lang.org/std/marker/trait.Sized.html>
- Reference — never type:
  <https://doc.rust-lang.org/reference/types/never.html>
- Reference — DSTs:
  <https://doc.rust-lang.org/reference/dynamically-sized-types.html>
