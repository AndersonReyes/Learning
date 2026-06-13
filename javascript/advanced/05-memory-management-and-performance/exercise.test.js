import { test, describe } from "node:test";
import assert from "node:assert/strict";
import {
  createLRUCache,
  createObjectPool,
  memoizeWithTTL,
  memoizeByReference,
  createBatcher,
} from "./exercise.js";

const sleep = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

describe("createLRUCache", () => {
  test("basic set and get", () => {
    const cache = createLRUCache(2);
    cache.set("a", 1);
    assert.equal(cache.get("a"), 1);
    assert.equal(cache.get("missing"), undefined);
  });

  test("exceeding capacity evicts the least-recently-used entry", () => {
    const cache = createLRUCache(2);
    cache.set("a", 1);
    cache.set("b", 2);
    cache.set("c", 3); // evicts "a"
    assert.equal(cache.has("a"), false);
    assert.equal(cache.has("b"), true);
    assert.equal(cache.has("c"), true);
    assert.equal(cache.size, 2);
  });

  test("get() refreshes recency, protecting an entry from eviction", () => {
    const cache = createLRUCache(2);
    cache.set("a", 1);
    cache.set("b", 2);
    cache.get("a"); // "a" is now most-recently-used; "b" is least
    cache.set("c", 3); // evicts "b"
    assert.equal(cache.has("a"), true);
    assert.equal(cache.has("b"), false);
    assert.equal(cache.has("c"), true);
  });

  test("set() on an existing key updates the value and refreshes recency", () => {
    const cache = createLRUCache(2);
    cache.set("a", 1);
    cache.set("b", 2);
    cache.set("a", 10); // updates value, "a" now most-recently-used
    cache.set("c", 3); // evicts "b"
    assert.equal(cache.has("b"), false);
    assert.equal(cache.get("a"), 10);
  });

  test("has() does not affect recency order", () => {
    const cache = createLRUCache(2);
    cache.set("a", 1);
    cache.set("b", 2);
    cache.has("a"); // should NOT refresh "a"
    cache.set("c", 3); // evicts "a" (still least-recently-used)
    assert.equal(cache.has("a"), false);
    assert.equal(cache.has("b"), true);
  });

  test("size reflects the current number of entries", () => {
    const cache = createLRUCache(3);
    assert.equal(cache.size, 0);
    cache.set("a", 1);
    cache.set("b", 2);
    assert.equal(cache.size, 2);
  });
});

describe("createObjectPool", () => {
  test("acquire() calls create() when the pool is empty", () => {
    let created = 0;
    const pool = createObjectPool(
      () => ({ id: ++created, dirty: true }),
      (obj) => {
        obj.dirty = false;
      },
      2,
    );
    const a = pool.acquire();
    assert.equal(created, 1);
    assert.equal(a.id, 1);
  });

  test("release() resets and returns an object; acquire() reuses it without calling create()", () => {
    let created = 0;
    const pool = createObjectPool(
      () => ({ id: ++created, dirty: true }),
      (obj) => {
        obj.dirty = false;
      },
      2,
    );
    const a = pool.acquire();
    pool.release(a);
    assert.equal(a.dirty, false);

    const b = pool.acquire();
    assert.equal(b, a);
    assert.equal(created, 1);
  });

  test("size reflects the number of available (released) objects", () => {
    const pool = createObjectPool(() => ({}), () => {}, 2);
    assert.equal(pool.size, 0);
    const a = pool.acquire();
    assert.equal(pool.size, 0);
    pool.release(a);
    assert.equal(pool.size, 1);
  });

  test("releasing more than maxSize objects discards the excess", () => {
    const pool = createObjectPool(() => ({}), () => {}, 2);
    const a = pool.acquire();
    const b = pool.acquire();
    const c = pool.acquire();
    pool.release(a);
    pool.release(b);
    pool.release(c); // pool already at maxSize 2 -> discarded
    assert.equal(pool.size, 2);
  });
});

