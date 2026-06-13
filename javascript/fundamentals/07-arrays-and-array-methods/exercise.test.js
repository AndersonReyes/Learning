import { describe, test } from "node:test";
import assert from "node:assert/strict";
import {
  groupBy,
  chunk,
  zip,
  intersection,
  sortByMultipleKeys,
} from "./exercise.js";

describe("groupBy", () => {
  test("groups numbers by even/odd", () => {
    assert.deepEqual(groupBy([1, 2, 3, 4, 5, 6], (n) => (n % 2 === 0 ? "even" : "odd")), {
      odd: [1, 3, 5],
      even: [2, 4, 6],
    });
  });

  test("groups words by first letter", () => {
    assert.deepEqual(groupBy(["apple", "banana", "avocado"], (w) => w[0]), {
      a: ["apple", "avocado"],
      b: ["banana"],
    });
  });

  test("handles an empty array", () => {
    assert.deepEqual(groupBy([], (x) => x), {});
  });
});

describe("chunk", () => {
  test("splits an array into even chunks", () => {
    assert.deepEqual(chunk([1, 2, 3, 4, 5], 2), [[1, 2], [3, 4], [5]]);
  });

  test("chunk size 1 returns single-element arrays", () => {
    assert.deepEqual(chunk([1, 2, 3], 1), [[1], [2], [3]]);
  });

  test("handles an empty array", () => {
    assert.deepEqual(chunk([], 3), []);
  });

  test("chunk size larger than the array returns one chunk", () => {
    assert.deepEqual(chunk([1, 2, 3], 5), [[1, 2, 3]]);
  });
});

describe("zip", () => {
  test("pairs up elements by index", () => {
    assert.deepEqual(zip([1, 2, 3], ["a", "b", "c"]), [[1, "a"], [2, "b"], [3, "c"]]);
  });

  test("stops at the shorter array", () => {
    assert.deepEqual(zip([1, 2], ["a", "b", "c"]), [[1, "a"], [2, "b"]]);
  });

  test("returns an empty array if either input is empty", () => {
    assert.deepEqual(zip([], ["a"]), []);
  });
});

describe("intersection", () => {
  test("returns values present in both arrays, deduplicated", () => {
    assert.deepEqual(intersection([1, 2, 2, 3], [2, 3, 4]), [2, 3]);
  });

  test("returns an empty array if there is no overlap", () => {
    assert.deepEqual(intersection([1, 2], [3, 4]), []);
  });

  test("deduplicates even when both arrays repeat the value", () => {
    assert.deepEqual(intersection([1, 1, 1], [1]), [1]);
  });
});

describe("sortByMultipleKeys", () => {
  test("sorts by last name then first name", () => {
    const people = [
      { last: "Smith", first: "Bob" },
      { last: "Adams", first: "Zoe" },
      { last: "Smith", first: "Ann" },
    ];
    assert.deepEqual(sortByMultipleKeys(people, ["last", "first"]), [
      { last: "Adams", first: "Zoe" },
      { last: "Smith", first: "Ann" },
      { last: "Smith", first: "Bob" },
    ]);
  });

  test("sorts numerically by multiple keys", () => {
    const items = [
      { a: 2, b: 1 },
      { a: 1, b: 2 },
      { a: 1, b: 1 },
    ];
    assert.deepEqual(sortByMultipleKeys(items, ["a", "b"]), [
      { a: 1, b: 1 },
      { a: 1, b: 2 },
      { a: 2, b: 1 },
    ]);
  });

  test("does not mutate the input array", () => {
    const items = [{ a: 2 }, { a: 1 }];
    const copy = [...items];
    sortByMultipleKeys(items, ["a"]);
    assert.deepEqual(items, copy);
  });
});
