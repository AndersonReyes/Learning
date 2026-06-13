import { describe, test } from "node:test";
import assert from "node:assert/strict";
import { partition, debounce, throttle, pipe, transduce } from "./exercise.js";

describe("partition", () => {
  test("splits matching and non-matching, preserving order", () => {
    assert.deepEqual(
      partition([1, 2, 3, 4, 5], (n) => n % 2 === 0),
      [[2, 4], [1, 3, 5]],
    );
  });

  test("empty array returns two empty arrays", () => {
    assert.deepEqual(partition([], (n) => n > 0), [[], []]);
  });

  test("all elements match", () => {
    assert.deepEqual(
      partition([2, 4, 6], (n) => n % 2 === 0),
      [[2, 4, 6], []],
    );
  });

  test("no elements match", () => {
    assert.deepEqual(
      partition([1, 3, 5], (n) => n % 2 === 0),
      [[], [1, 3, 5]],
    );
  });

  test("predicate receives index and array", () => {
    const seen = [];
    partition([10, 20, 30], (item, index, array) => {
      seen.push([item, index, array.length]);
      return index % 2 === 0;
    });
    assert.deepEqual(seen, [
      [10, 0, 3],
      [20, 1, 3],
      [30, 2, 3],
    ]);
  });
});

describe("debounce", () => {
  test("invokes fn once after delay with no further calls", (t) => {
    t.mock.timers.enable({ apis: ["setTimeout"] });
    let calls = 0;
    const debounced = debounce(() => calls++, 100);

    debounced();
    assert.equal(calls, 0);
    t.mock.timers.tick(100);
    assert.equal(calls, 1);
  });

  test("each call resets the timer", (t) => {
    t.mock.timers.enable({ apis: ["setTimeout"] });
    let calls = 0;
    const debounced = debounce(() => calls++, 100);

    debounced();
    t.mock.timers.tick(50);
    debounced(); // resets — 50ms elapsed is discarded
    t.mock.timers.tick(50);
    assert.equal(calls, 0); // only 50ms since the second call

    t.mock.timers.tick(50);
    assert.equal(calls, 1); // now 100ms since the second call
  });

  test("uses the most recent call's arguments", (t) => {
    t.mock.timers.enable({ apis: ["setTimeout"] });
    const received = [];
    const debounced = debounce((value) => received.push(value), 100);

    debounced("a");
    debounced("b");
    debounced("c");
    t.mock.timers.tick(100);

    assert.deepEqual(received, ["c"]);
  });

  test("cancel prevents a pending invocation", (t) => {
    t.mock.timers.enable({ apis: ["setTimeout"] });
    let calls = 0;
    const debounced = debounce(() => calls++, 100);

    debounced();
    debounced.cancel();
    t.mock.timers.tick(100);

    assert.equal(calls, 0);
  });

  test("cancel with no pending invocation is a no-op", (t) => {
    t.mock.timers.enable({ apis: ["setTimeout"] });
    const debounced = debounce(() => {}, 100);
    assert.doesNotThrow(() => debounced.cancel());
  });

  test("can be called again after firing", (t) => {
    t.mock.timers.enable({ apis: ["setTimeout"] });
    let calls = 0;
    const debounced = debounce(() => calls++, 100);

    debounced();
    t.mock.timers.tick(100);
    assert.equal(calls, 1);

    debounced();
    t.mock.timers.tick(100);
    assert.equal(calls, 2);
  });
});

