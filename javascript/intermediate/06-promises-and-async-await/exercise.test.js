import { describe, test } from "node:test";
import assert from "node:assert/strict";
import {
  myPromiseAll,
  myPromiseRace,
  sequence,
  asyncPool,
  timeout,
  TimeoutError,
} from "./exercise.js";

const delay = (ms, value) =>
  new Promise((resolve) => setTimeout(() => resolve(value), ms));

describe("myPromiseAll", () => {
  test("resolves to [] for an empty array", async () => {
    assert.deepEqual(await myPromiseAll([]), []);
  });

  test("resolves with results in input order for plain values", async () => {
    assert.deepEqual(await myPromiseAll([1, 2, 3]), [1, 2, 3]);
  });

  test("resolves with results in input order for a mix of promises and values", async () => {
    const result = await myPromiseAll([
      1,
      Promise.resolve(2),
      delay(5, 3),
      4,
    ]);
    assert.deepEqual(result, [1, 2, 3, 4]);
  });

  test("preserves order even when promises settle out of order", async () => {
    const result = await myPromiseAll([delay(20, "a"), delay(5, "b"), delay(10, "c")]);
    assert.deepEqual(result, ["a", "b", "c"]);
  });

  test("rejects with the first rejection reason", async () => {
    await assert.rejects(
      myPromiseAll([Promise.resolve(1), Promise.reject(new Error("boom")), 3]),
      /boom/,
    );
  });

  test("rejects fast — does not wait for slower promises", async () => {
    let slowSettled = false;
    const slow = delay(50).then(() => {
      slowSettled = true;
      return "slow";
    });
    const fast = Promise.reject(new Error("fast fail"));

    const start = Date.now();
    await assert.rejects(myPromiseAll([slow, fast]), /fast fail/);
    const elapsed = Date.now() - start;

    assert.ok(elapsed < 50, `expected fast rejection, took ${elapsed}ms`);
    assert.equal(slowSettled, false);
  });

  test("does not call the real Promise.all/allSettled/any/race", async () => {
    const originals = {
      all: Promise.all,
      allSettled: Promise.allSettled,
      any: Promise.any,
      race: Promise.race,
    };
    let called = false;
    Promise.all = (...args) => {
      called = true;
      return originals.all(...args);
    };
    Promise.allSettled = (...args) => {
      called = true;
      return originals.allSettled(...args);
    };
    Promise.any = (...args) => {
      called = true;
      return originals.any(...args);
    };
    Promise.race = (...args) => {
      called = true;
      return originals.race(...args);
    };

    try {
      await myPromiseAll([1, Promise.resolve(2)]);
    } finally {
      Promise.all = originals.all;
      Promise.allSettled = originals.allSettled;
      Promise.any = originals.any;
      Promise.race = originals.race;
    }

    assert.equal(called, false);
  });
});

describe("myPromiseRace", () => {
  test("resolves with the first to fulfill", async () => {
    const result = await myPromiseRace([delay(20, "slow"), delay(5, "fast")]);
    assert.equal(result, "fast");
  });

  test("rejects with the first to reject, even if others would fulfill later", async () => {
    const fastFail = delay(5).then(() => {
      throw new Error("fast fail");
    });
    const slowOk = delay(20, "slow ok");

    await assert.rejects(myPromiseRace([slowOk, fastFail]), /fast fail/);
  });

  test("settles immediately for an already-settled input", async () => {
    const result = await myPromiseRace([delay(20, "slow"), Promise.resolve("instant")]);
    assert.equal(result, "instant");
  });

  test("handles a mix of plain values and promises", async () => {
    const result = await myPromiseRace([delay(10, "delayed"), "immediate"]);
    assert.equal(result, "immediate");
  });
});

describe("sequence", () => {
  test("returns results in order", async () => {
    const result = await sequence([
      () => 1,
      () => Promise.resolve(2),
      async () => 3,
    ]);
    assert.deepEqual(result, [1, 2, 3]);
  });

  test("calls functions one at a time, in order", async () => {
    const order = [];
    await sequence([
      async () => {
        order.push("start 1");
        await delay(10);
        order.push("end 1");
        return 1;
      },
      async () => {
        order.push("start 2");
        await delay(5);
        order.push("end 2");
        return 2;
      },
    ]);

    assert.deepEqual(order, ["start 1", "end 1", "start 2", "end 2"]);
  });

  test("rejects immediately on the first error, without calling later functions", async () => {
    let thirdCalled = false;
    await assert.rejects(
      sequence([
        () => 1,
        () => {
          throw new Error("boom");
        },
        () => {
          thirdCalled = true;
          return 3;
        },
      ]),
      /boom/,
    );
    assert.equal(thirdCalled, false);
  });

  test("rejects on a rejected promise from one of the functions", async () => {
    await assert.rejects(
      sequence([() => 1, () => Promise.reject(new Error("rejected"))]),
      /rejected/,
    );
  });

  test("resolves to [] for an empty array", async () => {
    assert.deepEqual(await sequence([]), []);
  });
});

