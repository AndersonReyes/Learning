use fundamentals_04_ownership_and_borrowing::{
    drain_below_threshold, longest_common_prefix_owned, merge_sorted_into, partition_in_place,
    take_ownership_and_split,
};

#[test]
fn partition_in_place_mixed_values_is_stable() {
    let mut v = vec![5, 3, 8, 3, 1, 3, 9];
    assert_eq!(partition_in_place(&mut v, 3), (1, 3));
    assert_eq!(v, vec![1, 3, 3, 3, 5, 8, 9]);
}

#[test]
fn partition_in_place_all_less_or_all_greater() {
    let mut v = vec![1, 2, 3];
    assert_eq!(partition_in_place(&mut v, 5), (3, 0));
    assert_eq!(v, vec![1, 2, 3]);

    let mut v = vec![10, 20, 30];
    assert_eq!(partition_in_place(&mut v, 5), (0, 0));
    assert_eq!(v, vec![10, 20, 30]);
}

#[test]
fn partition_in_place_empty_vec() {
    let mut v: Vec<i32> = vec![];
    assert_eq!(partition_in_place(&mut v, 0), (0, 0));
    assert_eq!(v, Vec::<i32>::new());
}

#[test]
fn merge_sorted_into_interleaved() {
    let mut target = vec![1, 3, 5];
    let source = [2, 4, 6];
    merge_sorted_into(&mut target, &source);
    assert_eq!(target, vec![1, 2, 3, 4, 5, 6]);
    // `source` is only borrowed — still usable and unchanged afterward.
    assert_eq!(source, [2, 4, 6]);
}

#[test]
fn merge_sorted_into_empty_target() {
    let mut target: Vec<i32> = vec![];
    merge_sorted_into(&mut target, &[1, 2, 3]);
    assert_eq!(target, vec![1, 2, 3]);
}

#[test]
fn merge_sorted_into_empty_source() {
    let mut target = vec![1, 2, 3];
    merge_sorted_into(&mut target, &[]);
    assert_eq!(target, vec![1, 2, 3]);
}

#[test]
fn merge_sorted_into_with_duplicates() {
    let mut target = vec![2, 2, 4];
    merge_sorted_into(&mut target, &[2, 3]);
    assert_eq!(target, vec![2, 2, 2, 3, 4]);
}

#[test]
fn take_ownership_and_split_basic() {
    let v = vec![1, 8, 3, 9, 2, 7, 4];
    assert_eq!(
        take_ownership_and_split(v, 5),
        (vec![1, 3, 2, 4], vec![8, 9, 7])
    );
}

#[test]
fn take_ownership_and_split_threshold_matches_exactly() {
    let v = vec![5, 5, 1, 9];
    assert_eq!(take_ownership_and_split(v, 5), (vec![1], vec![5, 5, 9]));
}

#[test]
fn take_ownership_and_split_empty_and_extremes() {
    assert_eq!(
        take_ownership_and_split(vec![], 10),
        (Vec::<i32>::new(), Vec::<i32>::new())
    );
    assert_eq!(
        take_ownership_and_split(vec![1, 2, 3], 0),
        (Vec::<i32>::new(), vec![1, 2, 3])
    );
}

#[test]
fn drain_below_threshold_mixed_values() {
    let mut v = vec![5, 1, 8, 2, 9, 3];
    assert_eq!(drain_below_threshold(&mut v, 4), vec![1, 2, 3]);
    assert_eq!(v, vec![5, 8, 9]);
}

#[test]
fn drain_below_threshold_nothing_removed() {
    let mut v = vec![5, 6, 7];
    assert_eq!(drain_below_threshold(&mut v, 4), Vec::<i32>::new());
    assert_eq!(v, vec![5, 6, 7]);
}

#[test]
fn drain_below_threshold_everything_removed() {
    let mut v = vec![1, 2, 3];
    assert_eq!(drain_below_threshold(&mut v, 10), vec![1, 2, 3]);
    assert_eq!(v, Vec::<i32>::new());
}

#[test]
fn drain_below_threshold_empty_vec() {
    let mut v: Vec<i32> = vec![];
    assert_eq!(drain_below_threshold(&mut v, 100), Vec::<i32>::new());
    assert_eq!(v, Vec::<i32>::new());
}

#[test]
fn longest_common_prefix_owned_basic() {
    let strings = vec![
        String::from("flower"),
        String::from("flow"),
        String::from("flight"),
    ];
    assert_eq!(longest_common_prefix_owned(&strings), "fl");
    // `strings` is only borrowed — still usable afterward.
    assert_eq!(strings.len(), 3);
}

#[test]
fn longest_common_prefix_owned_no_common_prefix() {
    let strings = vec![String::from("dog"), String::from("racecar"), String::from("car")];
    assert_eq!(longest_common_prefix_owned(&strings), "");
}

#[test]
fn longest_common_prefix_owned_empty_input() {
    let strings: Vec<String> = vec![];
    assert_eq!(longest_common_prefix_owned(&strings), "");
}

#[test]
fn longest_common_prefix_owned_single_string() {
    let strings = vec![String::from("single")];
    assert_eq!(longest_common_prefix_owned(&strings), "single");
}

#[test]
fn longest_common_prefix_owned_one_is_a_prefix_of_another() {
    let strings = vec![String::from("ab"), String::from("a")];
    assert_eq!(longest_common_prefix_owned(&strings), "a");
}

#[test]
fn longest_common_prefix_owned_unicode() {
    let strings = vec![String::from("αβγ"), String::from("αβδ")];
    assert_eq!(longest_common_prefix_owned(&strings), "αβ");
}
