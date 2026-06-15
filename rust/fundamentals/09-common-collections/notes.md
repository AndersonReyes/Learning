# Common Collections: `Vec`, `String`, `HashMap`

## `Vec<T>`

A growable, heap-allocated array.

```rust
let mut v: Vec<i32> = Vec::new();
v.push(1);
v.push(2);
let v2 = vec![1, 2, 3]; // macro form, with initial elements

let third = v2[2];          // panics if out of bounds
let third = v2.get(2);      // Option<&i32> — None if out of bounds
```

- `v[i]` panics on out-of-bounds; `v.get(i)` returns `Option<&T>` for safe
  access.
- Iterating: `for x in &v` (borrows each element as `&T`), `for x in &mut v`
  (`&mut T`, can mutate in place via `*x += 1`), `for x in v` (consumes `v`,
  yields owned `T`).
- **Ownership gotcha**: you can't hold an immutable reference to an element
  (e.g. `&v[0]`) across a `v.push(...)` — `push` may reallocate, which would
  invalidate the reference. The borrow checker rejects this.
- Enums let a `Vec` hold "different types" as long as they're variants of
  one enum (`Vec<T>` itself requires a single concrete `T`).

## `String`

`String` is a growable, UTF-8 encoded `Vec<u8>` wrapper. (See topic 5 for
`&str`/slicing/`.chars()` vs `.len()`.)

```rust
let mut s = String::new();
s.push_str("foo"); // append a &str
s.push('!');       // append a single char

let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2;  // `+` takes ownership of s1, borrows s2 (via Deref)
// s1 is moved and no longer usable; s2 still is.

let s4 = format!("{s3} again"); // format! never takes ownership, always
                                 // returns a new String — usually preferred
                                 // over `+` for anything beyond one concat.
```

- **No indexing**: `s[0]` doesn't compile — a byte index might land
  mid-character. Use `&s[0..4]` (byte-range slicing, panics on a bad
  boundary), `.chars()`, `.bytes()`, or `.char_indices()` (all from topic 5).

## `HashMap<K, V>`

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

let score = scores.get("Blue"); // Option<&i32>
let score = scores.get("Blue").copied().unwrap_or(0); // owned i32, default 0

for (key, value) in &scores {
    println!("{key}: {value}");
}
```

- **Ownership**: `insert` moves owned keys/values (like `String`) into the
  map — `scores` now owns those `String`s. Types that implement `Copy`
  (like `i32`) are copied in instead.
- **Iteration order is unspecified** and can vary between runs — never rely
  on it. (This is why HashMap-based exercises below sort or otherwise
  normalize their output.)
- **Updating**:
  - Overwrite: `scores.insert(key, new_value)` — last `insert` wins.
  - Insert only if absent: `scores.entry(key).or_insert(default)`.
  - Update based on old value (e.g. counting): the classic word-counter
    idiom —
    ```rust
    let mut counts: HashMap<&str, i32> = HashMap::new();
    for word in text.split_whitespace() {
        *counts.entry(word).or_insert(0) += 1;
    }
    ```
    `entry(word)` returns an `Entry`; `.or_insert(0)` inserts `0` if the key
    is absent and returns `&mut i32` either way; `*... += 1` increments
    through that mutable reference.

### `HashSet<T>`

`std::collections::HashSet<T>` is, conceptually, `HashMap<T, ()>` — same
hashing/lookup, just membership (`.contains`, `.insert`, `.remove`) with no
associated value. Handy for "have I seen this before?" tracking.

## Further Reading

- [The Book, ch. 8 — Common Collections](https://doc.rust-lang.org/book/ch08-00-common-collections.html)
- [ch. 8.1 — Storing Lists of Values with Vectors](https://doc.rust-lang.org/book/ch08-01-vectors.html)
- [ch. 8.2 — Storing UTF-8 Encoded Text with Strings](https://doc.rust-lang.org/book/ch08-02-strings.html)
- [ch. 8.3 — Storing Keys with Associated Values in Hash Maps](https://doc.rust-lang.org/book/ch08-03-hash-maps.html)
- [`std::vec::Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html)
- [`std::collections::HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html)
- [`std::collections::HashSet`](https://doc.rust-lang.org/std/collections/struct.HashSet.html)
