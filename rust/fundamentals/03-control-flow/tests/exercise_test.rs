use fundamentals_03_control_flow::{
    count_steps_to_reach, digital_root, find_in_grid, is_armstrong_number, sum_of_multiples_below,
};

const GRID: [[i32; 4]; 4] = [
    [1, 2, 3, 4],
    [5, 6, 7, 8],
    [9, 10, 11, 12],
    [13, 14, 15, 16],
];

#[test]
fn find_in_grid_top_left_corner() {
    assert_eq!(find_in_grid(&GRID, 1), (0, 0));
}

#[test]
fn find_in_grid_bottom_right_corner() {
    assert_eq!(find_in_grid(&GRID, 16), (3, 3));
}

#[test]
fn find_in_grid_middle_value_requires_breaking_outer_loop() {
    assert_eq!(find_in_grid(&GRID, 7), (1, 2));
}

#[test]
fn find_in_grid_returns_first_occurrence_row_major() {
    let grid = [
        [1, 2, 1, 2],
        [3, 1, 3, 1],
        [5, 5, 5, 5],
        [0, 0, 0, 1],
    ];
    assert_eq!(find_in_grid(&grid, 1), (0, 0));
    assert_eq!(find_in_grid(&grid, 5), (2, 0));
    assert_eq!(find_in_grid(&grid, 2), (0, 1));
}

#[test]
fn is_armstrong_number_single_digits_are_always_armstrong() {
    assert_eq!(is_armstrong_number(0), true);
    assert_eq!(is_armstrong_number(5), true);
    assert_eq!(is_armstrong_number(9), true);
}

#[test]
fn is_armstrong_number_two_digit_numbers_are_never_armstrong() {
    assert_eq!(is_armstrong_number(10), false);
    assert_eq!(is_armstrong_number(99), false);
}

#[test]
fn is_armstrong_number_three_digit_known_values() {
    assert_eq!(is_armstrong_number(153), true); // 1^3+5^3+3^3
    assert_eq!(is_armstrong_number(370), true); // 3^3+7^3+0^3
    assert_eq!(is_armstrong_number(371), true); // 3^3+7^3+1^3
    assert_eq!(is_armstrong_number(372), false);
}

#[test]
fn is_armstrong_number_four_digit_known_value() {
    assert_eq!(is_armstrong_number(9474), true); // 9^4+4^4+7^4+4^4
    assert_eq!(is_armstrong_number(9475), false);
}

#[test]
fn sum_of_multiples_below_small_range() {
    assert_eq!(sum_of_multiples_below(10, &[3, 5]), 23); // 3+5+6+9
}

#[test]
fn sum_of_multiples_below_classic_project_euler_value() {
    assert_eq!(sum_of_multiples_below(1000, &[3, 5]), 233_168);
}

#[test]
fn sum_of_multiples_below_single_factor() {
    assert_eq!(sum_of_multiples_below(10, &[2]), 20); // 2+4+6+8
}

#[test]
fn sum_of_multiples_below_no_qualifying_values() {
    assert_eq!(sum_of_multiples_below(5, &[7]), 0);
    assert_eq!(sum_of_multiples_below(1, &[1]), 0); // empty range 1..1
}

#[test]
fn digital_root_base_cases() {
    assert_eq!(digital_root(0), 0);
    assert_eq!(digital_root(9), 9);
}

#[test]
fn digital_root_single_pass() {
    assert_eq!(digital_root(38), 2); // 3+8=11, 1+1=2
}

#[test]
fn digital_root_multiple_passes() {
    assert_eq!(digital_root(9875), 2); // 9+8+7+5=29, 2+9=11, 1+1=2
    assert_eq!(digital_root(123456789), 9); // digits sum to 45, 4+5=9
}

#[test]
fn count_steps_to_reach_positive_step() {
    assert_eq!(count_steps_to_reach(0, 3, 10), 4); // 0,3,6,9,12
}

#[test]
fn count_steps_to_reach_negative_step() {
    assert_eq!(count_steps_to_reach(20, -5, 0), 4); // 20,15,10,5,0
}

#[test]
fn count_steps_to_reach_already_at_target() {
    assert_eq!(count_steps_to_reach(5, 1, 5), 0);
    assert_eq!(count_steps_to_reach(5, -1, 5), 0);
}

#[test]
fn count_steps_to_reach_negative_numbers() {
    assert_eq!(count_steps_to_reach(-10, 3, -1), 3); // -10,-7,-4,-1
}

#[test]
fn count_steps_to_reach_single_step() {
    assert_eq!(count_steps_to_reach(0, 7, 1), 1); // 0,7
    assert_eq!(count_steps_to_reach(10, 2, 10), 0); // already >= target
}
