# Data Layout & Type Conversions

Nomicon "Data Layout", "Type Conversions".

---

## `repr` attributes

By default Rust chooses struct layout for performance (reordering fields, adding
padding). `#[repr(...)]` pins the layout:

### `#[repr(C)]`
- Fields laid out in declaration order, with C-compatible alignment/padding.
- Required for FFI structs.
- Size = sum of field sizes + padding for alignment.

```rust
#[repr(C)]
struct Point { x: f32, y: f32 }
// sizeof(Point) = 8, alignof(Point) = 4 — same as C
```

### `#[repr(transparent)]`
- Single-field struct (or newtype) with identical ABI to the inner type.
- Lets you pass a newtype through FFI as if it were the inner type.

```rust
#[repr(transparent)]
struct Meters(f64);  // ABI-identical to f64
```

### `#[repr(packed)]`
- Removes padding (alignment = 1). Fields may be unaligned — accessing them
  via references is UB. Use only when the binary format mandates it.

### `#[repr(u8)]`, `#[repr(i32)]`, …
- For enums: fixes the discriminant size.

## `From` / `Into`

`From<T>` is the canonical infallible conversion trait. Implementing
`From<T> for U` automatically provides `Into<U> for T` (blanket impl):

```rust
impl From<i32> for f64 {
    fn from(n: i32) -> f64 { n as f64 }
}

let x: f64 = 5_i32.into();   // uses From<i32> for f64
let y = f64::from(5_i32);    // same
```

Use `From`/`Into` for:
- Unit conversions (e.g. `Meters` ↔ `Feet`)
- Cheap, always-succeeding type widening

## `TryFrom` / `TryInto`

Fallible variant — returns `Result<T, E>`:

```rust
use std::convert::TryFrom;

let small: Result<i8, _> = i8::try_from(200_i32);  // Err: 200 > i8::MAX
let ok:    Result<i8, _> = i8::try_from(42_i32);   // Ok(42)
```

Define `TryFrom<Bytes>` to parse structured binary data.

## `as` casts

Raw, unchecked numeric coercions:

```rust
let f: f64 = 3.9;
let i: i32 = f as i32;    // truncates toward zero → 3
let n: u8  = 300_u16 as u8; // wraps → 44 (300 mod 256)
let s: i8  = 200_u8 as i8;  // reinterprets bits → -56
```

Rules:
- Integer → integer: truncates high bits or sign-extends (2's complement).
- Float → integer: truncates toward zero; out-of-range saturates to min/max
  since Rust 1.45 (was UB before).
- Integer → float: may lose precision for large integers.
- `*const T as *const U` / `as usize` — pointer ↔ integer casts: unsafe-free
  syntactically, but dereferencing the result may be UB.

## Transmute (`std::mem::transmute`)

Reinterprets the bits of a value as a different type with the same size.
Extremely unsafe — use only when you know the bit patterns are valid for the
target type:

```rust
let bits: u32 = 0x3F80_0000;
let f: f32 = unsafe { std::mem::transmute(bits) };
assert_eq!(f, 1.0_f32);
```

Prefer `f32::from_bits` / `f64::from_bits` for the float case (they're safe
wrappers around `transmute`).

## Alignment and size queries

```rust
std::mem::size_of::<T>()       // size in bytes
std::mem::align_of::<T>()      // minimum alignment in bytes
std::mem::size_of_val(&x)      // size of a value (useful for DSTs)
```

## Further Reading

- [Nomicon — Data Layouts](https://doc.rust-lang.org/nomicon/repr-rust.html)
- [Nomicon — Type Conversions](https://doc.rust-lang.org/nomicon/conversions.html)
- [Reference — `repr` attribute](https://doc.rust-lang.org/reference/type-layout.html#representations)
- [`std::convert`](https://doc.rust-lang.org/std/convert/index.html)
- [`std::mem`](https://doc.rust-lang.org/std/mem/index.html)
