# CLI I/O Project

Book ch.12 builds `minigrep`: a small CLI tool that reads command-line
arguments, reads a file, and searches its contents. The big lesson isn't
the algorithm — it's **separation of concerns**: parsing/logic goes in
`lib.rs` (testable), `main.rs` stays a thin shell (args -> lib -> exit code).

## Command-line arguments: `std::env::args`

```rust
use std::env;

let args: Vec<String> = env::args().collect();
// args[0] is the binary's own path -- always skip it.
let query = &args[1];
let path = &args[2];
```

- `env::args()` returns an iterator of `String`. **Panics** if any argument
  isn't valid Unicode — use `env::args_os()` (yields `OsString`) if you must
  handle arbitrary bytes.
- Always `.collect()` into a `Vec` if you need to index/slice; iterating
  once is fine for simple cases.

## Reading a file: `std::fs::read_to_string`

```rust
use std::fs;

let contents = fs::read_to_string(path)?; // io::Result<String>
```

Returns `Err(io::Error)` if the path doesn't exist, isn't readable, or
isn't valid UTF-8.

## `Config::build`: parsing into a struct

Pull argument-parsing into its own constructor returning `Result`, so
`main` can handle errors uniformly instead of `.unwrap()`-ing everywhere:

```rust
struct Config {
    query: String,
    path: String,
    ignore_case: bool,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, String> {
        if args.len() < 3 {
            return Err("not enough arguments".to_string());
        }
        Ok(Config {
            query: args[1].clone(),
            path: args[2].clone(),
            ignore_case: env::var("IGNORE_CASE").is_ok(),
        })
    }
}
```

`main` then does:

```rust
let config = Config::build(&args).unwrap_or_else(|err| {
    eprintln!("Problem parsing arguments: {err}");
    process::exit(1);
});
```

## `Box<dyn Error>`: one error type for many failure modes

A `run` function can fail for unrelated reasons (file I/O, bad config). A
trait object lets it return *any* error type without a custom enum:

```rust
use std::error::Error;

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.path)?; // io::Error -> Box<dyn Error>
    // ... use contents ...
    Ok(())
}
```

`?` converts any `E: Error` into `Box<dyn Error>` via a blanket `From`
impl. This is a stepping stone to `intermediate/06`'s deeper look at custom
`From` conversions.

## TDD: `search` and `search_case_insensitive`

Ch.12.4 develops the core logic test-first:

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}
```

- The lifetime `'a` says: the returned `&str`s borrow from `contents`, not
  `query` — the result can't outlive the text it was found in.
- `search_case_insensitive` lowercases both `query` and each `line` before
  comparing — note this *returns the original-case line*, only the
  comparison is folded.
- Write the test (`assert_eq!` on expected matches) **before** the
  implementation — it starts red, then turns green. This is the same
  red/green cycle `tests/exercise_test.rs` puts you through here.

## Environment variables: `std::env::var`

```rust
use std::env;

let ignore_case = env::var("IGNORE_CASE").is_ok();
```

- `env::var(key)` returns `Result<String, VarError>` — `Err` if the var is
  unset (or not valid Unicode).
- `.is_ok()` turns "is this var set at all" into a `bool` — a common
  pattern for boolean flags (`IGNORE_CASE=1 cargo run -- to poem.txt`), but
  it means `IGNORE_CASE=` (empty) and `IGNORE_CASE=0` both count as "set" =
  true. A stricter check inspects the *value*, not just presence.

## stdout vs. stderr: `println!` vs `eprintln!`

```rust
println!("{line}");           // normal output -> stdout
eprintln!("Application error: {e}"); // errors -> stderr
```

Why it matters: `cargo run > output.txt` redirects only stdout. Error
messages printed with `eprintln!` still show up on the terminal even when
normal output is redirected to a file — the user sees errors even if
they're capturing results.

## `main`'s shape

```rust
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
```

`main` only: collects args, builds config (exit 1 on error), calls `run`
(exit 1 on error). All real logic — and all the unit tests — live in
`lib.rs`.

## Further Reading (Book)

- [Ch. 12 — An I/O Project: Building a Command Line Program](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)
- [Ch. 12.1 — Accepting Command Line Arguments](https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html)
- [Ch. 12.2 — Reading a File](https://doc.rust-lang.org/book/ch12-02-reading-a-file.html)
- [Ch. 12.3 — Refactoring to Improve Modularity and Error Handling](https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html)
- [Ch. 12.4 — Developing the Library's Functionality with Test-Driven Development](https://doc.rust-lang.org/book/ch12-04-testing-the-librarys-functionality.html)
- [Ch. 12.5 — Working with Environment Variables](https://doc.rust-lang.org/book/ch12-05-working-with-environment-variables.html)
- [Ch. 12.6 — Writing Error Messages to Standard Error Instead of Standard Output](https://doc.rust-lang.org/book/ch12-06-writing-to-stderr.html)
- [`std::env::args`](https://doc.rust-lang.org/std/env/fn.args.html), [`std::env::var`](https://doc.rust-lang.org/std/env/fn.var.html), [`std::fs::read_to_string`](https://doc.rust-lang.org/std/fs/fn.read_to_string.html)
