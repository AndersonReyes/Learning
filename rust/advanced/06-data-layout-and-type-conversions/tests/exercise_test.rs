use advanced_06_data_layout_and_type_conversions::{
    cast_behaviors, cpoint_layout, f32_round_trip, CPoint, Pair, TemperatureCelsius,
    TemperatureFahrenheit, TemperatureKelvin,
};

// --- Exercise 1: TemperatureKelvin From/TryFrom -------------------------------

#[test]
fn celsius_to_kelvin_zero() {
    let k = TemperatureKelvin::from(TemperatureCelsius(0.0));
    assert!((k.0 - 273.15).abs() < 1e-9);
}

#[test]
fn celsius_to_kelvin_boiling() {
    let k = TemperatureKelvin::from(TemperatureCelsius(100.0));
    assert!((k.0 - 373.15).abs() < 1e-9);
}

#[test]
fn celsius_to_kelvin_negative() {
    let k = TemperatureKelvin::from(TemperatureCelsius(-273.15));
    assert!(k.0.abs() < 1e-9);
}

#[test]
fn celsius_into_kelvin() {
    let k: TemperatureKelvin = TemperatureCelsius(25.0).into();
    assert!((k.0 - 298.15).abs() < 1e-9);
}

#[test]
fn fahrenheit_to_kelvin_freezing() {
    let k = TemperatureKelvin::from(TemperatureFahrenheit(32.0));
    assert!((k.0 - 273.15).abs() < 1e-9);
}

#[test]
fn fahrenheit_to_kelvin_boiling() {
    let k = TemperatureKelvin::from(TemperatureFahrenheit(212.0));
    assert!((k.0 - 373.15).abs() < 1e-9);
}

#[test]
fn kelvin_to_celsius_ok() {
    let c = TemperatureCelsius::try_from(TemperatureKelvin(373.15)).unwrap();
    assert!((c.0 - 100.0).abs() < 1e-9);
}

#[test]
fn kelvin_to_celsius_absolute_zero() {
    let c = TemperatureCelsius::try_from(TemperatureKelvin(0.0)).unwrap();
    assert!((c.0 - (-273.15)).abs() < 1e-9);
}

#[test]
fn kelvin_to_celsius_negative_is_err() {
    assert!(TemperatureCelsius::try_from(TemperatureKelvin(-1.0)).is_err());
}

// --- Exercise 2: as casts ----------------------------------------------------

#[test]
fn cast_behaviors_correct() {
    assert_eq!(cast_behaviors(), (3, -3, 44, -56, 255));
}

// --- Exercise 3: f32 bits ----------------------------------------------------

#[test]
fn f32_round_trip_one() {
    let (bits, recovered) = f32_round_trip(1.0_f32);
    assert_eq!(bits, 0x3F80_0000_u32);
    assert_eq!(recovered, 1.0_f32);
}

#[test]
fn f32_round_trip_neg_zero() {
    let (bits, recovered) = f32_round_trip(-0.0_f32);
    assert_eq!(bits, 0x8000_0000_u32);
    assert_eq!(recovered.to_bits(), 0x8000_0000_u32);
}

#[test]
fn f32_round_trip_pi() {
    let pi = std::f32::consts::PI;
    let (bits, recovered) = f32_round_trip(pi);
    assert_eq!(bits, pi.to_bits());
    assert_eq!(recovered, pi);
}

#[test]
fn f32_round_trip_infinity() {
    let (bits, recovered) = f32_round_trip(f32::INFINITY);
    assert_eq!(bits, 0x7F80_0000_u32);
    assert!(recovered.is_infinite() && recovered.is_sign_positive());
}

// --- Exercise 4: CPoint layout -----------------------------------------------

#[test]
fn cpoint_layout_values() {
    assert_eq!(cpoint_layout(), (8, 4, 4));
}

#[test]
fn cpoint_size() {
    assert_eq!(std::mem::size_of::<CPoint>(), 8);
}

#[test]
fn cpoint_align() {
    assert_eq!(std::mem::align_of::<CPoint>(), 4);
}

// --- Exercise 5: Pair<i32> TryFrom<&str> -------------------------------------

#[test]
fn pair_try_from_ok() {
    let p = Pair::<i32>::try_from("3,4").unwrap();
    assert_eq!((p.first, p.second), (3, 4));
}

#[test]
fn pair_try_from_negatives() {
    let p = Pair::<i32>::try_from("-10,20").unwrap();
    assert_eq!((p.first, p.second), (-10, 20));
}

#[test]
fn pair_into_via_try_into() {
    let p: Pair<i32> = "7,8".try_into().unwrap();
    assert_eq!((p.first, p.second), (7, 8));
}

#[test]
fn pair_no_comma_err() {
    assert!(Pair::<i32>::try_from("abc").is_err());
}

#[test]
fn pair_multiple_commas_err() {
    assert!(Pair::<i32>::try_from("1,2,3").is_err());
}

#[test]
fn pair_non_numeric_err() {
    assert!(Pair::<i32>::try_from("a,b").is_err());
}

#[test]
fn pair_from_str() {
    let p: Pair<i32> = "5,6".parse().unwrap();
    assert_eq!((p.first, p.second), (5, 6));
}

#[test]
fn pair_zeros() {
    let p = Pair::<i32>::try_from("0,0").unwrap();
    assert_eq!((p.first, p.second), (0, 0));
}