describe("memoizeWithTTL", () => {
  test("the first call invokes fn and returns its result", () => {
    let calls = 0;
    const double = memoizeWithTTL((x) => {
      calls++;
      return x * 2;
    }, 1000);
    assert.equal(double(5), 10);
    assert.equal(calls, 1);
  });

  test("a call within the TTL returns the cached result without calling fn again", () => {
    let calls = 0;
    const double = memoizeWithTTL((x) => {
      calls++;
      return x * 2;
    }, 1000);
    double(5);
    double(5);
    assert.equal(calls, 1);
  });

  test("a call after the TTL expires recomputes the result", async () => {
    let calls = 0;
    const double = memoizeWithTTL((x) => {
      calls++;
      return x * 2;
    }, 20);
    assert.equal(double(5), 10);
    await sleep(50);
    assert.equal(double(5), 10);
    assert.equal(calls, 2);
  });

  test("different arguments are cached separately", () => {
    let calls = 0;
    const double = memoizeWithTTL((x) => {
      calls++;
      return x * 2;
    }, 1000);
    assert.equal(double(5), 10);
    assert.equal(double(7), 14);
    assert.equal(calls, 2);
    assert.equal(double(5), 10);
    assert.equal(double(7), 14);
    assert.equal(calls, 2);
  });
});

describe("memoizeByReference", () => {
  test("calling with the same object reference twice calls fn only once", () => {
    let calls = 0;
    const summarize = memoizeByReference((obj) => {
      calls++;
      return obj.items.length;
    });
    const data = { items: [1, 2, 3] };
    assert.equal(summarize(data), 3);
    assert.equal(summarize(data), 3);
    assert.equal(calls, 1);
  });

  test("a structurally-identical but different object recomputes", () => {
    let calls = 0;
    const summarize = memoizeByReference((obj) => {
      calls++;
      return obj.items.length;
    });
    summarize({ items: [1, 2, 3] });
    summarize({ items: [1, 2, 3] });
    assert.equal(calls, 2);
  });

  test("different objects each get their own cached result", () => {
    let calls = 0;
    const summarize = memoizeByReference((obj) => {
      calls++;
      return obj.items.length;
    });
    const a = { items: [1, 2] };
    const b = { items: [1, 2, 3, 4] };
    assert.equal(summarize(a), 2);
    assert.equal(summarize(b), 4);
    assert.equal(summarize(a), 2);
    assert.equal(summarize(b), 4);
    assert.equal(calls, 2);
  });

  test("calling with a non-object argument throws", () => {
    const fn = memoizeByReference((x) => x);
    assert.throws(() => fn(5), TypeError);
  });
});

describe("createBatcher", () => {
  test("reaching maxBatchSize flushes immediately with items in order", () => {
    const batches = [];
    const batcher = createBatcher((items) => batches.push(items), {
      maxBatchSize: 3,
      maxWaitMs: 1000,
    });
    batcher.add(1);
    batcher.add(2);
    batcher.add(3);
    assert.deepEqual(batches, [[1, 2, 3]]);
  });

  test("flush() processes a partial batch immediately", () => {
    const batches = [];
    const batcher = createBatcher((items) => batches.push(items), {
      maxBatchSize: 10,
      maxWaitMs: 1000,
    });
    batcher.add("a");
    batcher.add("b");
    batcher.flush();
    assert.deepEqual(batches, [["a", "b"]]);
  });

  test("flush() on an empty batch is a no-op", () => {
    const batches = [];
    const batcher = createBatcher((items) => batches.push(items), {
      maxBatchSize: 10,
      maxWaitMs: 1000,
    });
    batcher.flush();
    assert.deepEqual(batches, []);
  });

  test("a batch is flushed after maxWaitMs even if not full", async () => {
    const batches = [];
    const batcher = createBatcher((items) => batches.push(items), {
      maxBatchSize: 10,
      maxWaitMs: 20,
    });
    batcher.add("a");
    batcher.add("b");
    assert.deepEqual(batches, []);
    await sleep(50);
    assert.deepEqual(batches, [["a", "b"]]);
  });

  test("a new batch starts independently after a flush", () => {
    const batches = [];
    const batcher = createBatcher((items) => batches.push(items), {
      maxBatchSize: 2,
      maxWaitMs: 1000,
    });
    batcher.add(1);
    batcher.add(2); // flush -> [1, 2]
    batcher.add(3);
    batcher.add(4); // flush -> [3, 4]
    assert.deepEqual(batches, [
      [1, 2],
      [3, 4],
    ]);
  });
});
