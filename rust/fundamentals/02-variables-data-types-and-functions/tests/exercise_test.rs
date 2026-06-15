use fundamentals_02_variables_data_types_and_functions::{
    fixed_point_divide, overflowing_factorial, pack_rgb, rotate_array_left, unpack_rgb,
};

#[test]
fn rotate_array_left_zero_is_identity() {
    assert_eq!(rotate_array_left([1, 2, 3, 4, 5, 6], 0), [1, 2, 3, 4, 5, 6]);
}

#[test]
fn rotate_array_left_basic() {
    assert_eq!(rotate_array_left([1, 2, 3, 4, 5, 6], 1), [2, 3, 4, 5, 6, 1]);
    assert_eq!(rotate_array_left([1, 2, 3, 4, 5, 6], 2), [3, 4, 5, 6, 1, 2]);
}

#[test]
fn rotate_array_left_full_rotation_is_identity() {
    assert_eq!(rotate_array_left([1, 2, 3, 4, 5, 6], 6), [1, 2, 3, 4, 5, 6]);
}

#[test]
fn rotate_array_left_wraps_modulo_length() {
    assert_eq!(rotate_array_left([1, 2, 3, 4, 5, 6], 8), [3, 4, 5, 6, 1, 2]); // 8 mod 6 == 2
    assert_eq!(
        rotate_array_left([1, 2, 3, 4, 5, 6], 100),
        [5, 6, 1, 2, 3, 4]
    ); // 100 mod 6 == 4
}

#[test]
fn pack_rgb_pure_channels() {
    assert_eq!(pack_rgb(0xFF, 0x00, 0x00), 0xFF0000);
    assert_eq!(pack_rgb(0x00, 0xFF, 0x00), 0x00FF00);
    assert_eq!(pack_rgb(0x00, 0x00, 0xFF), 0x0000FF);
}

#[test]
fn pack_rgb_mixed_and_extremes() {
    assert_eq!(pack_rgb(0x12, 0x34, 0x56), 0x123456);
    assert_eq!(pack_rgb(0, 0, 0), 0);
    assert_eq!(pack_rgb(255, 255, 255), 0x00FF_FFFF);
}

#[test]
fn unpack_rgb_basic() {
    assert_eq!(unpack_rgb(0x123456), (0x12, 0x34, 0x56));
    assert_eq!(unpack_rgb(0), (0, 0, 0));
}

#[test]
fn unpack_rgb_ignores_bits_above_24() {
    assert_eq!(unpack_rgb(0xFFFF_FFFF), (0xFF, 0xFF, 0xFF));
    assert_eq!(unpack_rgb(0xAB12_3456), (0x12, 0x34, 0x56));
}

#[test]
fn pack_then_unpack_round_trips() {
    assert_eq!(unpack_rgb(pack_rgb(0x7A, 0x3C, 0x01)), (0x7A, 0x3C, 0x01));
    assert_eq!(unpack_rgb(pack_rgb(1, 2, 3)), (1, 2, 3));
}

#[test]
fn overflowing_factorial_base_cases() {
    assert_eq!(overflowing_factorial(0), (1, false));
    assert_eq!(overflowing_factorial(1), (1, false));
}

#[test]
fn overflowing_factorial_small_values() {
    assert_eq!(overflowing_factorial(5), (120, false));
    assert_eq!(overflowing_factorial(10), (3_628_800, false));
}

#[test]
fn overflowing_factorial_largest_that_fits_in_u32() {
    // 12! = 479,001,600 fits in u32 (max ~4.29e9); 13! does not.
    assert_eq!(overflowing_factorial(12), (479_001_600, false));
}

#[test]
fn overflowing_factorial_overflows_and_wraps() {
    // 13! = 6,227,020,800 ; mod 2^32 == 1,932,053,504
    assert_eq!(overflowing_factorial(13), (1_932_053_504, true));
}

#[test]
fn fixed_point_divide_basic_truncation() {
    assert_eq!(fixed_point_divide(10, 3, 4), 33_333); // 100000 / 3
    assert_eq!(fixed_point_divide(1, 4, 2), 25); // 100 / 4
}

#[test]
fn fixed_point_divide_zero_scale_is_plain_division() {
    assert_eq!(fixed_point_divide(7, 2, 0), 3);
    assert_eq!(fixed_point_divide(0, 5, 3), 0);
}

#[test]
fn fixed_point_divide_truncates_toward_zero_for_negatives() {
    assert_eq!(fixed_point_divide(-10, 3, 4), -33_333);
    assert_eq!(fixed_point_divide(10, -3, 4), -33_333);
    assert_eq!(fixed_point_divide(-10, -3, 4), 33_333);
}
