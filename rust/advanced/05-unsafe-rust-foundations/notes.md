# Unsafe Rust Foundations

Book ch. 20.1; Nomicon "Meet Safe and Unsafe", "Working with Unsafe".

---

## What `unsafe` does

Safe Rust provides strong guarantees — no data races, no dangling pointers,
no undefined behaviour. `unsafe` blocks and functions tell the compiler "I
take responsibility for these specific operations that the borrow checker
can't verify."

`unsafe` unlocks five capabilities:
1. **Dereference raw pointers** (`*const T`, `*mut T`)
2. **Call unsafe functions** (including `extern "C"` FFI)
3. **Access or modify mutable static variables**
4. **Implement unsafe traits** (`Send`, `Sync`, etc.)
5. **Access fields of `union`s**

Everything else — type safety, memory alignment, integer overflow in debug
mode — remains enforced as usual.

## Raw pointers

Raw pointers are created with `as *const T` / `as *mut T`. Unlike references:
- May be null, dangling, or misaligned
- May alias each other freely
- Not subject to borrow-checker lifetime rules
- Can be created in safe code; only *dereferencing* needs `unsafe`

```rust
let x = 42_i32;
let p: *const i32 = &x as *const i32;
unsafe { println!("{}", *p); }  // OK — p points at live stack data

let null: *const i32 = std::ptr::null();
// unsafe { *null }  // UB: null dereference
```

### `*mut T` and aliasing

Creating two `*mut T` pointing at the same location is safe; reading through
both simultaneously in ways that would be a data race is UB:

```rust
let mut v = 5_i32;
let a = &mut v as *mut i32;
let b = a;  // copy of the pointer — fine
unsafe {
    *a = 10;
    println!("{}", *b);  // 10 — same address
}
```

## Unsafe functions and blocks

Mark a function `unsafe fn` when the caller must uphold invariants the
compiler can't verify. Call it only inside `unsafe { ... }`:

```rust
unsafe fn dangerous_add(p: *mut i32, v: i32) {
    *p += v;
}

let mut x = 1;
unsafe { dangerous_add(&mut x as *mut i32, 41); }
assert_eq!(x, 42);
```

Safe wrapper pattern — expose a safe API while encapsulating the unsafety:

```rust
pub fn swap<T>(a: &mut T, b: &mut T) {
    unsafe { std::ptr::swap(a, b) }  // ptr::swap is safe *to call here*
}
```

## `std::ptr` utilities

```rust
ptr::read(src: *const T) -> T       // copies value out without moving src
ptr::write(dst: *mut T, val: T)     // writes val to dst without dropping old
ptr::swap(a: *mut T, b: *mut T)     // swaps two locations
ptr::copy(src, dst, count)          // memmove: overlapping-safe
ptr::copy_nonoverlapping(src,dst,n) // memcpy: src and dst must not overlap
ptr::null() / ptr::null_mut()       // typed null pointers
```

## `std::slice::from_raw_parts`

Reconstructs a `&[T]` from a raw pointer and a length:

```rust
unsafe fn first_two(p: *const i32, len: usize) -> &[i32] {
    // SAFETY: caller guarantees p is valid and len <= actual length
    std::slice::from_raw_parts(p, len.min(2))
}
```

Must satisfy: `p` is non-null, aligned, and valid for `len` elements for
the duration of the returned slice's lifetime.

## Extern functions (FFI)

```rust
extern "C" {
    fn abs(x: i32) -> i32;
}
unsafe { println!("{}", abs(-5)); }
```

Calling C functions is unsafe because Rust can't verify their contracts.

## Common UB to avoid

- Dereference null or dangling pointer → UB
- Read uninitialized memory → UB
- Violate pointer aliasing rules (two live `&mut` to same data) → UB
- Call a function through a pointer with wrong type → UB
- Integer overflow in `release` mode (use `wrapping_*` / `checked_*` instead)

## Further Reading

- [Book ch. 20.1 — Unsafe Rust](https://doc.rust-lang.org/book/ch20-01-unsafe-rust.html)
- [Nomicon — Meet Safe and Unsafe](https://doc.rust-lang.org/nomicon/meet-safe-and-unsafe.html)
- [Nomicon — Working with Unsafe](https://doc.rust-lang.org/nomicon/working-with-unsafe.html)
- [`std::ptr`](https://doc.rust-lang.org/std/ptr/index.html)
- [`std::slice::from_raw_parts`](https://doc.rust-lang.org/std/slice/fn.from_raw_parts.html)
