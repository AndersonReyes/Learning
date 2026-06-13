// Run with: node examples.js

// --- ArrayBuffer + multiple views sharing memory ---
console.log("=== Shared buffer, multiple views ===");
{
  const buffer = new ArrayBuffer(4);
  const ints = new Int32Array(buffer);
  const bytes = new Uint8Array(buffer);

  ints[0] = 1;
  console.log("  Int32Array:", [...ints]); // [1]
  console.log("  Uint8Array:", [...bytes]); // [1, 0, 0, 0] -- little-endian on this platform

  bytes[0] = 0xff;
  console.log("  after bytes[0] = 0xff, Int32Array:", [...ints]); // [255]
}

// --- subarray (view, shares memory) vs slice (copy) ---
console.log("\n=== subarray vs slice ===");
{
  const a = new Uint8Array([1, 2, 3, 4]);
  const view = a.subarray(1, 3); // shares memory with `a`
  const copy = a.slice(1, 3); // independent copy

  view[0] = 99;
  console.log("  after view[0] = 99:");
  console.log("    a:", [...a]); // [1, 99, 3, 4] -- mutated through the view
  console.log("    view:", [...view]); // [99, 3]
  console.log("    copy (unaffected):", [...copy]); // [2, 3]
}

// --- Overflow, wraparound, and clamping ---
console.log("\n=== Overflow / clamping ===");
{
  console.log("  Int8Array([200])[0]:", new Int8Array([200])[0]); // -56 (wraps)
  console.log("  Uint8Array([300])[0]:", new Uint8Array([300])[0]); // 44 (wraps)
  console.log(
    "  Uint8ClampedArray([300, -10, 127.5, 128.5]):",
    [...new Uint8ClampedArray([300, -10, 127.5, 128.5])],
  ); // [255, 0, 128, 128] -- clamped + rounded half-to-even
}

// --- DataView: explicit layout and endianness ---
console.log("\n=== DataView + endianness ===");
{
  const buffer = new ArrayBuffer(8);
  const view = new DataView(buffer);

  view.setUint16(0, 1, false); // big-endian
  view.setUint32(2, 1000, false); // big-endian

  console.log("  bytes:", [...new Uint8Array(buffer)]); // [0, 1, 0, 0, 3, 232, 0, 0]
  console.log("  getUint16(0, big-endian):", view.getUint16(0, false)); // 1
  console.log("  getUint16(0, little-endian):", view.getUint16(0, true)); // 256
  console.log("  getUint32(2, big-endian):", view.getUint32(2, false)); // 1000
}

// --- TextEncoder / TextDecoder ---
console.log("\n=== TextEncoder / TextDecoder ===");
{
  const bytes = new TextEncoder().encode("hi é"); // "hi é"
  console.log("  encoded:", [...bytes]); // UTF-8 bytes, é = 2 bytes (0xC3 0xA9)
  console.log("  decoded:", new TextDecoder().decode(bytes)); // "hi é"
}

// --- Bit-packing with bitwise operators ---
console.log("\n=== Bit-packing ===");
{
  let flags = 0;
  flags |= 1 << 0; // set bit 0
  flags |= 1 << 3; // set bit 3
  console.log("  flags after setting bits 0 and 3:", flags.toString(2).padStart(8, "0")); // 00001001
  console.log("  bit 3 set?", (flags & (1 << 3)) !== 0); // true
  console.log("  bit 1 set?", (flags & (1 << 1)) !== 0); // false
}
