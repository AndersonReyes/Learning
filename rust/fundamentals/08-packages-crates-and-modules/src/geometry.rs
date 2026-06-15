//! Geometry helpers operating on points `(f64, f64)`.

/// Returns the area of the simple polygon with the given `vertices`, listed
/// in order (clockwise or counter-clockwise) — via the
/// [shoelace formula](https://en.wikipedia.org/wiki/Shoelace_formula).
///
/// # Panics / Preconditions
///
/// `vertices` must have at least 3 points.
///
/// # Examples
///
/// ```ignore
/// use fundamentals_08_packages_crates_and_modules::geometry::polygon_area;
///
/// // unit square
/// assert_eq!(polygon_area(&[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]), 1.0);
/// // right triangle, legs 4 and 3 -> area 6
/// assert_eq!(polygon_area(&[(0.0, 0.0), (4.0, 0.0), (0.0, 3.0)]), 6.0);
/// ```
pub fn polygon_area(vertices: &[(f64, f64)]) -> f64 {
    todo!()
}

/// Returns the smallest Euclidean distance between any two of the given
/// `points` (brute-force over all pairs).
///
/// # Panics / Preconditions
///
/// `points` must have at least 2 points.
///
/// # Examples
///
/// ```ignore
/// use fundamentals_08_packages_crates_and_modules::geometry::closest_pair_distance;
///
/// assert_eq!(closest_pair_distance(&[(0.0, 0.0), (3.0, 4.0), (0.0, 1.0)]), 1.0);
/// assert_eq!(closest_pair_distance(&[(0.0, 0.0), (1.0, 0.0)]), 1.0);
/// ```
pub fn closest_pair_distance(points: &[(f64, f64)]) -> f64 {
    todo!()
}
