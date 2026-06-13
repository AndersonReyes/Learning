import { describe, test } from "node:test";
import assert from "node:assert/strict";
import {
  preciseTypeOf,
  deepEqual,
  countTruthy,
  safeNumberSum,
  cloneDeep,
} from "./exercise.js";

describe("preciseTypeOf", () => {
  test("distinguishes arrays, null, and objects", () => {
    assert.equal(preciseTypeOf([]), "array");
    assert.equal(preciseTypeOf({}), "object");
    assert.equal(preciseTypeOf(null), "null");
  });

  test("identifies built-in objects", () => {
    assert.equal(preciseTypeOf(new Date()), "date");
    assert.equal(preciseTypeOf(/abc/), "regexp");
    assert.equal(preciseTypeOf(new Map()), "map");
    assert.equal(preciseTypeOf(new Set()), "set");
  });

  test("matches typeof for primitives and functions", () => {
    assert.equal(preciseTypeOf(42), "number");
    assert.equal(preciseTypeOf("hi"), "string");
    assert.equal(preciseTypeOf(true), "boolean");
    assert.equal(preciseTypeOf(undefined), "undefined");
    assert.equal(preciseTypeOf(() => {}), "function");
  });
});

describe("deepEqual", () => {
  test("compares primitives", () => {
    assert.equal(deepEqual(1, 1), true);
    assert.equal(deepEqual(1, "1"), false);
  });

  test("treats NaN as equal to itself", () => {
    assert.equal(deepEqual(NaN, NaN), true);
  });

  test("compares nested arrays", () => {
    assert.equal(deepEqual([1, [2, 3]], [1, [2, 3]]), true);
    assert.equal(deepEqual([1, [2, 3]], [1, [2, 4]]), false);
  });

  test("compares nested objects", () => {
    assert.equal(deepEqual({ a: 1, b: { c: 2 } }, { a: 1, b: { c: 2 } }), true);
    assert.equal(deepEqual({ a: 1, b: 2 }, { a: 1 }), false);
  });

  test("null is not equal to undefined", () => {
    assert.equal(deepEqual(null, undefined), false);
    assert.equal(deepEqual(null, null), true);
  });

  test("an array is never equal to a plain object", () => {
    assert.equal(deepEqual([1, 2], { 0: 1, 1: 2 }), false);
  });
});

describe("countTruthy", () => {
  test("counts truthy values, including empty arrays/objects", () => {
    const values = [0, 1, "", "a", null, undefined, NaN, [], {}, false, true];
    assert.equal(countTruthy(values), 5);
  });

  test("returns 0 for an all-falsy array", () => {
    assert.equal(countTruthy([0, "", null, undefined, NaN, false]), 0);
  });

  test("returns 0 for an empty array", () => {
    assert.equal(countTruthy([]), 0);
  });
});

describe("safeNumberSum", () => {
  test("sums numeric strings", () => {
    assert.equal(safeNumberSum(["1", "2", "3"]), 6);
  });

  test("coerces mixed types", () => {
    assert.equal(safeNumberSum([1, "2", true]), 4);
  });

  test("returns null if any value is non-numeric", () => {
    assert.equal(safeNumberSum(["1", "abc", "3"]), null);
  });

  test("returns 0 for an empty array", () => {
    assert.equal(safeNumberSum([]), 0);
  });
});

describe("cloneDeep", () => {
  test("returns primitives unchanged", () => {
    assert.equal(cloneDeep(42), 42);
    assert.equal(cloneDeep(null), null);
    assert.equal(cloneDeep("hi"), "hi");
  });

  test("deep clones nested arrays and objects", () => {
    const original = { a: 1, b: [1, 2, { c: 3 }] };
    const clone = cloneDeep(original);

    assert.deepEqual(clone, original);
    assert.notEqual(clone, original);
    assert.notEqual(clone.b, original.b);
    assert.notEqual(clone.b[2], original.b[2]);
  });

  test("mutating the clone does not affect the original", () => {
    const original = { a: [1, 2, 3] };
    const clone = cloneDeep(original);

    clone.a.push(4);

    assert.deepEqual(original.a, [1, 2, 3]);
    assert.deepEqual(clone.a, [1, 2, 3, 4]);
  });
});
