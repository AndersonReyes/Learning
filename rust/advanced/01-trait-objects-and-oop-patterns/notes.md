# Trait Objects, Dynamic Dispatch & OOP Patterns

Book ch. 18. Rust isn't a classical OOP language (no struct inheritance), but
traits + trait objects give you encapsulation, polymorphism, and most
"design pattern" idioms from OOP languages.

## OOP characteristics in Rust (ch. 18.1)

- **Encapsulation**: structs bundle data + behavior; fields are private by
  default, `pub` opts in. Methods (`impl Block`) are the only way to mutate
  private fields from outside the module.
- **Inheritance**: Rust has none for structs. Two replacements:
  - **Composition**: a struct holds another struct/trait object as a field
    and delegates to it.
  - **Default trait methods**: a trait provides a default implementation
    that calls other (required) methods — "inheriting" shared behavior
    without inheriting data.
- **Polymorphism**: via generics (compile-time, monomorphized) or trait
  objects (runtime, dynamic dispatch) — see below.

## Trait objects & dynamic dispatch (ch. 18.2)

A **trait object** (`dyn Trait`) is a value of *any* type implementing
`Trait`, accessed through a pointer (`&dyn Trait`, `Box<dyn Trait>`,
`Rc<dyn Trait>`, ...). The pointer carries a **vtable** — a table of function
pointers for that concrete type's implementation of `Trait`'s methods.

```rust
trait Draw {
    fn draw(&self) -> String;
}

struct Button;
struct SelectBox;

impl Draw for Button { fn draw(&self) -> String { "Button".into() } }
impl Draw for SelectBox { fn draw(&self) -> String { "SelectBox".into() } }

// Heterogeneous collection -- impossible with generics (Vec<T> needs one T).
let components: Vec<Box<dyn Draw>> = vec![Box::new(Button), Box::new(SelectBox)];
for c in &components {
    println!("{}", c.draw()); // vtable lookup at runtime
}
```

- **Static dispatch** (`fn f<T: Draw>(x: T)` or `fn f(x: impl Draw)`): the
  compiler generates a separate copy of `f` per concrete type
  (monomorphization) — no runtime cost, but code size grows and `Vec<T>`
  can't hold mixed types.
- **Dynamic dispatch** (`fn f(x: &dyn Draw)` / `Box<dyn Draw>`): one copy of
  `f`, calls go through the vtable — small runtime cost, but enables
  heterogeneous collections and "plugin"-style APIs where the concrete types
  aren't known at compile time.
- **Edition 2021+** requires the `dyn` keyword explicitly (`dyn Draw`, not
  bare `Draw`, when referring to the trait-object type).

### Object safety

A trait can only be made into a trait object (`dyn Trait`) if it's **object
safe**:

