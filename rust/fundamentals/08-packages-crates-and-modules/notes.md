# Packages, Crates & Modules

## Terminology

- **Package** — a directory with a `Cargo.toml`. Can contain at most one
  library crate and any number of binary crates.
- **Crate** — a compilation unit. The library crate's root is `src/lib.rs`;
  each binary's root is `src/main.rs` (or `src/bin/*.rs`).
- **Module** (`mod`) — a namespace *within* a crate, for organization and
  privacy. This topic is about modules.

## Defining modules

`mod` declares a module. The crate root (`lib.rs`) is itself an implicit
module, conventionally referred to as `crate`:

```rust
// src/lib.rs
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}
```

This creates a tree:

```text
crate
└── front_of_house
    └── hosting
        └── add_to_waitlist
```

## Privacy rules

**Everything is private by default.** The exact rule:

- A module's items are visible to that module **and all of its
  descendants** (children, grandchildren, ...), regardless of `pub`.
- To be visible to anything *else* (siblings, ancestors, other crates), an
  item needs `pub`.

So a child can always see into its parent's private items, but a parent
needs `pub` to see into a child's items — and an external crate needs every
item *and every module on the path to it* to be `pub`.

```rust
mod back_of_house {
    fn fix_incorrect_order() {
        cook_order(); // OK: sibling fn in the same module
        super::serve_order(); // OK: ancestor's item, via `super`
    }
    fn cook_order() {}
}
fn serve_order() {}
```

## Paths

- **Absolute path**: starts with `crate::` — `crate::front_of_house::hosting::add_to_waitlist()`.
- **Relative path**: starts with `self::`, `super::`, or an identifier in
  the current module. `super::` refers to the parent module — useful for
  calling back "up" the tree (e.g. from a `tests` submodule to the function
  under test).

## `use` — bringing paths into scope

```rust
use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist(); // not just add_to_waitlist() — see idiom below
}
```

**Idiom**: for *functions*, bring the parent module into scope and call
`module::function()` — the module prefix at the call site signals "not
defined here." For *types* (structs, enums, traits), bring the full path
into scope and use the bare name: `use std::collections::HashMap;` then
`HashMap::new()`.

### `as`, nested paths, glob

```rust
use std::fmt::Result;
use std::io::Result as IoResult; // `as` to avoid a name clash

use std::{cmp::Ordering, io::{self, Write}}; // nested paths

use std::collections::*; // glob — brings in everything public; avoid in
                          // library code, fine for quick exploration/tests
```

### Re-exporting with `pub use`

`pub use` re-exports an imported path as part of *this* module's public
API — callers can use the new shorter path without knowing the internal
structure:

```rust
mod stats {
    pub fn median(values: &[f64]) -> f64 { /* ... */ todo!() }
}
pub use stats::median; // crate::median(...) now also works
```

## Separating modules into files

`mod garden;` (no body) tells Rust to load the module's contents from a
file:

```text
src/
├── lib.rs       // `pub mod garden;`
└── garden.rs    // contents of the `garden` module
```

A submodule of `garden` (e.g. `mod vegetables;` written inside
`garden.rs`) lives at `src/garden/vegetables.rs`. (Older code may use
`src/garden/mod.rs` instead of `src/garden.rs` — both work, but the
flat `src/garden.rs` + `src/garden/` form is the modern convention and
avoids having many files all named `mod.rs`.)

## Further Reading

- [The Book, ch. 7 — Managing Growing Projects with Packages, Crates, and Modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)
- [ch. 7.2 — Defining Modules to Control Scope and Privacy](https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html)
- [ch. 7.3 — Paths for Referring to an Item in the Module Tree](https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html)
- [ch. 7.4 — Bringing Paths into Scope with `use`](https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html)
- [ch. 7.5 — Separating Modules into Different Files](https://doc.rust-lang.org/book/ch07-05-separating-modules-into-different-files.html)
- [Reference — Modules](https://doc.rust-lang.org/reference/items/modules.html)
- [Reference — Visibility and privacy](https://doc.rust-lang.org/reference/visibility-and-privacy.html)
