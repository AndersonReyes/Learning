import { describe, test } from "node:test";
import assert from "node:assert/strict";
import {
  isPrime,
  primesUpTo,
  transpose,
  runLengthEncode,
  longestConsecutiveRun,
} from "./exercise.js";

describe("isPrime", () => {
  test("2 is prime", () => {
    assert.equal(isPrime(2), true);
  });

  test("1 and 0 are not prime", () => {
    assert.equal(isPrime(1), false);
    assert.equal(isPrime(0), false);
  });

  test("17 is prime, 18 is not", () => {
    assert.equal(isPrime(17), true);
    assert.equal(isPrime(18), false);
  });

  test("97 is prime", () => {
    assert.equal(isPrime(97), true);
  });
});

describe("primesUpTo", () => {
  test("primes up to 10", () => {
    assert.deepEqual(primesUpTo(10), [2, 3, 5, 7]);
  });

  test("primes up to 1 is empty", () => {
    assert.deepEqual(primesUpTo(1), []);
  });

  test("primes up to 2 is just [2]", () => {
    assert.deepEqual(primesUpTo(2), [2]);
  });

  test("primes up to 20", () => {
    assert.deepEqual(primesUpTo(20), [2, 3, 5, 7, 11, 13, 17, 19]);
  });
});

describe("transpose", () => {
  test("transposes a 2x3 matrix into a 3x2 matrix", () => {
    assert.deepEqual(
      transpose([[1, 2, 3], [4, 5, 6]]),
      [[1, 4], [2, 5], [3, 6]],
    );
  });

  test("transposes a square matrix", () => {
    assert.deepEqual(
      transpose([[1, 2], [3, 4]]),
      [[1, 3], [2, 4]],
    );
  });

  test("transposes a 1x1 matrix", () => {
    assert.deepEqual(transpose([[1]]), [[1]]);
  });
});

describe("runLengthEncode", () => {
  test("encodes runs of repeated characters", () => {
    assert.equal(runLengthEncode("aaabbc"), "a3b2c1");
  });

  test("encodes a string with no repeats", () => {
    assert.equal(runLengthEncode("abc"), "a1b1c1");
  });

  test("encodes an empty string", () => {
    assert.equal(runLengthEncode(""), "");
  });

  test("encodes a single run", () => {
    assert.equal(runLengthEncode("aaaa"), "a4");
  });
});

describe("longestConsecutiveRun", () => {
  test("finds the longest run", () => {
    assert.equal(longestConsecutiveRun([1, 1, 2, 2, 2, 3]), 3);
  });

  test("returns 1 when there are no repeats", () => {
    assert.equal(longestConsecutiveRun([1, 2, 3]), 1);
  });

  test("returns 0 for an empty array", () => {
    assert.equal(longestConsecutiveRun([]), 0);
  });

  test("handles an array that is entirely one run", () => {
    assert.equal(longestConsecutiveRun([5, 5, 5, 5]), 4);
  });
});
