use advanced_05_unsafe_rust_foundations::{
    count_zeros_unsafe, raw_swap, read_le_u32, split_at_mid, sum_slice_ptr,
};

// --- Exercise 1: raw_swap ---------------------------------------------------

#[test]
fn raw_swap_i32() {
    let mut x = 10_i32;
    let mut y = 20_i32;
    unsafe { raw_swap(&mut x as *mut i32, &mut y as *mut i32); }
    assert_eq!((x, y), (20, 10));
}

#[test]
fn raw_swap_string() {
    let mut a = "hello".to_string();
    let mut b = "world".to_string();
    unsafe { raw_swap(&mut a as *mut String, &mut b as *mut String); }
    assert_eq!(a, "world");
    assert_eq!(b, "hello");
}

#[test]
fn raw_swap_idempotent_double_swap() {
    let mut x = 42_i32;
    let mut y = 99_i32;
    unsafe {
        raw_swap(&mut x as *mut i32, &mut y as *mut i32);
        raw_swap(&mut x as *mut i32, &mut y as *mut i32);
    }
    assert_eq!((x, y), (42, 99));
}

#[test]
fn raw_swap_zero_values() {
    let mut x = 0_i32;
    let mut y = 0_i32;
    unsafe { raw_swap(&mut x as *mut i32, &mut y as *mut i32); }
    assert_eq!((x, y), (0, 0));
}

// --- Exercise 2: sum_slice_ptr -----------------------------------------------

#[test]
fn sum_slice_ptr_typical() {
    let data = [1_i64, 2, 3, 4, 5];
    let result = unsafe { sum_slice_ptr(data.as_ptr(), data.len()) };
    assert_eq!(result, 15);
}

#[test]
fn sum_slice_ptr_empty() {
    let data: [i64; 0] = [];
    let result = unsafe { sum_slice_ptr(data.as_ptr(), 0) };
    assert_eq!(result, 0);
}

#[test]
fn sum_slice_ptr_single() {
    let data = [42_i64];
    let result = unsafe { sum_slice_ptr(data.as_ptr(), 1) };
    assert_eq!(result, 42);
}

#[test]
fn sum_slice_ptr_negatives() {
    let data = [-3_i64, 7, -2, 8];
    let result = unsafe { sum_slice_ptr(data.as_ptr(), data.len()) };
    assert_eq!(result, 10);
}

#[test]
fn sum_slice_ptr_large() {
    let data: Vec<i64> = (1..=100).collect();
    let result = unsafe { sum_slice_ptr(data.as_ptr(), data.len()) };
    assert_eq!(result, 5050);
}

// --- Exercise 3: split_at_mid -----------------------------------------------

#[test]
fn split_at_mid_typical() {
    let v = [1_i32, 2, 3, 4, 5];
    assert_eq!(split_at_mid(&v, 2), (&[1, 2][..], &[3, 4, 5][..]));
}

#[test]
fn split_at_mid_zero() {
    let v = [1_i32, 2, 3];
    assert_eq!(split_at_mid(&v, 0), (&[][..], &v[..]));
}

#[test]
fn split_at_mid_full() {
    let v = [1_i32, 2, 3];
    assert_eq!(split_at_mid(&v, 3), (&v[..], &[][..]));
}

#[test]
fn split_at_mid_single_element() {
    let v = [7_i32];
    assert_eq!(split_at_mid(&v, 1), (&[7][..], &[][..]));
    assert_eq!(split_at_mid(&v, 0), (&[][..], &[7][..]));
}

#[test]
#[should_panic(expected = "mid out of bounds")]
fn split_at_mid_out_of_bounds() {
    let v = [1_i32, 2, 3];
    split_at_mid(&v, 4); // must panic
}

// --- Exercise 4: read_le_u32 -------------------------------------------------

#[test]
fn read_le_u32_one() {
    let bytes = [0x01_u8, 0x00, 0x00, 0x00];
    assert_eq!(unsafe { read_le_u32(bytes.as_ptr()) }, 1_u32);
}

#[test]
fn read_le_u32_255() {
    let bytes = [0xFF_u8, 0x00, 0x00, 0x00];
    assert_eq!(unsafe { read_le_u32(bytes.as_ptr()) }, 255_u32);
}

#[test]
fn read_le_u32_multi_byte() {
    let bytes = [0x78_u8, 0x56, 0x34, 0x12];
    assert_eq!(unsafe { read_le_u32(bytes.as_ptr()) }, 0x12345678_u32);
}

#[test]
fn read_le_u32_max() {
    let bytes = [0xFF_u8, 0xFF, 0xFF, 0xFF];
    assert_eq!(unsafe { read_le_u32(bytes.as_ptr()) }, u32::MAX);
}

#[test]
fn read_le_u32_zero() {
    let bytes = [0x00_u8, 0x00, 0x00, 0x00];
    assert_eq!(unsafe { read_le_u32(bytes.as_ptr()) }, 0_u32);
}

// --- Exercise 5: count_zeros_unsafe ------------------------------------------

#[test]
fn count_zeros_typical() {
    let data = [1_u8, 0, 2, 0, 0, 3];
    assert_eq!(unsafe { count_zeros_unsafe(data.as_ptr(), data.len()) }, 3);
}

#[test]
fn count_zeros_empty() {
    let data: [u8; 0] = [];
    assert_eq!(unsafe { count_zeros_unsafe(data.as_ptr(), 0) }, 0);
}

#[test]
fn count_zeros_none() {
    let data = [1_u8, 2, 3, 4];
    assert_eq!(unsafe { count_zeros_unsafe(data.as_ptr(), data.len()) }, 0);
}

#[test]
fn count_zeros_all() {
    let data = [0_u8; 5];
    assert_eq!(unsafe { count_zeros_unsafe(data.as_ptr(), data.len()) }, 5);
}

#[test]
fn count_zeros_single_nonzero() {
    let data = [0_u8, 0, 1, 0];
    assert_eq!(unsafe { count_zeros_unsafe(data.as_ptr(), data.len()) }, 3);
}
