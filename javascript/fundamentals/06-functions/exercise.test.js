import { describe, test } from "node:test";
import assert from "node:assert/strict";
import { factorial, fibonacci, compose, curry, flattenDeep } from "./exercise.js";

describe("factorial", () => {
  test("base cases", () => {
    assert.equal(factorial(0), 1);
    assert.equal(factorial(1), 1);
  });

  test("a larger value", () => {
    assert.equal(factorial(5), 120);
  });

  test("throws for negative numbers", () => {
    assert.throws(() => factorial(-1), /negative/);
  });
});

describe("fibonacci", () => {
  test("base cases", () => {
    assert.equal(fibonacci(0), 0);
    assert.equal(fibonacci(1), 1);
  });

  test("small values", () => {
    assert.equal(fibonacci(2), 1);
    assert.equal(fibonacci(3), 2);
    assert.equal(fibonacci(5), 5);
  });

  test("a larger value", () => {
    assert.equal(fibonacci(10), 55);
  });
});

describe("compose", () => {
  const double = (x) => x * 2;
  const increment = (x) => x + 1;

  test("applies functions right-to-left", () => {
    assert.equal(compose(double, increment)(3), 8); // double(increment(3)) = double(4)
    assert.equal(compose(increment, double)(3), 7); // increment(double(3)) = increment(6)
  });

  test("a single function behaves like that function", () => {
    assert.equal(compose(double)(5), 10);
  });

  test("no functions returns the input unchanged", () => {
    assert.equal(compose()(5), 5);
  });
});

describe("curry", () => {
  const add3 = (a, b, c) => a + b + c;

  test("one argument at a time", () => {
    const curried = curry(add3);
    assert.equal(curried(1)(2)(3), 6);
  });

  test("mixed grouping of arguments", () => {
    const curried = curry(add3);
    assert.equal(curried(1, 2)(3), 6);
    assert.equal(curried(1)(2, 3), 6);
  });

  test("all arguments at once", () => {
    const curried = curry(add3);
    assert.equal(curried(1, 2, 3), 6);
  });
});

describe("flattenDeep", () => {
  test("flattens deeply nested arrays", () => {
    assert.deepEqual(flattenDeep([1, [2, [3, [4, [5]]]]]), [1, 2, 3, 4, 5]);
  });

  test("leaves already-flat arrays unchanged", () => {
    assert.deepEqual(flattenDeep([1, 2, 3]), [1, 2, 3]);
  });

  test("handles an empty array", () => {
    assert.deepEqual(flattenDeep([]), []);
  });

  test("flattens multiple nested branches", () => {
    assert.deepEqual(flattenDeep([[1, 2], [3, [4, 5]]]), [1, 2, 3, 4, 5]);
  });
});
