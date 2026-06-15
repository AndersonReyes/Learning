use fundamentals_08_packages_crates_and_modules::geometry::{closest_pair_distance, polygon_area};
use fundamentals_08_packages_crates_and_modules::stats::{mode, standard_deviation};

#[test]
fn polygon_area_unit_square() {
    assert_eq!(
        polygon_area(&[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]),
        1.0
    );
}

#[test]
fn polygon_area_right_triangle() {
    assert_eq!(polygon_area(&[(0.0, 0.0), (4.0, 0.0), (0.0, 3.0)]), 6.0);
}

#[test]
fn polygon_area_larger_rectangle() {
    assert_eq!(
        polygon_area(&[(1.0, 1.0), (5.0, 1.0), (5.0, 4.0), (1.0, 4.0)]),
        12.0
    );
}

#[test]
fn polygon_area_is_orientation_independent() {
    // Same square, listed clockwise instead of counter-clockwise.
    assert_eq!(
        polygon_area(&[(0.0, 0.0), (0.0, 1.0), (1.0, 1.0), (1.0, 0.0)]),
        1.0
    );
}

#[test]
fn closest_pair_distance_basic() {
    assert_eq!(
        closest_pair_distance(&[(0.0, 0.0), (3.0, 4.0), (0.0, 1.0)]),
        1.0
    );
}

#[test]
fn closest_pair_distance_two_points() {
    assert_eq!(closest_pair_distance(&[(0.0, 0.0), (1.0, 0.0)]), 1.0);
}

#[test]
fn closest_pair_distance_picks_diagonal_neighbors() {
    // (0,0)-(1,1) distance sqrt(2) ~ 1.414, closer than any axis-aligned pair (2.0 apart)
    let points = [(0.0, 0.0), (1.0, 1.0), (3.0, 3.0)];
    let d = closest_pair_distance(&points);
    assert!((d - 2.0_f64.sqrt()).abs() < 1e-9);
}

// `crate::median` via the `pub use` re-export at the crate root.
use fundamentals_08_packages_crates_and_modules::median;

#[test]
fn median_odd_length() {
    assert_eq!(median(&[3.0, 1.0, 2.0]), 2.0);
}

#[test]
fn median_even_length_averages_middle_two() {
    assert_eq!(median(&[4.0, 1.0, 3.0, 2.0]), 2.5);
}

#[test]
fn median_single_element() {
    assert_eq!(median(&[5.0]), 5.0);
}

#[test]
fn median_does_not_modify_input_order() {
    let values = [4.0, 1.0, 3.0, 2.0];
    let _ = median(&values);
    assert_eq!(values, [4.0, 1.0, 3.0, 2.0]);
}

#[test]
fn standard_deviation_classic_example() {
    assert_eq!(
        standard_deviation(&[2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0]),
        2.0
    );
}

#[test]
fn standard_deviation_constant_values_is_zero() {
    assert_eq!(standard_deviation(&[2.0, 2.0, 2.0]), 0.0);
}

#[test]
fn standard_deviation_small_set() {
    let d = standard_deviation(&[1.0, 2.0, 3.0, 4.0]);
    assert!((d - 1.25_f64.sqrt()).abs() < 1e-9);
}

#[test]
fn mode_returns_all_ties_sorted_ascending() {
    assert_eq!(mode(&[1, 1, 2, 2, 3]), vec![1, 2]);
}

#[test]
fn mode_unique_mode() {
    assert_eq!(mode(&[1, 2, 2, 3, 3, 3]), vec![3]);
}

#[test]
fn mode_single_value() {
    assert_eq!(mode(&[5]), vec![5]);
}

#[test]
fn mode_empty_input_is_empty() {
    assert_eq!(mode(&[] as &[i32]), Vec::<i32>::new());
}

#[test]
fn mode_all_unique_returns_all_sorted() {
    assert_eq!(mode(&[3, 1, 2]), vec![1, 2, 3]);
}
