# Advanced Lifetimes, Variance & PhantomData

Nomicon "Ownership" (subtyping, variance, PhantomData, HRTB, splitting borrows).

---

## Lifetime subtyping

`'long: 'short` means `'long` *outlives* `'short`. A reference `&'long T`
can be used where `&'short T` is expected — that's the definition of
subtyping for lifetime parameters.

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

The single `'a` binds to the *shorter* of the two argument lifetimes — the
caller provides two potentially-different lifetimes and the compiler unifies
them.

## Explicit lifetime bounds on structs

When a struct holds a reference it must name the lifetime:

```rust
struct Important<'a> {
    part: &'a str,
}
```

Lifetime bounds on a method ensure the *output* lifetime is tied to the
*input* lifetime:

```rust
impl<'a> Important<'a> {
    fn level(&self) -> &str { self.part }
    //                  ^ actually &'a str, inferred via elision rule 3
}
```

## Variance

For a type `Container<T>`:
- **Covariant in T**: if `T: Subtype of U` then `Container<T>: Subtype of Container<U>`.
  `&T` is covariant in `T` (you can use a `&'long T` where `&'short T` is needed).
- **Contravariant in T**: `fn(T) -> ()` is contravariant in `T`.
- **Invariant in T**: `&mut T` and `Cell<T>` are invariant in `T` — you
  cannot substitute a subtype because that could lead to aliased mutation.

You rarely need to think about variance explicitly — the compiler infers
it from field types. Where it matters most: when building custom smart
pointers or collections, use `PhantomData` to communicate the intended
variance to the compiler.

## `PhantomData<T>`

A zero-sized marker type that tells the compiler your type *logically*
owns or uses `T`, even if `T` doesn't appear in any field. This affects:
- Drop check (the compiler needs to know if your type may drop a `T`)
- Variance
- `Send`/`Sync` inheritance

```rust
use std::marker::PhantomData;

struct MyBox<T> {
    ptr: *mut T,
    _marker: PhantomData<T>,  // "I logically own a T"
}
// Now MyBox<T>: Send iff T: Send (inherited from PhantomData<T>)
```

Common patterns:
- `PhantomData<T>` — covariant, owns T (affects drop)
- `PhantomData<*mut T>` — invariant, doesn't own T (no drop effect)
- `PhantomData<fn() -> T>` — covariant, does not affect drop
- `PhantomData<fn(T)>` — contravariant

## Higher-Rank Trait Bounds (HRTB) `for<'a>`

Sometimes a bound must hold for *all* lifetimes, not just a specific one:

```rust
fn apply_to_str<F>(f: F) where F: for<'a> Fn(&'a str) -> &'a str {
    let s = "hello".to_string();
    println!("{}", f(&s));
}
```

`for<'a> Fn(&'a str) -> &'a str` means "for any lifetime `'a`, `f` must
accept a `&'a str` and return a `&'a str`". This is how closures that
borrow their argument work in higher-order functions.

## Splitting borrows

The borrow checker tracks borrows per-field in a struct, not per-struct:

```rust
struct Point { x: i32, y: i32 }
let mut p = Point { x: 1, y: 2 };
let rx = &mut p.x;
let ry = &mut p.y;  // OK: different fields
// let rp = &mut p; // ERROR: p is borrowed via rx and ry
```

This doesn't work through indexing or pointer arithmetic — only through
direct field access. The compiler can split borrows for `struct` fields and
`tuple` fields, but not for `Vec` elements (use `split_at_mut` instead).

## Lifetime elision rules

In function signatures, lifetimes can often be omitted:
1. Each omitted `&` gets a distinct lifetime parameter.
2. If there's exactly one input lifetime, it's applied to all outputs.
3. If one input is `&self` or `&mut self`, its lifetime is applied to outputs.

```rust
fn first_word(s: &str) -> &str { /* ... */ }
// desugars to:
fn first_word<'a>(s: &'a str) -> &'a str { /* ... */ }
```

## Further Reading

- [Nomicon — Subtyping and Variance](https://doc.rust-lang.org/nomicon/subtyping.html)
- [Nomicon — PhantomData](https://doc.rust-lang.org/nomicon/phantomdata.html)
- [Nomicon — Higher-Rank Trait Bounds](https://doc.rust-lang.org/nomicon/hrtb.html)
- [Nomicon — Splitting Borrows](https://doc.rust-lang.org/nomicon/borrow-splitting.html)
- [Book ch. 10.3 — Validating References with Lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)
