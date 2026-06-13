import { describe, test } from "node:test";
import assert from "node:assert/strict";
import {
  createModule,
  mergeNamespaces,
  createPluginRegistry,
  defineLazyExport,
  createReadonlyNamespace,
} from "./exercise.js";

describe("createModule", () => {
  test("does not call factory before getInstance is invoked", () => {
    let calls = 0;
    createModule(() => {
      calls++;
      return {};
    });
    assert.equal(calls, 0);
  });

  test("calls factory exactly once across multiple getInstance calls", () => {
    let calls = 0;
    const getInstance = createModule(() => {
      calls++;
      return { id: calls };
    });

    getInstance();
    getInstance();
    getInstance();

    assert.equal(calls, 1);
  });

  test("returns the SAME cached instance every call", () => {
    const getInstance = createModule(() => ({ value: Math.random() }));

    const first = getInstance();
    const second = getInstance();
    const third = getInstance();

    assert.equal(first, second);
    assert.equal(second, third);
  });

  test("caches even a falsy or primitive return value", () => {
    let calls = 0;
    const getInstance = createModule(() => {
      calls++;
      return 0;
    });

    assert.equal(getInstance(), 0);
    assert.equal(getInstance(), 0);
    assert.equal(calls, 1);
  });

  test("independent createModule calls have independent factories/caches", () => {
    let callsA = 0;
    let callsB = 0;
    const getA = createModule(() => {
      callsA++;
      return "a";
    });
    const getB = createModule(() => {
      callsB++;
      return "b";
    });

    getA();
    getA();
    getB();

    assert.equal(callsA, 1);
    assert.equal(callsB, 1);
  });
});

describe("mergeNamespaces", () => {
  test("merges keys from multiple namespaces", () => {
    const merged = mergeNamespaces({ a: 1, b: 2 }, { c: 3 });
    assert.deepEqual(merged, { a: 1, b: 2, c: 3 });
  });

  test("merges with no arguments returns an empty object", () => {
    assert.deepEqual(mergeNamespaces(), {});
  });

  test("merges a single namespace as-is", () => {
    assert.deepEqual(mergeNamespaces({ a: 1 }), { a: 1 });
  });

  test("same key with the SAME value (===) across namespaces is allowed", () => {
    const shared = { x: 1 };
    const merged = mergeNamespaces({ shared, a: 1 }, { shared, b: 2 });
    assert.equal(merged.shared, shared);
    assert.deepEqual(merged, { shared, a: 1, b: 2 });
  });

  test("same key with primitive equal values across namespaces is allowed", () => {
    const merged = mergeNamespaces({ VERSION: "1.0" }, { VERSION: "1.0" });
    assert.equal(merged.VERSION, "1.0");
  });

  test("same key with DIFFERENT values throws an Error naming the key", () => {
    assert.throws(
      () => mergeNamespaces({ a: 1 }, { a: 2 }),
      (err) => err instanceof Error && /a/.test(err.message),
    );
  });

  test("collision error message identifies the colliding namespace indices", () => {
    assert.throws(
      () => mergeNamespaces({ x: 1 }, { y: 2 }, { x: 99 }),
      (err) =>
        err instanceof Error &&
        /x/.test(err.message) &&
        /0/.test(err.message) &&
        /2/.test(err.message),
    );
  });

  test("non-colliding keys still merge correctly alongside an unrelated collision-free pair", () => {
    const merged = mergeNamespaces({ a: 1, shared: "v" }, { b: 2, shared: "v" });
    assert.deepEqual(merged, { a: 1, b: 2, shared: "v" });
  });
});

