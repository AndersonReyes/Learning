# Ownership & Borrowing

Rust's core memory-safety mechanism, enforced entirely at compile time (the
"borrow checker") — no garbage collector, no manual `free`.

## The three ownership rules

1. Each value has exactly one **owner** at a time.
2. When the owner goes out of scope, the value is **dropped** (its
   destructor runs, memory freed).
3. Ownership can be **moved** (transferred) — assignment, passing to a
   function, or returning from a function all potentially move ownership.

## Move semantics

For heap-backed types (`String`, `Vec<T>`, `Box<T>`, ...), assignment
**moves** — it does *not* copy the underlying data:

```rust
let s1 = String::from("hello");
let s2 = s1; // s1 is MOVED into s2
// println!("{s1}"); // compile error: "value borrowed here after move"
```

Both `s1` and `s2` are just (pointer, length, capacity) on the stack
pointing at the same heap buffer — if both were valid, dropping both would
free the buffer twice (a double-free). Rust prevents this by making `s1`
invalid after the move; only `s2` can be used (and only `s2` is dropped).

This is **shallow** (just the stack representation is copied) but treated as
a **move**, not a copy — the old binding is deliberately invalidated.

## `Clone` — explicit deep copy

To actually duplicate heap data, call `.clone()`:

```rust
let s1 = String::from("hello");
let s2 = s1.clone(); // deep copy: separate heap buffer
println!("{s1} {s2}"); // both valid
```

`Clone` is a trait; `String`, `Vec<T>`, and most collections implement it.
It's an explicit, visible cost — Rust never clones implicitly.

## `Copy` — types that don't move

Simple stack-only types (`i32`, `f64`, `bool`, `char`, and tuples/arrays
composed entirely of `Copy` types) implement `Copy`. Assigning a `Copy` type
**copies** it; the original remains valid:

```rust
let x = 5;
let y = x; // copy, not move
println!("{x} {y}"); // both valid — i32 is Copy
```

A type can't be both `Copy` and own heap data (e.g. `String` is not `Copy`,
because copying it bitwise would create two owners of the same buffer).

## Ownership and functions

Passing a value to a function moves or copies it, exactly like assignment;
returning a value transfers ownership out:

```rust
fn takes_ownership(s: String) -> usize {
    s.len() // s is dropped at the end of this function
}

let s = String::from("hello");
let len = takes_ownership(s);
// s is no longer valid here — it was moved into takes_ownership
```

To use a value *without* taking ownership, pass a **reference**.

## References and borrowing

`&T` is an immutable (shared) reference — "borrowing" a value without
taking ownership:

```rust
fn calculate_length(s: &String) -> usize {
    s.len()
}

let s = String::from("hello");
let len = calculate_length(&s); // &s borrows s
println!("{s} has length {len}"); // s still valid — only borrowed
```

`&mut T` is a mutable reference — allows modifying the borrowed value:

```rust
fn append_world(s: &mut String) {
    s.push_str(" world");
}

let mut s = String::from("hello");
append_world(&mut s);
println!("{s}"); // "hello world"
```

`*` is the deref operator — explicitly used when you need to *replace* what
a `&mut` points to, e.g. `*target = new_value;` (assigns through the
reference rather than rebinding the local reference variable).

## Borrowing rules (the aliasing invariant)

At any point, for a given value, you may have **either**:

- any number of `&T` (shared/immutable) references, **or**
- exactly **one** `&mut T` (exclusive/mutable) reference,

but never both at once. This prevents data races and iterator invalidation
at compile time. Since Rust 2018 (non-lexical lifetimes), a reference's
"liveness" ends at its **last use**, not at the end of its scope — so this
compiles:

```rust
let mut v = vec![1, 2, 3];
let first = &v[0];
println!("{first}");   // last use of `first`
v.push(4);              // OK: `first`'s borrow already ended
```

But this does not:

```rust
let mut v = vec![1, 2, 3];
let first = &v[0];
v.push(4);              // ERROR: cannot borrow `v` as mutable while
                         // borrowed as immutable (`first` still alive)
println!("{first}");
```

## Dangling references

The borrow checker rejects references that would outlive the data they
point to:

```rust
fn dangle() -> &String {  // ERROR: missing lifetime specifier
    let s = String::from("hello");
    &s // s is dropped at the end of this function — &s would dangle
}
```

The fix is to return the owned `String` itself (transfer ownership out), not
a reference to a local.

## Practical patterns used in this topic's exercises

- **Mutate in place via `&mut Vec<T>`**: either mutate elements/length
  directly (`values.push(...)`, `values[i] = ...`), or build a new `Vec` and
  reassign through the reference: `*values = new_vec;`.
- **Read without consuming via `&[T]` / `&[String]`**: iterate with
  `.iter()` — the caller's data is untouched and still owned by them
  afterward.
- **Returning owned data derived from borrowed input**: if a function takes
  `&[String]` but must return a `String`, it can't return a reference into
  the input (lifetime/ownership mismatch without extra annotations) — build
  a *new* `String` (e.g. via `String::new()` + `.push()`/`.push_str()`, or
  `.clone()` an element).
- **Consuming ownership on purpose**: a function taking `Vec<T>` (not
  `&Vec<T>`) signals it consumes/redistributes the data — useful when the
  caller won't need the original afterward and you want to avoid a clone.

## Further Reading (Rust Book)

- [Ch. 4.1 — What is Ownership?](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
- [Ch. 4.2 — References and Borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
- [`std::clone::Clone`](https://doc.rust-lang.org/std/clone/trait.Clone.html)
- [`std::marker::Copy`](https://doc.rust-lang.org/std/marker/trait.Copy.html)
