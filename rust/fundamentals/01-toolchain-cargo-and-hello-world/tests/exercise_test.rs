use fundamentals_01_toolchain_cargo_and_hello_world::{
    caesar_cipher, collatz_steps, is_prime, longest_run, matrix_transpose,
};

#[test]
fn collatz_steps_base_case() {
    assert_eq!(collatz_steps(1), 0);
}

#[test]
fn collatz_steps_short_chains() {
    assert_eq!(collatz_steps(2), 1); // 2 -> 1
    assert_eq!(collatz_steps(4), 2); // 4 -> 2 -> 1
    assert_eq!(collatz_steps(6), 8); // 6 -> 3 -> 10 -> 5 -> 16 -> 8 -> 4 -> 2 -> 1
}

#[test]
fn collatz_steps_longer_chains() {
    assert_eq!(collatz_steps(7), 16);
    assert_eq!(collatz_steps(27), 111);
}

#[test]
fn is_prime_edge_cases() {
    assert!(!is_prime(0));
    assert!(!is_prime(1));
    assert!(is_prime(2));
    assert!(is_prime(3));
    assert!(!is_prime(4));
}

#[test]
fn is_prime_composites_with_large_factors() {
    assert!(!is_prime(91)); // 7 * 13
    assert!(!is_prime(1_000_000)); // even
    assert!(!is_prime(999_999)); // divisible by 3
}

#[test]
fn is_prime_known_primes() {
    assert!(is_prime(17));
    assert!(is_prime(97));
    assert!(is_prime(999_983)); // largest prime below 1,000,000
}

#[test]
fn longest_run_empty_string() {
    assert_eq!(longest_run(""), ('\0', 0));
}

#[test]
fn longest_run_single_character() {
    assert_eq!(longest_run("a"), ('a', 1));
}

#[test]
fn longest_run_ties_pick_first() {
    // "aaa" and "bbb" both have length 3; "aaa" comes first.
    assert_eq!(longest_run("aaabbbcc"), ('a', 3));
}

#[test]
fn longest_run_no_repeats() {
    assert_eq!(longest_run("abcabc"), ('a', 1));
}

#[test]
fn longest_run_repeat_in_the_middle() {
    assert_eq!(longest_run("aabbbbaa"), ('b', 4));
}

#[test]
fn longest_run_unicode_chars() {
    // Each run ("αα", "ββ") has length 2; "α" comes first.
    assert_eq!(longest_run("ααββ"), ('α', 2));
}

#[test]
fn caesar_cipher_basic_shift() {
    assert_eq!(caesar_cipher("abc", 1), "bcd");
    assert_eq!(caesar_cipher("xyz", 3), "abc");
}

#[test]
fn caesar_cipher_preserves_case_and_punctuation() {
    assert_eq!(caesar_cipher("Hello, World!", 5), "Mjqqt, Btwqi!");
}

#[test]
fn caesar_cipher_negative_shift() {
    assert_eq!(caesar_cipher("abc", -1), "zab");
    assert_eq!(caesar_cipher("Hello", -5), "Czggj");
}

#[test]
fn caesar_cipher_shift_outside_plus_minus_26() {
    assert_eq!(caesar_cipher("ABC", 29), "DEF"); // 29 mod 26 == 3
    assert_eq!(caesar_cipher("abc", -29), "xyz"); // -29 mod 26 == 23, i.e. shift -3
}

#[test]
fn caesar_cipher_zero_shift_and_empty_string() {
    assert_eq!(caesar_cipher("Test123", 0), "Test123");
    assert_eq!(caesar_cipher("", 7), "");
}

#[test]
fn matrix_transpose_empty_matrix() {
    let empty: &[Vec<i32>] = &[];
    assert_eq!(matrix_transpose(empty), Vec::<Vec<i32>>::new());
}

#[test]
fn matrix_transpose_row_of_empty_rows() {
    let one_empty_row: &[Vec<i32>] = &[vec![]];
    assert_eq!(matrix_transpose(one_empty_row), Vec::<Vec<i32>>::new());
}

#[test]
fn matrix_transpose_rectangular() {
    let m = vec![vec![1, 2, 3], vec![4, 5, 6]];
    assert_eq!(
        matrix_transpose(&m),
        vec![vec![1, 4], vec![2, 5], vec![3, 6]]
    );
}

#[test]
fn matrix_transpose_column_to_row() {
    let m = vec![vec![1], vec![2], vec![3]];
    assert_eq!(matrix_transpose(&m), vec![vec![1, 2, 3]]);
}

#[test]
fn matrix_transpose_is_its_own_inverse() {
    let m = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
    let transposed = matrix_transpose(&m);
    let back = matrix_transpose(&transposed);
    assert_eq!(back, m);
}
