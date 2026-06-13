import { test, describe } from "node:test";
import assert from "node:assert/strict";
import {
  createValidatedObject,
  createNegativeIndexArray,
  deepFreeze,
  createObservable,
  createMethodLogger,
} from "./exercise.js";

describe("createValidatedObject", () => {
  test("setting and reading properties that match the schema type", () => {
    const user = createValidatedObject({ name: "string", age: "number" });
    user.name = "Ada";
    user.age = 30;
    assert.equal(user.name, "Ada");
    assert.equal(user.age, 30);
  });

  test("setting a property to the wrong type throws a TypeError mentioning the property and expected type", () => {
    const user = createValidatedObject({ name: "string", age: "number" });
    user.age = 30;
    assert.throws(
      () => {
        user.age = "thirty";
      },
      (err) => err instanceof TypeError && /age/.test(err.message) && /number/.test(err.message),
    );
  });

  test("a rejected assignment leaves the existing value unchanged", () => {
    const user = createValidatedObject({ age: "number" });
    user.age = 30;
    assert.throws(() => {
      user.age = "thirty";
    }, TypeError);
    assert.equal(user.age, 30);
  });

  test("properties not in the schema accept any value", () => {
    const user = createValidatedObject({ age: "number" });
    user.extra = { anything: true };
    assert.deepEqual(user.extra, { anything: true });
  });

  test("an unset schema property reads as undefined", () => {
    const user = createValidatedObject({ name: "string" });
    assert.equal(user.name, undefined);
  });
});

describe("createNegativeIndexArray", () => {
  test("positive indices behave as on the plain array", () => {
    const a = createNegativeIndexArray([10, 20, 30]);
    assert.equal(a[0], 10);
    assert.equal(a[1], 20);
    assert.equal(a[2], 30);
  });

  test("negative indices read from the end of the array", () => {
    const a = createNegativeIndexArray([10, 20, 30]);
    assert.equal(a[-1], 30);
    assert.equal(a[-2], 20);
    assert.equal(a[-3], 10);
  });

  test("length passes through unchanged", () => {
    const a = createNegativeIndexArray([10, 20, 30]);
    assert.equal(a.length, 3);
  });

  test("array methods like map work correctly", () => {
    const a = createNegativeIndexArray([1, 2, 3]);
    assert.deepEqual(a.map((x) => x * 2), [2, 4, 6]);
  });

  test("assigning to a negative index mutates the underlying element", () => {
    const a = createNegativeIndexArray([10, 20, 30]);
    a[-1] = 99;
    assert.equal(a[-1], 99);
    assert.equal(a[2], 99);
  });

  test("an out-of-range negative index reads as undefined", () => {
    const a = createNegativeIndexArray([10, 20, 30]);
    assert.equal(a[-100], undefined);
  });
});

describe("deepFreeze", () => {
  test("reading top-level and nested properties works normally", () => {
    const frozen = deepFreeze({ a: 1, b: { c: 2 } });
    assert.equal(frozen.a, 1);
    assert.equal(frozen.b.c, 2);
  });

  test("setting a top-level property throws TypeError", () => {
    const frozen = deepFreeze({ a: 1 });
    assert.throws(() => {
      frozen.a = 5;
    }, TypeError);
  });

  test("setting a nested property throws TypeError", () => {
    const frozen = deepFreeze({ b: { c: 2 } });
    assert.throws(() => {
      frozen.b.c = 5;
    }, TypeError);
  });

  test("deleting a property throws TypeError", () => {
    const frozen = deepFreeze({ a: 1 });
    assert.throws(() => {
      delete frozen.a;
    }, TypeError);
  });

  test("the underlying object is unchanged after rejected mutations", () => {
    const original = { a: 1, b: { c: 2 } };
    const frozen = deepFreeze(original);
    assert.throws(() => {
      frozen.a = 5;
    });
    assert.throws(() => {
      frozen.b.c = 5;
    });
    assert.equal(original.a, 1);
    assert.equal(original.b.c, 2);
  });
});

describe("createObservable", () => {
  test("setting a new property calls onChange with oldValue undefined", () => {
    const changes = [];
    const state = createObservable({}, (c) => changes.push(c));
    state.count = 1;
    assert.deepEqual(changes, [{ property: "count", oldValue: undefined, newValue: 1 }]);
  });

  test("setting a property to its current value does not call onChange", () => {
    const changes = [];
    const state = createObservable({ count: 1 }, (c) => changes.push(c));
    state.count = 1;
    assert.deepEqual(changes, []);
  });

  test("setting a property to a new value calls onChange with both old and new values", () => {
    const changes = [];
    const state = createObservable({ count: 1 }, (c) => changes.push(c));
    state.count = 2;
    assert.deepEqual(changes, [{ property: "count", oldValue: 1, newValue: 2 }]);
  });

  test("reading a property returns its current value", () => {
    const state = createObservable({ count: 1 }, () => {});
    state.count = 5;
    assert.equal(state.count, 5);
  });

  test("changes to independent properties are tracked separately", () => {
    const changes = [];
    const state = createObservable({}, (c) => changes.push(c));
    state.a = 1;
    state.b = 2;
    state.a = 1; // unchanged, no entry
    state.b = 3;
    assert.deepEqual(changes, [
      { property: "a", oldValue: undefined, newValue: 1 },
      { property: "b", oldValue: undefined, newValue: 2 },
      { property: "b", oldValue: 2, newValue: 3 },
    ]);
  });
});

describe("createMethodLogger", () => {
  test("a successful call logs method, args, and result", () => {
    const log = [];
    const calc = createMethodLogger(
      {
        add(a, b) {
          return a + b;
        },
      },
      log,
    );
    assert.equal(calc.add(2, 3), 5);
    assert.deepEqual(log, [{ method: "add", args: [2, 3], result: 5 }]);
  });

  test("multiple calls are appended to the log in order", () => {
    const log = [];
    const calc = createMethodLogger(
      {
        double(x) {
          return x * 2;
        },
      },
      log,
    );
    calc.double(1);
    calc.double(2);
    assert.deepEqual(log, [
      { method: "double", args: [1], result: 2 },
      { method: "double", args: [2], result: 4 },
    ]);
  });

  test("a method that throws logs the error and re-throws it", () => {
    const log = [];
    const boom = new Error("boom");
    const calc = createMethodLogger(
      {
        fail() {
          throw boom;
        },
      },
      log,
    );
    assert.throws(() => calc.fail(), /boom/);
    assert.deepEqual(log, [{ method: "fail", args: [], error: boom }]);
  });

  test("non-function properties pass through unchanged and are not logged", () => {
    const log = [];
    const obj = createMethodLogger({ value: 42 }, log);
    assert.equal(obj.value, 42);
    assert.deepEqual(log, []);
  });

  test("arguments are captured as an array even with zero or multiple args", () => {
    const log = [];
    const obj = createMethodLogger(
      {
        sum(...nums) {
          return nums.reduce((a, b) => a + b, 0);
        },
      },
      log,
    );
    obj.sum(1, 2, 3);
    assert.deepEqual(log, [{ method: "sum", args: [1, 2, 3], result: 6 }]);
  });
});
