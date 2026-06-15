//! Fundamentals 03 — Control Flow.
//!
//! Exercises focus on the `if`/`else`-as-expression, `loop`/`while`/`for`,
//! `break`/`continue`, and loop-label constructs from `notes.md`.

/// Searches `grid` in row-major order (row 0 first, left to right within
/// each row) and returns the `(row, col)` of the first cell equal to
/// `target`.
///
/// # Panics / Preconditions
///
/// `target` is guaranteed to appear at least once in `grid` (no fallback
/// value is defined for "not found").
///
/// # Examples
///
/// ```ignore
/// let grid = [
///     [1, 2, 3, 4],
///     [5, 6, 7, 8],
///     [9, 10, 11, 12],
///     [13, 14, 15, 16],
/// ];
/// assert_eq!(find_in_grid(&grid, 7), (1, 2));
/// assert_eq!(find_in_grid(&grid, 1), (0, 0));
/// assert_eq!(find_in_grid(&grid, 16), (3, 3));
/// ```
pub fn find_in_grid(grid: &[[i32; 4]; 4], target: i32) -> (usize, usize) {
    todo!()
}

/// Returns `true` if `n` is an [Armstrong number][1]: the sum of each of its
/// decimal digits raised to the power of the digit *count* equals `n`
/// itself. Single-digit numbers (including `0`) are always Armstrong
/// numbers.
///
/// [1]: https://en.wikipedia.org/wiki/Narcissistic_number
///
/// # Examples
///
/// ```ignore
/// assert_eq!(is_armstrong_number(0), true);
/// assert_eq!(is_armstrong_number(5), true);
/// assert_eq!(is_armstrong_number(10), false);
/// assert_eq!(is_armstrong_number(153), true);  // 1^3 + 5^3 + 3^3 == 153
/// assert_eq!(is_armstrong_number(9474), true); // 9^4+4^4+7^4+4^4 == 9474
/// assert_eq!(is_armstrong_number(9475), false);
/// ```
pub fn is_armstrong_number(n: u32) -> bool {
    todo!()
}

/// Sums every integer in `1..limit` (exclusive) that is divisible by **at
/// least one** value in `factors`, counting each qualifying integer only
/// once even if it's divisible by multiple factors.
///
/// # Panics / Preconditions
///
/// Every element of `factors` must be non-zero.
///
/// # Examples
///
/// ```ignore
/// assert_eq!(sum_of_multiples_below(10, &[3, 5]), 23);   // 3+5+6+9
/// assert_eq!(sum_of_multiples_below(1000, &[3, 5]), 233_168);
/// assert_eq!(sum_of_multiples_below(10, &[2]), 20);       // 2+4+6+8
/// assert_eq!(sum_of_multiples_below(5, &[7]), 0);
/// ```
pub fn sum_of_multiples_below(limit: u32, factors: &[u32]) -> u64 {
    todo!()
}

/// Repeatedly sums the decimal digits of `n` until a single digit (0-9)
/// remains, returning that digit (the [digital root][1]).
///
/// [1]: https://en.wikipedia.org/wiki/Digital_root
///
/// # Examples
///
/// ```ignore
/// assert_eq!(digital_root(0), 0);
/// assert_eq!(digital_root(9), 9);
/// assert_eq!(digital_root(38), 2);        // 3+8=11, 1+1=2
/// assert_eq!(digital_root(9875), 2);      // 9+8+7+5=29, 2+9=11, 1+1=2
/// assert_eq!(digital_root(123456789), 9); // digits sum to 45, 4+5=9
/// ```
pub fn digital_root(n: u64) -> u8 {
    todo!()
}

/// Starting at `start`, repeatedly adds `step` and counts how many additions
/// are needed before the running total has reached or passed `target`:
///
/// - If `step > 0`, the loop stops once the total is `>= target`.
/// - If `step < 0`, the loop stops once the total is `<= target`.
///
/// Returns the number of additions performed (`0` if `start` already
/// satisfies the stopping condition).
///
/// # Panics / Preconditions
///
/// `step` must be non-zero (a zero step that hasn't already reached `target`
/// would never terminate).
///
/// # Examples
///
/// ```ignore
/// assert_eq!(count_steps_to_reach(0, 3, 10), 4);   // 0,3,6,9,12 -> 4 steps
/// assert_eq!(count_steps_to_reach(20, -5, 0), 4);  // 20,15,10,5,0 -> 4 steps
/// assert_eq!(count_steps_to_reach(5, 1, 5), 0);    // already there
/// assert_eq!(count_steps_to_reach(5, -1, 5), 0);   // already there
/// assert_eq!(count_steps_to_reach(-10, 3, -1), 3); // -10,-7,-4,-1 -> 3 steps
/// ```
pub fn count_steps_to_reach(start: i32, step: i32, target: i32) -> u32 {
    todo!()
}
