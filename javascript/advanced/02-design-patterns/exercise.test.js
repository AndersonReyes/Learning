import { test, describe } from "node:test";
import assert from "node:assert/strict";
import {
  createEventBus,
  createCommandHistory,
  createStateMachine,
  createQueryBuilder,
  createPipeline,
} from "./exercise.js";

describe("createEventBus", () => {
  test("calls handlers for an event in registration order with the emitted args", () => {
    const bus = createEventBus();
    const calls = [];
    bus.on("greet", (name) => calls.push(`a:${name}`));
    bus.on("greet", (name) => calls.push(`b:${name}`));
    bus.emit("greet", "world");
    assert.deepEqual(calls, ["a:world", "b:world"]);
  });

  test("the unsubscribe function returned by on() stops future calls", () => {
    const bus = createEventBus();
    const calls = [];
    const unsubscribe = bus.on("x", () => calls.push("handler"));
    bus.emit("x");
    unsubscribe();
    bus.emit("x");
    assert.deepEqual(calls, ["handler"]);
  });

  test("off(event, handler) removes a previously-registered handler", () => {
    const bus = createEventBus();
    const calls = [];
    const handler = () => calls.push("handler");
    bus.on("x", handler);
    bus.off("x", handler);
    bus.emit("x");
    assert.deepEqual(calls, []);
  });

  test("off() for a handler that was never registered is a no-op", () => {
    const bus = createEventBus();
    assert.doesNotThrow(() => bus.off("x", () => {}));
  });

  test("once() handlers fire exactly once even with multiple emits", () => {
    const bus = createEventBus();
    const calls = [];
    bus.once("x", () => calls.push("fired"));
    bus.emit("x");
    bus.emit("x");
    bus.emit("x");
    assert.deepEqual(calls, ["fired"]);
  });

  test("emitting an event with no handlers is a no-op", () => {
    const bus = createEventBus();
    assert.doesNotThrow(() => bus.emit("nothing", 1, 2, 3));
  });

  test("if a handler throws, sibling handlers still run, then emit throws AggregateError", () => {
    const bus = createEventBus();
    const calls = [];
    bus.on("x", () => calls.push("a"));
    bus.on("x", () => {
      throw new Error("boom");
    });
    bus.on("x", () => calls.push("c"));

    assert.throws(() => bus.emit("x"), AggregateError);
    assert.deepEqual(calls, ["a", "c"]);
  });

  test("handlers added during emit do not run in the same emit cycle", () => {
    const bus = createEventBus();
    const calls = [];
    bus.on("x", () => {
      calls.push("first");
      bus.on("x", () => calls.push("added-during-emit"));
    });
    bus.emit("x");
    assert.deepEqual(calls, ["first"]);
    bus.emit("x");
    assert.deepEqual(calls, ["first", "first", "added-during-emit"]);
  });
});

describe("createCommandHistory", () => {
  function makeCounter() {
    let value = 0;
    const increment = (amount) => ({
      execute: () => (value += amount),
      undo: () => (value -= amount),
    });
    return { increment, getValue: () => value };
  }

  test("execute applies the command and returns its result", () => {
    const { increment, getValue } = makeCounter();
    const history = createCommandHistory();
    const result = history.execute(increment(5));
    assert.equal(result, 5);
    assert.equal(getValue(), 5);
  });

  test("undo reverses the most recent command; redo reapplies it", () => {
    const { increment, getValue } = makeCounter();
    const history = createCommandHistory();
    history.execute(increment(5));
    history.execute(increment(3));
    assert.equal(getValue(), 8);

    assert.equal(history.undo(), true);
    assert.equal(getValue(), 5);

    assert.equal(history.undo(), true);
    assert.equal(getValue(), 0);

    assert.equal(history.canUndo(), false);
    assert.equal(history.undo(), false);
    assert.equal(getValue(), 0);

    assert.equal(history.redo(), true);
    assert.equal(getValue(), 5);
  });

  test("executing a new command after undo() clears the redo stack", () => {
    const { increment, getValue } = makeCounter();
    const history = createCommandHistory();
    history.execute(increment(5));
    history.execute(increment(3));
    history.undo();
    assert.equal(history.canRedo(), true);

    history.execute(increment(10));
    assert.equal(getValue(), 15);
    assert.equal(history.canRedo(), false);
    assert.equal(history.redo(), false);
  });

  test("canUndo/canRedo reflect stack state from a fresh history", () => {
    const history = createCommandHistory();
    assert.equal(history.canUndo(), false);
    assert.equal(history.canRedo(), false);
  });
});

