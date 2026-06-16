//! Advanced 04 — Advanced Functions, Closures & Macros (Book ch. 20.4-20.5).
//!
//! Five exercises: `apply_all` (dispatch table of `fn` pointers), `make_pipeline`
//! (returning `Box<dyn Fn(i32) -> i32>` composed from a slice of boxed closures),
//! `call_with_one` (accepting any `Fn(i32) -> i32`), `sum_of_squares!` (a
//! `macro_rules!` macro summing the squares of its arguments), and `fold_with`
//! (generic fold over a slice using an `FnMut` accumulator).

// ---------------------------------------------------------------------------
// Exercise 1 — fn pointers: apply_all
// ---------------------------------------------------------------------------

/// Applies each function in `ops` to the corresponding element in `values`.
///
/// `ops` is a slice of bare function pointers `fn(i32) -> i32`.
/// Returns a `Vec<i32>` of length `min(ops.len(), values.len())`.
///
/// # Examples
///
/// ```ignore
/// use advanced_04_advanced_functions_and_macros::apply_all;
///
/// fn double(x: i32) -> i32 { x * 2 }
/// fn negate(x: i32) -> i32 { -x }
/// fn square(x: i32) -> i32 { x * x }
///
/// assert_eq!(apply_all(&[double, negate, square], &[3, 5, 4]), vec![6, -5, 16]);
/// assert_eq!(apply_all(&[], &[1, 2, 3]), vec![]);
/// ```
pub fn apply_all(ops: &[fn(i32) -> i32], values: &[i32]) -> Vec<i32> {
    todo!()
}

// ---------------------------------------------------------------------------
// Exercise 2 — returning closures: make_pipeline
// ---------------------------------------------------------------------------

/// Builds a pipeline by composing `steps` left-to-right.
///
/// Returns `Box<dyn Fn(i32) -> i32>`. Empty `steps` → identity function.
///
/// # Examples
///
/// ```ignore
/// use advanced_04_advanced_functions_and_macros::make_pipeline;
///
/// let f = make_pipeline(vec![
///     Box::new(|x: i32| x + 1) as Box<dyn Fn(i32) -> i32>,
///     Box::new(|x: i32| x * 2),
///     Box::new(|x: i32| x - 3),
/// ]);
/// assert_eq!(f(5), (5 + 1) * 2 - 3); // 9
/// assert_eq!(make_pipeline(vec![])(42), 42);
/// ```
pub fn make_pipeline(steps: Vec<Box<dyn Fn(i32) -> i32>>) -> Box<dyn Fn(i32) -> i32> {
    todo!()
}

// ---------------------------------------------------------------------------
// Exercise 3 — accepting Fn: call_with_one
// ---------------------------------------------------------------------------

/// Calls `f` with `1` and returns the result.
///
/// Accepts both function pointers and closures (anything `Fn(i32) -> i32`).
///
/// # Examples
///
/// ```ignore
/// use advanced_04_advanced_functions_and_macros::call_with_one;
///
/// assert_eq!(call_with_one(|x| x + 10), 11);
/// fn triple(x: i32) -> i32 { x * 3 }
/// assert_eq!(call_with_one(triple), 3);
/// ```
pub fn call_with_one<F: Fn(i32) -> i32>(f: F) -> i32 {
    todo!()
}

// ---------------------------------------------------------------------------
// Exercise 4 — macro_rules!: sum_of_squares!
// ---------------------------------------------------------------------------

/// Expands to the sum of the squares of all its arguments (as `i64`).
///
/// Accepts one or more comma-separated expressions.
///
/// # Examples
///
/// ```ignore
/// use advanced_04_advanced_functions_and_macros::sum_of_squares;
///
/// assert_eq!(sum_of_squares!(3), 9_i64);
/// assert_eq!(sum_of_squares!(3, 4), 25_i64);
/// assert_eq!(sum_of_squares!(1, 2, 3, 4), 30_i64);
/// ```
#[macro_export]
macro_rules! sum_of_squares {
    ($($x:expr),+) => {
        todo!()
    };
}

// ---------------------------------------------------------------------------
// Exercise 5 — FnMut: fold_with
// ---------------------------------------------------------------------------

/// Folds `slice` left-to-right using `f(accumulator, &element) -> accumulator`.
///
/// Equivalent to `slice.iter().fold(init, |acc, x| f(acc, x))`.
///
/// # Examples
///
/// ```ignore
/// use advanced_04_advanced_functions_and_macros::fold_with;
///
/// assert_eq!(fold_with(&[1, 2, 3, 4], 0, |acc, &x| acc + x), 10);
/// assert_eq!(fold_with(&[1, 2, 3, 4], 1, |acc, &x| acc * x), 24);
/// assert_eq!(fold_with::<i32, i32, _>(&[], 99, |acc, &x| acc + x), 99);
/// ```
pub fn fold_with<T, B, F>(slice: &[T], init: B, f: F) -> B
where
    F: FnMut(B, &T) -> B,
{
    todo!()
}
