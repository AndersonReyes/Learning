use fundamentals_05_the_slice_type_and_str::{
    chunk_slices, first_n_chars, longest_palindromic_substring_slice, max_subarray_slice,
    split_on_whitespace_runs,
};

#[test]
fn split_on_whitespace_runs_collapses_runs_and_trims() {
    assert_eq!(
        split_on_whitespace_runs("  hello   world  "),
        vec!["hello", "world"]
    );
}

#[test]
fn split_on_whitespace_runs_single_word() {
    assert_eq!(split_on_whitespace_runs("a"), vec!["a"]);
}

#[test]
fn split_on_whitespace_runs_all_whitespace_or_empty() {
    assert_eq!(split_on_whitespace_runs("   "), Vec::<&str>::new());
    assert_eq!(split_on_whitespace_runs(""), Vec::<&str>::new());
}

#[test]
fn split_on_whitespace_runs_mixed_whitespace_chars() {
    assert_eq!(
        split_on_whitespace_runs("one\ttwo\nthree"),
        vec!["one", "two", "three"]
    );
}

#[test]
fn first_n_chars_ascii() {
    assert_eq!(first_n_chars("hello world", 5), "hello");
}

#[test]
fn first_n_chars_multibyte_utf8() {
    assert_eq!(first_n_chars("héllo", 2), "hé"); // 'é' is 2 bytes
    assert_eq!(first_n_chars("🦀rust", 1), "🦀"); // 🦀 is 4 bytes
}

#[test]
fn first_n_chars_n_exceeds_length() {
    assert_eq!(first_n_chars("hi", 10), "hi");
}

#[test]
fn first_n_chars_empty_string_or_zero() {
    assert_eq!(first_n_chars("", 3), "");
    assert_eq!(first_n_chars("hello", 0), "");
}

#[test]
fn longest_palindromic_substring_slice_odd_length_tie_picks_leftmost() {
    assert_eq!(longest_palindromic_substring_slice("babad"), "bab");
}

#[test]
fn longest_palindromic_substring_slice_even_length() {
    assert_eq!(longest_palindromic_substring_slice("cbbd"), "bb");
}

#[test]
fn longest_palindromic_substring_slice_single_char_and_empty() {
    assert_eq!(longest_palindromic_substring_slice("a"), "a");
    assert_eq!(longest_palindromic_substring_slice(""), "");
}

#[test]
fn longest_palindromic_substring_slice_whole_string_is_palindrome() {
    assert_eq!(longest_palindromic_substring_slice("racecar"), "racecar");
}

#[test]
fn longest_palindromic_substring_slice_no_repeats() {
    assert_eq!(longest_palindromic_substring_slice("abcde"), "a");
}

#[test]
fn max_subarray_slice_classic_example() {
    assert_eq!(
        max_subarray_slice(&[-2, 1, -3, 4, -1, 2, 1, -5, 4]),
        &[4, -1, 2, 1]
    );
}

#[test]
fn max_subarray_slice_all_negative_picks_least_negative_element() {
    assert_eq!(max_subarray_slice(&[-3, -1, -2]), &[-1]);
}

#[test]
fn max_subarray_slice_single_element() {
    assert_eq!(max_subarray_slice(&[5]), &[5]);
}

#[test]
fn max_subarray_slice_all_positive_is_whole_slice() {
    assert_eq!(max_subarray_slice(&[1, 2, 3, 4]), &[1, 2, 3, 4]);
}

#[test]
fn chunk_slices_even_division() {
    assert_eq!(
        chunk_slices(&[1, 2, 3, 4], 2),
        vec![&[1, 2][..], &[3, 4][..]]
    );
}

#[test]
fn chunk_slices_uneven_division_has_shorter_last_chunk() {
    assert_eq!(
        chunk_slices(&[1, 2, 3, 4, 5], 2),
        vec![&[1, 2][..], &[3, 4][..], &[5][..]]
    );
}

#[test]
fn chunk_slices_empty_input() {
    assert_eq!(chunk_slices(&[] as &[i32], 3), Vec::<&[i32]>::new());
}

#[test]
fn chunk_slices_chunk_size_larger_than_input() {
    assert_eq!(chunk_slices(&[1, 2, 3], 5), vec![&[1, 2, 3][..]]);
}
