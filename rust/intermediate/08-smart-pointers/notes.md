# Smart Pointers: `Box`, `Deref`, `Drop`, `Rc`, `RefCell`, `Weak`

Book ch. 15. Smart pointers are structs that act like references but carry
extra metadata/capabilities (ownership, reference counting, runtime borrow
checks).

## `Box<T>` — heap allocation, recursive types (ch. 15.1)

`Box<T>` puts `T` on the heap; the `Box<T>` itself (a pointer + nothing else)
lives on the stack. Two main uses:

- **Recursive types**. `enum List { Cons(i32, List), Nil }` doesn't compile —
  Rust needs a known size for `List`, and `Cons(i32, List)` would be
  infinitely large. `Box<List>` has a fixed size (one pointer), breaking the
  recursion:
  ```rust
  enum List {
      Cons(i32, Box<List>),
      Nil,
  }
  ```
- **Moving large data without copying** — `Box::new(x)` moves `x` to the
  heap; the box itself is cheap to move.

`Box<T>` implements `Deref`/`DerefMut` (transparent access to `T`) and
`Drop` (frees the heap allocation).

## `Deref` / `DerefMut` — custom smart pointers (ch. 15.2)

```rust
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}
```

`*my_box` desugars to `*(my_box.deref())`. **Deref coercion**: the compiler
inserts `.deref()` calls automatically when a `&T` is expected but a
`&U` is found and `U: Deref<Target = T>` — chained as many times as needed
(`&MyBox<String>` -> `&String` -> `&str`). This is also why
`my_box.some_method()` works even if `MyBox` itself has no such method:
method lookup follows the deref chain. `DerefMut` is the `&mut` analog,
required for `*my_box = value` and `&mut *my_box`.

Deref coercion only applies to **references** (`&`/`&mut`), never to owned
values — `my_box` itself is not a `T`.

## `Drop` — cleanup on scope exit (ch. 15.3)

```rust
struct Guard(String);

impl Drop for Guard {
    fn drop(&mut self) {
        println!("dropping {}", self.0);
    }
}
```

- Runs automatically when a value goes out of scope — **reverse** of
  declaration order within a scope (last declared, first dropped).
