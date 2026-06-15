//! Run with: `cargo run --example examples -p fundamentals-02-variables-data-types-and-functions`
//!
//! Demonstrates `notes.md`'s variables/mutability/shadowing, scalar types and
//! overflow-aware arithmetic, integer division and `as` casts, tuples and
//! arrays, and the statement-vs-expression distinction in functions.
//! Self-contained: doesn't call into this package's exercises (those are
//! unimplemented `todo!()` stubs until you finish them).

const MAX_POINTS: u32 = 100_000;

fn main() {
    // --- Variables and mutability ---

    let x = 5;
    // x = 6; // would be a compile error: `x` is immutable.
    let mut y = 5;
    y += 1;
    println!("x = {x}, y = {y}, MAX_POINTS = {MAX_POINTS}");

    // Shadowing: same name, can change type, old binding is gone (but not
    // dropped early if something else still holds it).
    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces (now a {}): {spaces}", std::any::type_name::<usize>());

    // --- Scalar types: overflow-aware arithmetic ---

    let a: u8 = 250;
    println!("a = {a}");
    println!("a.checked_add(10)    = {:?}", a.checked_add(10)); // None: would overflow
    println!("a.checked_add(5)     = {:?}", a.checked_add(5)); // Some(255)
    println!("a.wrapping_add(10)   = {}", a.wrapping_add(10)); // wraps: 260 mod 256 = 4
    println!("a.saturating_add(10) = {}", a.saturating_add(10)); // clamps to 255
    println!("a.overflowing_add(10) = {:?}", a.overflowing_add(10)); // (4, true)

    // --- Integer division and `as` casts ---

    println!("-7 / 2 = {}", -7 / 2); // truncates toward zero: -3, not -4
    println!("-7 % 2 = {}", -7 % 2); // -1

    let big: u32 = 300;
    let truncated = big as u8;
    println!("300u32 as u8 = {truncated}"); // 300 mod 256 = 44

    let small: u8 = 200;
    let widened = small as u32;
    println!("200u8 as u32 = {widened}"); // zero-extends, value unchanged

    let too_big: f64 = 1e10;
    let saturated = too_big as i32;
    println!("1e10_f64 as i32 = {saturated}"); // saturates at i32::MAX

    let not_a_number = f64::NAN as i32;
    println!("NaN as i32 = {not_a_number}"); // 0

    // --- Compound types: tuples and arrays ---

    let t: (i32, f64, u8) = (500, 6.4, 1);
    let (first, second, third) = t;
    println!("tuple: t.0={}, t.1={}, t.2={} (destructured: {first}, {second}, {third})", t.0, t.1, t.2);

    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let repeated = [0; 5];
    println!("arr = {arr:?}, repeated = {repeated:?}, arr.len() = {}", arr.len());

    // --- Functions: statements vs expressions ---

    println!("add_one(41) = {}", add_one(41));
    println!("classify_parity(7) = {}", classify_parity(7));
    println!("classify_parity(8) = {}", classify_parity(8));
}

/// Tail expression (no semicolon) is the return value.
fn add_one(x: i32) -> i32 {
    x + 1
}

/// `if`/`else` as an expression: both arms must produce the same type, and
/// the whole `if` becomes the function's return value since it's the tail
/// expression (no semicolon after the closing brace).
fn classify_parity(n: i32) -> &'static str {
    if n % 2 == 0 {
        "even"
    } else {
        "odd"
    }
}
