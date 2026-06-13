# Advanced 07. Typed Arrays

## The problem: raw binary data

Regular arrays store arbitrary values with overhead per element. Typed arrays
give you fixed-size, contiguous, binary-layout views over raw memory —
needed for binary file formats, network protocols, audio/image data, WebGL,
etc.

## `ArrayBuffer`

A fixed-length block of raw bytes. You can't read/write it directly — you
need a **view** (a typed array or `DataView`) on top of it.

```js
const buffer = new ArrayBuffer(16); // 16 bytes, all zeroed
buffer.byteLength; // 16
```

## Typed array views

Each typed array interprets the buffer's bytes as a fixed-width numeric type:

| Type | Bytes | Range |
|------|-------|-------|
| `Int8Array` | 1 | -128 to 127 |
| `Uint8Array` | 1 | 0 to 255 |
| `Uint8ClampedArray` | 1 | 0 to 255 (clamped, not wrapped) |
| `Int16Array` | 2 | -32768 to 32767 |
| `Uint16Array` | 2 | 0 to 65535 |
| `Int32Array` | 4 | -2³¹ to 2³¹-1 |
| `Uint32Array` | 4 | 0 to 2³²-1 |
| `Float32Array` | 4 | IEEE 754 single |
| `Float64Array` | 8 | IEEE 754 double |
| `BigInt64Array` | 8 | `BigInt` signed 64-bit |
| `BigUint64Array` | 8 | `BigInt` unsigned 64-bit |

### Constructor forms — easy to confuse

```js
new Int32Array(4);              // NEW buffer, length 4 (16 bytes), zero-filled
new Int32Array([1, 2, 3]);      // NEW buffer, from array-like/iterable
new Int32Array(buffer);         // VIEW over existing buffer, from byte 0
new Int32Array(buffer, 8);      // VIEW starting at byte offset 8
new Int32Array(buffer, 8, 2);   // VIEW: byte offset 8, 2 elements (8 bytes)
```

A number argument means "allocate"; an `ArrayBuffer` argument means "view
onto existing memory". `byteOffset` for a typed-array view must be a multiple
of `BYTES_PER_ELEMENT` (`DataView` has no such restriction).

### Properties

```js
const ta = new Int32Array(buffer, 4, 2);
ta.buffer;            // the underlying ArrayBuffer
ta.byteOffset;        // 4
ta.byteLength;        // 8 (2 elements * 4 bytes)
ta.length;            // 2 (elements)
Int32Array.BYTES_PER_ELEMENT; // 4
```

## Multiple views, one buffer (shared memory)

Views over the same buffer see each other's writes:

```js
const buffer = new ArrayBuffer(4);
const ints = new Int32Array(buffer);
const bytes = new Uint8Array(buffer);
ints[0] = 1;
bytes; // Uint8Array(4) [1, 0, 0, 0]  <- little-endian on most platforms
```

## `.subarray()` vs `.slice()`

- **`subarray(start, end)`** — returns a new typed array **view over the same
  buffer**. Writes through it mutate the original (and vice versa).
- **`slice(start, end)`** — returns a new typed array with a **copied
  buffer**. Independent of the original.

```js
const a = new Uint8Array([1, 2, 3, 4]);
const view = a.subarray(1, 3);  // [2, 3], shares memory
const copy = a.slice(1, 3);     // [2, 3], independent copy
view[0] = 99;
a; // Uint8Array [1, 99, 3, 4] -- view mutated the original
copy[0] = 0; // does not affect `a`
```

## Overflow, wraparound, and clamping

Integer typed arrays **silently wrap** on overflow — no error:

```js
new Int8Array([200])[0];  // -56  (200 - 256)
new Uint8Array([300])[0]; // 44   (300 - 256)
```

`Uint8ClampedArray` instead **clamps** to `[0, 255]` and rounds
half-to-even ("banker's rounding"):

```js
new Uint8ClampedArray([300, -10, 127.5, 128.5, 2.5, 3.5]);
// [255, 0, 128, 128, 2, 4]
```

## `DataView` — explicit layout and endianness

`DataView` reads/writes multi-byte values at arbitrary byte offsets, with an
explicit endianness flag — essential for binary protocols where fields have
different sizes (a header with a 2-byte version + 4-byte length, say).

```js
const buffer = new ArrayBuffer(8);
const view = new DataView(buffer);
view.setUint16(0, 1, false);  // big-endian (false = big-endian)
view.setUint32(2, 1000, false);
view.getUint16(0, false); // 1
view.getUint32(2, false); // 1000
```

**Endianness**: the order bytes are stored in memory.
- **Little-endian**: least-significant byte first (x86/ARM native).
- **Big-endian**: most-significant byte first, aka **"network byte order"** —
  most binary network protocols and file formats use big-endian for
  multi-byte header fields so they're portable across architectures.

`DataView` methods take a `littleEndian` boolean (default `false` = big-endian,
opposite of most platforms' native order — easy to get backwards).

## `TextEncoder` / `TextDecoder` — strings as bytes

```js
const bytes = new TextEncoder().encode("hi"); // Uint8Array [104, 105] (UTF-8)
new TextDecoder().decode(bytes); // "hi"
```

## Bit-packing

Bitwise operators (`&`, `|`, `<<`, `>>`) work on individual numbers, but
combine with typed arrays to pack many small values densely — e.g. 8 booleans
into 1 byte:

```js
let byte = 0;
byte |= 1 << 7; // set bit 7 (MSB)
byte & (1 << 7); // test bit 7 (non-zero if set)
```

## Gotchas

- **Fixed length**: no `push`/`pop`/`splice` — typed arrays can't grow or
  shrink. Build a plain array, then `new Uint8Array(plainArray)` at the end,
  or pre-compute total length.
- **`subarray` shares memory; `slice` copies** — mixing these up causes
  either unexpected mutation-at-a-distance or unnecessary copies.
- **Silent overflow**: assigning `300` to a `Uint8Array` element gives `44`,
  not an error and not `255` — only `Uint8ClampedArray` clamps.
- **Endianness mismatches** between writer and reader corrupt multi-byte
  values silently (no error — you just get a different, wrong, number).
- **`byteOffset` alignment**: `new Int32Array(buffer, 1)` throws
  `RangeError` if `1` isn't a multiple of 4. `DataView` has no such
  restriction — use `DataView` for tightly-packed mixed-size binary formats.
- **`new TypedArray(n)` vs `new TypedArray(buffer)`**: a number allocates a
  new zero-filled buffer of that *length* (in elements); an `ArrayBuffer`
  creates a *view* over existing bytes. Passing a number where you meant an
  offset (or vice versa) is a common bug.

## Further Reading (MDN)

- [Typed arrays](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Typed_arrays)
- [`ArrayBuffer`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/ArrayBuffer)
- [`DataView`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView)
- [`TextEncoder`](https://developer.mozilla.org/en-US/docs/Web/API/TextEncoder)
- [`TextDecoder`](https://developer.mozilla.org/en-US/docs/Web/API/TextDecoder)
