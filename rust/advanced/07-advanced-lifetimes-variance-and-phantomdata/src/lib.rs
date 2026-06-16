//! Advanced 07 — Advanced Lifetimes, Variance & PhantomData (Nomicon).
//!
//! Five exercises exercising explicit lifetimes and `PhantomData`:
//! `longest_with_announcement` (multiple lifetime params + `'b: 'a` bound),
//! `StrSplit` (a struct-based iterator splitting on a delimiter with two
//! lifetime params), `Token` / `PhantomData` branding (typed IDs that can't
//! be mixed), `split_fields` (splitting borrows across struct fields), and
//! `apply_all_refs` (HRTB `for<'a> Fn(&'a T) -> &'a T`).

use std::marker::PhantomData;

// ---------------------------------------------------------------------------
// Exercise 1 — multiple lifetimes: longest_with_announcement
// ---------------------------------------------------------------------------

/// Returns the longer of `x` and `y` (by byte length) and prints `ann`.
///
/// `'b: 'a` means `ann` must live at least as long as the string slices.
///
/// # Examples
///
/// ```ignore
/// use advanced_07_advanced_lifetimes_variance_and_phantomdata::longest_with_announcement;
///
/// assert_eq!(longest_with_announcement("long string", "xy", "note"), "long string");
/// ```
pub fn longest_with_announcement<'a, 'b: 'a>(
    x: &'a str,
    y: &'a str,
    ann: &'b str,
) -> &'a str {
    todo!()
}

// ---------------------------------------------------------------------------
// Exercise 2 — lifetime in struct: StrSplit
// ---------------------------------------------------------------------------

/// An iterator that splits `haystack` on `delimiter`, yielding `&'h str` slices.
///
/// `'h` is the haystack lifetime; `'d` is the delimiter lifetime.
pub struct StrSplit<'h, 'd> {
    remainder: Option<&'h str>,
    delimiter: &'d str,
}

impl<'h, 'd> StrSplit<'h, 'd> {
    /// Creates a new `StrSplit`.
    pub fn new(haystack: &'h str, delimiter: &'d str) -> Self {
        todo!()
    }
}

impl<'h, 'd> Iterator for StrSplit<'h, 'd> {
    type Item = &'h str;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

// ---------------------------------------------------------------------------
// Exercise 3 — PhantomData branding: Token
// ---------------------------------------------------------------------------

/// A typed token branded with a phantom `Brand` type.
///
/// Prevents mixing tokens of different brands even if they hold the same `u64`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token<Brand> {
    value: u64,
    _brand: PhantomData<Brand>,
}

impl<Brand> Token<Brand> {
    /// Creates a new `Token`.
    pub fn new(value: u64) -> Self {
        todo!()
    }

    /// Returns the inner value.
    pub fn value(self) -> u64 {
        todo!()
    }
}

// ---------------------------------------------------------------------------
// Exercise 4 — splitting borrows: split_fields
// ---------------------------------------------------------------------------

/// A record with two independently-borrowable string fields.
pub struct Record {
    pub name: String,
    pub description: String,
}

/// Returns `(&mut record.name, &mut record.description)` simultaneously.
///
/// The compiler can verify this is safe because the two fields are disjoint.
pub fn split_fields(record: &mut Record) -> (&mut String, &mut String) {
    todo!()
}

// ---------------------------------------------------------------------------
// Exercise 5 — HRTB: apply_all_refs
// ---------------------------------------------------------------------------

/// Applies `f` to each element and returns references to the results.
///
/// `F: for<'a> Fn(&'a T) -> &'a T` — `f` must work for any lifetime.
pub fn apply_all_refs<T, F>(items: &[T], f: F) -> Vec<&T>
where
    F: for<'a> Fn(&'a T) -> &'a T,
{
    todo!()
}
