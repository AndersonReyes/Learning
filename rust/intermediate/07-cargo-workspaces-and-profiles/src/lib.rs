//! Intermediate 07 — Cargo Workspaces, Profiles & Iterator Performance.
//!
//! `notes.md` covers Book ch. 13.4 (loops vs. iterators, zero-cost
//! abstractions, `.windows()`/`.fold()`/`.scan()`/`.zip()`/`.flat_map()`/
//! `.chain()`) and ch. 14 (release profiles, publishing, workspaces, `cargo
//! install`, custom subcommands) conceptually. The 5 exercises below are all
//! ch. 13.4: each asks for an **iterator-chain** implementation (no explicit
//! `for`/`while` loops, no manual indexing) of a problem a loop would
//! otherwise solve — translating "loop with running state" into `.fold()`/
//! `.scan()`/`.windows()` chains.

// --- 1. longest_increasing_run -------------------------------------------------------

/// Returns the length of the longest contiguous run of *strictly
/// increasing* elements in `data`.
///
/// A run of length 1 (a single element) always counts, so the result is
/// `0` only when `data` is empty.
///
/// Implement using `data.windows(2)` and `.fold()` to track `(longest_so_far,
/// current_run_length)` — do not use an explicit loop or index into `data`.
///
/// # Examples
///
/// ```ignore
/// use intermediate_07_cargo_workspaces_and_profiles::longest_increasing_run;
///
/// assert_eq!(longest_increasing_run(&[1, 2, 3, 2, 3, 4, 5, 1]), 4);
/// assert_eq!(longest_increasing_run(&[5, 4, 3, 2, 1]), 1);
/// assert_eq!(longest_increasing_run(&[1, 2, 3, 4, 5]), 5);
/// assert_eq!(longest_increasing_run(&[7]), 1);
/// assert_eq!(longest_increasing_run(&[]), 0);
/// ```
pub fn longest_increasing_run(data: &[i32]) -> usize {
    todo!()
}

// --- 2. moving_average -------------------------------------------------------------

/// Returns the sliding-window average of `data` for each window of size
/// `window`.
///
/// The result has `data.len() - window + 1` elements (each the mean of one
/// contiguous `window`-sized slice), in order. Returns an empty `Vec` if
/// `window == 0` or `window > data.len()`.
///
/// Implement using `data.windows(window)` and `.map()`/`.sum()` — do not use
/// an explicit loop or index into `data`. Guard the `window == 0` case
/// before calling `.windows()` (it panics on a zero size).
///
/// # Examples
///
/// ```ignore
/// use intermediate_07_cargo_workspaces_and_profiles::moving_average;
///
/// assert_eq!(moving_average(&[1.0, 2.0, 3.0, 4.0, 5.0], 2), vec![1.5, 2.5, 3.5, 4.5]);
/// assert_eq!(moving_average(&[2.0, 2.0, 2.0], 3), vec![2.0]);
/// assert_eq!(moving_average(&[1.0, 2.0], 5), Vec::<f64>::new());
/// assert_eq!(moving_average(&[], 1), Vec::<f64>::new());
/// assert_eq!(moving_average(&[1.0, 2.0, 3.0], 0), Vec::<f64>::new());
/// ```
pub fn moving_average(data: &[f64], window: usize) -> Vec<f64> {
    todo!()
}

// --- 3. zigzag_merge -------------------------------------------------------------

/// Interleaves `a` and `b` element-by-element: `a[0], b[0], a[1], b[1],
/// ...`. Once the shorter slice is exhausted, appends the remaining elements
/// of the longer slice in order.
///
/// Implement using `.zip()`, `.flat_map()`, and `.chain()` on slices of the
/// common-prefix length — do not use an explicit loop or index past the
/// common length in a branch-specific way beyond slicing.
///
/// # Examples
///
/// ```ignore
/// use intermediate_07_cargo_workspaces_and_profiles::zigzag_merge;
///
/// assert_eq!(zigzag_merge(&[1, 3, 5], &[2, 4, 6]), vec![1, 2, 3, 4, 5, 6]);
/// assert_eq!(zigzag_merge(&[1, 3, 5], &[2, 4]), vec![1, 2, 3, 4, 5]);
/// assert_eq!(zigzag_merge(&[1, 3], &[2, 4, 6, 8]), vec![1, 2, 3, 4, 6, 8]);
/// assert_eq!(zigzag_merge(&[], &[1, 2, 3]), vec![1, 2, 3]);
/// assert_eq!(zigzag_merge(&[1, 2, 3], &[]), vec![1, 2, 3]);
/// ```
pub fn zigzag_merge(a: &[i32], b: &[i32]) -> Vec<i32> {
    todo!()
}

// --- 4. count_local_maxima -------------------------------------------------------------

/// Counts elements of `data` that are a *strict local maximum*: greater than
/// all of their immediate neighbors.
///
/// Interior elements are compared to both neighbors via `data.windows(3)`.
/// The first and last elements (if `data.len() >= 2`) are compared only to
/// their single neighbor. A single-element slice (`data.len() == 1`) counts
/// as 1 (its one element is trivially a local maximum). An empty slice
/// counts as 0.
///
/// Implement using `.windows(3)` + `.filter()` + `.count()` for the interior,
/// combined with direct comparisons for the two endpoints — do not use an
/// explicit loop.
///
/// # Examples
///
/// ```ignore
/// use intermediate_07_cargo_workspaces_and_profiles::count_local_maxima;
///
/// assert_eq!(count_local_maxima(&[1, 3, 2, 4, 1, 5]), 3);
/// assert_eq!(count_local_maxima(&[5, 5, 5]), 0);
/// assert_eq!(count_local_maxima(&[1, 2]), 1);
/// assert_eq!(count_local_maxima(&[7]), 1);
/// assert_eq!(count_local_maxima(&[]), 0);
/// ```
pub fn count_local_maxima(data: &[i32]) -> usize {
    todo!()
}

// --- 5. exponential_moving_average -------------------------------------------------------------

/// Computes the exponential moving average (EMA) of `data` with smoothing
/// factor `alpha`.
///
/// `ema[0] = data[0]`; for `i > 0`, `ema[i] = alpha * data[i] + (1.0 -
/// alpha) * ema[i - 1]`. Returns an empty `Vec` for empty `data`.
///
/// Implement using `.scan()` to carry the running EMA value, with
/// `std::iter::once` to seed the first (unmodified) element — do not use an
/// explicit loop or a separate `Vec` that you push into by index.
///
/// # Examples
///
/// ```ignore
/// use intermediate_07_cargo_workspaces_and_profiles::exponential_moving_average;
///
/// assert_eq!(exponential_moving_average(&[1.0, 2.0, 3.0], 0.5), vec![1.0, 1.5, 2.25]);
/// assert_eq!(exponential_moving_average(&[10.0, 20.0, 10.0], 0.5), vec![10.0, 15.0, 12.5]);
/// assert_eq!(exponential_moving_average(&[5.0], 0.3), vec![5.0]);
/// assert_eq!(exponential_moving_average(&[], 0.5), Vec::<f64>::new());
/// ```
pub fn exponential_moving_average(data: &[f64], alpha: f64) -> Vec<f64> {
    todo!()
}
