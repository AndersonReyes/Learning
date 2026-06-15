use intermediate_07_cargo_workspaces_and_profiles::{
    count_local_maxima, exponential_moving_average, longest_increasing_run, moving_average,
    zigzag_merge,
};

// --- longest_increasing_run -------------------------------------------------------------

#[test]
fn longest_increasing_run_with_a_break() {
    assert_eq!(longest_increasing_run(&[1, 2, 3, 2, 3, 4, 5, 1]), 4);
}

#[test]
fn longest_increasing_run_strictly_decreasing() {
    assert_eq!(longest_increasing_run(&[5, 4, 3, 2, 1]), 1);
}

#[test]
fn longest_increasing_run_all_increasing() {
    assert_eq!(longest_increasing_run(&[1, 2, 3, 4, 5]), 5);
}

#[test]
fn longest_increasing_run_single_element() {
    assert_eq!(longest_increasing_run(&[7]), 1);
}

#[test]
fn longest_increasing_run_empty() {
    assert_eq!(longest_increasing_run(&[]), 0);
}

#[test]
fn longest_increasing_run_plateau_resets_current_run() {
    // [1,2] is increasing (len 2), then 2==2 resets, then [2,3] is increasing (len 2).
    assert_eq!(longest_increasing_run(&[1, 2, 2, 3]), 2);
}

#[test]
fn longest_increasing_run_all_equal() {
    assert_eq!(longest_increasing_run(&[3, 3, 3, 3]), 1);
}

// --- moving_average -------------------------------------------------------------

#[test]
fn moving_average_window_two() {
    assert_eq!(
        moving_average(&[1.0, 2.0, 3.0, 4.0, 5.0], 2),
        vec![1.5, 2.5, 3.5, 4.5]
    );
}

#[test]
fn moving_average_window_equals_len() {
    assert_eq!(moving_average(&[2.0, 2.0, 2.0], 3), vec![2.0]);
}

#[test]
fn moving_average_window_larger_than_data() {
    assert_eq!(moving_average(&[1.0, 2.0], 5), Vec::<f64>::new());
}

#[test]
fn moving_average_empty_data() {
    assert_eq!(moving_average(&[], 1), Vec::<f64>::new());
}

#[test]
fn moving_average_window_zero() {
    assert_eq!(moving_average(&[1.0, 2.0, 3.0], 0), Vec::<f64>::new());
}

#[test]
fn moving_average_window_one_is_identity() {
    assert_eq!(
        moving_average(&[1.0, 2.0, 3.0], 1),
        vec![1.0, 2.0, 3.0]
    );
}

#[test]
fn moving_average_negative_values() {
    assert_eq!(
        moving_average(&[-1.0, 1.0, -1.0, 1.0], 2),
        vec![0.0, 0.0, 0.0]
    );
}

// --- zigzag_merge -------------------------------------------------------------

#[test]
fn zigzag_merge_equal_lengths() {
    assert_eq!(
        zigzag_merge(&[1, 3, 5], &[2, 4, 6]),
        vec![1, 2, 3, 4, 5, 6]
    );
}

#[test]
fn zigzag_merge_a_longer() {
    assert_eq!(zigzag_merge(&[1, 3, 5], &[2, 4]), vec![1, 2, 3, 4, 5]);
}

#[test]
fn zigzag_merge_b_longer() {
    assert_eq!(
        zigzag_merge(&[1, 3], &[2, 4, 6, 8]),
        vec![1, 2, 3, 4, 6, 8]
    );
}

#[test]
fn zigzag_merge_a_empty() {
    assert_eq!(zigzag_merge(&[], &[1, 2, 3]), vec![1, 2, 3]);
}

#[test]
fn zigzag_merge_b_empty() {
    assert_eq!(zigzag_merge(&[1, 2, 3], &[]), vec![1, 2, 3]);
}

#[test]
fn zigzag_merge_both_empty() {
    assert_eq!(zigzag_merge(&[], &[]), Vec::<i32>::new());
}

#[test]
fn zigzag_merge_single_elements() {
    assert_eq!(zigzag_merge(&[1], &[2]), vec![1, 2]);
}

#[test]
fn zigzag_merge_b_longer_by_one() {
    assert_eq!(
        zigzag_merge(&[1, 2, 3], &[10, 20]),
        vec![1, 10, 2, 20, 3]
    );
}

// --- count_local_maxima -------------------------------------------------------------

#[test]
fn count_local_maxima_mixed() {
    assert_eq!(count_local_maxima(&[1, 3, 2, 4, 1, 5]), 3);
}

#[test]
fn count_local_maxima_all_equal() {
    assert_eq!(count_local_maxima(&[5, 5, 5]), 0);
}

#[test]
fn count_local_maxima_two_elements() {
    assert_eq!(count_local_maxima(&[1, 2]), 1);
}

#[test]
fn count_local_maxima_single_element() {
    assert_eq!(count_local_maxima(&[7]), 1);
}

#[test]
fn count_local_maxima_empty() {
    assert_eq!(count_local_maxima(&[]), 0);
}

#[test]
fn count_local_maxima_alternating() {
    assert_eq!(count_local_maxima(&[1, 2, 1, 2, 1]), 2);
}

#[test]
fn count_local_maxima_monotonic_increasing() {
    assert_eq!(count_local_maxima(&[1, 2, 3, 4, 5]), 1);
}

#[test]
fn count_local_maxima_two_equal_elements() {
    assert_eq!(count_local_maxima(&[3, 3]), 0);
}

// --- exponential_moving_average -------------------------------------------------------------

#[test]
fn exponential_moving_average_basic() {
    assert_eq!(
        exponential_moving_average(&[1.0, 2.0, 3.0], 0.5),
        vec![1.0, 1.5, 2.25]
    );
}

#[test]
fn exponential_moving_average_oscillating() {
    assert_eq!(
        exponential_moving_average(&[10.0, 20.0, 10.0], 0.5),
        vec![10.0, 15.0, 12.5]
    );
}

#[test]
fn exponential_moving_average_single_element() {
    assert_eq!(exponential_moving_average(&[5.0], 0.3), vec![5.0]);
}

#[test]
fn exponential_moving_average_empty() {
    assert_eq!(
        exponential_moving_average(&[], 0.5),
        Vec::<f64>::new()
    );
}

#[test]
fn exponential_moving_average_alpha_one_is_identity() {
    assert_eq!(
        exponential_moving_average(&[1.0, 5.0, 2.0], 1.0),
        vec![1.0, 5.0, 2.0]
    );
}

#[test]
fn exponential_moving_average_alpha_zero_holds_first_value() {
    assert_eq!(
        exponential_moving_average(&[1.0, 5.0, 2.0], 0.0),
        vec![1.0, 1.0, 1.0]
    );
}
