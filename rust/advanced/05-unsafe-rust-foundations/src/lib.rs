//! Advanced 05 — Unsafe Rust Foundations (Book ch. 20.1; Nomicon).
//!
//! Five exercises using `unsafe` blocks: `raw_swap` (swap two `*mut T` values),
//! `sum_slice_ptr` (sum a slice via raw pointer arithmetic), `split_at_mid`
//! (split a `&[T]` at an index using `from_raw_parts`), `read_le_u32` (read a
//! little-endian `u32` from a 4-byte raw pointer), and `count_zeros_unsafe`
//! (count zero bytes in a raw byte buffer without constructing a safe slice).

use std::ptr;

// ---------------------------------------------------------------------------
// Exercise 1 — raw_swap
// ---------------------------------------------------------------------------

/// Swaps the values at `a` and `b` using only raw pointer operations.
///
/// # Safety
///
/// Both `a` and `b` must be non-null, aligned, and valid for reads and writes
/// of `T`. They must not alias.
///
/// # Examples
///
/// ```ignore
/// use advanced_05_unsafe_rust_foundations::raw_swap;
///
/// let mut x = 10_i32;
/// let mut y = 20_i32;
/// unsafe { raw_swap(&mut x as *mut i32, &mut y as *mut i32); }
/// assert_eq!((x, y), (20, 10));
/// ```
pub unsafe fn raw_swap<T>(a: *mut T, b: *mut T) {
    todo!()
}

// ---------------------------------------------------------------------------
// Exercise 2 — sum_slice_ptr
// ---------------------------------------------------------------------------

/// Sums all `i64` values in `[ptr, ptr + len)` using raw pointer arithmetic.
///
/// # Safety
///
/// `ptr` must be non-null, aligned to `i64`, and valid for `len` reads.
///
/// # Examples
///
/// ```ignore
/// use advanced_05_unsafe_rust_foundations::sum_slice_ptr;
///
/// let data = [1_i64, 2, 3, 4, 5];
/// assert_eq!(unsafe { sum_slice_ptr(data.as_ptr(), data.len()) }, 15);
/// ```
pub unsafe fn sum_slice_ptr(ptr: *const i64, len: usize) -> i64 {
    todo!()
}

// ---------------------------------------------------------------------------
// Exercise 3 — split_at_mid
// ---------------------------------------------------------------------------

/// Splits `slice` at `mid`, returning `(&slice[..mid], &slice[mid..])`.
///
/// Panics if `mid > slice.len()`. Implements the split using
/// `std::slice::from_raw_parts`.
///
/// # Examples
///
/// ```ignore
/// use advanced_05_unsafe_rust_foundations::split_at_mid;
///
/// let v = [1_i32, 2, 3, 4, 5];
/// assert_eq!(split_at_mid(&v, 2), (&[1, 2][..], &[3, 4, 5][..]));
/// ```
pub fn split_at_mid<T>(slice: &[T], mid: usize) -> (&[T], &[T]) {
    todo!()
}

// ---------------------------------------------------------------------------
// Exercise 4 — read_le_u32
// ---------------------------------------------------------------------------

/// Reads a `u32` in little-endian order from the 4-byte buffer at `ptr`.
///
/// # Safety
///
/// `ptr` must be non-null and valid for 4 consecutive `u8` reads.
///
/// # Examples
///
/// ```ignore
/// use advanced_05_unsafe_rust_foundations::read_le_u32;
///
/// let bytes = [0x78_u8, 0x56, 0x34, 0x12];
/// assert_eq!(unsafe { read_le_u32(bytes.as_ptr()) }, 0x12345678_u32);
/// ```
pub unsafe fn read_le_u32(ptr: *const u8) -> u32 {
    todo!()
}

// ---------------------------------------------------------------------------
// Exercise 5 — count_zeros_unsafe
// ---------------------------------------------------------------------------

/// Counts zero bytes in `[ptr, ptr + len)` via raw pointer arithmetic.
///
/// # Safety
///
/// `ptr` must be non-null and valid for `len` `u8` reads.
///
/// # Examples
///
/// ```ignore
/// use advanced_05_unsafe_rust_foundations::count_zeros_unsafe;
///
/// let data = [1_u8, 0, 2, 0, 0, 3];
/// assert_eq!(unsafe { count_zeros_unsafe(data.as_ptr(), data.len()) }, 3);
/// ```
pub unsafe fn count_zeros_unsafe(ptr: *const u8, len: usize) -> usize {
    todo!()
}
