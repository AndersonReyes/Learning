import { describe, test } from "node:test";
import assert from "node:assert/strict";
import {
  AppError,
  ValidationError,
  NotFoundError,
  safeJsonParse,
  retry,
  aggregateErrors,
  getErrorChain,
} from "./exercise.js";

describe("AppError", () => {
  test("sets name, message, and code", () => {
    const err = new AppError("boom", { code: "BOOM" });
    assert.equal(err.name, "AppError");
    assert.equal(err.message, "boom");
    assert.equal(err.code, "BOOM");
    assert.equal(err.cause, undefined);
  });

  test("wires up cause via super", () => {
    const root = new Error("root");
    const err = new AppError("boom", { cause: root });
    assert.equal(err.cause, root);
  });

  test("works with no options at all", () => {
    const err = new AppError("boom");
    assert.equal(err.name, "AppError");
    assert.equal(err.message, "boom");
    assert.equal(err.code, undefined);
    assert.equal(err.cause, undefined);
  });

  test("is an instance of Error", () => {
    const err = new AppError("boom");
    assert.ok(err instanceof Error);
    assert.ok(err instanceof AppError);
    assert.ok(err.stack); // real Error instance -> has a stack trace
  });

  describe("toJSON", () => {
    test("includes name, message, code, and cause message", () => {
      const err = new AppError("boom", {
        code: "BOOM",
        cause: new Error("root"),
      });
      assert.deepEqual(err.toJSON(), {
        name: "AppError",
        message: "boom",
        code: "BOOM",
        cause: "root",
      });
    });

    test("cause and code are undefined when not provided", () => {
      const err = new AppError("boom");
      assert.deepEqual(err.toJSON(), {
        name: "AppError",
        message: "boom",
        code: undefined,
        cause: undefined,
      });
    });
  });
});

describe("ValidationError / NotFoundError", () => {
  test("ValidationError is distinct from NotFoundError", () => {
    const v = new ValidationError("bad input");
    const n = new NotFoundError("missing");

    assert.ok(v instanceof ValidationError);
    assert.ok(v instanceof AppError);
    assert.ok(v instanceof Error);
    assert.ok(!(v instanceof NotFoundError));

    assert.ok(n instanceof NotFoundError);
    assert.ok(n instanceof AppError);
    assert.ok(!(n instanceof ValidationError));
  });

  test("each subclass reports its own constructor name", () => {
    assert.equal(new ValidationError("x").name, "ValidationError");
    assert.equal(new NotFoundError("x").name, "NotFoundError");
  });

  test("subclasses support code and cause options", () => {
    const root = new Error("root cause");
    const err = new ValidationError("bad field", {
      code: "INVALID_FIELD",
      cause: root,
    });
    assert.equal(err.code, "INVALID_FIELD");
    assert.equal(err.cause, root);
    assert.deepEqual(err.toJSON(), {
      name: "ValidationError",
      message: "bad field",
      code: "INVALID_FIELD",
      cause: "root cause",
    });
  });
});

describe("safeJsonParse", () => {
  test("returns ok:true with the parsed value on valid JSON", () => {
    assert.deepEqual(safeJsonParse('{"a":1}'), { ok: true, value: { a: 1 } });
    assert.deepEqual(safeJsonParse("[1,2,3]"), { ok: true, value: [1, 2, 3] });
    assert.deepEqual(safeJsonParse("42"), { ok: true, value: 42 });
    assert.deepEqual(safeJsonParse("null"), { ok: true, value: null });
  });

  test("returns ok:false with an Error on invalid JSON", () => {
    const result = safeJsonParse("not json");
    assert.equal(result.ok, false);
    assert.ok(result.error instanceof Error);
  });

  test("returns ok:false on empty string input", () => {
    const result = safeJsonParse("");
    assert.equal(result.ok, false);
    assert.ok(result.error instanceof Error);
  });

  test("never throws", () => {
    assert.doesNotThrow(() => safeJsonParse("{bad"));
    assert.doesNotThrow(() => safeJsonParse(""));
    assert.doesNotThrow(() => safeJsonParse("{}"));
  });
});

