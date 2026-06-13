import { describe, test } from "node:test";
import assert from "node:assert/strict";
import {
  createCounter,
  once,
  memoize,
  createEventEmitter,
  createLRUCache,
} from "./exercise.js";

describe("createCounter", () => {
  test("starts at 0 by default", () => {
    const counter = createCounter();
    assert.equal(counter.value(), 0);
  });

  test("starts at the given value", () => {
    const counter = createCounter(10);
    assert.equal(counter.value(), 10);
  });

  test("increment defaults to step 1", () => {
    const counter = createCounter();
    assert.equal(counter.increment(), 1);
    assert.equal(counter.increment(), 2);
  });

  test("increment/decrement accept a custom step", () => {
    const counter = createCounter(10);
    assert.equal(counter.increment(5), 15);
    assert.equal(counter.decrement(2), 13);
  });

  test("reset returns to the original start value", () => {
    const counter = createCounter(5);
    counter.increment(10);
    counter.decrement(3);
    assert.equal(counter.reset(), 5);
    assert.equal(counter.value(), 5);
  });

  test("count is not accessible as a property", () => {
    const counter = createCounter(5);
    assert.equal(counter.count, undefined);
    assert.equal(counter.value, counter.value); // method exists
    assert.deepEqual(Object.keys(counter).sort(), [
      "decrement",
      "increment",
      "reset",
      "value",
    ]);
  });

  test("separate counters have independent state", () => {
    const a = createCounter();
    const b = createCounter();
    a.increment();
    a.increment();
    b.increment();
    assert.equal(a.value(), 2);
    assert.equal(b.value(), 1);
  });
});

describe("once", () => {
  test("calls fn only on the first invocation", () => {
    let calls = 0;
    const init = once((x) => {
      calls++;
      return x * 2;
    });

    assert.equal(init(5), 10);
    assert.equal(calls, 1);
    assert.equal(init(5), 10);
    assert.equal(calls, 1);
  });

  test("subsequent calls return the FIRST result regardless of new args", () => {
    const init = once((x) => x * 2);

    assert.equal(init(5), 10);
    assert.equal(init(100), 10);
    assert.equal(init(), 10);
  });

  test("works for functions with no arguments", () => {
    let calls = 0;
    const setup = once(() => {
      calls++;
      return "configured";
    });

    assert.equal(setup(), "configured");
    assert.equal(setup(), "configured");
    assert.equal(calls, 1);
  });

  test("independent wrappers around the same fn have independent state", () => {
    let calls = 0;
    const fn = (x) => {
      calls++;
      return x;
    };
    const onceA = once(fn);
    const onceB = once(fn);

    assert.equal(onceA(1), 1);
    assert.equal(onceB(2), 2);
    assert.equal(calls, 2);
  });
});

describe("memoize", () => {
  test("caches results for repeated calls with the same arguments", () => {
    let calls = 0;
    const add = memoize((a, b) => {
      calls++;
      return a + b;
    });

    assert.equal(add(1, 2), 3);
    assert.equal(add(1, 2), 3);
    assert.equal(calls, 1);
  });

  test("computes separately for different arguments", () => {
    let calls = 0;
    const add = memoize((a, b) => {
      calls++;
      return a + b;
    });

    assert.equal(add(1, 2), 3);
    assert.equal(add(2, 3), 5);
    assert.equal(calls, 2);
  });

  test("default keyFn distinguishes different argument shapes", () => {
    let calls = 0;
    const fn = memoize((...args) => {
      calls++;
      return args.length;
    });

    assert.equal(fn(1, 2), 2);
    assert.equal(fn(1, 2, 3), 3);
    assert.equal(calls, 2);
  });

  test("custom keyFn controls cache key", () => {
    let calls = 0;
    const fn = memoize(
      (obj) => {
        calls++;
        return obj.value * 2;
      },
      (obj) => obj.id,
    );

    assert.equal(fn({ id: "a", value: 1 }), 2);
    // Same id, different `value` -> cache hit, returns stale cached result.
    assert.equal(fn({ id: "a", value: 99 }), 2);
    assert.equal(calls, 1);
  });

  test("two memoized wrappers of the same fn don't share a cache", () => {
    let calls = 0;
    const fn = (x) => {
      calls++;
      return x * 2;
    };
    const memoA = memoize(fn);
    const memoB = memoize(fn);

    memoA(5);
    memoB(5);
    assert.equal(calls, 2);
  });
});