- You cannot call `value.drop()` directly (E0040) — that would risk a
  double-free when the implicit drop runs too. Use `std::mem::drop(value)`
  to force early cleanup (it just moves `value` into a function that
  immediately returns, triggering the implicit drop at the end of *that*
  function's scope).
- `Box<T>`'s `Drop` impl frees the heap allocation; `Rc<T>`'s decrements the
  refcount and only drops `T` when it hits zero.

## `Rc<T>` — shared ownership via reference counting (ch. 15.4)

```rust
use std::rc::Rc;

let a = Rc::new(5);
let b = Rc::clone(&a); // bumps strong count, doesn't copy the 5
println!("{}", Rc::strong_count(&a)); // 2
```

- `Rc::clone` is `O(1)` — increments a counter, doesn't deep-copy.
- `Rc<T>` only gives **immutable** access (`Deref<Target = T>`, no
  `DerefMut`) — combine with `RefCell<T>` for shared *mutable* state.
- Single-threaded only (`Rc` is not `Send`/`Sync`); the threadsafe
  equivalent is `Arc<T>` (ch. 16).
- `Rc::ptr_eq(&a, &b)` checks whether two `Rc`s point to the *same*
  allocation (identity), vs. `==` which compares the pointed-to values.

## `RefCell<T>` — interior mutability (ch. 15.5)

`RefCell<T>` enforces Rust's borrowing rules (one `&mut` XOR many `&`) at
**runtime** instead of compile time, via `borrow()`/`borrow_mut()`. This lets
you mutate through an `&RefCell<T>` (i.e., through a shared reference):

```rust
use std::cell::RefCell;

let cell = RefCell::new(vec![1, 2, 3]);
cell.borrow_mut().push(4);
assert_eq!(cell.borrow().len(), 4);
```

- `borrow()` / `borrow_mut()` return `Ref<T>` / `RefMut<T>` guards
  (`Deref`/`DerefMut` to `T`). Holding two `borrow_mut()`s (or a `borrow()`
  and a `borrow_mut()`) at once **panics** at runtime ("already borrowed").
  Keep borrow guards short-lived — don't hold one across a call that might
  borrow again.
- `Cell<T>` is a simpler sibling: `get()`/`set()` for `Copy` types, no borrow
  guards, never panics — use it when you just need to mutate a small `Copy`
  field (e.g. a counter) through `&self`.
- **`Rc<RefCell<T>>`** combines shared ownership with shared mutability —
  the standard pattern for "multiple owners, one of which can mutate the
  shared data."

## `Weak<T>` — non-owning references, breaking cycles (ch. 15.6)

Two `Rc`s pointing to each other (e.g. a child holding `Rc<Parent>` and a
parent holding `Rc<Child>`) form a **reference cycle**: neither strong count
ever reaches zero, so neither is ever dropped — a memory leak. Fix: make one
direction a `Weak<T>` (non-owning, doesn't affect the strong count).

```rust
use std::rc::{Rc, Weak};
use std::cell::RefCell;

struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,   // non-owning: doesn't keep parent alive
    children: RefCell<Vec<Rc<Node>>>, // owning
}
```

- `Rc::downgrade(&rc)` produces a `Weak<T>` (bumps the *weak* count, not the
  strong count).
- `weak.upgrade()` returns `Option<Rc<T>>` — `Some` if the value is still
  alive, `None` if all strong references were dropped. Always check this;
  a `Weak<T>` does **not** guarantee its target still exists.
- Typical layout: children hold strong (`Rc`) references down to their
  children, and a weak reference up to their parent — so dropping the root
  cascades down, but a child doesn't keep its parent alive.

## Gotchas

- `RefCell` borrow violations are **runtime panics**, not compile errors —
  the borrow checker is deferred, not removed.
- `Rc<T>` and `RefCell<T>` are **not thread-safe** — `Arc<T>`/`Mutex<T>` are
  the multithreaded equivalents (ch. 16).
- `Rc::clone(&a)` vs `a.clone()`: both work (same method, via `Deref`), but
  `Rc::clone(&a)` is conventional — it's visually distinct from a deep clone
  of `T`, and doesn't require `T: Clone`.
- A `Weak<T>` whose target was dropped still exists as a value (zero-sized
  pointer) — `upgrade()` returning `None` is normal, not an error condition.
- Deref coercion can mask which type a method actually resolves on — if in
  doubt, write `(*x).method()` or `Type::method(&x)` to disambiguate.

## Further Reading (Book)

- [Ch. 15.1 — Using `Box<T>` to Point to Data on the Heap](https://doc.rust-lang.org/book/ch15-01-box.html)
- [Ch. 15.2 — Treating Smart Pointers Like Regular References with `Deref`](https://doc.rust-lang.org/book/ch15-02-deref.html)
- [Ch. 15.3 — Running Code on Cleanup with the `Drop` Trait](https://doc.rust-lang.org/book/ch15-03-drop.html)
- [Ch. 15.4 — `Rc<T>`, the Reference Counted Smart Pointer](https://doc.rust-lang.org/book/ch15-04-rc.html)
- [Ch. 15.5 — `RefCell<T>` and the Interior Mutability Pattern](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html)
- [Ch. 15.6 — Reference Cycles Can Leak Memory](https://doc.rust-lang.org/book/ch15-06-reference-cycles.html)
- [`std::cell::Cell`](https://doc.rust-lang.org/std/cell/struct.Cell.html), [`std::cell::RefCell`](https://doc.rust-lang.org/std/cell/struct.RefCell.html)
- [`std::rc::Rc`](https://doc.rust-lang.org/std/rc/struct.Rc.html), [`std::rc::Weak`](https://doc.rust-lang.org/std/rc/struct.Weak.html)
- [`std::ops::Deref`](https://doc.rust-lang.org/std/ops/trait.Deref.html), [`std::ops::DerefMut`](https://doc.rust-lang.org/std/ops/trait.DerefMut.html)
