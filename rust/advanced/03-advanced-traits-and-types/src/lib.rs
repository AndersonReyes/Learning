//! Advanced 03 — Advanced Traits & Types (Book ch. 20.2-20.3).
//!
//! Five exercises covering: associated types with a `Magnitude` trait,
//! operator overloading on `Matrix2x2` (Add + scalar Mul + matrix multiply),
//! the newtype pattern with `WordCloud` (HashMap wrapper + Display + top_n),
//! supertraits with `Summarize: Display`, and fully qualified syntax to
//! disambiguate two traits (`Greet` and `Farewell`) that expose the same
//! method name on `Person`.

use std::collections::HashMap;

// ---------------------------------------------------------------------------
// Exercise 1 — Associated type: Magnitude
// ---------------------------------------------------------------------------

/// A trait whose implementors can report their "magnitude" as an associated
/// output type.
///
/// # Associated type
///
/// `Output` must implement `PartialOrd + Copy` so that magnitudes can be
/// compared and used without moving.
pub trait Magnitude {
    type Output: PartialOrd + Copy;

    /// Returns the magnitude of `self`.
    fn magnitude(&self) -> Self::Output;
}

/// Implement `Magnitude` for `i64`:
/// - `type Output = u64`
/// - `magnitude()` returns `self.unsigned_abs()` (absolute value as `u64`)
impl Magnitude for i64 {
    type Output = u64;

    fn magnitude(&self) -> u64 {
        todo!()
    }
}

/// Implement `Magnitude` for `(f64, f64)` (a 2-D vector):
/// - `type Output = f64`
/// - `magnitude()` returns `(x² + y²).sqrt()`
impl Magnitude for (f64, f64) {
    type Output = f64;

    fn magnitude(&self) -> f64 {
        todo!()
    }
}

/// Returns the largest magnitude among `items`, or `None` if `items` is empty.
///
/// Uses `Iterator::map` + `Iterator::reduce`, comparing with `PartialOrd`.
///
/// # Examples
///
/// ```ignore
/// use advanced_03_advanced_traits_and_types::{Magnitude, max_magnitude};
///
/// assert_eq!(max_magnitude(&[3_i64, -7, 2]), Some(7_u64));
/// assert_eq!(max_magnitude(&[] as &[i64]),   None);
/// assert_eq!(max_magnitude(&[(3.0_f64, 4.0_f64)]), Some(5.0_f64));
/// ```
pub fn max_magnitude<T: Magnitude>(items: &[T]) -> Option<T::Output> {
    todo!()
}

// ---------------------------------------------------------------------------
// Exercise 2 — Operator overloading: Matrix2x2
// ---------------------------------------------------------------------------

/// A 2×2 matrix stored in row-major order.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix2x2 {
    pub data: [[f64; 2]; 2],
}

/// Element-wise addition of two `Matrix2x2` values.
impl std::ops::Add for Matrix2x2 {
    type Output = Matrix2x2;

    fn add(self, rhs: Matrix2x2) -> Matrix2x2 {
        todo!()
    }
}

/// Scalar multiplication: each element multiplied by `rhs: f64`.
impl std::ops::Mul<f64> for Matrix2x2 {
    type Output = Matrix2x2;

    fn mul(self, rhs: f64) -> Matrix2x2 {
        todo!()
    }
}

/// Standard 2×2 matrix multiplication: `C = A × B`.
///
/// `C[i][j] = A[i][0]*B[0][j] + A[i][1]*B[1][j]`
pub fn mat_mul(a: Matrix2x2, b: Matrix2x2) -> Matrix2x2 {
    todo!()
}

// ---------------------------------------------------------------------------
// Exercise 3 — Newtype pattern: WordCloud
// ---------------------------------------------------------------------------

/// Newtype wrapper around a word-frequency map.
pub struct WordCloud(pub HashMap<String, usize>);

/// Display implementation: entries sorted descending by count; ties broken
/// ascending alphabetically. Each entry is formatted as `"word: N\n"`.
impl std::fmt::Display for WordCloud {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

/// Returns the top-`n` entries from `cloud` in the same sort order used by
/// `Display` (descending count, ascending alpha on ties).
///
/// If `n` exceeds the number of entries, returns all entries.
pub fn top_n(cloud: &WordCloud, n: usize) -> Vec<(&str, usize)> {
    todo!()
}

// ---------------------------------------------------------------------------
// Exercise 4 — Supertrait: Summarize
// ---------------------------------------------------------------------------

/// A trait for types that can produce a one-line summary of themselves.
///
/// Requires `Display` as a supertrait — `headline` uses `self`'s `Display`
/// output and `author()`.
pub trait Summarize: std::fmt::Display {
    /// Returns the author's name.
    fn author(&self) -> String;

    /// Returns a headline string: `"{Display output} — by {author}"`.
    fn headline(&self) -> String {
        format!("{self} \u{2014} by {}", self.author())
    }
}

/// A news article with a title, author, and word count.
#[derive(Debug, Clone)]
pub struct NewsArticle {
    pub title: String,
    pub author: String,
    pub word_count: u32,
}

/// Display format: `"{title} ({word_count} words)"`.
impl std::fmt::Display for NewsArticle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

/// `Summarize` impl: `author()` returns `self.author.clone()`.
impl Summarize for NewsArticle {
    fn author(&self) -> String {
        todo!()
    }
}

/// Collects `item.headline()` for every element in `items`.
pub fn headlines<T: Summarize>(items: &[T]) -> Vec<String> {
    todo!()
}

// ---------------------------------------------------------------------------
// Exercise 5 — Fully qualified syntax
// ---------------------------------------------------------------------------

/// A greeting message.
pub trait Greet {
    /// Returns a greeting: `"Hello, {name}!"`.
    fn message(&self) -> String;
}

/// A farewell message.
pub trait Farewell {
    /// Returns a farewell: `"Goodbye, {name}!"`.
    fn message(&self) -> String;
}

/// A person with a name.
pub struct Person {
    pub name: String,
}

impl Greet for Person {
    fn message(&self) -> String {
        todo!()
    }
}

impl Farewell for Person {
    fn message(&self) -> String {
        todo!()
    }
}

/// Returns `(greet_message, farewell_message)` using fully qualified syntax
/// to disambiguate the two `message` methods.
pub fn greet_and_farewell(p: &Person) -> (String, String) {
    todo!()
}
