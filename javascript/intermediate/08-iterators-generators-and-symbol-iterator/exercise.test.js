import { describe, test } from "node:test";
import assert from "node:assert/strict";
import {
  Range,
  naturalNumbers,
  take,
  chunkIterable,
  zipIterables,
} from "./exercise.js";

describe("Range", () => {
  test("ascending range with default step", () => {
    assert.deepEqual([...new Range(0, 5)], [0, 1, 2, 3, 4]);
  });

  test("ascending range with explicit step, end exclusive", () => {
    assert.deepEqual([...new Range(0, 10, 2)], [0, 2, 4, 6, 8]);
  });

  test("descending range with negative step", () => {
    assert.deepEqual([...new Range(5, 0, -1)], [5, 4, 3, 2, 1]);
  });

  test("empty range when start === end", () => {
    assert.deepEqual([...new Range(0, 0)], []);
  });

  test("empty range when step direction disagrees with start/end", () => {
    assert.deepEqual([...new Range(0, 5, -1)], []);
    assert.deepEqual([...new Range(5, 0, 1)], []);
  });

  test("throws when step is 0", () => {
    assert.throws(() => new Range(0, 5, 0));
  });

  test("is iterable multiple times independently", () => {
    const r = new Range(0, 3);
    assert.deepEqual([...r], [0, 1, 2]);
    assert.deepEqual([...r], [0, 1, 2]);
  });

  test("two separate for...of loops over the same instance both see the full sequence", () => {
    const r = new Range(0, 3);
    const first = [];
    const second = [];
    for (const n of r) first.push(n);
    for (const n of r) second.push(n);
    assert.deepEqual(first, [0, 1, 2]);
    assert.deepEqual(second, [0, 1, 2]);
  });

  test("works with Array.from and destructuring", () => {
    assert.deepEqual(Array.from(new Range(1, 4)), [1, 2, 3]);
    const [a, b] = new Range(10, 13);
    assert.equal(a, 10);
    assert.equal(b, 11);
  });

  test("fractional step", () => {
    const result = [...new Range(0, 1, 0.5)];
    assert.equal(result.length, 2);
    assert.equal(result[0], 0);
    assert.equal(result[1], 0.5);
  });
});

describe("naturalNumbers", () => {
  test("yields 1, 2, 3, ... and never completes", () => {
    const gen = naturalNumbers();
    assert.deepEqual(gen.next(), { value: 1, done: false });
    assert.deepEqual(gen.next(), { value: 2, done: false });
    assert.deepEqual(gen.next(), { value: 3, done: false });
    assert.deepEqual(gen.next(), { value: 4, done: false });
  });

  test("each call returns a fresh generator starting at 1", () => {
    const a = naturalNumbers();
    const b = naturalNumbers();
    a.next();
    a.next();
    assert.equal(b.next().value, 1);
  });
});

describe("take", () => {
  test("takes n items from an infinite iterable", () => {
    assert.deepEqual([...take(naturalNumbers(), 5)], [1, 2, 3, 4, 5]);
  });

  test("takes all items when iterable has fewer than n", () => {
    assert.deepEqual([...take([1, 2], 5)], [1, 2]);
  });

  test("takes exactly n items when iterable has exactly n", () => {
    assert.deepEqual([...take([1, 2, 3], 3)], [1, 2, 3]);
  });

  test("n <= 0 yields nothing", () => {
    assert.deepEqual([...take([1, 2, 3], 0)], []);
    assert.deepEqual([...take([1, 2, 3], -1)], []);
  });

  test("does not over-consume the underlying iterable (n+1th item never pulled)", () => {
    let pulls = 0;
    function* counting() {
      while (true) {
        pulls++;
        yield pulls;
      }
    }
    const result = [...take(counting(), 3)];
    assert.deepEqual(result, [1, 2, 3]);
    assert.equal(pulls, 3);
  });

  test("n <= 0 does not pull from the iterable at all", () => {
    let pulls = 0;
    function* counting() {
      while (true) {
        pulls++;
        yield pulls;
      }
    }
    assert.deepEqual([...take(counting(), 0)], []);
    assert.equal(pulls, 0);
  });

  test("works with a plain array as the iterable", () => {
    assert.deepEqual([...take(new Range(0, 100), 3)], [0, 1, 2]);
  });
});

describe("chunkIterable", () => {
  test("splits an array into chunks of the given size", () => {
    assert.deepEqual([...chunkIterable([1, 2, 3, 4, 5], 2)], [
      [1, 2],
      [3, 4],
      [5],
    ]);
  });

  test("empty iterable yields no chunks", () => {
    assert.deepEqual([...chunkIterable([], 3)], []);
  });

  test("size larger than iterable length yields one short chunk", () => {
    assert.deepEqual([...chunkIterable([1, 2], 5)], [[1, 2]]);
  });

  test("size exactly divides the iterable length", () => {
    assert.deepEqual([...chunkIterable([1, 2, 3, 4], 2)], [
      [1, 2],
      [3, 4],
    ]);
  });

  test("throws when size <= 0", () => {
    assert.throws(() => [...chunkIterable([1, 2, 3], 0)]);
    assert.throws(() => [...chunkIterable([1, 2, 3], -1)]);
  });

  test("works lazily with take over an infinite source", () => {
    assert.deepEqual([...take(chunkIterable(naturalNumbers(), 2), 3)], [
      [1, 2],
      [3, 4],
      [5, 6],
    ]);
  });
});

describe("zipIterables", () => {
  test("zips two equal-length arrays", () => {
    assert.deepEqual(
      [...zipIterables([1, 2, 3], ["a", "b", "c"])],
      [
        [1, "a"],
        [2, "b"],
        [3, "c"],
      ],
    );
  });

  test("stops at the shortest iterable", () => {
    assert.deepEqual(
      [...zipIterables([1, 2], ["a", "b", "c"])],
      [
        [1, "a"],
        [2, "b"],
      ],
    );
  });

  test("supports more than two iterables", () => {
    assert.deepEqual(
      [...zipIterables([1, 2], ["a", "b", "c"], [true, false])],
      [
        [1, "a", true],
        [2, "b", false],
      ],
    );
  });

  test("works with an infinite iterable as one of the inputs without hanging", () => {
    assert.deepEqual(
      [...zipIterables(["a", "b", "c"], naturalNumbers())],
      [
        ["a", 1],
        ["b", 2],
        ["c", 3],
      ],
    );
  });

  test("no iterables yields nothing", () => {
    assert.deepEqual([...zipIterables()], []);
  });

  test("single iterable yields single-element arrays", () => {
    assert.deepEqual([...zipIterables([1, 2, 3])], [[1], [2], [3]]);
  });

  test("any empty iterable produces an empty result", () => {
    assert.deepEqual([...zipIterables([], naturalNumbers())], []);
  });
});
