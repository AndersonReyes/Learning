import { describe, test } from "node:test";
import assert from "node:assert/strict";
import {
  compareVersions,
  encodeFlags,
  decodeFlags,
  safeDivide,
  rangesOverlap,
} from "./exercise.js";

describe("compareVersions", () => {
  test("compares patch versions", () => {
    assert.equal(compareVersions("1.2.3", "1.2.4"), -1);
    assert.equal(compareVersions("1.2.4", "1.2.3"), 1);
  });

  test("compares numerically, not lexically", () => {
    assert.equal(compareVersions("1.10.0", "1.2.0"), 1);
  });

  test("treats missing parts as 0", () => {
    assert.equal(compareVersions("1.2", "1.2.0"), 0);
    assert.equal(compareVersions("1.0.0", "1.0.0.1"), -1);
  });

  test("equal versions return 0", () => {
    assert.equal(compareVersions("2.0.0", "2.0.0"), 0);
  });
});

describe("encodeFlags / decodeFlags", () => {
  test("encodeFlags packs booleans into a bitmask", () => {
    assert.equal(encodeFlags([true, false, true]), 5);
    assert.equal(encodeFlags([false, false, false]), 0);
    assert.equal(encodeFlags([true, true, true, true]), 15);
    assert.equal(encodeFlags([]), 0);
  });

  test("decodeFlags unpacks a bitmask into booleans", () => {
    assert.deepEqual(decodeFlags(5, 3), [true, false, true]);
    assert.deepEqual(decodeFlags(0, 3), [false, false, false]);
    assert.deepEqual(decodeFlags(15, 4), [true, true, true, true]);
  });

  test("decodeFlags and encodeFlags are inverses", () => {
    const flags = [true, false, false, true, true];
    assert.deepEqual(decodeFlags(encodeFlags(flags), flags.length), flags);
  });
});

describe("safeDivide", () => {
  test("divides normally", () => {
    assert.equal(safeDivide(10, 2), 5);
    assert.equal(safeDivide(-10, 2), -5);
  });

  test("returns null for division by zero", () => {
    assert.equal(safeDivide(10, 0), null);
  });

  test("returns null for 0/0 (NaN)", () => {
    assert.equal(safeDivide(0, 0), null);
  });
});

describe("rangesOverlap", () => {
  test("overlapping ranges", () => {
    assert.equal(rangesOverlap([1, 5], [4, 10]), true);
  });

  test("non-overlapping ranges", () => {
    assert.equal(rangesOverlap([1, 5], [6, 10]), false);
  });

  test("ranges touching at one point overlap", () => {
    assert.equal(rangesOverlap([1, 5], [5, 10]), true);
  });

  test("one range fully contains the other", () => {
    assert.equal(rangesOverlap([1, 10], [3, 5]), true);
  });
});
