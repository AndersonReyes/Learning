# Control Flow

## `if`/`else` is an expression

Unlike statements (`let x = 5;`), `if`/`else` produces a value — every arm
must evaluate to the **same type**:

```rust
let n = 7;
let parity = if n % 2 == 0 { "even" } else { "odd" }; // &str
```

- No parens around the condition; braces are mandatory (no single-statement
  bodies without `{}`).
- `else if` chains are just nested `if`/`else` — no separate `elif` keyword.
- If you omit `else`, the type of the `if` arm must be `()` — useful for
  side-effecting `if` used as a statement (`if cond { do_thing(); }`), but a
  type error if you try to bind its value: `let x = if cond { 1 };` fails
  because the implicit `else` branch is `()`, which doesn't match `1`'s type
  `i32`.

## Loops: `loop`, `while`, `for`

### `loop` — infinite, `break` to exit

```rust
let mut i = 0;
let result = loop {
    i += 1;
    if i == 10 {
        break i * 2; // `break <value>` makes the *loop expression* evaluate
                      // to that value. Only `loop` supports this — `while`
                      // and `for` always evaluate to `()`.
    }
};
assert_eq!(result, 20);
```

### `while condition { }`

Runs while `condition` is `true`; checked *before* each iteration (so zero
iterations if false from the start). Always evaluates to `()` — can't
`break value` out of it.

### `for item in iterable { }`

Preferred for iterating collections/ranges — no manual indexing, no
off-by-one risk:

```rust
for i in 0..5 {      // 0,1,2,3,4 — exclusive upper bound
    println!("{i}");
}
for i in 0..=5 {     // 0,1,2,3,4,5 — inclusive upper bound
    println!("{i}");
}
```

**Array iteration gotcha**: as of the 2021 edition, `[T; N]` implements
`IntoIterator` *by value*:

```rust
let arr = [1, 2, 3];
for x in arr {       // x: i32, a copy of each element (arr still usable after)
    println!("{x}");
}
for x in &arr {      // x: &i32 — iterate by reference instead
    println!("{x}");
}
```

Both loops above are valid and `arr` remains usable afterward because `i32`
is `Copy`; for non-`Copy` element types, `for x in arr` *moves* each element
out, consuming `arr`.

## `continue`

Skips the rest of the current iteration and re-evaluates the loop's
condition/iterator:

```rust
for i in 0..10 {
    if i % 2 == 0 {
        continue; // skip even numbers
    }
    println!("{i}"); // 1, 3, 5, 7, 9
}
```

## Loop labels — `'label: loop { ... }`

Needed to `break`/`continue` an **outer** loop from inside a **nested** one
(a bare `break`/`continue` always applies to the innermost loop):

```rust
let mut found = None;
'outer: for i in 0..5 {
    for j in 0..5 {
        if i * j == 6 {
            found = Some((i, j));
            break 'outer; // exits BOTH loops
        }
    }
}
assert_eq!(found, Some((2, 3)));
```

- Labels start with `'` (same sigil as lifetimes, but a different
  namespace — no relation).
- `continue 'outer;` jumps to the next iteration of the *outer* loop,
  skipping the rest of the inner loop entirely.
- `break 'outer value;` also works if `'outer` is a `loop` (not `while`/`for`)
  — propagates `value` as that loop's result.

## Gotchas

- **Infinite loops**: `while`/`loop` conditions must eventually become
  false/`break`-reached. A common bug: a termination check whose direction
  (`>=` vs `<=`) doesn't match the direction the loop variable is moving —
  e.g. incrementing toward a target but checking `<=` when you meant `>=`.
- **`while`/`for` can't `break value`** — only `loop` can. If you need a
  value out of a `for`/`while`, assign to a `mut` binding declared before the
  loop and read it after.
- Variables declared *inside* a loop body are dropped/reset each iteration —
  declare accumulators *before* the loop if they need to persist.

## Further Reading (Rust Book)

- [Ch. 3.4 — Control Flow: `if` Expressions](https://doc.rust-lang.org/book/ch03-05-control-flow.html#if-expressions)
- [Ch. 3.5 — Repetition with Loops](https://doc.rust-lang.org/book/ch03-05-control-flow.html#repetition-with-loops)
- [Reference — Loop expressions (`loop`, `while`, `for`, labels)](https://doc.rust-lang.org/reference/expressions/loop-expr.html)
- [Reference — `if` and `if let` expressions](https://doc.rust-lang.org/reference/expressions/if-expr.html)