describe("retry", () => {
  test("returns the result immediately on first success (sync fn)", async () => {
    let calls = 0;
    const result = await retry(() => {
      calls++;
      return "ok";
    });
    assert.equal(result, "ok");
    assert.equal(calls, 1);
  });

  test("returns the resolved value on first success (async fn)", async () => {
    let calls = 0;
    const result = await retry(async () => {
      calls++;
      return "ok-async";
    });
    assert.equal(result, "ok-async");
    assert.equal(calls, 1);
  });

  test("retries on failure and returns success once fn stops throwing", async (t) => {
    t.mock.timers.enable({ apis: ["setTimeout"] });

    let calls = 0;
    const promise = retry(
      () => {
        calls++;
        if (calls < 3) throw new Error(`fail ${calls}`);
        return "ok";
      },
      { attempts: 5, delayMs: 100 },
    );

    // First call happens synchronously (fails), then waits delayMs.
    await t.mock.timers.tickAsync(100); // -> 2nd call (fails), wait again
    await t.mock.timers.tickAsync(100); // -> 3rd call (succeeds)

    assert.equal(await promise, "ok");
    assert.equal(calls, 3);
  });

  test("throws the LAST error if all attempts fail", async (t) => {
    t.mock.timers.enable({ apis: ["setTimeout"] });

    let calls = 0;
    const promise = retry(
      () => {
        calls++;
        throw new Error(`fail ${calls}`);
      },
      { attempts: 3, delayMs: 50 },
    );

    await t.mock.timers.tickAsync(50);
    await t.mock.timers.tickAsync(50);

    await assert.rejects(promise, (err) => {
      assert.equal(err.message, "fail 3");
      return true;
    });
    assert.equal(calls, 3);
  });

  test("propagates a rejected promise as the failure", async (t) => {
    t.mock.timers.enable({ apis: ["setTimeout"] });

    let calls = 0;
    const promise = retry(
      async () => {
        calls++;
        if (calls < 2) throw new Error("async fail");
        return "recovered";
      },
      { attempts: 3, delayMs: 10 },
    );

    await t.mock.timers.tickAsync(10);

    assert.equal(await promise, "recovered");
    assert.equal(calls, 2);
  });

  test("default options: attempts=3, delayMs=0", async (t) => {
    t.mock.timers.enable({ apis: ["setTimeout"] });

    let calls = 0;
    const promise = retry(() => {
      calls++;
      throw new Error(`fail ${calls}`);
    });

    await t.mock.timers.tickAsync(0);
    await t.mock.timers.tickAsync(0);

    await assert.rejects(promise, /fail 3/);
    assert.equal(calls, 3);
  });

  test("does not retry when attempts is 1", async () => {
    let calls = 0;
    const promise = retry(
      () => {
        calls++;
        throw new Error("only try");
      },
      { attempts: 1, delayMs: 100 },
    );

    await assert.rejects(promise, /only try/);
    assert.equal(calls, 1);
  });
});

describe("aggregateErrors", () => {
  test("collects results and errors, preserving order", () => {
    const boom = new Error("bad");
    const result = aggregateErrors([
      () => 1,
      () => {
        throw boom;
      },
      () => 3,
    ]);

    assert.deepEqual(result.results, [1, undefined, 3]);
    assert.equal(result.errors.length, 1);
    assert.equal(result.errors[0].index, 1);
    assert.equal(result.errors[0].error, boom);
  });

  test("all functions succeed -> empty errors array", () => {
    const result = aggregateErrors([() => "a", () => "b"]);
    assert.deepEqual(result.results, ["a", "b"]);
    assert.deepEqual(result.errors, []);
  });

  test("all functions throw -> results all undefined", () => {
    const errA = new Error("a");
    const errB = new Error("b");
    const result = aggregateErrors([
      () => {
        throw errA;
      },
      () => {
        throw errB;
      },
    ]);

    assert.deepEqual(result.results, [undefined, undefined]);
    assert.deepEqual(
      result.errors.map((e) => e.index),
      [0, 1],
    );
    assert.equal(result.errors[0].error, errA);
    assert.equal(result.errors[1].error, errB);
  });

  test("empty input -> empty results and errors", () => {
    const result = aggregateErrors([]);
    assert.deepEqual(result.results, []);
    assert.deepEqual(result.errors, []);
  });

  test("results length always matches fns length", () => {
    const result = aggregateErrors([
      () => "ok",
      () => {
        throw new Error("x");
      },
      () => "also ok",
      () => {
        throw new Error("y");
      },
    ]);

    assert.equal(result.results.length, 4);
    assert.deepEqual(result.results, ["ok", undefined, "also ok", undefined]);
    assert.deepEqual(
      result.errors.map((e) => e.index),
      [1, 3],
    );
  });
});

describe("getErrorChain", () => {
  test("returns just the error itself when there is no cause", () => {
    const err = new Error("solo");
    assert.deepEqual(getErrorChain(err), [{ name: "Error", message: "solo" }]);
  });

  test("follows a chain of causes in order", () => {
    const root = new Error("root");
    const mid = new Error("mid", { cause: root });
    const top = new Error("top", { cause: mid });

    assert.deepEqual(getErrorChain(top), [
      { name: "Error", message: "top" },
      { name: "Error", message: "mid" },
      { name: "Error", message: "root" },
    ]);
  });

  test("includes custom error names in the chain", () => {
    const root = new TypeError("bad type");
    const top = new AppError("wrapped", { cause: root });

    assert.deepEqual(getErrorChain(top), [
      { name: "AppError", message: "wrapped" },
      { name: "TypeError", message: "bad type" },
    ]);
  });

  test("stops if cause is not an Error", () => {
    const top = new Error("top", { cause: "just a string" });
    assert.deepEqual(getErrorChain(top), [{ name: "Error", message: "top" }]);
  });

  test("caps a circular cause chain instead of looping forever", () => {
    const a = new Error("a");
    const b = new Error("b", { cause: a });
    a.cause = b; // a -> b -> a -> ... circular

    const chain = getErrorChain(a);

    assert.ok(chain.length <= 10);
    assert.ok(chain.length >= 2);
    assert.deepEqual(chain[0], { name: "Error", message: "a" });
    assert.deepEqual(chain[1], { name: "Error", message: "b" });
  });
});
