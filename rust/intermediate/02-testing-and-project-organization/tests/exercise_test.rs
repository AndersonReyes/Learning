use intermediate_02_testing_and_project_organization::{
    binary_search, kth_smallest, longest_increasing_subsequence, merge_intervals, min_coins,
};

// --- binary_search ---------------------------------------------------------

#[test]
fn binary_search_finds_present_elements() {
    let sorted = [1, 3, 5, 7, 9, 11];
    assert_eq!(binary_search(&sorted, &1), Some(0));
    assert_eq!(binary_search(&sorted, &7), Some(3));
    assert_eq!(binary_search(&sorted, &11), Some(5));
}

#[test]
fn binary_search_returns_none_for_absent_elements() {
    let sorted = [1, 3, 5, 7, 9, 11];
    assert_eq!(binary_search(&sorted, &4), None);
    assert_eq!(binary_search(&sorted, &0), None);
    assert_eq!(binary_search(&sorted, &12), None);
}

#[test]
fn binary_search_empty_slice() {
    assert_eq!(binary_search::<i32>(&[], &5), None);
}

#[test]
fn binary_search_single_element() {
    assert_eq!(binary_search(&[42], &42), Some(0));
    assert_eq!(binary_search(&[42], &7), None);
}

#[test]
fn binary_search_strings() {
    let words = ["apple", "banana", "cherry", "date", "fig"];
    assert_eq!(binary_search(&words, &"cherry"), Some(2));
    assert_eq!(binary_search(&words, &"apple"), Some(0));
    assert_eq!(binary_search(&words, &"fig"), Some(4));
    assert_eq!(binary_search(&words, &"grape"), None);
}

// --- kth_smallest -----------------------------------------------------------

#[test]
fn kth_smallest_no_duplicates() {
    let values = [3, 1, 2];
    assert_eq!(kth_smallest(&values, 1), 1);
    assert_eq!(kth_smallest(&values, 2), 2);
    assert_eq!(kth_smallest(&values, 3), 3);
}

#[test]
fn kth_smallest_with_duplicates() {
    let values = [5, 5, 5, 1, 2];
    assert_eq!(kth_smallest(&values, 1), 1);
    assert_eq!(kth_smallest(&values, 2), 2);
    assert_eq!(kth_smallest(&values, 3), 5);
    assert_eq!(kth_smallest(&values, 5), 5);
}

#[test]
fn kth_smallest_single_element() {
    assert_eq!(kth_smallest(&[42], 1), 42);
}

#[test]
fn kth_smallest_does_not_mutate_input() {
    let values = [3, 1, 2];
    let _ = kth_smallest(&values, 1);
    assert_eq!(values, [3, 1, 2]);
}

#[test]
#[should_panic(expected = "k must be between 1 and values.len()")]
fn kth_smallest_panics_on_zero() {
    kth_smallest(&[1, 2, 3], 0);
}

#[test]
#[should_panic(expected = "k must be between 1 and values.len()")]
fn kth_smallest_panics_on_too_large() {
    kth_smallest(&[1, 2, 3], 4);
}

// --- merge_intervals ---------------------------------------------------------

#[test]
fn merge_intervals_classic_overlap() {
    let intervals = [(1, 3), (2, 6), (8, 10), (15, 18)];
    assert_eq!(merge_intervals(&intervals), vec![(1, 6), (8, 10), (15, 18)]);
}

#[test]
fn merge_intervals_touching_counts_as_overlap() {
    assert_eq!(merge_intervals(&[(1, 4), (4, 5)]), vec![(1, 5)]);
}

#[test]
fn merge_intervals_empty_input() {
    assert_eq!(merge_intervals(&[]), Vec::<(i32, i32)>::new());
}

#[test]
fn merge_intervals_single_interval() {
    assert_eq!(merge_intervals(&[(5, 10)]), vec![(5, 10)]);
}

#[test]
fn merge_intervals_fully_contained() {
    assert_eq!(merge_intervals(&[(1, 10), (2, 5)]), vec![(1, 10)]);
}

#[test]
fn merge_intervals_unsorted_input() {
    let intervals = [(15, 18), (1, 3), (8, 10), (2, 6)];
    assert_eq!(merge_intervals(&intervals), vec![(1, 6), (8, 10), (15, 18)]);
}

#[test]
fn merge_intervals_no_overlap_stays_separate() -> Result<(), String> {
    let intervals = [(1, 2), (3, 4), (5, 6)];
    let merged = merge_intervals(&intervals);
    if merged == vec![(1, 2), (3, 4), (5, 6)] {
        Ok(())
    } else {
        Err(format!("expected no merges, got {merged:?}"))
    }
}

// --- longest_increasing_subsequence -------------------------------------------

#[test]
fn lis_classic_example() {
    assert_eq!(
        longest_increasing_subsequence(&[10, 9, 2, 5, 3, 7, 101, 18]),
        4
    );
}

#[test]
fn lis_with_repeated_growth() {
    assert_eq!(longest_increasing_subsequence(&[0, 1, 0, 3, 2, 3]), 4);
}

#[test]
fn lis_all_equal() {
    assert_eq!(longest_increasing_subsequence(&[7, 7, 7, 7]), 1);
}

#[test]
fn lis_empty() {
    assert_eq!(longest_increasing_subsequence(&[]), 0);
}

#[test]
fn lis_strictly_ascending() {
    assert_eq!(longest_increasing_subsequence(&[1, 2, 3, 4, 5]), 5);
}

#[test]
fn lis_strictly_descending() {
    assert_eq!(longest_increasing_subsequence(&[5, 4, 3, 2, 1]), 1);
}

// --- min_coins ----------------------------------------------------------------

#[test]
fn min_coins_classic_example() {
    assert_eq!(min_coins(11, &[1, 2, 5]), Some(3));
}

#[test]
fn min_coins_zero_amount() {
    assert_eq!(min_coins(0, &[1, 2, 5]), Some(0));
}

#[test]
fn min_coins_impossible() {
    assert_eq!(min_coins(3, &[2]), None);
}

#[test]
fn min_coins_exact_multiples() {
    assert_eq!(min_coins(6, &[1, 3, 4]), Some(2));
}

#[test]
fn min_coins_parity_mismatch() {
    assert_eq!(min_coins(7, &[2, 4]), None);
}

#[test]
fn min_coins_large_amount() {
    assert_eq!(min_coins(100, &[1, 5, 10, 25]), Some(4));
}