describe("createEventEmitter", () => {
  test("emit calls all handlers in registration order", () => {
    const emitter = createEventEmitter();
    const calls = [];
    emitter.on("greet", (name) => calls.push(`hi ${name}`));
    emitter.on("greet", (name) => calls.push(`hello ${name}`));

    const count = emitter.emit("greet", "Ada");

    assert.equal(count, 2);
    assert.deepEqual(calls, ["hi Ada", "hello Ada"]);
  });

  test("emitting an event with no handlers returns 0", () => {
    const emitter = createEventEmitter();
    assert.equal(emitter.emit("nothing"), 0);
  });

  test("on returns an unsubscribe function", () => {
    const emitter = createEventEmitter();
    const calls = [];
    const unsubscribe = emitter.on("tick", () => calls.push("tick"));

    emitter.emit("tick");
    unsubscribe();
    emitter.emit("tick");

    assert.deepEqual(calls, ["tick"]);
  });

  test("off removes a registered handler", () => {
    const emitter = createEventEmitter();
    const calls = [];
    const handler = () => calls.push("called");
    emitter.on("tick", handler);

    emitter.off("tick", handler);
    const count = emitter.emit("tick");

    assert.equal(count, 0);
    assert.deepEqual(calls, []);
  });

  test("off with an unregistered handler is a no-op", () => {
    const emitter = createEventEmitter();
    const calls = [];
    emitter.on("tick", () => calls.push("called"));

    assert.doesNotThrow(() => emitter.off("tick", () => {}));
    assert.doesNotThrow(() => emitter.off("unknown-event", () => {}));

    const count = emitter.emit("tick");
    assert.equal(count, 1);
    assert.deepEqual(calls, ["called"]);
  });

  test("a throwing handler does not stop other handlers, and emit does not throw", () => {
    const emitter = createEventEmitter();
    const calls = [];
    emitter.on("go", () => {
      throw new Error("boom");
    });
    emitter.on("go", () => calls.push("second ran"));

    let count;
    assert.doesNotThrow(() => {
      count = emitter.emit("go");
    });

    assert.deepEqual(calls, ["second ran"]);
    assert.equal(count, 2); // throwing handler still counts as "called"
  });
});

describe("createLRUCache", () => {
  test("get returns undefined for a missing key", () => {
    const cache = createLRUCache(2);
    assert.equal(cache.get("missing"), undefined);
  });

  test("set then get round-trips a value", () => {
    const cache = createLRUCache(2);
    cache.set("a", 1);
    assert.equal(cache.get("a"), 1);
  });

  test("evicts the least-recently-used key when capacity is exceeded", () => {
    const cache = createLRUCache(2);
    cache.set("a", 1);
    cache.set("b", 2);
    cache.set("c", 3); // evicts "a" (least recently used)

    assert.equal(cache.get("a"), undefined);
    assert.equal(cache.get("b"), 2);
    assert.equal(cache.get("c"), 3);
  });

  test("get marks a key as recently used, protecting it from eviction", () => {
    const cache = createLRUCache(2);
    cache.set("a", 1);
    cache.set("b", 2);
    cache.get("a"); // "a" is now most recently used; "b" is least
    cache.set("c", 3); // evicts "b"

    assert.equal(cache.get("b"), undefined);
    assert.equal(cache.get("a"), 1);
    assert.equal(cache.get("c"), 3);
  });

  test("set on an existing key updates its value and marks it recently used", () => {
    const cache = createLRUCache(2);
    cache.set("a", 1);
    cache.set("b", 2);
    cache.set("a", 100); // updates "a" and makes it most recently used; "b" is least
    cache.set("c", 3); // evicts "b"

    assert.equal(cache.get("a"), 100);
    assert.equal(cache.get("b"), undefined);
    assert.equal(cache.get("c"), 3);
  });

  test("capacity of 1 keeps only the single most recently used key", () => {
    const cache = createLRUCache(1);
    cache.set("a", 1);
    cache.set("b", 2); // evicts "a"

    assert.equal(cache.get("a"), undefined);
    assert.equal(cache.get("b"), 2);

    cache.get("b"); // "b" remains most recently used
    cache.set("c", 3); // evicts "b"
    assert.equal(cache.get("b"), undefined);
    assert.equal(cache.get("c"), 3);
  });
});
