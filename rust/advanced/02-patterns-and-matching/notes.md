# Patterns & Matching Deep Dive

Book ch. 19. Patterns appear in many places beyond `match`, and the full
syntax is richer than ch. 6 (enums) or ch. 18 (trait objects) covered.

## Where patterns can appear (ch. 19.1)

```rust
let PATTERN = EXPRESSION;         // let statements
for PATTERN in ITERATOR { }       // for loops
fn f(PATTERN: TYPE) { }           // function parameters

if let PATTERN = expr { }         // if let
while let PATTERN = expr { }      // while let
```

### `let...else` (Rust 1.65+)
Refutable pattern in a `let` that must match — if it doesn't, the `else`
block runs and **must diverge** (`return`, `break`, `continue`, `panic!`):

```rust
let Ok(n) = "42".parse::<i32>() else { return };
// n: i32 is bound here — no .unwrap() needed
```

## Refutability (ch. 19.2)

- **Irrefutable**: always matches, required by `let`, `for`, `fn` params —
  e.g. `let (x, y) = pair;`.
- **Refutable**: may not match — required by `if let`, `while let`,
  `match` arms — e.g. `if let Some(x) = option`.
- Mixing them is a compile error: `let Some(x) = option` (irrefutable
  context but refutable pattern) → use `if let` instead.

## Pattern syntax (ch. 19.3)

### Literals, named variables & multiple patterns (`|`)

```rust
match x {
    1 => "one",
    2 | 3 => "two or three",   // | joins alternatives
    _ => "other",
}
```

### Ranges (`..=`)

```rust
match score {
    0..=59 => "fail",
    60..=79 => "pass",
    80..=100 => "distinction",
    _ => unreachable!(),
}
```

### Destructuring structs

```rust
struct Point { x: i32, y: i32 }
let p = Point { x: 3, y: 7 };

match p {
    Point { x: 0, y } => println!("on y-axis at {y}"),
    Point { x, y: 0 } => println!("on x-axis at {x}"),
    Point { x, y }    => println!("({x}, {y})"),
}
```

### Destructuring enums (including nested)

```rust
match msg {
    Message::Move { x, y } => ...  // struct-like variant
    Message::Write(text) => ...     // tuple-like variant
    Message::ChangeColor(r, g, b) => ...
}
```

Nested — match a variant inside another:

```rust
match msg {
    Message::ChangeColor(Color::Rgb(r, g, b)) => ...
    Message::ChangeColor(Color::Hsv(h, s, v)) => ...
}
```

### Ignoring values (`_`, `..`, `_name`)

```rust
let _ = expensive_fn();    // _ — discards (does NOT bind — value dropped here)
let _result = compute();   // _name — binds (stays alive) but suppresses warning

// .. ignores remaining fields / elements
struct Foo { x: i32, y: i32, z: i32 }
let Foo { x, .. } = foo;           // ignore y, z
let (first, .., last) = (1,2,3,4); // first=1, last=4
```

### Slice patterns

```rust
match slice {
    [] => "empty",
    [x] => "one element",
    [first, .., last] => "first and last",
    [a, b, c] => "exactly three",
}
```

Array patterns work identically: `let [a, b, c] = array;` destructures a
`[T; 3]`.

### Match guards (extra `if` after a pattern)

Guards can reference bindings from the pattern:

```rust
match (x, y) {
    (a, b) if a == b => "equal",
    (a, _) if a > 0  => "positive x",
    _                => "other",
}
```

- Guards are **not** taken into account for exhaustiveness checking — if the
  guard can theoretically be false, the compiler won't count that arm as
  covering its pattern.

### `@` bindings

Bind a value to a name **while also testing** a pattern:

```rust
match n {
    x @ 1..=10 => println!("got small number {x}"),
    x @ 11..   => println!("got large number {x}"),
    _           => println!("zero or negative"),
}
```

- Useful when you need the matched value but also want to constrain it with a
  range or literal pattern.
- Can appear inside nested destructuring too:
  `Some(x @ 1..=5)` — binds `x` to the inner value only if it's 1–5.

### Alternative patterns `|` across bindings (Rust 1.53+)

All alternatives of a `|` pattern must bind the same set of names:

```rust
match (l, r) {
    (x, 0) | (0, x) => x,   // x binds from left in first, right in second
    _ => -1,
}
```

## Gotchas

- `_` does NOT bind — a value matched with `_` is dropped at that point.
  `_name` DOES bind (keeps the value alive until end of scope).
- A guard makes an arm potentially-non-exhaustive — add `_` at the end to
  silence the compiler.
- `|` in a match arm vs. or-patterns (inside patterns): before Rust 1.53,
  `|` only worked at the top level of a match arm; after, it works anywhere
  inside a pattern (e.g., `Some(1 | 2 | 3)`).
- The `..=` range pattern is inclusive; bare `..` is not a valid pattern (it's
  only for "ignore remaining fields/elements" in struct/tuple/slice patterns).
- Slice patterns require knowing the length at compile time for the `[a, b, c]`
  form; `[first, .., last]` can match any length ≥ 2.

## Further Reading

- [Ch. 19.1 — All the Places Patterns Can Be Used](https://doc.rust-lang.org/book/ch19-01-all-the-places-for-patterns.html)
- [Ch. 19.2 — Refutability: Whether a Pattern Might Fail to Match](https://doc.rust-lang.org/book/ch19-02-refutability.html)
- [Ch. 19.3 — Pattern Syntax](https://doc.rust-lang.org/book/ch19-03-pattern-syntax.html)
- [Reference — Patterns](https://doc.rust-lang.org/reference/patterns.html)
