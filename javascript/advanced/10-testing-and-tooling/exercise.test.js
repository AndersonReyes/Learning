import { test, describe } from "node:test";
import assert from "node:assert/strict";
import {
  createAssert,
  createMockFn,
  spyOn,
  createTestRunner,
  createSnapshotMatcher,
} from "./exercise.js";

describe("createAssert", () => {
  const assertions = createAssert();

  test("equal() passes for Object.is-equal values, including NaN", () => {
    assert.doesNotThrow(() => assertions.equal(1, 1));
    assert.doesNotThrow(() => assertions.equal(NaN, NaN));
    assert.doesNotThrow(() => assertions.equal("a", "a"));
  });

  test("equal() throws a descriptive Error on mismatch", () => {
    assert.throws(() => assertions.equal(1, 2), /1/);
  });

  test("equal() throws the given custom message verbatim", () => {
    assert.throws(() => assertions.equal(1, 2, "custom message"), { message: "custom message" });
  });

  test("deepEqual() passes for structurally equal objects/arrays", () => {
    assert.doesNotThrow(() => assertions.deepEqual({ a: 1, b: [1, 2] }, { a: 1, b: [1, 2] }));
    assert.doesNotThrow(() => assertions.deepEqual([1, [2, 3]], [1, [2, 3]]));
  });

  test("deepEqual() throws on structural mismatch", () => {
    assert.throws(() => assertions.deepEqual({ a: 1 }, { a: 2 }), Error);
    assert.throws(() => assertions.deepEqual([1, 2], [1, 2, 3]), Error);
  });

  test("ok() passes for truthy values and throws for falsy values", () => {
    assert.doesNotThrow(() => assertions.ok("non-empty"));
    assert.doesNotThrow(() => assertions.ok(1));
    assert.throws(() => assertions.ok(0), Error);
    assert.throws(() => assertions.ok(""), Error);
  });

  test("throws() passes when fn throws and throws when fn does not", () => {
    assert.doesNotThrow(() => assertions.throws(() => { throw new Error("boom"); }));
    assert.throws(() => assertions.throws(() => {}), Error);
  });
});

describe("createMockFn", () => {
  test("records call arguments in order", () => {
    const mock = createMockFn();
    mock(1, 2);
    mock(3);
    assert.deepEqual(mock.calls, [[1, 2], [3]]);
  });

  test("default implementation returns undefined and records a 'return' result", () => {
    const mock = createMockFn();
    const result = mock(1);
    assert.equal(result, undefined);
    assert.deepEqual(mock.results, [{ type: "return", value: undefined }]);
  });

  test("mockReturnValue() makes subsequent calls return that value", () => {
    const mock = createMockFn();
    mock.mockReturnValue(42);
    assert.equal(mock(), 42);
    assert.equal(mock("ignored"), 42);
  });

  test("mockImplementation() changes the call behavior", () => {
    const mock = createMockFn();
    mock.mockImplementation((a, b) => a + b);
    assert.equal(mock(2, 3), 5);
  });

  test("records a 'throw' result and re-throws when the implementation throws", () => {
    const mock = createMockFn(() => {
      throw new Error("fail");
    });
    assert.throws(() => mock(), /fail/);
    assert.equal(mock.results[0].type, "throw");
    assert.ok(mock.results[0].value instanceof Error);
  });

  test("reset() clears calls and results", () => {
    const mock = createMockFn();
    mock(1);
    mock.reset();
    assert.deepEqual(mock.calls, []);
    assert.deepEqual(mock.results, []);
  });
});

