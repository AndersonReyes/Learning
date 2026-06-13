import { describe, test } from "node:test";
import assert from "node:assert/strict";
import {
  deepMerge,
  getPath,
  setPath,
  deepFreeze,
  groupByTypeofValue,
} from "./exercise.js";

describe("deepMerge", () => {
  test("recursively merges nested objects", () => {
    assert.deepEqual(
      deepMerge({ a: 1, b: { x: 1, y: 2 } }, { b: { y: 3, z: 4 }, c: 5 }),
      { a: 1, b: { x: 1, y: 3, z: 4 }, c: 5 },
    );
  });

  test("arrays in source replace arrays in target wholesale", () => {
    assert.deepEqual(deepMerge({ a: [1, 2] }, { a: [3, 4, 5] }), { a: [3, 4, 5] });
  });

  test("handles an empty target", () => {
    assert.deepEqual(deepMerge({}, { a: 1 }), { a: 1 });
  });

  test("does not mutate either input", () => {
    const target = { a: 1, b: { x: 1 } };
    const source = { b: { y: 2 } };
    deepMerge(target, source);
    assert.deepEqual(target, { a: 1, b: { x: 1 } });
    assert.deepEqual(source, { b: { y: 2 } });
  });
});

describe("getPath", () => {
  test("reads a deeply nested value", () => {
    assert.equal(getPath({ a: { b: { c: 42 } } }, "a.b.c"), 42);
  });

  test("returns undefined when an intermediate key is missing", () => {
    assert.equal(getPath({ a: { b: { c: 42 } } }, "a.x.c"), undefined);
  });

  test("works with a single-segment path", () => {
    assert.equal(getPath({ a: 1 }, "a"), 1);
  });

  test("returns undefined for a missing path on an empty object", () => {
    assert.equal(getPath({}, "a.b"), undefined);
  });
});

describe("setPath", () => {
  test("adds a new key alongside existing ones", () => {
    assert.deepEqual(setPath({ a: { b: 1 } }, "a.c", 2), { a: { b: 1, c: 2 } });
  });

  test("creates intermediate objects as needed", () => {
    assert.deepEqual(setPath({}, "a.b.c", 42), { a: { b: { c: 42 } } });
  });

  test("overwrites an existing deep value without mutating the original", () => {
    const original = { a: { b: { c: 1 } } };
    const result = setPath(original, "a.b.c", 99);
    assert.deepEqual(result, { a: { b: { c: 99 } } });
    assert.deepEqual(original, { a: { b: { c: 1 } } });
  });
});

describe("deepFreeze", () => {
  test("freezes the top-level object and returns the same reference", () => {
    const obj = { a: 1, b: { c: 2 } };
    const frozen = deepFreeze(obj);
    assert.equal(frozen, obj);
    assert.equal(Object.isFrozen(frozen), true);
  });

  test("recursively freezes nested objects", () => {
    const frozen = deepFreeze({ a: 1, b: { c: 2 } });
    assert.equal(Object.isFrozen(frozen.b), true);
  });

  test("mutating a top-level property throws", () => {
    const frozen = deepFreeze({ a: 1 });
    assert.throws(() => {
      frozen.a = 100;
    }, TypeError);
  });

  test("mutating a nested property throws", () => {
    const frozen = deepFreeze({ a: 1, b: { c: 2 } });
    assert.throws(() => {
      frozen.b.c = 99;
    }, TypeError);
  });
});

describe("groupByTypeofValue", () => {
  test("groups keys by the typeof their value", () => {
    assert.deepEqual(groupByTypeofValue({ a: 1, b: "x", c: true, d: 2, e: "y" }), {
      number: ["a", "d"],
      string: ["b", "e"],
      boolean: ["c"],
    });
  });

  test("handles an empty object", () => {
    assert.deepEqual(groupByTypeofValue({}), {});
  });
});