- No method can return `Self` by value (the concrete size is erased — the
  compiler can't know how big the return value is). `fn clone(&self) ->
  Self` is why `Clone` is **not** object safe; `dyn Clone` doesn't compile.
- No method can have generic type parameters (`fn foo<T>(&self, x: T)`) — a
  vtable needs one fixed function pointer per method, not one per `T`.
- Methods can take `self`, `&self`, `&mut self`, or `self: Box<Self>` —
  the last one ("arbitrary self types") is key to the State pattern below.

## The State pattern (ch. 18.3)

Model a value whose *behavior* changes based on internal state, without
exposing the state type to callers. Each state is a type implementing a
shared `State` trait; transitions consume `Box<dyn State>` and return a new
`Box<dyn State>`:

```rust
trait State {
    // `self: Box<Self>` consumes the old state, returns the new one.
    fn next(self: Box<Self>) -> Box<dyn State>;
    fn name(&self) -> &'static str;
}

struct Locked;
struct Unlocked;

impl State for Locked {
    fn next(self: Box<Self>) -> Box<dyn State> { Box::new(Unlocked) }
    fn name(&self) -> &'static str { "Locked" }
}
impl State for Unlocked {
    fn next(self: Box<Self>) -> Box<dyn State> { Box::new(Locked) }
    fn name(&self) -> &'static str { "Unlocked" }
}

let mut state: Box<dyn State> = Box::new(Locked);
state = state.next(); // consumes the Locked box, produces Box::new(Unlocked)
```

- `self: Box<Self>` is only callable on a `Box<dyn State>` (or
  `Box<ConcreteType>`) — it *moves out of* the box, so the old state is
  dropped and a brand-new boxed state is returned. This is how Rust expresses
  "transition to a new state" without `unsafe` or mutable type-punning.
- The state machine's *shape* (which states exist, valid transitions) lives
  entirely in the `impl State for ...` blocks — adding a state means adding
  a type + impl, not editing a central `match`.

## Runtime type identification: `dyn Any` (`std::any::Any`)

Every `'static` type implements `Any`. `dyn Any` lets you recover the
concrete type from a trait object at runtime:

```rust
use std::any::Any;

trait Shape: Any {
    fn area(&self) -> f64;
    fn as_any(&self) -> &dyn Any; // boilerplate: lets callers downcast
}

struct Circle { radius: f64 }
impl Shape for Circle {
    fn area(&self) -> f64 { std::f64::consts::PI * self.radius * self.radius }
    fn as_any(&self) -> &dyn Any { self }
}

let shapes: Vec<Box<dyn Shape>> = vec![Box::new(Circle { radius: 1.0 })];
if let Some(circle) = shapes[0].as_any().downcast_ref::<Circle>() {
    println!("radius = {}", circle.radius);
}
```

- `.downcast_ref::<T>()` returns `Option<&T>` — `None` if the trait object's
  concrete type isn't `T`. `.is::<T>()` checks without extracting.
- **Caveat**: reaching for `Any` to switch on concrete types is usually a
  sign the trait should have had another method instead (it reintroduces the
  "type switch" that trait objects are meant to replace). It's legitimate for
  plugin registries, heterogeneous type maps, and similar "I genuinely don't
  know all the types up front" cases.

## The Decorator pattern via nested trait objects

A decorator *wraps* another implementor of the same trait, adding behavior
before/after delegating to the wrapped value:

```rust
trait Notifier {
    fn send(&self, msg: &str) -> Vec<String>;
}

struct Email;
impl Notifier for Email {
    fn send(&self, msg: &str) -> Vec<String> { vec![format!("email: {msg}")] }
}

struct WithSms { inner: Box<dyn Notifier> }
impl Notifier for WithSms {
    fn send(&self, msg: &str) -> Vec<String> {
        let mut out = vec![format!("sms: {msg}")];
        out.extend(self.inner.send(msg)); // delegate
        out
    }
}

let n: Box<dyn Notifier> = Box::new(WithSms { inner: Box::new(Email) });
n.send("hi"); // ["sms: hi", "email: hi"]
```

Each layer only knows about `Box<dyn Notifier>` — layers can be composed in
any order/combination at runtime, unlike a fixed inheritance hierarchy.

## Gotchas

- `dyn Trait` is unsized (`!Sized`) — it must always be behind a pointer
  (`&dyn Trait`, `Box<dyn Trait>`, `Rc<dyn Trait>`, ...). `let x: dyn Trait =
  ...` doesn't compile.
- A trait object's lifetime defaults to `'static` for `Box<dyn Trait>` —
  write `Box<dyn Trait + 'a>` if it must borrow data with a shorter lifetime.
- `Vec<Box<dyn Trait>>::sort_by_key` (and `sort_by`) are **stable sorts** —
  elements that compare equal keep their relative order. Useful for
  priority queues where insertion order should be the tiebreaker.
- A `self: Box<Self>` method can only be called when you actually own a
  `Box<Self>` (or `Box<dyn Trait>`) — you can't call it through `&self` or
  `&mut self`. This is what makes State-pattern transitions "consume and
  replace" rather than "mutate in place".
- Calling an object-unsafe trait as `dyn Trait` is a compile error pointing
  at the offending method — e.g. "the trait `Clone` cannot be made into an
  object" because `clone(&self) -> Self` returns `Self` by value.

## Further Reading (Book / Reference)

- [Ch. 18.1 — Characteristics of Object-Oriented Languages](https://doc.rust-lang.org/book/ch18-01-what-is-oo.html)
- [Ch. 18.2 — Using Trait Objects That Allow for Values of Different Types](https://doc.rust-lang.org/book/ch18-02-trait-objects.html)
- [Ch. 18.3 — Implementing an Object-Oriented Design Pattern](https://doc.rust-lang.org/book/ch18-03-oo-design-patterns.html)
- [Reference — Trait and lifetime bounds: object safety](https://doc.rust-lang.org/reference/items/traits.html#object-safety)
- [`std::any::Any`](https://doc.rust-lang.org/std/any/trait.Any.html)
- [`std::boxed::Box` — `self: Box<Self>` methods](https://doc.rust-lang.org/std/boxed/index.html#method-calls)
