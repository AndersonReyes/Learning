//! Fundamentals 04 — Ownership & Borrowing.
//!
//! Exercises focus on the move/`Clone`/`Copy`, `&T`/`&mut T` borrowing, and
//! "owned output from borrowed input" patterns from `notes.md`.

/// Reorders `values` in place into a **stable** 3-way partition around
/// `pivot`: all elements `< pivot` (in their original relative order),
/// followed by all elements `== pivot`, followed by all elements `>
/// pivot` (each group preserving original relative order). Returns
/// `(count_less, count_equal)` — the boundary indices of the partition.
///
/// # Examples
///
/// ```ignore
/// let mut v = vec![5, 3, 8, 3, 1, 3, 9];
/// assert_eq!(partition_in_place(&mut v, 3), (1, 3));
/// assert_eq!(v, vec![1, 3, 3, 3, 5, 8, 9]);
///
/// let mut v = vec![10, 20, 30];
/// assert_eq!(partition_in_place(&mut v, 5), (0, 0));
/// assert_eq!(v, vec![10, 20, 30]);
/// ```
pub fn partition_in_place(values: &mut Vec<i32>, pivot: i32) -> (usize, usize) {
    todo!()
}

/// Merges the sorted slice `source` into the already-sorted `target` in
/// place, keeping `target` sorted (the merge step of merge sort). `source`
/// is only borrowed — it's left unmodified and still usable after the call.
///
/// # Examples
///
/// ```ignore
/// let mut target = vec![1, 3, 5];
/// merge_sorted_into(&mut target, &[2, 4, 6]);
/// assert_eq!(target, vec![1, 2, 3, 4, 5, 6]);
///
/// let mut target = vec![];
/// merge_sorted_into(&mut target, &[1, 2, 3]);
/// assert_eq!(target, vec![1, 2, 3]);
/// ```
pub fn merge_sorted_into(target: &mut Vec<i32>, source: &[i32]) {
    todo!()
}

/// Consumes `values` (ownership moves in) and splits it into two new owned
/// vectors: elements `< threshold` (in original relative order), and
/// elements `>= threshold` (in original relative order).
///
/// # Examples
///
/// ```ignore
/// let v = vec![1, 8, 3, 9, 2, 7, 4];
/// assert_eq!(
///     take_ownership_and_split(v, 5),
///     (vec![1, 3, 2, 4], vec![8, 9, 7])
/// );
///
/// let v = vec![5, 5, 1, 9];
/// assert_eq!(take_ownership_and_split(v, 5), (vec![1], vec![5, 5, 9]));
/// ```
pub fn take_ownership_and_split(values: Vec<i32>, threshold: i32) -> (Vec<i32>, Vec<i32>) {
    todo!()
}

/// Removes all elements `< threshold` from `values` in place (preserving
/// the relative order of the elements that remain), and returns a new owned
/// `Vec` containing the removed elements, in their original relative order.
///
/// # Examples
///
/// ```ignore
/// let mut v = vec![5, 1, 8, 2, 9, 3];
/// assert_eq!(drain_below_threshold(&mut v, 4), vec![1, 2, 3]);
/// assert_eq!(v, vec![5, 8, 9]);
///
/// let mut v = vec![5, 6, 7];
/// assert_eq!(drain_below_threshold(&mut v, 4), Vec::<i32>::new());
/// assert_eq!(v, vec![5, 6, 7]);
/// ```
pub fn drain_below_threshold(values: &mut Vec<i32>, threshold: i32) -> Vec<i32> {
    todo!()
}

/// Returns the longest common prefix of `strings` as a new owned `String`
/// (empty if `strings` is empty or there is no common prefix). `strings` is
/// only borrowed.
///
/// # Examples
///
/// ```ignore
/// let strings = vec![
///     String::from("flower"),
///     String::from("flow"),
///     String::from("flight"),
/// ];
/// assert_eq!(longest_common_prefix_owned(&strings), "fl");
///
/// let strings = vec![String::from("dog"), String::from("racecar")];
/// assert_eq!(longest_common_prefix_owned(&strings), "");
///
/// assert_eq!(longest_common_prefix_owned(&[]), "");
/// ```
pub fn longest_common_prefix_owned(strings: &[String]) -> String {
    todo!()
}
