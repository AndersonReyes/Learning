# Variables, Data Types & Functions

## Variables and mutability

- `let x = 5;` — bindings are **immutable by default**. Reassigning `x = 6`
  is a compile error.
- `let mut x = 5;` — opt into mutability explicitly. Forces you to mark, at
  the declaration site, every binding that changes.
- `const MAX_POINTS: u32 = 100_000;` — always immutable, must have a type
  annotation, must be a value computable at compile time (no function calls
  except `const fn`), and conventionally `SCREAMING_SNAKE_CASE`. Valid for
  the entire program, even inlined at every use site.
- **Shadowing**: `let x = 5; let x = x * 2;` declares a *new* binding that
  reuses the name `x`, optionally with a different type. Different from
  `mut`: each `let` can change the type, and the old binding still exists
  (e.g. captured by a closure) — only the name `x` now refers to the new
  one. Common idiom: `let guess: u32 = guess.trim().parse().unwrap();` shadows
  a `String` `guess` with a `u32` `guess`.

```rust
let spaces = "   ";       // &str
let spaces = spaces.len(); // shadowed to usize — same name, new type, fine
```

## Scalar types

| Kind | Types | Notes |
|---|---|---|
| Signed integers | `i8 i16 i32 i64 i128 isize` | `i32` is the default if unconstrained |
| Unsigned integers | `u8 u16 u32 u64 u128 usize` | `usize`/`isize` are pointer-width (used for indexing) |
| Floating point | `f32 f64` | `f64` is the default; IEEE-754 |
| Boolean | `bool` | `true`/`false`, 1 byte |
| Character | `char` | 4 bytes, a Unicode **scalar value** (not just ASCII) — `'a'`, `'α'`, `'🦀'` are all one `char` |

### Integer overflow

`let x: u8 = 255; x + 1` — what happens depends on build profile:

- **Debug builds**: panics ("attempt to add with overflow").
- **Release builds** (`--release`): **wraps** silently (two's-complement),
  i.e. `255u8 + 1 == 0`. This is *not* a guarantee you should rely on —
  it's `wrapping_*`'s behavior, just made implicit and dangerous.

Don't rely on either default. Use the explicit methods every integer type
has:

- `checked_add(rhs) -> Option<Self>` — `None` on overflow.
- `wrapping_add(rhs) -> Self` — always wraps (two's complement), no panic.
- `saturating_add(rhs) -> Self` — clamps to `MIN`/`MAX` instead of wrapping.
- `overflowing_add(rhs) -> (Self, bool)` — wrapped result **and** whether it
  overflowed.

Same family for `_sub`, `_mul`, `_div`, `_pow`, etc. `i32::MAX`, `u8::MAX`,
`u32::MIN`, etc. give type-level limits.

### Integer division and casts (`as`)

- Integer `/` **truncates toward zero**, not toward negative infinity:
  `-7 / 2 == -3` (not `-4`), `-7 % 2 == -1`. (Python's `//`/`%` floor instead
  — don't assume Rust matches.)
- `as` performs a primitive cast: widening (`u8 as u32`) zero-extends;
  narrowing (`u32 as u8`) **truncates** — keeps only the low bits, silently,
  no panic, no wrap-around semantics beyond "drop the high bits". `300u32 as
  u8 == 44` (300 mod 256).
- Float-to-int `as` casts saturate at the target type's bounds (and `NaN`
  becomes `0`) since Rust 1.45 — but still avoid relying on this; check
  ranges explicitly when it matters.

## Compound types

- **Tuples**: fixed-length, heterogeneous. `let t: (i32, f64, u8) = (500,
  6.4, 1);` — access with `t.0`, `t.1`, `t.2`, or destructure: `let (x, y,
  z) = t;`. The empty tuple `()` is the "unit" type — the implicit return
  type of functions that "return nothing".
- **Arrays**: fixed-length, homogeneous, stack-allocated. `let a: [i32; 5] =
  [1, 2, 3, 4, 5];`. `[0; 5]` repeats a value 5 times. Length is part of the
  type (`[i32; 5]` and `[i32; 6]` are different types) — use a `Vec<i32>`
  (covered in `fundamentals/09`) when the length isn't known at compile
  time. Indexing out of bounds panics at runtime (bounds-checked, unlike C).

## Functions

```rust
fn add_one(x: i32) -> i32 {
    x + 1   // no semicolon: this is the tail *expression*, and becomes the
            // return value. `x + 1;` would make it a statement returning
            // `()`, a type error against `-> i32`.
}
```

- Parameters require explicit type annotations — no inference from call
  sites (unlike `let`).
- **Statements** (`let x = 5;`, `x + 1;` with a semicolon) evaluate to `()`
  and don't produce a value. **Expressions** (`5 + 6`, `if`/`match`/blocks)
  evaluate to a value. A function body is a block, and a block's value is
  its last expression — if it has one and it's not semicolon-terminated.
- `return` exits early with a value; the implicit tail-expression form is
  idiomatic for the final value.
- Function and variable names use `snake_case`.

## Further Reading (Rust Book)

- [Ch. 3.1 — Variables and Mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)
- [Ch. 3.2 — Data Types](https://doc.rust-lang.org/book/ch03-02-data-types.html)
- [Ch. 3.3 — Functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
- [`std::primitive` integer types](https://doc.rust-lang.org/std/primitive.i32.html) —
  `checked_*`/`wrapping_*`/`saturating_*`/`overflowing_*` methods
- [Reference — Type cast expressions](https://doc.rust-lang.org/reference/expressions/operator-expr.html#type-cast-expressions)
