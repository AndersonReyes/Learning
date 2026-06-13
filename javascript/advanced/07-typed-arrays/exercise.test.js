import { test, describe } from "node:test";
import assert from "node:assert/strict";
import {
  concatBytes,
  packBits,
  unpackBits,
  readPacketHeader,
  writePacketHeader,
} from "./exercise.js";

describe("concatBytes", () => {
  test("concatenating an empty array of chunks returns an empty Uint8Array", () => {
    const result = concatBytes([]);
    assert.equal(result.length, 0);
    assert.ok(result instanceof Uint8Array);
  });

  test("concatenates multiple chunks, including empty ones, preserving order", () => {
    const result = concatBytes([
      new Uint8Array([1, 2]),
      new Uint8Array([]),
      new Uint8Array([3]),
      new Uint8Array([4, 5, 6]),
    ]);
    assert.deepEqual([...result], [1, 2, 3, 4, 5, 6]);
  });

  test("result is an independent copy, not a view over the input chunks", () => {
    const chunk = new Uint8Array([1, 2, 3]);
    const result = concatBytes([chunk]);
    chunk[0] = 99;
    assert.deepEqual([...result], [1, 2, 3]);
  });
});

describe("packBits", () => {
  test("packs an empty array into an empty Uint8Array", () => {
    const result = packBits([]);
    assert.equal(result.length, 0);
  });

  test("packs exactly 8 booleans into 1 byte with no padding", () => {
    assert.deepEqual(
      [...packBits([true, true, true, true, true, true, true, true])],
      [0b11111111],
    );
    assert.deepEqual(
      [...packBits([false, false, false, false, false, false, false, false])],
      [0b00000000],
    );
  });

  test("packs a single boolean into the most significant bit", () => {
    assert.deepEqual([...packBits([true])], [0b10000000]);
    assert.deepEqual([...packBits([false])], [0b00000000]);
  });

  test("packs 9 booleans into 2 bytes, padding the last byte with zeros", () => {
    const bools = [true, false, true, true, false, false, false, false, true];
    assert.deepEqual([...packBits(bools)], [0b10110000, 0b10000000]);
  });

  test("packs 3 booleans into the top 3 bits of 1 byte", () => {
    assert.deepEqual([...packBits([true, false, true])], [0b10100000]);
  });
});

describe("unpackBits", () => {
  test("unpacking 0 bits returns an empty array", () => {
    assert.deepEqual(unpackBits(new Uint8Array([]), 0), []);
    assert.deepEqual(unpackBits(new Uint8Array([255]), 0), []);
  });

  test("unpacks bits MSB-first from a single byte", () => {
    assert.deepEqual(unpackBits(new Uint8Array([0b10100000]), 3), [true, false, true]);
  });

  test("unpacks 9 bits across 2 bytes", () => {
    const bytes = new Uint8Array([0b10110000, 0b10000000]);
    assert.deepEqual(unpackBits(bytes, 9), [
      true, false, true, true, false, false, false, false, true,
    ]);
  });

  test("round-trips with packBits for various lengths", () => {
    for (const length of [0, 1, 7, 8, 9, 16, 17]) {
      const bools = Array.from({ length }, (_, i) => i % 3 === 0);
      assert.deepEqual(unpackBits(packBits(bools), length), bools);
    }
  });
});

describe("readPacketHeader", () => {
  test("parses a valid 12-byte header", () => {
    const buffer = new ArrayBuffer(12);
    const bytes = new Uint8Array(buffer);
    bytes.set([0x50, 0x4b, 0x54, 0x31], 0); // "PKT1"
    const view = new DataView(buffer);
    view.setUint16(4, 7, false);
    view.setUint16(6, 3, false);
    view.setUint32(8, 1024, false);

    assert.deepEqual(readPacketHeader(buffer), {
      magic: "PKT1",
      version: 7,
      flags: 3,
      payloadLength: 1024,
    });
  });

  test("throws if the buffer is shorter than 12 bytes", () => {
    const buffer = new ArrayBuffer(8);
    assert.throws(() => readPacketHeader(buffer), /12 bytes|too short/i);
  });

  test('throws if the magic bytes aren\'t "PKT1"', () => {
    const buffer = new ArrayBuffer(12);
    const bytes = new Uint8Array(buffer);
    bytes.set([0, 0, 0, 0], 0);
    assert.throws(() => readPacketHeader(buffer), /magic/i);
  });
});

describe("writePacketHeader", () => {
  test("writes the magic bytes and big-endian fields", () => {
    const buffer = writePacketHeader({ version: 1, flags: 2, payloadLength: 256 });
    assert.deepEqual(
      [...new Uint8Array(buffer)],
      [
        0x50, 0x4b, 0x54, 0x31, // "PKT1"
        0, 1, // version = 1
        0, 2, // flags = 2
        0, 0, 1, 0, // payloadLength = 256
      ],
    );
  });

  test("produces a 12-byte buffer", () => {
    const buffer = writePacketHeader({ version: 0, flags: 0, payloadLength: 0 });
    assert.equal(buffer.byteLength, 12);
  });

  test("round-trips with readPacketHeader, including max uint16/uint32 values", () => {
    const header = { version: 65535, flags: 1, payloadLength: 4294967295 };
    assert.deepEqual(readPacketHeader(writePacketHeader(header)), {
      magic: "PKT1",
      ...header,
    });
  });
});
