//! Advanced 06 — Data Layout & Type Conversions (Nomicon).
//!
//! Five exercises: `TemperatureKelvin` newtype with `From<TemperatureCelsius>`,
//! `From<TemperatureFahrenheit>`, and `TryFrom<TemperatureKelvin>` for
//! `TemperatureCelsius`; `cast_behaviors()` returning a tuple of five `as`-cast
//! results; `f32_round_trip` using `to_bits`/`from_bits`; `CPoint` (`#[repr(C)]`
//! struct) with `cpoint_layout()` returning `(size, align, offset_of_y)`; and
//! `Pair<i32>` with `TryFrom<&str>` parsing `"a,b"` format.

// ---------------------------------------------------------------------------
// Exercise 1 — From/Into/TryFrom: TemperatureKelvin
// ---------------------------------------------------------------------------

/// A temperature in Kelvin, always ≥ 0.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct TemperatureKelvin(pub f64);

/// A temperature in Celsius.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct TemperatureCelsius(pub f64);

/// A temperature in Fahrenheit.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct TemperatureFahrenheit(pub f64);

/// Converts Celsius → Kelvin: `K = C + 273.15`.
impl From<TemperatureCelsius> for TemperatureKelvin {
    fn from(c: TemperatureCelsius) -> Self {
        todo!()
    }
}

/// Converts Fahrenheit → Kelvin: `K = (F - 32) × 5/9 + 273.15`.
impl From<TemperatureFahrenheit> for TemperatureKelvin {
    fn from(f: TemperatureFahrenheit) -> Self {
        todo!()
    }
}

/// Converts Kelvin → Celsius: `C = K - 273.15`.
/// Returns `Err("temperature below absolute zero")` if `k.0 < 0.0`.
impl TryFrom<TemperatureKelvin> for TemperatureCelsius {
    type Error = &'static str;

    fn try_from(k: TemperatureKelvin) -> Result<Self, Self::Error> {
        todo!()
    }
}

// ---------------------------------------------------------------------------
// Exercise 2 — as casts: cast_behaviors
// ---------------------------------------------------------------------------

/// Returns `(3.9_f64 as i32, -3.9_f64 as i32, 300_u16 as u8, 200_u8 as i8, 999.0_f64 as u8)`.
///
/// Demonstrates truncation, wrap, bit-reinterpretation, and saturation.
///
/// # Examples
///
/// ```ignore
/// use advanced_06_data_layout_and_type_conversions::cast_behaviors;
///
/// assert_eq!(cast_behaviors(), (3, -3, 44, -56, 255));
/// ```
pub fn cast_behaviors() -> (i32, i32, u8, i8, u8) {
    todo!()
}

// ---------------------------------------------------------------------------
// Exercise 3 — f32 bits: f32_round_trip
// ---------------------------------------------------------------------------

/// Returns `(x.to_bits(), f32::from_bits(x.to_bits()))`.
///
/// # Examples
///
/// ```ignore
/// use advanced_06_data_layout_and_type_conversions::f32_round_trip;
///
/// let (bits, recovered) = f32_round_trip(1.0_f32);
/// assert_eq!(bits, 0x3F80_0000_u32);
/// assert_eq!(recovered, 1.0_f32);
/// ```
pub fn f32_round_trip(x: f32) -> (u32, f32) {
    todo!()
}

// ---------------------------------------------------------------------------
// Exercise 4 — repr(C): CPoint layout
// ---------------------------------------------------------------------------

/// A `#[repr(C)]` 2-D point.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CPoint {
    pub x: f32,
    pub y: f32,
}

/// Returns `(size_of::<CPoint>(), align_of::<CPoint>(), offset_of_y_in_bytes)`.
///
/// Expected: `(8, 4, 4)`.
///
/// # Examples
///
/// ```ignore
/// use advanced_06_data_layout_and_type_conversions::cpoint_layout;
///
/// assert_eq!(cpoint_layout(), (8, 4, 4));
/// ```
pub fn cpoint_layout() -> (usize, usize, usize) {
    todo!()
}

// ---------------------------------------------------------------------------
// Exercise 5 — TryFrom<&str>: Pair
// ---------------------------------------------------------------------------

/// A pair of values.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pair<T> {
    pub first: T,
    pub second: T,
}

/// Parses `"a,b"` into `Pair<i32>`.
///
/// - `Err("no comma")` if no comma present.
/// - `Err("multiple commas")` if more than one comma.
/// - `Err(<ParseIntError message>)` if either part fails to parse.
///
/// # Examples
///
/// ```ignore
/// use advanced_06_data_layout_and_type_conversions::Pair;
///
/// assert_eq!(Pair::<i32>::try_from("3,4").unwrap(), Pair { first: 3, second: 4 });
/// assert!(Pair::<i32>::try_from("abc").is_err());
/// assert!(Pair::<i32>::try_from("1,2,3").is_err());
/// ```
impl TryFrom<&str> for Pair<i32> {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl std::str::FromStr for Pair<i32> {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Pair::try_from(s)
    }
}