describe("createStateMachine", () => {
  const definition = {
    initial: "idle",
    states: {
      idle: { on: { FETCH: "loading" } },
      loading: { on: { SUCCESS: "success", ERROR: "failure" } },
      success: { on: { FETCH: "loading" } },
      failure: { on: { FETCH: "loading", RETRY: "loading" } },
    },
  };

  test("starts in the initial state with a one-element history", () => {
    const machine = createStateMachine(definition);
    assert.equal(machine.state, "idle");
    assert.deepEqual(machine.history, ["idle"]);
  });

  test("send() on a valid transition updates state and returns the new state", () => {
    const machine = createStateMachine(definition);
    const result = machine.send("FETCH");
    assert.equal(result, "loading");
    assert.equal(machine.state, "loading");
  });

  test("can() reflects valid/invalid transitions from the current state", () => {
    const machine = createStateMachine(definition);
    assert.equal(machine.can("FETCH"), true);
    assert.equal(machine.can("SUCCESS"), false);
    machine.send("FETCH");
    assert.equal(machine.can("SUCCESS"), true);
    assert.equal(machine.can("FETCH"), false);
  });

  test("send() on an invalid transition throws, mentioning the event and current state", () => {
    const machine = createStateMachine(definition);
    assert.throws(() => machine.send("SUCCESS"), /SUCCESS/);
    assert.throws(() => machine.send("SUCCESS"), /idle/);
    assert.equal(machine.state, "idle"); // unchanged after a failed send
  });

  test("history tracks every visited state in order and is returned as a copy", () => {
    const machine = createStateMachine(definition);
    machine.send("FETCH"); // idle -> loading
    machine.send("ERROR"); // loading -> failure
    machine.send("RETRY"); // failure -> loading
    machine.send("SUCCESS"); // loading -> success

    assert.deepEqual(machine.history, [
      "idle",
      "loading",
      "failure",
      "loading",
      "success",
    ]);

    const snapshot = machine.history;
    snapshot.push("mutated");
    assert.deepEqual(machine.history, [
      "idle",
      "loading",
      "failure",
      "loading",
      "success",
    ]);
  });
});

describe("createQueryBuilder", () => {
  test("builds a full query with select/from/where/orderBy/limit", () => {
    const sql = createQueryBuilder()
      .select("id", "name")
      .from("users")
      .where("age > 18")
      .where("active = true")
      .orderBy("name")
      .limit(10)
      .build();

    assert.equal(
      sql,
      "SELECT id, name FROM users WHERE age > 18 AND active = true ORDER BY name ASC LIMIT 10",
    );
  });

  test("defaults to SELECT * when select() is never called", () => {
    const sql = createQueryBuilder().from("users").build();
    assert.equal(sql, "SELECT * FROM users");
  });

  test("build() throws if from() was never called", () => {
    assert.throws(() => createQueryBuilder().select("id").build(), Error);
  });

  test("orderBy() with an explicit direction is respected", () => {
    const sql = createQueryBuilder()
      .from("users")
      .orderBy("created_at", "DESC")
      .build();
    assert.equal(sql, "SELECT * FROM users ORDER BY created_at DESC");
  });

  test("omits WHERE, ORDER BY, and LIMIT clauses when not configured", () => {
    const sql = createQueryBuilder().from("users").build();
    assert.equal(sql, "SELECT * FROM users");
  });

  test("each chainable method returns the same builder instance", () => {
    const builder = createQueryBuilder();
    assert.equal(builder.select("id"), builder);
    assert.equal(builder.from("users"), builder);
    assert.equal(builder.where("1=1"), builder);
    assert.equal(builder.orderBy("id"), builder);
    assert.equal(builder.limit(5), builder);
  });
});

describe("createPipeline", () => {
  test("runs middleware in onion order: before/after around next()", async () => {
    const pipeline = createPipeline();
    pipeline.use(async (ctx, next) => {
      ctx.log.push("a-before");
      await next();
      ctx.log.push("a-after");
    });
    pipeline.use(async (ctx, next) => {
      ctx.log.push("b-before");
      await next();
      ctx.log.push("b-after");
    });
    pipeline.use(async (ctx, next) => {
      ctx.log.push("c");
      await next();
    });

    const ctx = { log: [] };
    await pipeline.run(ctx);

    assert.deepEqual(ctx.log, [
      "a-before",
      "b-before",
      "c",
      "b-after",
      "a-after",
    ]);
  });

  test("a middleware that doesn't call next() short-circuits later middleware but earlier ones still unwind", async () => {
    const pipeline = createPipeline();
    pipeline.use(async (ctx, next) => {
      ctx.log.push("a-before");
      await next();
      ctx.log.push("a-after");
    });
    pipeline.use(async (ctx) => {
      ctx.log.push("b-no-next");
    });
    pipeline.use(async (ctx) => {
      ctx.log.push("c-never-runs");
    });

    const ctx = { log: [] };
    await pipeline.run(ctx);

    assert.deepEqual(ctx.log, ["a-before", "b-no-next", "a-after"]);
  });

  test("calling next() more than once rejects run()", async () => {
    const pipeline = createPipeline();
    pipeline.use(async (ctx, next) => {
      await next();
      await next();
    });
    pipeline.use(async () => {});

    await assert.rejects(pipeline.run({}));
  });

  test("running an empty pipeline resolves with ctx unchanged", async () => {
    const pipeline = createPipeline();
    const ctx = { log: [] };
    const result = await pipeline.run(ctx);
    assert.deepEqual(result, { log: [] });
  });

  test("use() returns the pipeline, enabling chained .use().use()", () => {
    const pipeline = createPipeline();
    const result = pipeline.use(async () => {}).use(async () => {});
    assert.equal(result, pipeline);
  });

  test("an error thrown inside a middleware rejects run()", async () => {
    const pipeline = createPipeline();
    pipeline.use(async () => {
      throw new Error("middleware failed");
    });

    await assert.rejects(pipeline.run({}), /middleware failed/);
  });
});
