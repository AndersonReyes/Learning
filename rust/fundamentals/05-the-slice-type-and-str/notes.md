# The Slice Type & `&str`

A **slice** is a reference to a contiguous sequence of elements within a
collection, without taking ownership: `&[T]` for general slices, `&str` for
string slices. Slices are "fat pointers" — `(pointer, length)` — and, like
any reference, can't outlive the data they point to (the borrow checker
applies the same rules from `fundamentals/04`).

## Range syntax

```rust
let v = vec![1, 2, 3, 4, 5];
let a = &v[1..3];  // [2, 3]  -- start..end, end EXCLUSIVE
let b = &v[..2];   // [1, 2]  -- from the start
let c = &v[3..];   // [4, 5]  -- to the end
let d = &v[..];    // [1, 2, 3, 4, 5] -- the whole thing
```

Same syntax works on arrays, `Vec<T>`, and `&str`. Indexing out of bounds
panics ("range end index N out of range for slice of length M").

## `&str` is a slice of UTF-8 bytes

```rust
let s = String::from("hello world");
let hello = &s[0..5]; // "hello" — a &str borrowing from s
let world = &s[6..11]; // "world"
```

`s[0..5]` is a **byte range**, not a character range. `String`/`str` are
UTF-8 encoded — most non-ASCII characters take 2-4 bytes. Slicing at a byte
index that isn't on a character boundary **panics**:
`"byte index N is not a char boundary"`.

```rust
let s = "héllo"; // 'é' is 2 bytes in UTF-8: h(1) é(2) l(1) l(1) o(1) = 6 bytes
// &s[0..2] would panic: byte 2 is in the middle of 'é'
let he = &s[0..3]; // OK: "hé" — byte 3 is right after 'é'
```

### Iterating safely: `.chars()` and `.char_indices()`

- `.chars()` iterates `char`s (Unicode scalar values) — correct, but you
  lose byte positions.
- `.char_indices()` yields `(byte_index, char)` pairs — use this to find
  *valid* slice boundaries when working with non-ASCII input:

```rust
let s = "héllo";
for (i, c) in s.char_indices() {
    println!("byte {i}: {c}");
}
// byte 0: h
// byte 1: é   (occupies bytes 1-2)
// byte 3: l
// byte 4: l
// byte 5: o
```

- `.len()` on a `&str` returns the **byte** length, not the character count.
  `"héllo".len() == 6`, but it has 5 characters.

## `str` vs `String`

- `String` — owned, growable, heap-allocated (from `fundamentals/04`).
- `str` (almost always seen as `&str`) — an immutable *view* into UTF-8
  bytes, usually borrowed. String literals (`"hello"`) have type `&'static
  str` — baked into the binary, valid for the whole program.

### Deref coercion: prefer `&str` parameters

`&String` automatically coerces to `&str`, so a function taking `&str`
accepts both:

```rust
fn print_it(s: &str) {
    println!("{s}");
}

let owned = String::from("owned");
print_it(&owned);   // &String -> &str via deref coercion
print_it("literal"); // already &str
```

Writing `&String` as a parameter type is needlessly restrictive — always
prefer `&str` (or `&[T]` over `&Vec<T>`) for read-only access.

## General slices `&[T]`

Everything above (range syntax, borrow-checker lifetime rules) applies to
slices of any type, not just strings:

```rust
fn first_two(s: &[i32]) -> &[i32] {
    &s[..2]
}

let v = vec![10, 20, 30];
let pair = first_two(&v); // &[10, 20] — pair borrows from v
```

A function returning `&[T]` (or `&str`) that's derived from an input slice
returns a *view* into the same memory — no copying. The returned reference's
lifetime is tied to the input's, enforced by the borrow checker (a function
can't return a slice of a local `Vec`/`String` it created — same dangling
reference problem as `fundamentals/04`).

## Useful `&str` methods

| Method | Does |
|---|---|
| `.trim()` | strips leading/trailing whitespace, returns `&str` |
| `.split(pat)` / `.split_whitespace()` | iterator of `&str` pieces |
| `.find(pat)` / `.rfind(pat)` | `Option<usize>` byte index of first/last match |
| `.starts_with(pat)` / `.ends_with(pat)` | bool |
| `.is_empty()` | `s.len() == 0` |
| `.chars()` / `.bytes()` / `.char_indices()` | iterate scalars / raw bytes / `(byte_idx, char)` |

## Mutable slices

`&mut [T]` allows in-place modification of a borrowed range (e.g.
`slice.sort()`, `slice.reverse()`, or indexed writes `s[i] = x`) — same
exclusivity rule as `&mut T` from `fundamentals/04`: while a `&mut [T]`
borrow is alive, no other reference to that data may exist.

## Further Reading (Rust Book)

- [Ch. 4.3 — The Slice Type](https://doc.rust-lang.org/book/ch04-03-slices.html)
- [`std::str` — string slice methods](https://doc.rust-lang.org/std/primitive.str.html)
- [`std::primitive::slice` — slice methods](https://doc.rust-lang.org/std/primitive.slice.html)
- [Reference — Slice types](https://doc.rust-lang.org/reference/types/slice.html)
