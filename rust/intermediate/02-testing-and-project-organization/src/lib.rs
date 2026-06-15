//! Intermediate 02 — Writing Tests & Project Organization.
//!
//! `notes.md` covers `cargo test` mechanics (`#[should_panic]`, tests
//! returning `Result`, unit vs. integration tests); the 5 exercises below
//! are standard hard algorithmic problems, exercised by
//! `tests/exercise_test.rs` using several of those techniques.

/// Performs binary search for `target` in `sorted` (must be sorted
/// ascending). Returns `Some(index)` if found, `None` otherwise.
///
/// If `sorted` contains duplicates of `target`, the returned index may be
/// any one of them.
///
/// # Examples
///
/// ```ignore
/// use intermediate_02_testing_and_project_organization::binary_search;
///
/// assert_eq!(binary_search(&[1, 3, 5, 7, 9, 11], &7), Some(3));
/// assert_eq!(binary_search(&[1, 3, 5, 7, 9, 11], &4), None);
/// assert_eq!(binary_search::<i32>(&[], &5), None);
/// ```
pub fn binary_search<T: Ord>(sorted: &[T], target: &T) -> Option<usize> {
    todo!()
}

/// Returns the `k`-th smallest value in `values` (1-indexed), without
/// modifying `values`.
///
/// # Panics
///
/// Panics if `k == 0` or `k > values.len()`, with a message containing
/// `"k must be between 1 and values.len()"`.
///
/// # Examples
///
/// ```ignore
/// use intermediate_02_testing_and_project_organization::kth_smallest;
///
/// assert_eq!(kth_smallest(&[3, 1, 2], 1), 1);
/// assert_eq!(kth_smallest(&[3, 1, 2], 3), 3);
/// assert_eq!(kth_smallest(&[5, 5, 5, 1, 2], 3), 5);
/// ```
pub fn kth_smallest(values: &[i32], k: usize) -> i32 {
    todo!()
}

/// Merges overlapping (or touching) `(start, end)` intervals.
///
/// Intervals need not be sorted on input. Two intervals `a` and `b` are
/// merged if `b.0 <= a.1` (touching counts as overlapping) once sorted by
/// start. The result is sorted by start, with no overlapping/touching
/// intervals remaining.
///
/// # Examples
///
/// ```ignore
/// use intermediate_02_testing_and_project_organization::merge_intervals;
///
/// assert_eq!(
///     merge_intervals(&[(1, 3), (2, 6), (8, 10), (15, 18)]),
///     vec![(1, 6), (8, 10), (15, 18)]
/// );
/// assert_eq!(merge_intervals(&[(1, 4), (4, 5)]), vec![(1, 5)]);
/// assert_eq!(merge_intervals(&[]), Vec::<(i32, i32)>::new());
/// ```
pub fn merge_intervals(intervals: &[(i32, i32)]) -> Vec<(i32, i32)> {
    todo!()
}

/// Returns the length of the longest **strictly increasing** subsequence
/// of `values`.
///
/// # Examples
///
/// ```ignore
/// use intermediate_02_testing_and_project_organization::longest_increasing_subsequence;
///
/// assert_eq!(
///     longest_increasing_subsequence(&[10, 9, 2, 5, 3, 7, 101, 18]),
///     4
/// );
/// assert_eq!(longest_increasing_subsequence(&[7, 7, 7, 7]), 1);
/// assert_eq!(longest_increasing_subsequence(&[]), 0);
/// ```
pub fn longest_increasing_subsequence(values: &[i32]) -> usize {
    todo!()
}

/// Returns the minimum number of coins (from `denominations`, each usable
/// any number of times) that sum to exactly `amount`, or `None` if
/// impossible.
///
/// `amount == 0` always returns `Some(0)`.
///
/// # Examples
///
/// ```ignore
/// use intermediate_02_testing_and_project_organization::min_coins;
///
/// assert_eq!(min_coins(11, &[1, 2, 5]), Some(3)); // 5 + 5 + 1
/// assert_eq!(min_coins(0, &[1, 2, 5]), Some(0));
/// assert_eq!(min_coins(3, &[2]), None);
/// ```
pub fn min_coins(amount: u32, denominations: &[u32]) -> Option<u32> {
    todo!()
}
