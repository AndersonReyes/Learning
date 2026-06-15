//! Fundamentals 01 — Toolchain, Cargo & Hello World.
//!
//! These exercises don't depend on anything from `notes.md` beyond what
//! `cargo new` gives you for free: variables, `if`/`else`, `while`/`for`
//! loops, functions, and basic types (`u64`, `i32`, `bool`, `char`, `String`,
//! `&str`, `Vec`). The goal here is "first real Rust programs", not
//! toolchain trivia — the toolchain/Cargo material in `notes.md` is
//! conceptual background for the rest of the track.

/// Counts the number of steps for `n` to reach `1` under the Collatz
/// conjecture: while the current value is not `1`, if it's even divide it by
/// `2`, otherwise replace it with `3 * value + 1`. Each such replacement
/// counts as one step.
///
/// # Panics
///
/// May be called with any `n >= 1`. Behavior for `n == 0` is unspecified
/// (the sequence `0 -> 0 -> 0 -> ...` never reaches `1`).
///
/// # Examples
///
/// ```ignore
/// assert_eq!(collatz_steps(1), 0);
/// assert_eq!(collatz_steps(2), 1);   // 2 -> 1
/// assert_eq!(collatz_steps(6), 8);   // 6 -> 3 -> 10 -> 5 -> 16 -> 8 -> 4 -> 2 -> 1
/// ```
pub fn collatz_steps(n: u64) -> u32 {
    todo!("count Collatz steps from n down to 1")
}

/// Returns `true` if `n` is a prime number, `false` otherwise.
///
/// `0` and `1` are not prime. `2` is the only even prime.
///
/// # Examples
///
/// ```ignore
/// assert!(!is_prime(0));
/// assert!(!is_prime(1));
/// assert!(is_prime(2));
/// assert!(is_prime(97));
/// assert!(!is_prime(91)); // 7 * 13
/// ```
pub fn is_prime(n: u64) -> bool {
    todo!("trial-division primality test")
}

/// Finds the longest run of consecutive, identical characters in `s` and
/// returns `(character, run_length)`.
///
/// If multiple runs share the maximum length, returns the **first** one
/// (leftmost starting position). If `s` is empty, returns `('\0', 0)`.
///
/// Operates on `char`s (Unicode scalar values), not bytes — multi-byte UTF-8
/// characters count as a single character.
///
/// # Examples
///
/// ```ignore
/// assert_eq!(longest_run("aaabbbcc"), ('a', 3)); // "aaa" and "bbb" tie; "aaa" is first
/// assert_eq!(longest_run("abcabc"), ('a', 1));
/// assert_eq!(longest_run(""), ('\0', 0));
/// assert_eq!(longest_run("aabbbbaa"), ('b', 4));
/// ```
pub fn longest_run(s: &str) -> (char, usize) {
    todo!("scan for the longest run of identical characters")
}

/// Applies a Caesar cipher to `s`, shifting each ASCII letter by `shift`
/// positions in the alphabet (wrapping around), preserving case.
/// Non-alphabetic characters (digits, punctuation, whitespace) pass through
/// unchanged. `shift` may be negative or have absolute value greater than
/// `26`; both are reduced modulo `26`.
///
/// # Examples
///
/// ```ignore
/// assert_eq!(caesar_cipher("abc", 1), "bcd");
/// assert_eq!(caesar_cipher("xyz", 3), "abc");
/// assert_eq!(caesar_cipher("Hello, World!", 5), "Mjqqt, Btwqi!");
/// assert_eq!(caesar_cipher("abc", -1), "zab");
/// assert_eq!(caesar_cipher("ABC", 29), "DEF"); // 29 mod 26 == 3
/// ```
pub fn caesar_cipher(s: &str, shift: i32) -> String {
    todo!("shift each letter by `shift` positions, wrapping mod 26")
}

/// Transposes a 2D matrix represented as a slice of rows.
///
/// Assumes the matrix is rectangular (every row has the same length). If
/// `matrix` is empty, or every row is empty, returns an empty `Vec`.
///
/// # Examples
///
/// ```ignore
/// assert_eq!(matrix_transpose(&[vec![1, 2, 3], vec![4, 5, 6]]),
///            vec![vec![1, 4], vec![2, 5], vec![3, 6]]);
/// assert_eq!(matrix_transpose(&[vec![1], vec![2], vec![3]]),
///            vec![vec![1, 2, 3]]);
/// assert_eq!(matrix_transpose(&[] as &[Vec<i32>]), Vec::<Vec<i32>>::new());
/// ```
pub fn matrix_transpose(matrix: &[Vec<i32>]) -> Vec<Vec<i32>> {
    todo!("swap rows and columns")
}
