# Closures & Iterators

## Closures

```rust
let add_one = |x: i32| x + 1;
let add_one = |x| x + 1; // types usually inferred from first use
```

- Anonymous functions that can capture variables from their enclosing scope.
- Each closure has its own anonymous, compiler-generated type — two closures
  with identical bodies are *different types*. This is why generic code uses
  `impl Fn(...)` / `<F: Fn(...)>`, not a concrete "closure type".

### `Fn` / `FnMut` / `FnOnce`

Every closure implements at least `FnOnce`; which other traits it implements
depends on *how* it captures variables:

| Trait | Capture mode | Can call... |
|-------|-------------|-------------|
| `Fn` | by reference (`&T`) | any number of times |
| `FnMut` | by mutable reference (`&mut T`), if body mutates a capture | any number of times, needs `&mut self` |
| `FnOnce` | by value (moves captures out), if body consumes a capture | exactly once |

`move` forces capture **by value** even if the body wouldn't otherwise
require it — needed when the closure must outlive the current scope (e.g.
returned from a function, or sent to another thread).

```rust
let s = String::from("hi");
let f = move || println!("{s}"); // `s` moved into `f`; can't use `s` after this
```

### `Cacher`-style structs: storing a closure

A struct can hold a closure as a generic field bound by `Fn`/`FnMut`:

```rust
struct Cacher<F: Fn(u32) -> u32> {
    calculation: F,
    value: Option<u32>,
}

impl<F: Fn(u32) -> u32> Cacher<F> {
    fn new(calculation: F) -> Cacher<F> {
        Cacher { calculation, value: None }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}
```

The book's basic version only ever caches the *first* result, regardless of
`arg` — a known limitation, fixed by keying a `HashMap<arg, result>` instead
of a single `Option`.

### Shared mutable state: `Rc<RefCell<T>>` in closures

An `Fn` closure captures by reference and can't mutate captures directly.
To let a closure record side effects (e.g. a call counter for testing) while
still implementing `Fn`, wrap the state in `Rc<RefCell<T>>` and `.clone()` the
`Rc` into the closure with `move`:

```rust
use std::cell::RefCell;
use std::rc::Rc;

let calls = Rc::new(RefCell::new(0));
let calls_clone = Rc::clone(&calls);
let f = move |x: i32| {
    *calls_clone.borrow_mut() += 1; // mutate through &self via RefCell
    x * 2
};
```

`borrow_mut()` panics if called while another borrow is outstanding —
fine here since each call is short-lived and non-reentrant.

## Iterators

```rust
let v = vec![1, 2, 3];
let iter = v.iter(); // lazy -- nothing happens yet
let total: i32 = iter.sum(); // consumes the iterator
```

- `Iterator` is one trait, `next(&mut self) -> Option<Self::Item>`. Iterators
  do nothing until consumed.
- `.iter()` -> `&T` items (borrow), `.into_iter()` -> `T` items (takes
  ownership), `.iter_mut()` -> `&mut T` items.

### Consuming adaptors

Call `.next()` internally, consuming the iterator: `.sum()`, `.count()`,
`.collect()`, `.fold(init, |acc, x| ...)`, `.max()`/`.min()`,
`.any()`/`.all()`.

### Iterator adaptors (lazy, chainable)

Return a new iterator, do nothing until something consumes it:

```rust
let result: Vec<i32> = (1..=10)
    .filter(|x| x % 2 == 0) // 2,4,6,8,10
    .map(|x| x * x)         // 4,16,36,64,100
    .take(3)                // 4,16,36
    .collect();
```

- `.map(f)` / `.filter(predicate)` — take closures.
- `.zip(other)` — pairs up two iterators, stops at the shorter.
- `.enumerate()` — yields `(index, item)`.
- `.take(n)` / `.skip(n)` — first/skip-first `n` items.
- `.rev()` — requires `DoubleEndedIterator`.
- `.chain(other)` — concatenates two iterators.
- `.scan(initial_state, |state: &mut S, item| -> Option<B>)` — like `.map`,
  but threads mutable state through each step; returning `None` stops
  iteration early.

```rust
// running total
let totals: Vec<i32> = [1, 2, 3, 4]
    .iter()
    .scan(0, |sum, &x| {
        *sum += x;
        Some(*sum)
    })
    .collect();
// totals == [1, 3, 6, 10]
```

### Closures + iterators

Adaptor methods (`map`, `filter`, `sort_by`, ...) take closures, so "closures"
and "iterators" are really one topic: closures supply the per-item logic,
iterators supply the control flow (and the compiler inlines/optimizes the
whole chain — "zero-cost abstraction").

## Returning closures

A function can't return `Fn(...)` directly (unsized) — use `impl Fn(...)` (one
concrete closure type, monomorphized, zero-cost) or `Box<dyn Fn(...)>`
(heap-allocated trait object, lets different call sites return *different*
closure types):

```rust
fn adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n // `n` must be moved in -- it doesn't outlive `adder`
}
```

## Further Reading (Book)

- [Ch. 13 — Functional Language Features: Iterators and Closures](https://doc.rust-lang.org/book/ch13-00-functional-features.html)
- [Ch. 13.1 — Closures](https://doc.rust-lang.org/book/ch13-01-closures.html)
- [Ch. 13.2 — Processing a Series of Items with Iterators](https://doc.rust-lang.org/book/ch13-02-iterators.html)
- [`std::ops::Fn`/`FnMut`/`FnOnce`](https://doc.rust-lang.org/std/ops/trait.Fn.html)
- [`std::iter::Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html) (see `scan`, `fold`, `take`)
- [`std::rc::Rc`](https://doc.rust-lang.org/std/rc/struct.Rc.html), [`std::cell::RefCell`](https://doc.rust-lang.org/std/cell/struct.RefCell.html)
