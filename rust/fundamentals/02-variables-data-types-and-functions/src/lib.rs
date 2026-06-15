//! Fundamentals 02 — Variables, Data Types & Functions.
//!
//! Exercises focus on the integer/float/bool/char/tuple/array types from
//! `notes.md`: fixed-size arrays, bit packing with `as` casts, and
//! overflow-aware arithmetic.

/// Rotates a 6-element array left by `positions` (mod 6) and returns the
/// result. The original array is unchanged (arrays are `Copy`).
///
/// # Examples
///
/// ```ignore
/// assert_eq!(rotate_array_left([1, 2, 3, 4, 5, 6], 0), [1, 2, 3, 4, 5, 6]);
/// assert_eq!(rotate_array_left([1, 2, 3, 4, 5, 6], 1), [2, 3, 4, 5, 6, 1]);
/// assert_eq!(rotate_array_left([1, 2, 3, 4, 5, 6], 8), [3, 4, 5, 6, 1, 2]); // 8 mod 6 == 2
/// ```
pub fn rotate_array_left(arr: [i32; 6], positions: usize) -> [i32; 6] {
    todo!()
}

/// Packs three 8-bit color channels into the low 24 bits of a `u32`: `r` in
/// bits 16-23, `g` in bits 8-15, `b` in bits 0-7. The top 8 bits (24-31) of
/// the result are always `0`.
///
/// # Examples
///
/// ```ignore
/// assert_eq!(pack_rgb(0xFF, 0x00, 0x00), 0xFF0000);
/// assert_eq!(pack_rgb(0x12, 0x34, 0x56), 0x123456);
/// assert_eq!(pack_rgb(0, 0, 0), 0);
/// ```
pub fn pack_rgb(r: u8, g: u8, b: u8) -> u32 {
    todo!()
}

/// Unpacks the low 24 bits of `packed` into `(r, g, b)` byte channels, the
/// inverse of [`pack_rgb`]. Any bits above bit 23 are ignored.
///
/// # Examples
///
/// ```ignore
/// assert_eq!(unpack_rgb(0x123456), (0x12, 0x34, 0x56));
/// assert_eq!(unpack_rgb(0xFFFFFFFF), (0xFF, 0xFF, 0xFF)); // top byte ignored
/// assert_eq!(unpack_rgb(0), (0, 0, 0));
/// ```
pub fn unpack_rgb(packed: u32) -> (u8, u8, u8) {
    todo!()
}

/// Computes `n!` (factorial) using wrapping `u32` arithmetic, returning
/// `(result, overflowed)` where `result` is the value of `n!` modulo
/// `2^32` and `overflowed` is `true` if the *true* mathematical value of
/// `n!` exceeds [`u32::MAX`] at any point during the computation.
///
/// `0! == 1`.
///
/// # Examples
///
/// ```ignore
/// assert_eq!(overflowing_factorial(0), (1, false));
/// assert_eq!(overflowing_factorial(5), (120, false));
/// assert_eq!(overflowing_factorial(12), (479_001_600, false)); // largest factorial that fits in u32
/// assert_eq!(overflowing_factorial(13), (1_932_053_504, true)); // 13! wraps
/// ```
pub fn overflowing_factorial(n: u32) -> (u32, bool) {
    todo!()
}

/// Computes fixed-point division: `(numerator * 10^scale) / denominator`,
/// using `i64` arithmetic to avoid overflow from the `10^scale` widening.
/// Integer division truncates toward zero (Rust's default `/` behavior for
/// signed integers).
///
/// # Panics
///
/// Panics if `denominator == 0` (same as any integer division by zero).
///
/// # Examples
///
/// ```ignore
/// assert_eq!(fixed_point_divide(10, 3, 4), 33_333);   // 100000 / 3, truncated
/// assert_eq!(fixed_point_divide(1, 4, 2), 25);        // 100 / 4
/// assert_eq!(fixed_point_divide(-10, 3, 4), -33_333); // truncates toward zero
/// assert_eq!(fixed_point_divide(7, 2, 0), 3);         // scale 0: plain integer division
/// ```
pub fn fixed_point_divide(numerator: i64, denominator: i64, scale: u32) -> i64 {
    todo!()
}
