import { test, describe } from "node:test";
import assert from "node:assert/strict";
import {
  createTypeGuards,
  validate,
  match,
  createResult,
  createTypedList,
} from "./exercise.js";

describe("createTypeGuards", () => {
  const g = createTypeGuards();

  test("isString/isNumber/isBoolean check basic types", () => {
    assert.equal(g.isString("a"), true);
    assert.equal(g.isString(1), false);
    assert.equal(g.isNumber(1), true);
    assert.equal(g.isNumber("1"), false);
    assert.equal(g.isBoolean(true), true);
    assert.equal(g.isBoolean(0), false);
  });

  test("isNumber() rejects NaN", () => {
    assert.equal(g.isNumber(NaN), false);
  });

  test("isArrayOf() checks every element, including the empty-array case", () => {
    assert.equal(g.isArrayOf(g.isString)(["a", "b"]), true);
    assert.equal(g.isArrayOf(g.isString)(["a", 1]), false);
    assert.equal(g.isArrayOf(g.isString)([]), true);
    assert.equal(g.isArrayOf(g.isString)("not an array"), false);
  });

  test("isObjectOf() requires shape keys to pass, ignores extra keys, rejects null/arrays", () => {
    const isPoint = g.isObjectOf({ x: g.isNumber, y: g.isNumber });
    assert.equal(isPoint({ x: 1, y: 2 }), true);
    assert.equal(isPoint({ x: 1, y: 2, z: 3 }), true);
    assert.equal(isPoint({ x: 1 }), false);
    assert.equal(isPoint(null), false);
    assert.equal(isPoint([1, 2]), false);
  });

  test("isOptional() allows undefined in addition to the wrapped guard", () => {
    const guard = g.isOptional(g.isString);
    assert.equal(guard(undefined), true);
    assert.equal(guard("x"), true);
    assert.equal(guard(1), false);
  });

  test("isUnion() passes if any guard matches", () => {
    const guard = g.isUnion(g.isString, g.isNumber);
    assert.equal(guard("x"), true);
    assert.equal(guard(1), true);
    assert.equal(guard(true), false);
  });

  test("isLiteral() matches only the given values", () => {
    const guard = g.isLiteral("admin", "user");
    assert.equal(guard("admin"), true);
    assert.equal(guard("user"), true);
    assert.equal(guard("guest"), false);
  });

  test("composes into a nested shape guard (discriminated 'User' shape)", () => {
    const isUser = g.isObjectOf({
      name: g.isString,
      role: g.isUnion(g.isLiteral("admin"), g.isLiteral("user")),
      tags: g.isArrayOf(g.isString),
      nickname: g.isOptional(g.isString),
    });

    assert.equal(isUser({ name: "Ada", role: "admin", tags: ["x"] }), true);
    assert.equal(isUser({ name: "Ada", role: "admin", tags: ["x"], nickname: "Ace" }), true);
    assert.equal(isUser({ name: "Ada", role: "guest", tags: ["x"] }), false);
    assert.equal(isUser({ name: "Ada", role: "admin", tags: [1] }), false);
  });
});

describe("validate", () => {
  const userSchema = {
    type: "object",
    properties: {
      name: { type: "string" },
      age: { type: "number" },
      role: {
        type: "union",
        options: [{ type: "literal", value: "admin" }, { type: "literal", value: "user" }],
      },
      tags: { type: "array", items: { type: "string" } },
    },
    required: ["name", "age", "role"],
  };

  test("a fully valid value reports valid: true and no errors", () => {
    const result = validate(userSchema, { name: "Ada", age: 30, role: "admin", tags: ["x", "y"] });
    assert.deepEqual(result, { valid: true, errors: [] });
  });

  test("collects multiple errors (type mismatches, union mismatch, nested array item)", () => {
    const result = validate(userSchema, { name: 1, age: "30", role: "guest", tags: ["x", 2] });
    assert.equal(result.valid, false);
    assert.deepEqual(result.errors, [
      "value.name: expected string, got number",
      "value.age: expected number, got string",
      "value.role: value did not match any option in union",
      "value.tags[1]: expected string, got number",
    ]);
  });

  test("reports missing required properties without checking absent optional ones", () => {
    const result = validate(userSchema, { age: 30, role: "admin" });
    assert.deepEqual(result, { valid: false, errors: ["value.name: missing required property"] });
  });

  test("top-level primitive mismatch", () => {
    assert.deepEqual(validate({ type: "string" }, 42), {
      valid: false,
      errors: ["value: expected string, got number"],
    });
  });

  test("top-level object schema against null and an array reports 'null'/'array' as the actual type", () => {
    const schema = { type: "object", properties: {}, required: [] };
    assert.deepEqual(validate(schema, null), { valid: false, errors: ["value: expected object, got null"] });
    assert.deepEqual(validate(schema, []), { valid: false, errors: ["value: expected object, got array"] });
  });

  test("literal mismatch reports both values via JSON.stringify", () => {
    const result = validate({ type: "literal", value: "admin" }, "user");
    assert.deepEqual(result, { valid: false, errors: ['value: expected "admin", got "user"'] });
  });

  test("nested array of objects accumulates per-item errors with indexed paths", () => {
    const schema = {
      type: "array",
      items: { type: "object", properties: { id: { type: "number" } }, required: ["id"] },
    };
    const result = validate(schema, [{ id: 1 }, { id: "x" }, {}]);
    assert.deepEqual(result, {
      valid: false,
      errors: ["value[1].id: expected number, got string", "value[2].id: missing required property"],
    });
  });

  test("extra properties not listed in the schema are ignored", () => {
    const schema = { type: "object", properties: { id: { type: "number" } }, required: ["id"] };
    assert.deepEqual(validate(schema, { id: 1, extra: "ignored" }), { valid: true, errors: [] });
  });
});

