//! Fundamentals 05 — The Slice Type & `&str`.
//!
//! Exercises focus on the `&[T]`/`&str` slicing, range syntax, and
//! UTF-8 byte-vs-char distinctions from `notes.md`.

/// Splits `s` on runs of one or more whitespace characters, returning the
/// non-empty pieces in between as `&str` slices borrowing from `s` (leading,
/// trailing, and repeated whitespace produce no empty pieces).
///
/// # Examples
///
/// ```ignore
/// assert_eq!(split_on_whitespace_runs("  hello   world  "), vec!["hello", "world"]);
/// assert_eq!(split_on_whitespace_runs("a"), vec!["a"]);
/// assert_eq!(split_on_whitespace_runs("   "), Vec::<&str>::new());
/// assert_eq!(split_on_whitespace_runs(""), Vec::<&str>::new());
/// assert_eq!(split_on_whitespace_runs("one\ttwo\nthree"), vec!["one", "two", "three"]);
/// ```
pub fn split_on_whitespace_runs(s: &str) -> Vec<&str> {
    todo!()
}

/// Returns a `&str` slice containing the first `n` *characters* (not bytes)
/// of `s`. If `s` has fewer than `n` characters, returns all of `s`. Handles
/// multi-byte UTF-8 characters correctly (never panics on a char-boundary
/// violation).
///
/// # Examples
///
/// ```ignore
/// assert_eq!(first_n_chars("hello world", 5), "hello");
/// assert_eq!(first_n_chars("héllo", 2), "hé");   // 'é' is 2 bytes
/// assert_eq!(first_n_chars("🦀rust", 1), "🦀");   // 🦀 is 4 bytes
/// assert_eq!(first_n_chars("hi", 10), "hi");
/// assert_eq!(first_n_chars("", 3), "");
/// ```
pub fn first_n_chars(s: &str, n: usize) -> &str {
    todo!()
}

/// Returns the longest palindromic substring of `s` as a `&str` slice
/// (borrowing from `s`). Ties are broken by leftmost starting position.
///
/// # Panics / Preconditions
///
/// `s` must be ASCII (so byte indices coincide with character positions).
///
/// # Examples
///
/// ```ignore
/// assert_eq!(longest_palindromic_substring_slice("babad"), "bab"); // "aba" ties, "bab" is leftmost
/// assert_eq!(longest_palindromic_substring_slice("cbbd"), "bb");
/// assert_eq!(longest_palindromic_substring_slice("a"), "a");
/// assert_eq!(longest_palindromic_substring_slice(""), "");
/// ```
pub fn longest_palindromic_substring_slice(s: &str) -> &str {
    todo!()
}

/// Returns the contiguous sub-slice of `values` with the maximum sum
/// (Kadane's algorithm), as a `&[i32]` slice borrowing from `values`. Ties
/// (multiple sub-slices with the same maximum sum) are broken by leftmost
/// starting position.
///
/// # Panics / Preconditions
///
/// `values` must be non-empty.
///
/// # Examples
///
/// ```ignore
/// assert_eq!(
///     max_subarray_slice(&[-2, 1, -3, 4, -1, 2, 1, -5, 4]),
///     &[4, -1, 2, 1]
/// ); // sum == 6
/// assert_eq!(max_subarray_slice(&[-3, -1, -2]), &[-1]); // least-negative single element
/// assert_eq!(max_subarray_slice(&[5]), &[5]);
/// assert_eq!(max_subarray_slice(&[1, 2, 3, 4]), &[1, 2, 3, 4]);
/// ```
pub fn max_subarray_slice(values: &[i32]) -> &[i32] {
    todo!()
}

/// Splits `values` into consecutive, non-overlapping sub-slices of length
/// `chunk_size`, in order (the final chunk may be shorter if `values.len()`
/// isn't a multiple of `chunk_size`).
///
/// # Panics / Preconditions
///
/// `chunk_size` must be greater than `0`.
///
/// # Examples
///
/// ```ignore
/// assert_eq!(chunk_slices(&[1, 2, 3, 4, 5], 2), vec![&[1, 2][..], &[3, 4][..], &[5][..]]);
/// assert_eq!(chunk_slices(&[1, 2, 3, 4], 2), vec![&[1, 2][..], &[3, 4][..]]);
/// assert_eq!(chunk_slices(&[] as &[i32], 3), Vec::<&[i32]>::new());
/// assert_eq!(chunk_slices(&[1, 2, 3], 5), vec![&[1, 2, 3][..]]);
/// ```
pub fn chunk_slices(values: &[i32], chunk_size: usize) -> Vec<&[i32]> {
    todo!()
}
