# Advanced Functions, Closures & Macros

Book ch. 20.4-20.5.

---

## Function pointers (`fn` items)

A bare `fn(T) -> U` is a *function pointer* — a type, not a trait. Unlike
closures, function pointers don't capture environment.

```rust
fn double(x: i32) -> i32 { x * 2 }

let f: fn(i32) -> i32 = double;  // OK — named function coerces to fn ptr
println!("{}", f(3));             // 6
```

`fn` pointers implement all three closure traits: `Fn`, `FnMut`, `FnOnce`.
So a `fn` pointer is acceptable anywhere a closure is, but not vice versa.

```rust
fn apply<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 { f(x) }
apply(double, 5);            // fn ptr coerces to Fn — fine
apply(|x| x * 2, 5);        // closure — also fine
```

### Enum variant constructors

Enum variants that take arguments behave like `fn` pointers:

```rust
let v: Vec<Option<i32>> = (0..5).map(Some).collect();
// Some is a fn(i32) -> Option<i32>
```

## Returning closures

Closures are unsized — they can't appear as bare return types. Use
`Box<dyn Fn>` or `impl Fn` (when the concrete type is fixed):

```rust
fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n          // OK: single concrete closure type per call site
}

fn make_adder_dyn(n: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |x| x + n) // OK: erased, heap-allocated
}
```

Use `Box<dyn Fn...>` when the concrete type varies (e.g. stored in a
`Vec`, returned from a branch, or in a `struct` field).

## Dispatch tables (function pointers in data structures)

```rust
type Handler = fn(&str) -> String;
let mut table: HashMap<&str, Handler> = HashMap::new();
table.insert("upper", |s| s.to_uppercase());  // closure with no captures
// — coerces to fn ptr because it doesn't capture environment
```

Non-capturing closures coerce to `fn` pointers; capturing closures don't.

## Macros

Macros expand before type-checking. Two kinds:

### Declarative macros (`macro_rules!`)

Pattern-based text substitution. Each arm matches a fragment kind:

| specifier | matches                             |
|-----------|-------------------------------------|
| `expr`    | any expression                      |
| `stmt`    | any statement                       |
| `ty`      | a type                              |
| `ident`   | an identifier                       |
| `tt`      | a single "token tree" (anything)    |
| `$(...)*` | repeat zero or more times           |
| `$(...)+` | repeat one or more times            |

```rust
macro_rules! assert_close {
    ($a:expr, $b:expr, $eps:expr) => {
        assert!(($a - $b).abs() < $eps, "{} and {} differ by > {}", $a, $b, $eps)
    };
}
```

### Procedural macros (proc macros)

Run Rust code at compile time that transforms a `TokenStream`. Three
flavours: `#[derive(...)]`, attribute (`#[my_attr]`), and function-like
(`my_macro!(...)`). Require their own crate with `proc-macro = true`. Not
covered in exercises — too dependent on external crate plumbing.

## Gotchas

- `fn` pointer vs. `Fn` trait: `fn(i32) -> i32` is a concrete type; `Fn(i32)
  -> i32` is a trait. Use `fn` pointers in dispatch tables; use trait bounds
  for generic parameters.
- Capturing closures can't coerce to `fn(...)` — only zero-capture closures
  can, because a `fn` pointer holds no environment.
- `impl Fn` in return position is monomorphic per call — the caller can't
  dynamically swap which closure is returned. Use `Box<dyn Fn>` for that.
- `macro_rules!` hygiene: identifiers introduced inside a macro arm don't
  leak into the caller's scope (by default). To intentionally export an
  identifier, receive it as a `$name:ident` argument.

## Further Reading

- [Book ch. 20.4 — Advanced Functions and Closures](https://doc.rust-lang.org/book/ch20-04-advanced-functions-and-closures.html)
- [Book ch. 20.5 — Macros](https://doc.rust-lang.org/book/ch20-05-macros.html)
- [Reference — Function pointers](https://doc.rust-lang.org/reference/types/function-pointer.html)
- [Reference — Macros by example](https://doc.rust-lang.org/reference/macros-by-example.html)
- [`std::ops::Fn` / `FnMut` / `FnOnce`](https://doc.rust-lang.org/std/ops/trait.Fn.html)