describe("throttle", () => {
  test("invokes fn immediately on the first call", (t) => {
    t.mock.timers.enable({ apis: ["setTimeout"] });
    let calls = 0;
    const throttled = throttle(() => calls++, 100);

    throttled();
    assert.equal(calls, 1);
  });

  test("ignores calls within the interval, then fires trailing call with latest args", (t) => {
    t.mock.timers.enable({ apis: ["setTimeout"] });
    const received = [];
    const throttled = throttle((value) => received.push(value), 100);

    throttled("a"); // immediate
    t.mock.timers.tick(30);
    throttled("b"); // within interval — suppressed
    t.mock.timers.tick(30);
    throttled("c"); // within interval — suppressed, overwrites "b"
    assert.deepEqual(received, ["a"]);

    t.mock.timers.tick(40); // total 100ms since "a" -> trailing call fires
    assert.deepEqual(received, ["a", "c"]);
  });

  test("no trailing call if no calls happened during the interval", (t) => {
    t.mock.timers.enable({ apis: ["setTimeout"] });
    let calls = 0;
    const throttled = throttle(() => calls++, 100);

    throttled();
    assert.equal(calls, 1);
    t.mock.timers.tick(100);
    assert.equal(calls, 1); // no further calls were made — no trailing invocation
  });

  test("a call after the interval (with no calls during it) fires immediately again", (t) => {
    t.mock.timers.enable({ apis: ["setTimeout"] });
    const received = [];
    const throttled = throttle((value) => received.push(value), 100);

    throttled("a");
    t.mock.timers.tick(100);
    throttled("b"); // new interval, no pending trailing call -> immediate
    assert.deepEqual(received, ["a", "b"]);
  });

  test("trailing call restarts the interval", (t) => {
    t.mock.timers.enable({ apis: ["setTimeout"] });
    const received = [];
    const throttled = throttle((value) => received.push(value), 100);

    throttled("a"); // immediate, t=0
    throttled("b"); // pending
    t.mock.timers.tick(100); // trailing "b" fires at t=100, restarts interval
    assert.deepEqual(received, ["a", "b"]);

    throttled("c"); // within new interval (t=100..200) — suppressed
    t.mock.timers.tick(100); // trailing "c" fires at t=200
    assert.deepEqual(received, ["a", "b", "c"]);
  });
});

describe("pipe", () => {
  const double = (x) => x * 2;
  const increment = (x) => x + 1;

  test("applies functions left-to-right", () => {
    assert.equal(pipe(double, increment)(3), 7); // increment(double(3)) = increment(6) = 7
    assert.equal(pipe(increment, double)(3), 8); // double(increment(3)) = double(4) = 8
  });

  test("a single function behaves like that function", () => {
    assert.equal(pipe(double)(5), 10);
  });

  test("no functions returns the input unchanged (identity)", () => {
    assert.equal(pipe()(5), 5);
    assert.equal(pipe()("hello"), "hello");
  });

  test("only the first function receives multiple arguments", () => {
    const add = (a, b) => a + b;
    const square = (x) => x * x;
    assert.equal(pipe(add, square)(2, 3), 25); // square(add(2, 3)) = square(5) = 25
  });
});

describe("transduce", () => {
  test("maps then filters in a single pass", () => {
    const result = transduce(
      [
        { type: "map", fn: (x) => x * 2 },
        { type: "filter", fn: (x) => x > 5 },
      ],
      (acc, x) => acc + x,
      0,
      [1, 2, 3, 4],
    );
    // map: [2, 4, 6, 8] -> filter >5: [6, 8] -> sum: 14
    assert.equal(result, 14);
  });

  test("with no transformers, reduces the array as-is", () => {
    assert.equal(transduce([], (acc, x) => acc + x, 0, [1, 2, 3]), 6);
  });

  test("filter before map", () => {
    const result = transduce(
      [
        { type: "filter", fn: (x) => x % 2 === 0 },
        { type: "map", fn: (x) => x * 10 },
      ],
      (acc, x) => acc + x,
      0,
      [1, 2, 3, 4, 5, 6],
    );
    // filter evens: [2, 4, 6] -> map *10: [20, 40, 60] -> sum: 120
    assert.equal(result, 120);
  });

  test("collects into an array via reducerFn", () => {
    const result = transduce(
      [
        { type: "map", fn: (x) => x * x },
        { type: "filter", fn: (x) => x % 2 === 0 },
      ],
      (acc, x) => [...acc, x],
      [],
      [1, 2, 3, 4, 5],
    );
    // map squares: [1, 4, 9, 16, 25] -> filter even: [4, 16]
    assert.deepEqual(result, [4, 16]);
  });

  test("empty input array returns initialValue", () => {
    const result = transduce(
      [{ type: "map", fn: (x) => x * 2 }],
      (acc, x) => acc + x,
      100,
      [],
    );
    assert.equal(result, 100);
  });

  test("multiple maps and filters interleaved", () => {
    const result = transduce(
      [
        { type: "map", fn: (x) => x + 1 },
        { type: "filter", fn: (x) => x % 2 === 0 },
        { type: "map", fn: (x) => x * 100 },
        { type: "filter", fn: (x) => x <= 400 },
      ],
      (acc, x) => acc + x,
      0,
      [1, 2, 3, 4, 5],
    );
    // +1: [2,3,4,5,6] -> even: [2,4,6] -> *100: [200,400,600] -> <=400: [200,400] -> sum: 600
    assert.equal(result, 600);
  });
});
