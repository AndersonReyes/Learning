import { test, describe } from "node:test";
import assert from "node:assert/strict";
import {
  withResource,
  withAsyncResource,
  createDisposableStack,
  acquireAll,
  createLazyResource,
} from "./exercise.js";

function makeResource(name, log, { throwOnDispose = false } = {}) {
  return {
    name,
    [Symbol.dispose]() {
      log.push(`dispose:${name}`);
      if (throwOnDispose) throw new Error(`dispose failed: ${name}`);
    },
  };
}

function makeAsyncResource(name, log) {
  return {
    name,
    async [Symbol.asyncDispose]() {
      log.push(`dispose:${name}`);
    },
  };
}

describe("withResource", () => {
  test("disposes the resource after use() returns, passing through the result", () => {
    const log = [];
    const result = withResource(
      () => makeResource("a", log),
      (r) => {
        log.push(`use:${r.name}`);
        return "ok";
      },
    );
    assert.equal(result, "ok");
    assert.deepEqual(log, ["use:a", "dispose:a"]);
  });

  test("disposes the resource even if use() throws, then re-throws", () => {
    const log = [];
    assert.throws(
      () =>
        withResource(
          () => makeResource("a", log),
          () => {
            log.push("use:a");
            throw new Error("boom");
          },
        ),
      /boom/,
    );
    assert.deepEqual(log, ["use:a", "dispose:a"]);
  });
});

describe("withAsyncResource", () => {
  test("awaits asyncDispose after use() resolves, passing through the result", async () => {
    const log = [];
    const result = await withAsyncResource(
      async () => makeAsyncResource("a", log),
      async (r) => {
        log.push(`use:${r.name}`);
        return "ok";
      },
    );
    assert.equal(result, "ok");
    assert.deepEqual(log, ["use:a", "dispose:a"]);
  });

  test("awaits asyncDispose even if use() rejects, then re-rejects", async () => {
    const log = [];
    await assert.rejects(
      withAsyncResource(
        async () => makeAsyncResource("a", log),
        async () => {
          log.push("use:a");
          throw new Error("boom");
        },
      ),
      /boom/,
    );
    assert.deepEqual(log, ["use:a", "dispose:a"]);
  });
});

describe("createDisposableStack", () => {
  test("disposes registered resources in reverse (LIFO) order", () => {
    const log = [];
    const stack = createDisposableStack();
    stack.use(makeResource("a", log));
    stack.use(makeResource("b", log));
    stack.dispose();
    assert.deepEqual(log, ["dispose:b", "dispose:a"]);
  });

  test("disposed reflects whether dispose() has been called", () => {
    const log = [];
    const stack = createDisposableStack();
    stack.use(makeResource("a", log));
    assert.equal(stack.disposed, false);
    stack.dispose();
    assert.equal(stack.disposed, true);
  });

  test("calling dispose() a second time is a no-op", () => {
    const log = [];
    const stack = createDisposableStack();
    stack.use(makeResource("a", log));
    stack.dispose();
    stack.dispose();
    assert.deepEqual(log, ["dispose:a"]);
  });

  test("use() returns the resource unchanged", () => {
    const log = [];
    const stack = createDisposableStack();
    const resource = makeResource("a", log);
    assert.equal(stack.use(resource), resource);
    stack.dispose();
  });

  test("if multiple resources throw while disposing, all are attempted and an AggregateError is thrown", () => {
    const log = [];
    const stack = createDisposableStack();
    stack.use(makeResource("a", log, { throwOnDispose: true }));
    stack.use(makeResource("b", log, { throwOnDispose: true }));
    assert.throws(() => stack.dispose(), AggregateError);
    assert.deepEqual(log, ["dispose:b", "dispose:a"]);
  });

  test("use() after dispose() throws", () => {
    const log = [];
    const stack = createDisposableStack();
    stack.dispose();
    assert.throws(() => stack.use(makeResource("a", log)), Error);
  });
});

describe("acquireAll", () => {
  test("acquires resources in order, runs use(), and disposes in reverse order", () => {
    const log = [];
    const result = acquireAll(
      [
        () => {
          log.push("acquire:a");
          return makeResource("a", log);
        },
        () => {
          log.push("acquire:b");
          return makeResource("b", log);
        },
      ],
      (resources) => {
        log.push("use");
        return resources.map((r) => r.name).join(",");
      },
    );
    assert.equal(result, "a,b");
    assert.deepEqual(log, ["acquire:a", "acquire:b", "use", "dispose:b", "dispose:a"]);
  });

  test("if use() throws, all resources are still disposed in reverse order before re-throwing", () => {
    const log = [];
    assert.throws(
      () =>
        acquireAll(
          [
            () => {
              log.push("acquire:a");
              return makeResource("a", log);
            },
            () => {
              log.push("acquire:b");
              return makeResource("b", log);
            },
          ],
          () => {
            log.push("use");
            throw new Error("boom");
          },
        ),
      /boom/,
    );
    assert.deepEqual(log, ["acquire:a", "acquire:b", "use", "dispose:b", "dispose:a"]);
  });

  test("if a later acquisition throws, earlier resources are disposed and use() is never called", () => {
    const log = [];
    assert.throws(
      () =>
        acquireAll(
          [
            () => {
              log.push("acquire:a");
              return makeResource("a", log);
            },
            () => {
              log.push("acquire:b");
              throw new Error("acquire failed");
            },
          ],
          () => {
            log.push("use");
          },
        ),
      /acquire failed/,
    );
    assert.deepEqual(log, ["acquire:a", "acquire:b", "dispose:a"]);
  });
});

describe("createLazyResource", () => {
  test("dispose without ever calling get() never invokes the factory", () => {
    let created = 0;
    const lazy = createLazyResource(() => {
      created++;
      return { [Symbol.dispose]() {} };
    });
    lazy[Symbol.dispose]();
    assert.equal(created, 0);
  });

  test("get() invokes the factory once and caches the result", () => {
    let created = 0;
    const lazy = createLazyResource(() => {
      created++;
      return { id: created, [Symbol.dispose]() {} };
    });
    const first = lazy.get();
    const second = lazy.get();
    assert.equal(first, second);
    assert.equal(created, 1);
  });

  test("dispose() after get() disposes the underlying resource", () => {
    const log = [];
    const lazy = createLazyResource(() => makeResource("a", log));
    lazy.get();
    lazy[Symbol.dispose]();
    assert.deepEqual(log, ["dispose:a"]);
  });

  test("dispose() called twice only disposes once", () => {
    const log = [];
    const lazy = createLazyResource(() => makeResource("a", log));
    lazy.get();
    lazy[Symbol.dispose]();
    lazy[Symbol.dispose]();
    assert.deepEqual(log, ["dispose:a"]);
  });

  test("get() after dispose() throws", () => {
    const log = [];
    const lazy = createLazyResource(() => makeResource("a", log));
    lazy.get();
    lazy[Symbol.dispose]();
    assert.throws(() => lazy.get(), Error);
  });
});
