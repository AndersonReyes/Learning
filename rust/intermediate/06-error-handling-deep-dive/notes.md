# Error Handling Deep Dive: `From`, `Box<dyn Error>`, `source()`

`fundamentals/10` covered custom error enums + `.map_err()`. This topic
covers the alternative the `?` operator is built around: `From`-based
automatic conversion, type-erasure with `Box<dyn Error>`, error chains via
`source()`, and downcasting.

## The `std::error::Error` trait

```rust
pub trait Error: Debug + Display {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None // default: no underlying cause
    }
}
```

Any type that implements `Debug` + `Display` can implement `Error` — usually
with an empty body (the `Display` impl supplies the message):

```rust
use std::fmt;
use std::error::Error;

#[derive(Debug)]
struct EmptyInputError;

impl fmt::Display for EmptyInputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "input is empty")
    }
}

impl Error for EmptyInputError {}
```

## `From` and `?`: automatic error conversion

`expr?` does roughly:

```text
match expr {
    Ok(v) => v,
    Err(e) => return Err(From::from(e)),
}
```

So if your function returns `Result<T, MyError>` and `expr` is
`Result<T, ParseIntError>`, `expr?` compiles *as long as* `impl
From<ParseIntError> for MyError` exists — no `.map_err()` needed:

```rust
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
enum ConfigError {
    Empty,
    BadNumber(ParseIntError),
}

impl From<ParseIntError> for ConfigError {
    fn from(e: ParseIntError) -> Self {
        ConfigError::BadNumber(e)
    }
}

fn parse_count(s: &str) -> Result<u32, ConfigError> {
    if s.is_empty() {
        return Err(ConfigError::Empty);
    }
    let n = s.parse::<u32>()?; // ParseIntError -> ConfigError via From
    Ok(n)
}
```

One `From<E> for MyError` impl per *source* error type `E`. If two different
operations both produce `ParseIntError` but should map to different `MyError`
variants, `From` can't disambiguate — fall back to `.map_err()` for those.

## `Box<dyn Error>`: type-erased catch-all

The standard library provides a blanket impl:

```text
impl<'a, E: Error + 'a> From<E> for Box<dyn Error + 'a>
```

So `Result<T, Box<dyn Error>>` accepts `?` from **any** error type — useful
when a function calls several operations with unrelated error types and you
don't want to define a new enum for every combination:

```rust
use std::error::Error;

fn sum_fields(line: &str) -> Result<i64, Box<dyn Error>> {
    if line.trim().is_empty() {
        return Err(Box::new(EmptyInputError)); // custom error, boxed
    }
    let mut total = 0;
    for field in line.split(',') {
        total += field.trim().parse::<i64>()?; // ParseIntError, auto-boxed
    }
    Ok(total)
}
```

Trade-off: callers lose static type information about *which* error
occurred — they only know it's "some `Error`". Use a custom enum (with
`From` impls) when callers need to `match` on failure modes; use `Box<dyn
Error>` for "just propagate/report it" code (`main`, CLI tools, glue code).

## `source()`: error chains

`source()` exposes the *underlying cause* of an error, letting callers walk a
chain from the outermost wrapper to the root cause:

```rust
#[derive(Debug)]
struct WrappedError {
    message: String,
    source: Option<Box<dyn Error>>,
}

impl fmt::Display for WrappedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for WrappedError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_deref()
    }
}

fn print_chain(err: &dyn Error) {
    eprintln!("{err}");
    let mut cause = err.source();
    while let Some(e) = cause {
        eprintln!("caused by: {e}");
        cause = e.source();
    }
}
```

## Downcasting `dyn Error`

`dyn Error + 'static` has inherent `downcast_ref`/`downcast_mut`/`downcast`
methods (like `dyn Any`), letting you recover the concrete type when you need
to branch on *which* error occurred without a shared enum:

```rust
fn handle(err: &(dyn Error + 'static)) {
    if let Some(e) = err.downcast_ref::<EmptyInputError>() {
        eprintln!("empty input: {e}");
    } else {
        eprintln!("other error: {err}");
    }
}
```

`Box<dyn Error>` derefs to `dyn Error`, so `boxed_err.downcast_ref::<T>()`
works directly on the box too.

## Gotchas

- `Box<dyn Error>` defaults to `Box<dyn Error + 'static>` — the boxed error
  can't (transitively) borrow data with a shorter lifetime.
- `source()`'s default returns `None`; forgetting to override it for a
  wrapper error silently breaks chain traversal (no compile error).
- `From<E> for MyError` only fires for `?`'s implicit conversion — calling
  `MyError::from(e)` or `e.into()` explicitly works the same way, but a plain
  `Err(e)` (no `?`) does **not** convert.

## Further Reading (Book)

- [Ch. 9.2 — Recoverable Errors with `Result`](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html) ("A Shortcut for Propagating Errors: the `?` Operator")
- [`std::error::Error`](https://doc.rust-lang.org/std/error/trait.Error.html) (`source`, `downcast_ref`)
- [`std::convert::From`](https://doc.rust-lang.org/std/convert/trait.From.html)
- [`impl<'a, E: Error + 'a> From<E> for Box<dyn Error + 'a>`](https://doc.rust-lang.org/std/boxed/struct.Box.html#impl-From%3CE%3E-for-Box%3Cdyn+Error+%2B+'a%3E)
