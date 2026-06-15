//! Basic descriptive statistics over slices of numbers.

/// Returns the median of `values`: the middle element after sorting (for
/// odd length), or the average of the two middle elements (for even
/// length). `values` is not modified.
///
/// # Panics / Preconditions
///
/// `values` must be non-empty.
///
/// # Examples
///
/// ```ignore
/// use fundamentals_08_packages_crates_and_modules::stats::median;
///
/// assert_eq!(median(&[3.0, 1.0, 2.0]), 2.0);
/// assert_eq!(median(&[4.0, 1.0, 3.0, 2.0]), 2.5);
/// assert_eq!(median(&[5.0]), 5.0);
/// ```
pub fn median(values: &[f64]) -> f64 {
    todo!()
}

/// Returns the population standard deviation of `values`: `sqrt(mean of
/// (x - mean)^2)`.
///
/// # Panics / Preconditions
///
/// `values` must be non-empty.
///
/// # Examples
///
/// ```ignore
/// use fundamentals_08_packages_crates_and_modules::stats::standard_deviation;
///
/// assert_eq!(standard_deviation(&[2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0]), 2.0);
/// assert_eq!(standard_deviation(&[2.0, 2.0, 2.0]), 0.0);
/// ```
pub fn standard_deviation(values: &[f64]) -> f64 {
    todo!()
}

/// Returns every value in `values` that occurs most frequently (ties
/// included), sorted in ascending order. Returns an empty `Vec` if
/// `values` is empty.
///
/// # Examples
///
/// ```ignore
/// use fundamentals_08_packages_crates_and_modules::stats::mode;
///
/// assert_eq!(mode(&[1, 1, 2, 2, 3]), vec![1, 2]); // 1 and 2 tie at 2 occurrences
/// assert_eq!(mode(&[1, 2, 2, 3, 3, 3]), vec![3]); // 3 is the unique mode
/// assert_eq!(mode(&[5]), vec![5]);
/// assert_eq!(mode(&[] as &[i32]), Vec::<i32>::new());
/// ```
pub fn mode(values: &[i32]) -> Vec<i32> {
    todo!()
}