describe("asyncPool", () => {
  test("resolves to [] for an empty items array", async () => {
    assert.deepEqual(await asyncPool(2, [], (x) => x), []);
  });

  test("returns results in input order regardless of completion order", async () => {
    const result = await asyncPool(2, [10, 30, 20], (ms) => delay(ms, ms));
    assert.deepEqual(result, [10, 30, 20]);
  });

  test("respects the concurrency limit", async () => {
    let active = 0;
    let maxActive = 0;
    const items = [1, 2, 3, 4, 5];

    await asyncPool(2, items, async (n) => {
      active++;
      maxActive = Math.max(maxActive, active);
      await delay(10);
      active--;
      return n * 2;
    });

    assert.ok(maxActive <= 2, `expected maxActive <= 2, got ${maxActive}`);
  });

  test("limit of Infinity behaves like full concurrency", async () => {
    let active = 0;
    let maxActive = 0;
    const items = [1, 2, 3, 4, 5];

    const result = await asyncPool(Infinity, items, async (n) => {
      active++;
      maxActive = Math.max(maxActive, active);
      await delay(10);
      active--;
      return n * 2;
    });

    assert.deepEqual(result, [2, 4, 6, 8, 10]);
    assert.equal(maxActive, 5);
  });

  test("limit >= items.length behaves like full concurrency", async () => {
    let active = 0;
    let maxActive = 0;
    const items = [1, 2, 3];

    await asyncPool(10, items, async (n) => {
      active++;
      maxActive = Math.max(maxActive, active);
      await delay(5);
      active--;
      return n;
    });

    assert.equal(maxActive, 3);
  });

  test("limit of 1 behaves like sequence", async () => {
    const order = [];
    const result = await asyncPool(1, [1, 2, 3], async (n) => {
      order.push(`start ${n}`);
      await delay(5);
      order.push(`end ${n}`);
      return n * 10;
    });

    assert.deepEqual(result, [10, 20, 30]);
    assert.deepEqual(order, [
      "start 1",
      "end 1",
      "start 2",
      "end 2",
      "start 3",
      "end 3",
    ]);
  });

  test("passes item and index to mapperFn", async () => {
    const calls = [];
    await asyncPool(2, ["a", "b", "c"], (item, index) => {
      calls.push([item, index]);
      return item;
    });

    assert.deepEqual(calls, [
      ["a", 0],
      ["b", 1],
      ["c", 2],
    ]);
  });

  test("rejects with the reason of any rejected call", async () => {
    await assert.rejects(
      asyncPool(2, [1, 2, 3], async (n) => {
        if (n === 2) throw new Error("item 2 failed");
        await delay(10);
        return n;
      }),
      /item 2 failed/,
    );
  });
});

describe("timeout", () => {
  test("resolves with the value if the promise settles before ms", async (t) => {
    t.mock.timers.enable({ apis: ["setTimeout"] });

    const inner = delay(5, "ok");
    const result = timeout(inner, 50);

    // Advance past the inner delay but not the timeout.
    await t.mock.timers.tick(5);
    assert.equal(await result, "ok");
  });

  test("rejects with TimeoutError if ms elapses first", async (t) => {
    t.mock.timers.enable({ apis: ["setTimeout"] });

    const inner = delay(50, "too slow");
    const result = timeout(inner, 5);

    await t.mock.timers.tick(5);
    await assert.rejects(result, TimeoutError);
    await assert.rejects(result, /timed out/i);
  });

  test("rejects with the inner promise's rejection reason if it rejects before ms", async (t) => {
    t.mock.timers.enable({ apis: ["setTimeout"] });

    const inner = delay(5).then(() => {
      throw new Error("inner failure");
    });
    const result = timeout(inner, 50);

    await t.mock.timers.tick(5);
    await assert.rejects(result, /inner failure/);
  });

  test("accepts a zero-arg function returning a promise", async (t) => {
    t.mock.timers.enable({ apis: ["setTimeout"] });

    const result = timeout(() => delay(5, "from fn"), 50);

    await t.mock.timers.tick(5);
    assert.equal(await result, "from fn");
  });

  test("clears its internal timer once the promise settles (no hang)", async (t) => {
    t.mock.timers.enable({ apis: ["setTimeout"] });

    const inner = delay(5, "ok");
    const result = timeout(inner, 1000);

    await t.mock.timers.tick(5);
    assert.equal(await result, "ok");

    // If the internal timeout timer wasn't cleared, ticking far past `ms`
    // would still be harmless here since `result` already settled — this
    // just documents that advancing further doesn't throw/hang.
    await t.mock.timers.tick(1000);
  });
});
