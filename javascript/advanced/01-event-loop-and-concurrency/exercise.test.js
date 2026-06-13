import { test, describe } from "node:test";
import assert from "node:assert/strict";
import {
  debounce,
  throttle,
  asyncPool,
  retryWithBackoff,
  createMutex,
} from "./exercise.js";

const sleep = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

describe("debounce", () => {
  test("delays invocation until wait ms after the last call, using the latest args", async () => {
    const calls = [];
    const debounced = debounce((x) => calls.push(x), 30);
    debounced(1);
    await sleep(10);
    debounced(2);
    await sleep(10);
    debounced(3);
    assert.deepEqual(calls, []);
    await sleep(50);
    assert.deepEqual(calls, [3]);
  });

  test("preserves `this` context", async () => {
    const calls = [];
    const obj = {
      value: 42,
      record: debounce(function () {
        calls.push(this.value);
      }, 20),
    };
    obj.record();
    await sleep(40);
    assert.deepEqual(calls, [42]);
  });

  test("calls separated by more than `wait` each trigger fn independently", async () => {
    const calls = [];
    const debounced = debounce((x) => calls.push(x), 20);
    debounced("a");
    await sleep(40);
    debounced("b");
    await sleep(40);
    assert.deepEqual(calls, ["a", "b"]);
  });

  test("rapid repeated calls invoke fn exactly once", async () => {
    let count = 0;
    const debounced = debounce(() => {
      count++;
    }, 20);
    for (let i = 0; i < 5; i++) {
      debounced();
      await sleep(5);
    }
    await sleep(40);
    assert.equal(count, 1);
  });
});

describe("throttle", () => {
  test("invokes fn immediately on the first call (leading edge)", () => {
    const calls = [];
    const throttled = throttle((x) => calls.push(x), 30);
    throttled("a");
    assert.deepEqual(calls, ["a"]);
  });

  test("fires once more after the window with the latest args (trailing edge)", async () => {
    const calls = [];
    const throttled = throttle((x) => calls.push(x), 30);
    throttled("a");
    await sleep(5);
    throttled("b");
    await sleep(5);
    throttled("c");
    assert.deepEqual(calls, ["a"]);
    await sleep(40);
    assert.deepEqual(calls, ["a", "c"]);
  });

  test("does not fire a trailing call if no calls happened during the window", async () => {
    const calls = [];
    const throttled = throttle((x) => calls.push(x), 20);
    throttled("x");
    await sleep(50);
    assert.deepEqual(calls, ["x"]);
  });

  test("after the window closes with no pending call, the next call leads immediately", async () => {
    const calls = [];
    const throttled = throttle((x) => calls.push(x), 20);
    throttled("first");
    await sleep(30);
    throttled("second");
    assert.deepEqual(calls, ["first", "second"]);
  });
});

describe("asyncPool", () => {
  test("runs with limited concurrency and preserves result order", async () => {
    const items = [1, 2, 3, 4, 5];
    let running = 0;
    let maxRunning = 0;
    const result = await asyncPool(2, items, async (item) => {
      running++;
      maxRunning = Math.max(maxRunning, running);
      await sleep(10);
      running--;
      return item * 2;
    });
    assert.deepEqual(result, [2, 4, 6, 8, 10]);
    assert.ok(maxRunning <= 2, `expected maxRunning <= 2, got ${maxRunning}`);
  });

  test("rejects if any iteratorFn call rejects", async () => {
    await assert.rejects(
      asyncPool(2, [1, 2, 3], async (item) => {
        if (item === 2) throw new Error("boom");
        await sleep(5);
        return item;
      }),
      /boom/,
    );
  });

  test("resolves to [] for an empty items array", async () => {
    const result = await asyncPool(3, [], async (x) => x);
    assert.deepEqual(result, []);
  });

  test("works when limit exceeds items.length", async () => {
    const result = await asyncPool(10, [1, 2, 3], async (x) => x * 10);
    assert.deepEqual(result, [10, 20, 30]);
  });

  test("with limit 1, items run strictly sequentially", async () => {
    const order = [];
    await asyncPool(1, ["a", "b", "c"], async (item) => {
      order.push(`${item}-start`);
      await sleep(5);
      order.push(`${item}-end`);
      return item;
    });
    assert.deepEqual(order, [
      "a-start",
      "a-end",
      "b-start",
      "b-end",
      "c-start",
      "c-end",
    ]);
  });
});

describe("retryWithBackoff", () => {
  test("retries on failure and resolves on eventual success", async () => {
    let calls = 0;
    const fn = async () => {
      calls++;
      if (calls < 3) throw new Error(`fail ${calls}`);
      return "success";
    };
    const result = await retryWithBackoff(fn, 3, 5);
    assert.equal(result, "success");
    assert.equal(calls, 3);
  });

  test("rejects with the last error after exhausting retries", async () => {
    let calls = 0;
    const fn = async () => {
      calls++;
      throw new Error(`fail ${calls}`);
    };
    await assert.rejects(retryWithBackoff(fn, 2, 5), /fail 3/);
    assert.equal(calls, 3);
  });

  test("resolves immediately if fn succeeds on the first attempt", async () => {
    let calls = 0;
    const fn = async () => {
      calls++;
      return "ok";
    };
    const result = await retryWithBackoff(fn, 5, 5);
    assert.equal(result, "ok");
    assert.equal(calls, 1);
  });

  test("retries=0 makes exactly one attempt", async () => {
    let calls = 0;
    const fn = async () => {
      calls++;
      throw new Error("nope");
    };
    await assert.rejects(retryWithBackoff(fn, 0, 5), /nope/);
    assert.equal(calls, 1);
  });

  test("waits with exponential backoff between attempts", async () => {
    const timestamps = [];
    let calls = 0;
    const fn = async () => {
      timestamps.push(Date.now());
      calls++;
      if (calls < 3) throw new Error("fail");
      return "done";
    };
    await retryWithBackoff(fn, 3, 10);
    const gap1 = timestamps[1] - timestamps[0];
    const gap2 = timestamps[2] - timestamps[1];
    assert.ok(gap1 >= 9, `expected gap1 >= ~10ms, got ${gap1}`);
    assert.ok(gap2 >= 19, `expected gap2 >= ~20ms, got ${gap2}`);
  });
});

describe("createMutex", () => {
  test("serializes concurrent runExclusive calls in call order", async () => {
    const mutex = createMutex();
    const log = [];
    const task = (id, ms) =>
      mutex.runExclusive(async () => {
        log.push(`${id}-start`);
        await sleep(ms);
        log.push(`${id}-end`);
        return id;
      });

    const results = await Promise.all([
      task("a", 15),
      task("b", 5),
      task("c", 5),
    ]);

    assert.deepEqual(log, [
      "a-start",
      "a-end",
      "b-start",
      "b-end",
      "c-start",
      "c-end",
    ]);
    assert.deepEqual(results, ["a", "b", "c"]);
  });

  test("a rejected task does not block subsequently queued tasks", async () => {
    const mutex = createMutex();
    const log = [];

    const p1 = mutex.runExclusive(async () => {
      log.push("task1");
      throw new Error("task1 failed");
    });
    const p2 = mutex.runExclusive(async () => {
      log.push("task2");
      return "task2-result";
    });

    await assert.rejects(p1, /task1 failed/);
    assert.equal(await p2, "task2-result");
    assert.deepEqual(log, ["task1", "task2"]);
  });

  test("runExclusive resolves with fn's return value", async () => {
    const mutex = createMutex();
    const result = await mutex.runExclusive(() => 42);
    assert.equal(result, 42);
  });
});
