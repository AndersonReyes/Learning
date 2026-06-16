//! Run with: `cargo run --example examples -p advanced-05-unsafe-rust-foundations`

fn main() {
    // --- raw pointers --------------------------------------------------------
    println!("-- raw pointers --");

    let x = 42_i32;
    let p: *const i32 = &x as *const i32;
    let q: *const i32 = &x as *const i32;
    // Two raw pointers to the same location — allowed
    unsafe {
        println!("*p = {}, *q = {}", *p, *q);
        println!("p == q: {}", p == q);
    }

    let null: *const i32 = std::ptr::null();
    println!("null is_null: {}", null.is_null());

    // --- mutable raw pointer ------------------------------------------------
    println!("\n-- mutable raw pointer --");

    let mut v = [1_i32, 2, 3, 4, 5];
    let ptr: *mut i32 = v.as_mut_ptr();
    unsafe {
        // Pointer arithmetic: write to v[2] directly
        *ptr.add(2) = 99;
    }
    println!("v after *ptr.add(2) = 99: {:?}", v);

    // --- ptr::swap, ptr::copy -----------------------------------------------
    println!("\n-- ptr utilities --");

    let mut a = 10_i32;
    let mut b = 20_i32;
    unsafe { std::ptr::swap(&mut a as *mut i32, &mut b as *mut i32); }
    println!("after swap: a={}, b={}", a, b);

    let src = [1_i64, 2, 3];
    let mut dst = [0_i64; 3];
    unsafe {
        std::ptr::copy_nonoverlapping(src.as_ptr(), dst.as_mut_ptr(), src.len());
    }
    println!("copy_nonoverlapping dst: {:?}", dst);

    // --- from_raw_parts ------------------------------------------------------
    println!("\n-- from_raw_parts --");

    let data = [10_i32, 20, 30, 40, 50];
    let first_three: &[i32] = unsafe {
        std::slice::from_raw_parts(data.as_ptr(), 3)
    };
    println!("first_three: {:?}", first_three);

    // --- little-endian read --------------------------------------------------
    println!("\n-- little-endian byte read --");

    let bytes = [0x78_u8, 0x56, 0x34, 0x12];
    let value = u32::from_le_bytes(bytes);
    println!("0x{:08X} = {}", value, value);

    // Manual byte-by-byte
    unsafe {
        let p = bytes.as_ptr();
        let manual = (*p as u32)
            | ((*p.add(1) as u32) << 8)
            | ((*p.add(2) as u32) << 16)
            | ((*p.add(3) as u32) << 24);
        println!("manual read: 0x{:08X}", manual);
    }

    // --- unsafe fn with safe wrapper -----------------------------------------
    println!("\n-- safe wrapper pattern --");

    // Safe wrapper: encapsulates unsafe inside a function with a safe signature
    fn safe_fill(slice: &mut [i32], val: i32) {
        // SAFETY: ptr valid for slice.len() writes, values are i32
        unsafe {
            let p = slice.as_mut_ptr();
            for i in 0..slice.len() {
                std::ptr::write(p.add(i), val);
            }
        }
    }

    let mut buf = [0_i32; 5];
    safe_fill(&mut buf, 7);
    println!("safe_fill(&mut buf, 7): {:?}", buf);
}