describe("createPluginRegistry", () => {
  test("load resolves to the loader's return value", async () => {
    const registry = createPluginRegistry();
    registry.register("math", () => ({ square: (x) => x * x }));

    const mod = await registry.load("math");
    assert.equal(mod.square(4), 16);
  });

  test("load on an unregistered name rejects with an Error", async () => {
    const registry = createPluginRegistry();
    await assert.rejects(() => registry.load("missing"), Error);
  });

  test("loader is called only once across repeated sequential loads", async () => {
    const registry = createPluginRegistry();
    let calls = 0;
    registry.register("config", () => {
      calls++;
      return { ready: true };
    });

    const first = await registry.load("config");
    const second = await registry.load("config");

    assert.equal(first, second);
    assert.equal(calls, 1);
  });

  test("supports async loaders (Promise-returning)", async () => {
    const registry = createPluginRegistry();
    let calls = 0;
    registry.register("async-plugin", async () => {
      calls++;
      return { id: "async" };
    });

    const mod = await registry.load("async-plugin");
    assert.deepEqual(mod, { id: "async" });
    assert.equal(calls, 1);
  });

  test("concurrent loads before resolution call loader only once and share the result", async () => {
    const registry = createPluginRegistry();
    let calls = 0;
    registry.register("shared", async () => {
      calls++;
      await new Promise((resolve) => setTimeout(resolve, 5));
      return { token: calls };
    });

    const [a, b, c] = await Promise.all([
      registry.load("shared"),
      registry.load("shared"),
      registry.load("shared"),
    ]);

    assert.equal(calls, 1);
    assert.equal(a, b);
    assert.equal(b, c);
    assert.deepEqual(a, { token: 1 });
  });

  test("different plugin names are loaded independently", async () => {
    const registry = createPluginRegistry();
    let callsA = 0;
    let callsB = 0;
    registry.register("a", () => {
      callsA++;
      return "A";
    });
    registry.register("b", () => {
      callsB++;
      return "B";
    });

    assert.equal(await registry.load("a"), "A");
    assert.equal(await registry.load("b"), "B");
    assert.equal(callsA, 1);
    assert.equal(callsB, 1);
  });

  test("re-registering a name resets it to use the new loader for future loads", async () => {
    const registry = createPluginRegistry();
    registry.register("x", () => "first");
    assert.equal(await registry.load("x"), "first");

    registry.register("x", () => "second");
    assert.equal(await registry.load("x"), "second");
  });
});

describe("defineLazyExport", () => {
  test("factory is not called before first access", () => {
    let calls = 0;
    const target = {};
    defineLazyExport(target, "config", () => {
      calls++;
      return { ready: true };
    });

    assert.equal(calls, 0);
  });

  test("first read computes and returns the value via the getter", () => {
    let calls = 0;
    const target = {};
    defineLazyExport(target, "config", () => {
      calls++;
      return { ready: true };
    });

    assert.deepEqual(target.config, { ready: true });
    assert.equal(calls, 1);
  });

  test("subsequent reads return the cached value without recomputation", () => {
    let calls = 0;
    const target = {};
    defineLazyExport(target, "config", () => {
      calls++;
      return { ready: true };
    });

    const first = target.config;
    const second = target.config;
    const third = target.config;

    assert.equal(first, second);
    assert.equal(second, third);
    assert.equal(calls, 1);
  });

  test("returns the target object", () => {
    const target = {};
    const result = defineLazyExport(target, "value", () => 42);
    assert.equal(result, target);
  });

  test("caches a falsy computed value correctly", () => {
    let calls = 0;
    const target = {};
    defineLazyExport(target, "flag", () => {
      calls++;
      return false;
    });

    assert.equal(target.flag, false);
    assert.equal(target.flag, false);
    assert.equal(calls, 1);
  });

  test("multiple lazy exports on the same target are independent", () => {
    let callsA = 0;
    let callsB = 0;
    const target = {};
    defineLazyExport(target, "a", () => {
      callsA++;
      return "A";
    });
    defineLazyExport(target, "b", () => {
      callsB++;
      return "B";
    });

    assert.equal(target.b, "B");
    assert.equal(callsA, 0);
    assert.equal(callsB, 1);

    assert.equal(target.a, "A");
    assert.equal(callsA, 1);
  });
});

describe("createReadonlyNamespace", () => {
  test("exposes the same enumerable keys/values as the input", () => {
    const ns = createReadonlyNamespace({ PI: 3.14, square: (x) => x * x });
    assert.equal(ns.PI, 3.14);
    assert.equal(typeof ns.square, "function");
    assert.deepEqual(Object.keys(ns).sort(), ["PI", "square"]);
  });

  test("reading existing properties works normally", () => {
    const ns = createReadonlyNamespace({ a: 1, b: "two" });
    assert.equal(ns.a, 1);
    assert.equal(ns.b, "two");
  });

  test("assigning to an existing property throws TypeError", () => {
    const ns = createReadonlyNamespace({ PI: 3.14 });
    assert.throws(() => {
      ns.PI = 4;
    }, TypeError);
    assert.equal(ns.PI, 3.14); // unchanged
  });

  test("adding a new property throws TypeError", () => {
    const ns = createReadonlyNamespace({ a: 1 });
    assert.throws(() => {
      ns.EXTRA = "no";
    }, TypeError);
    assert.equal("EXTRA" in ns, false);
  });

  test("does not mutate the original moduleExports object's reference behavior", () => {
    const original = { count: 1 };
    const ns = createReadonlyNamespace(original);
    assert.equal(ns.count, 1);
    // The returned namespace itself must reject writes regardless of `original`.
    assert.throws(() => {
      ns.count = 99;
    }, TypeError);
  });
});