describe("match", () => {
  const handlers = {
    circle: (s) => Math.PI * s.radius ** 2,
    rectangle: (s) => s.width * s.height,
  };

  test("dispatches to the handler matching value.kind", () => {
    assert.equal(match({ kind: "rectangle", width: 3, height: 4 }, handlers), 12);
    assert.equal(match({ kind: "circle", radius: 2 }, handlers), Math.PI * 4);
  });

  test("passes the full value object to the handler", () => {
    let received;
    match({ kind: "rectangle", width: 3, height: 4 }, {
      rectangle: (value) => {
        received = value;
      },
    });
    assert.deepEqual(received, { kind: "rectangle", width: 3, height: 4 });
  });

  test("throws an Error mentioning the unhandled kind", () => {
    assert.throws(() => match({ kind: "triangle", a: 1, b: 2, c: 3 }, handlers), /triangle/);
  });

  test("returns the handler's return value, including undefined", () => {
    assert.equal(match({ kind: "circle", radius: 0 }, handlers), 0);
  });
});

describe("createResult", () => {
  const R = createResult();

  test("ok()/err() produce tagged objects", () => {
    assert.deepEqual(R.ok(5), { kind: "ok", value: 5 });
    assert.deepEqual(R.err("boom"), { kind: "err", error: "boom" });
  });

  test("isOk()/isErr() report the variant", () => {
    assert.equal(R.isOk(R.ok(1)), true);
    assert.equal(R.isErr(R.ok(1)), false);
    assert.equal(R.isOk(R.err("x")), false);
    assert.equal(R.isErr(R.err("x")), true);
  });

  test("map() transforms ok values and passes through err unchanged", () => {
    assert.deepEqual(R.map(R.ok(5), (x) => x * 2), { kind: "ok", value: 10 });
    const failure = R.err("boom");
    assert.equal(R.map(failure, (x) => x * 2), failure);
  });

  test("mapErr() transforms err values and passes through ok unchanged", () => {
    assert.deepEqual(R.mapErr(R.err("boom"), (e) => e.toUpperCase()), { kind: "err", error: "BOOM" });
    const success = R.ok(5);
    assert.equal(R.mapErr(success, (e) => e.toUpperCase()), success);
  });

  test("unwrapOr() returns the value for ok and the default for err", () => {
    assert.equal(R.unwrapOr(R.ok(5), 0), 5);
    assert.equal(R.unwrapOr(R.err("boom"), 0), 0);
  });

  test("andThen() chains ok results and short-circuits on err", () => {
    assert.deepEqual(R.andThen(R.ok(5), (x) => R.ok(x + 1)), { kind: "ok", value: 6 });
    assert.deepEqual(R.andThen(R.ok(5), () => R.err("fail")), { kind: "err", error: "fail" });
    const failure = R.err("boom");
    assert.equal(R.andThen(failure, (x) => R.ok(x + 1)), failure);
  });
});

describe("createTypedList", () => {
  test("push() accepts values matching the guard, tracked in insertion order", () => {
    const numbers = createTypedList((v) => typeof v === "number", "number");
    numbers.push(1);
    numbers.push(2);
    assert.equal(numbers.length, 2);
    assert.deepEqual(numbers.toArray(), [1, 2]);
  });

  test("push() rejects values failing the guard with a descriptive TypeError", () => {
    const numbers = createTypedList((v) => typeof v === "number", "number");
    assert.throws(() => numbers.push("x"), TypeError);
    assert.throws(() => numbers.push("x"), /Expected number, got "x"/);
    assert.equal(numbers.length, 0);
  });

  test("pop() removes and returns the last element", () => {
    const numbers = createTypedList((v) => typeof v === "number", "number");
    numbers.push(1);
    numbers.push(2);
    assert.equal(numbers.pop(), 2);
    assert.equal(numbers.length, 1);
    assert.deepEqual(numbers.toArray(), [1]);
  });

  test("pop() on an empty list throws an Error mentioning 'empty'", () => {
    const empty = createTypedList(() => true, "any");
    assert.throws(() => empty.pop(), /empty/);
  });

  test("toArray() returns a copy that doesn't alias internal state", () => {
    const numbers = createTypedList((v) => typeof v === "number", "number");
    numbers.push(1);
    const copy = numbers.toArray();
    copy.push(999);
    assert.deepEqual(numbers.toArray(), [1]);
  });
});
