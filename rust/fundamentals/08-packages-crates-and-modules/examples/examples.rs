//! Run with: `cargo run --example examples -p fundamentals-08-packages-crates-and-modules`
//!
//! Demonstrates `notes.md`'s module-system concepts using a small,
//! self-contained module tree defined right here: `mod`, `pub`, privacy
//! rules (children see into private ancestors; parents need `pub` to see
//! into children), `crate::`/`self::`/`super::` paths, `use` (including
//! `as` and nested paths), and `pub use` re-exporting. Self-contained:
//! doesn't call into this package's exercises (those are unimplemented
//! `todo!()` stubs until you finish them).

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() -> &'static str {
            "added to waitlist"
        }

        // Private to `hosting`, but visible to `hosting`'s descendants
        // (there are none here) and to `hosting` itself.
        fn seat_at_table() -> &'static str {
            "seated"
        }

        pub fn seat_next_in_line() -> &'static str {
            // A module can always use its own private items.
            seat_at_table()
        }
    }

    // Private module — only visible within `front_of_house` and its
    // descendants (e.g. `back_of_house` below, via `super::`).
    mod back_of_house {
        pub fn fix_incorrect_order() -> &'static str {
            // `super::` reaches the parent module (`front_of_house`),
            // and from there into its public `hosting` submodule.
            super::hosting::add_to_waitlist()
        }
    }

    pub fn serve_from_kitchen() -> &'static str {
        back_of_house::fix_incorrect_order()
    }
}

// Re-export: callers of THIS crate can reach `add_to_waitlist` as
// `crate::add_to_waitlist`, without knowing about `front_of_house::hosting`.
pub use front_of_house::hosting::add_to_waitlist;

// `use` for a type: bring the full path into scope, use the bare name.
use std::collections::HashMap;

// `use ... as`: avoid a name clash between two `Result` types.
use std::fmt::Result as FmtResult;
use std::io::Result as IoResult;

// Nested path: two imports from `std::cmp` and `std::collections` in one `use`.
use std::{cmp::Ordering, collections::HashSet};

fn fmt_ok() -> FmtResult {
    Ok(())
}

fn io_ok() -> IoResult<()> {
    Ok(())
}

fn main() {
    // --- Absolute path from the crate root ---
    println!(
        "crate::front_of_house::hosting::add_to_waitlist() = {}",
        front_of_house::hosting::add_to_waitlist()
    );

    // --- pub use re-export: same function, shorter path ---
    println!("add_to_waitlist() via re-export = {}", add_to_waitlist());

    // --- private item used by its own module ---
    println!(
        "hosting::seat_next_in_line() = {}",
        front_of_house::hosting::seat_next_in_line()
    );

    // --- super:: reaching from a child module back to its parent's sibling ---
    println!(
        "front_of_house::serve_from_kitchen() = {}",
        front_of_house::serve_from_kitchen()
    );

    // --- use for types: HashMap/HashSet by bare name ---
    let mut counts: HashMap<&str, i32> = HashMap::new();
    counts.insert("apples", 3);
    let mut seen: HashSet<&str> = HashSet::new();
    seen.insert("apples");
    println!("counts = {counts:?}, seen = {seen:?}");

    // --- `as` aliasing avoids the Result name clash ---
    println!("fmt_ok() = {:?}, io_ok() = {:?}", fmt_ok(), io_ok());

    // --- Ordering from the nested `use` ---
    let ord = 3.cmp(&5);
    println!("3.cmp(&5) = {ord:?}, is Less = {}", ord == Ordering::Less);
}