describe("spyOn", () => {
  test("calls through to the original method by default, recording calls", () => {
    const obj = { greet(name) { return `Hello, ${name}!`; } };
    const spy = spyOn(obj, "greet");
    assert.equal(obj.greet("Ada"), "Hello, Ada!");
    assert.deepEqual(spy.calls, [["Ada"]]);
  });

  test("restore() puts the original method back", () => {
    const original = function greet(name) { return `Hi, ${name}`; };
    const obj = { greet: original };
    const spy = spyOn(obj, "greet");
    spy.restore();
    assert.equal(obj.greet, original);
  });

  test("after restore(), further calls don't affect spy.calls", () => {
    const obj = { greet(name) { return `Hello, ${name}!`; } };
    const spy = spyOn(obj, "greet");
    obj.greet("Ada");
    spy.restore();
    obj.greet("Bob");
    assert.deepEqual(spy.calls, [["Ada"]]);
  });

  test("mockReturnValue() overrides the call-through behavior", () => {
    const obj = { greet(name) { return `Hello, ${name}!`; } };
    const spy = spyOn(obj, "greet");
    spy.mockReturnValue("mocked");
    assert.equal(obj.greet("X"), "mocked");
    assert.deepEqual(spy.calls, [["X"]]);
  });

  test("records multiple calls in order", () => {
    const obj = { add(a, b) { return a + b; } };
    const spy = spyOn(obj, "add");
    obj.add(1, 2);
    obj.add(3, 4);
    assert.deepEqual(spy.calls, [[1, 2], [3, 4]]);
  });
});

describe("createTestRunner", () => {
  test("an empty runner reports all-zero totals", async () => {
    const runner = createTestRunner();
    assert.deepEqual(await runner.run(), { total: 0, passed: 0, failed: 0, failures: [] });
  });

  test("counts passing and failing sync tests", async () => {
    const runner = createTestRunner();
    runner.test("passes", () => {});
    runner.test("fails", () => {
      throw new Error("nope");
    });
    const summary = await runner.run();
    assert.equal(summary.total, 2);
    assert.equal(summary.passed, 1);
    assert.equal(summary.failed, 1);
    assert.equal(summary.failures[0].name, "fails");
    assert.equal(summary.failures[0].error.message, "nope");
  });

  test("awaits async tests, treating rejections as failures", async () => {
    const runner = createTestRunner();
    runner.test("async passes", async () => {
      await Promise.resolve();
    });
    runner.test("async fails", async () => {
      throw new Error("async nope");
    });
    const summary = await runner.run();
    assert.equal(summary.passed, 1);
    assert.equal(summary.failed, 1);
    assert.equal(summary.failures[0].name, "async fails");
    assert.equal(summary.failures[0].error.message, "async nope");
  });

  test("one failing test does not stop the others from running", async () => {
    const order = [];
    const runner = createTestRunner();
    runner.test("first", () => {
      order.push("first");
      throw new Error("boom");
    });
    runner.test("second", () => {
      order.push("second");
    });
    const summary = await runner.run();
    assert.deepEqual(order, ["first", "second"]);
    assert.equal(summary.total, 2);
    assert.equal(summary.passed, 1);
    assert.equal(summary.failed, 1);
  });

  test("tests run in registration order", async () => {
    const order = [];
    const runner = createTestRunner();
    runner.test("first", async () => {
      order.push("first");
    });
    runner.test("second", () => {
      order.push("second");
    });
    await runner.run();
    assert.deepEqual(order, ["first", "second"]);
  });
});

describe("createSnapshotMatcher", () => {
  test("the first match() for a name records it without throwing", () => {
    const matcher = createSnapshotMatcher();
    assert.doesNotThrow(() => matcher.match("user", { id: 1, name: "Ada" }));
  });

  test("a matching value on a later call does not throw", () => {
    const matcher = createSnapshotMatcher();
    matcher.match("user", { id: 1, name: "Ada" });
    assert.doesNotThrow(() => matcher.match("user", { id: 1, name: "Ada" }));
  });

  test("a mismatched value throws an Error mentioning the name", () => {
    const matcher = createSnapshotMatcher();
    matcher.match("user", { id: 1, name: "Ada" });
    assert.throws(() => matcher.match("user", { id: 1, name: "Bob" }), /user/);
  });

  test("different names are tracked independently", () => {
    const matcher = createSnapshotMatcher();
    matcher.match("a", 1);
    matcher.match("b", 2);
    assert.doesNotThrow(() => matcher.match("a", 1));
    assert.doesNotThrow(() => matcher.match("b", 2));
  });

  test("snapshots getter returns all recorded values", () => {
    const matcher = createSnapshotMatcher();
    matcher.match("user", { id: 1, name: "Ada" });
    matcher.match("list", [1, 2, 3]);
    assert.deepEqual(matcher.snapshots, { user: { id: 1, name: "Ada" }, list: [1, 2, 3] });
  });
});
