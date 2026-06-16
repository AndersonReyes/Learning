use advanced_09_async_await_and_futures::{
    async_double, async_filter, async_first_match, async_fold, async_map,
};

// --- Exercise 1: async_double ------------------------------------------------

#[test]
fn double_21() {
    assert_eq!(pollster::block_on(async_double(21)), 42);
}

#[test]
fn double_zero() {
    assert_eq!(pollster::block_on(async_double(0)), 0);
}

#[test]
fn double_negative() {
    assert_eq!(pollster::block_on(async_double(-5)), -10);
}

// --- Exercise 2: async_map ---------------------------------------------------

#[test]
fn map_multiply_by_10() {
    let result = pollster::block_on(async_map(&[1_i32, 2, 3], |x| async move { x * 10 }));
    assert_eq!(result, vec![10, 20, 30]);
}

#[test]
fn map_empty() {
    let result = pollster::block_on(async_map(&[] as &[i32], |x| async move { x * 10 }));
    assert_eq!(result, Vec::<i32>::new());
}

#[test]
fn map_to_string() {
    let result = pollster::block_on(async_map(&[1_i32, 2, 3], |x| async move { x.to_string() }));
    assert_eq!(result, vec!["1", "2", "3"]);
}

#[test]
fn map_single() {
    let result = pollster::block_on(async_map(&[7_i32], |x| async move { x + 1 }));
    assert_eq!(result, vec![8]);
}

#[test]
fn map_preserves_order() {
    let result = pollster::block_on(async_map(&[3_i32, 1, 4, 1, 5], |x| async move { x * x }));
    assert_eq!(result, vec![9, 1, 16, 1, 25]);
}

// --- Exercise 3: async_filter ------------------------------------------------

#[test]
fn filter_evens() {
    let result = pollster::block_on(async_filter(&[1_i32, 2, 3, 4, 5], |x| async move { x % 2 == 0 }));
    assert_eq!(result, vec![2, 4]);
}

#[test]
fn filter_none_match() {
    let result = pollster::block_on(async_filter(&[1_i32, 3, 5], |x| async move { x % 2 == 0 }));
    assert_eq!(result, Vec::<i32>::new());
}

#[test]
fn filter_all_match() {
    let result = pollster::block_on(async_filter(&[2_i32, 4, 6], |x| async move { x % 2 == 0 }));
    assert_eq!(result, vec![2, 4, 6]);
}

#[test]
fn filter_empty() {
    let result = pollster::block_on(async_filter(&[] as &[i32], |x| async move { x > 0 }));
    assert_eq!(result, Vec::<i32>::new());
}

#[test]
fn filter_positive() {
    let result = pollster::block_on(async_filter(&[-2_i32, -1, 0, 1, 2], |x| async move { x > 0 }));
    assert_eq!(result, vec![1, 2]);
}

// --- Exercise 4: async_first_match -------------------------------------------

#[test]
fn first_match_found() {
    let result = pollster::block_on(async_first_match(&[1_i32, 3, 4, 7], |x| async move { x % 2 == 0 }));
    assert_eq!(result, Some(4));
}

#[test]
fn first_match_not_found() {
    let result = pollster::block_on(async_first_match(&[1_i32, 3, 5], |x| async move { x % 2 == 0 }));
    assert_eq!(result, None);
}

#[test]
fn first_match_empty() {
    let result = pollster::block_on(async_first_match(&[] as &[i32], |x| async move { x > 0 }));
    assert_eq!(result, None);
}

#[test]
fn first_match_first_element() {
    let result = pollster::block_on(async_first_match(&[2_i32, 4, 6], |x| async move { x % 2 == 0 }));
    assert_eq!(result, Some(2));
}

#[test]
fn first_match_last_element() {
    let result = pollster::block_on(async_first_match(&[1_i32, 3, 5, 6], |x| async move { x % 2 == 0 }));
    assert_eq!(result, Some(6));
}

// --- Exercise 5: async_fold --------------------------------------------------

#[test]
fn fold_sum() {
    let result = pollster::block_on(async_fold(&[1_i32, 2, 3, 4], 0, |acc, x| async move { acc + x }));
    assert_eq!(result, 10);
}

#[test]
fn fold_product() {
    let result = pollster::block_on(async_fold(&[1_i32, 2, 3, 4], 1, |acc, x| async move { acc * x }));
    assert_eq!(result, 24);
}

#[test]
fn fold_empty() {
    let result = pollster::block_on(async_fold(&[] as &[i32], 99, |acc, x| async move { acc + x }));
    assert_eq!(result, 99);
}

#[test]
fn fold_string_concat() {
    let result = pollster::block_on(async_fold(
        &["a", "b", "c"],
        String::new(),
        |mut acc, x| async move { acc.push_str(x); acc },
    ));
    assert_eq!(result, "abc");
}

#[test]
fn fold_max() {
    let result = pollster::block_on(async_fold(
        &[3_i32, 1, 4, 1, 5, 9, 2],
        i32::MIN,
        |acc, x| async move { acc.max(x) },
    ));
    assert_eq!(result, 9);
}
