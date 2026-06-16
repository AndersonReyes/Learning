//! Run with: `cargo run --example examples -p advanced-06-data-layout-and-type-conversions`

// --- From / Into helpers ----------------------------------------------------

struct Miles(f64);
struct Kilometers(f64);

impl From<Miles> for Kilometers {
    fn from(m: Miles) -> Self {
        Kilometers(m.0 * 1.60934)
    }
}

// --- repr(C) layout helpers -------------------------------------------------

#[repr(C)]
struct CAligned { a: u8, b: u32, c: u8 }

#[repr(C)]
struct CPoint2 { x: f32, y: f32 }

fn main() {
    // --- From / Into ---------------------------------------------------------
    println!("-- From / Into --");

    let marathon = Miles(26.2188);
    let km: Kilometers = marathon.into();
    println!("marathon in km: {:.3}", km.0);

    let big: i32 = 200;
    let small: Result<i8, _> = i8::try_from(big);
    println!("i8::try_from(200): {:?}", small);
    let ok_small: Result<i8, _> = i8::try_from(42_i32);
    println!("i8::try_from(42): {:?}", ok_small);

    // --- as casts ------------------------------------------------------------
    println!("\n-- as casts --");

    let f: f64 = 3.9;
    println!("3.9_f64 as i32 = {}", f as i32);
    println!("-3.9_f64 as i32 = {}", -f as i32);

    let big_u16: u16 = 300;
    println!("300_u16 as u8 = {}", big_u16 as u8);

    let signed: u8 = 200;
    println!("200_u8 as i8 = {}", signed as i8);

    println!("999.0_f64 as u8 = {}", 999.0_f64 as u8);

    // --- f32 bit patterns ----------------------------------------------------
    println!("\n-- f32 bits --");

    let values = [0.0_f32, 1.0, -1.0, f32::INFINITY, f32::NAN];
    for v in values {
        println!("  {:>10} -> bits: 0x{:08X}", v, v.to_bits());
    }
    let bits: u32 = 0x3F80_0000;
    println!("from_bits(0x3F800000) = {}", f32::from_bits(bits));

    // --- repr(C) layout ------------------------------------------------------
    println!("\n-- repr(C) layout --");

    println!("size_of::<CAligned>() = {}", std::mem::size_of::<CAligned>());
    println!("align_of::<CAligned>() = {}", std::mem::align_of::<CAligned>());
    println!("size_of::<CPoint2>() = {}", std::mem::size_of::<CPoint2>());

    // Compute field offset via raw pointers
    let p = CPoint2 { x: 0.0, y: 0.0 };
    let base = &p as *const CPoint2 as usize;
    let y_offset = &p.y as *const f32 as usize - base;
    println!("offset_of CPoint2::y = {}", y_offset);

    // --- TryFrom<&str> -------------------------------------------------------
    println!("\n-- TryFrom<&str> parsing --");

    fn parse_pair(s: &str) -> Result<(i32, i32), String> {
        let parts: Vec<&str> = s.split(',').collect();
        match parts.as_slice() {
            [a, b] => {
                let x = a.parse::<i32>().map_err(|e| e.to_string())?;
                let y = b.parse::<i32>().map_err(|e| e.to_string())?;
                Ok((x, y))
            }
            [_] => Err("no comma".to_string()),
            _ => Err("multiple commas".to_string()),
        }
    }

    for input in ["3,4", "abc", "1,2,3", "-5,10"] {
        println!("  {:>10} -> {:?}", input, parse_pair(input));
    }
}
