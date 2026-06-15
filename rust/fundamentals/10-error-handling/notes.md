# Error Handling: `panic!`, `Result<T, E>`, and `?`

## Two kinds of errors

- **Unrecoverable** (`panic!`): the program can't continue safely — bugs,
  violated invariants, "this should never happen". Unwinds the stack
  (default) or aborts (with `panic = "abort"` in `Cargo.toml`), printing a
  message and location.
- **Recoverable** (`Result<T, E>`): expected failure modes the *caller*
  should handle — bad input, missing data, division by zero.

```rust
fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("division by zero"); // unrecoverable — bug in caller
    }
    a / b
}
```

`panic!` also fires implicitly: out-of-bounds `v[i]`, integer overflow in
debug builds, `.unwrap()`/`.expect()` on `None`/`Err`.

## `Result<T, E>`

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

```rust
use std::fs::File;

let f = File::open("hello.txt");
let f = match f {
    Ok(file) => file,
    Err(e) => panic!("failed to open file: {e}"),
};
```

### Shortcuts

| Method | On `Ok(v)` | On `Err(e)` |
|---|---|---|
| `.unwrap()` | returns `v` | panics with debug-formatted `e` |
| `.expect("msg")` | returns `v` | panics with `"msg: {e:?}"` |
| `.unwrap_or(default)` | returns `v` | returns `default` |
| `.unwrap_or_else(f)` | returns `v` | returns `f(e)` |
| `.unwrap_or_default()` | returns `v` | returns `T::default()` |
| `.ok()` | `Some(v)` | `None` (discards `e`) |
| `.map(f)` | `Ok(f(v))` | `Err(e)` unchanged |
| `.map_err(f)` | `Ok(v)` unchanged | `Err(f(e))` |

`.unwrap()`/`.expect()` turn a recoverable `Result` into an unrecoverable
panic — fine for prototypes, tests, and "the type system guarantees this is
`Ok`", risky for library code handling untrusted input.

## The `?` operator

`expr?` on a `Result`: if `Ok(v)`, evaluates to `v`; if `Err(e)`, **returns
`Err(e)` from the enclosing function immediately**. Only usable in functions
that themselves return `Result` (or `Option`, with the same short-circuit on
`None`).

```rust
fn read_username(path: &str) -> Result<String, io::Error> {
    let mut s = String::new();
    File::open(path)?.read_to_string(&mut s)?;
    Ok(s)
}
```

Equivalent to a `match` that early-returns on `Err`. `?` also applies
`From::from` to convert the error type into the function's error type — a
trick covered in depth in `intermediate/06`.

**Without a `From` impl, `?` requires the error types to match exactly.**
To bridge mismatched error types now, call `.map_err(...)` *before* `?`:

```rust
fn parse_two(a: &str, b: &str) -> Result<(i32, i32), String> {
    let a = a.parse::<i32>().map_err(|e| format!("bad a: {e}"))?;
    let b = b.parse::<i32>().map_err(|e| format!("bad b: {e}"))?;
    Ok((a, b))
}
```

### `?` with `Option`

```rust
fn first_char_upper(s: &str) -> Option<char> {
    let c = s.chars().next()?; // None short-circuits to None
    Some(c.to_ascii_uppercase())
}
```

## Custom error enums

One variant per failure mode, often carrying context fields:

```rust
#[derive(Debug, PartialEq)]
enum ConfigError {
    MissingKey(String),
    InvalidValue { key: String, value: String },
}
```

`#[derive(Debug)]` is required for `.unwrap()`/`.expect()` to format the
error; `PartialEq` lets tests `assert_eq!` against an expected `Err(...)`.

## Early returns / guard clauses

Idiomatic Rust validates preconditions up front and returns `Err`
immediately, keeping the "happy path" unindented:

```rust
fn withdraw(balance: i64, amount: i64) -> Result<i64, String> {
    if amount <= 0 {
        return Err("amount must be positive".to_string());
    }
    if amount > balance {
        return Err("insufficient funds".to_string());
    }
    Ok(balance - amount)
}
```

## Accumulating multiple errors

`?` and early-return give you the *first* error. To report *every* problem
(e.g. form validation), collect into a `Vec<E>` instead of short-circuiting:

```rust
fn validate(n: i32) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();
    if n < 0 {
        errors.push("must be non-negative".to_string());
    }
    if n % 2 != 0 {
        errors.push("must be even".to_string());
    }
    if errors.is_empty() { Ok(()) } else { Err(errors) }
}
```

## `.parse()` is strict

`"42".parse::<i32>()` is `Ok(42)`, but `"  42".parse::<i32>()` and
`"42 ".parse::<i32>()` are both `Err` — leading/trailing whitespace is
**not** trimmed automatically. Call `.trim()` first if the input might have
surrounding whitespace.

## To `panic!` or not to `panic!`

- **`Result`**: expected failure modes the caller can recover from (bad
  input, missing key, division by zero) — especially in library code, where
  the caller decides whether it's actually fatal.
- **`panic!`**: violated invariants / bugs / states that "can't happen" —
  e.g. an internal helper indexing a `Vec` it just built with a known-valid
  index. Also fine in prototypes, examples, and tests via
  `.unwrap()`/`.expect()`.

## `main` returning `Result`

```rust
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let contents = std::fs::read_to_string("config.txt")?;
    println!("{contents}");
    Ok(())
}
```

Returning `Err` from `main` prints the error (via `Debug`) and exits with a
nonzero status. `Box<dyn Error>` — a trait object covered in
`intermediate/06` — lets `main` accept any error type via `?`.

## Further Reading (Rust Book)

- [Ch. 9 — Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [Ch. 9.1 — Unrecoverable Errors with `panic!`](https://doc.rust-lang.org/book/ch09-01-unrecoverable-errors-with-panic.html)
- [Ch. 9.2 — Recoverable Errors with `Result`](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
- [Ch. 9.3 — To `panic!` or Not to `panic!`](https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html)
- [`std::result::Result`](https://doc.rust-lang.org/std/result/enum.Result.html)
- [`std::option::Option`](https://doc.rust-lang.org/std/option/enum.Option.html)
