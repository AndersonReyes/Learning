// Run with: node examples.js
//
// Minimal inline re-implementations of each pattern (not imported from
// exercise.js) so this file runs standalone regardless of exercise.js's
// state.

// --- Observer / Pub-Sub ---
console.log("=== Observer / Pub-Sub ===");
{
  const listeners = new Map();
  function on(event, handler) {
    if (!listeners.has(event)) listeners.set(event, new Set());
    listeners.get(event).add(handler);
    return () => listeners.get(event)?.delete(handler);
  }
  function emit(event, ...args) {
    for (const handler of listeners.get(event) ?? []) handler(...args);
  }

  const unsubscribe = on("user:created", (user) =>
    console.log(`welcome email sent to ${user.name}`),
  );
  on("user:created", (user) => console.log(`audit log: created ${user.id}`));
  emit("user:created", { id: 1, name: "Ada" });
  unsubscribe();
  console.log("-- after unsubscribe --");
  emit("user:created", { id: 2, name: "Grace" }); // only audit log fires
}

// --- Command (undo/redo) ---
console.log("\n=== Command (undo/redo) ===");
{
  const undoStack = [];
  const redoStack = [];
  function execute(command) {
    const result = command.execute();
    undoStack.push(command);
    redoStack.length = 0;
    return result;
  }
  function undo() {
    const command = undoStack.pop();
    command?.undo();
    if (command) redoStack.push(command);
  }
  function redo() {
    const command = redoStack.pop();
    command?.execute();
    if (command) undoStack.push(command);
  }

  let text = "";
  const insert = (str) => ({
    execute: () => (text += str),
    undo: () => (text = text.slice(0, -str.length)),
  });

  execute(insert("Hello"));
  execute(insert(", world"));
  console.log("text:", JSON.stringify(text)); // "Hello, world"
  undo();
  console.log("after undo:", JSON.stringify(text)); // "Hello"
  redo();
  console.log("after redo:", JSON.stringify(text)); // "Hello, world"
}

// --- State (finite state machine) ---
console.log("\n=== State (finite state machine) ===");
{
  const definition = {
    initial: "idle",
    states: {
      idle: { on: { FETCH: "loading" } },
      loading: { on: { SUCCESS: "success", ERROR: "failure" } },
      success: { on: { FETCH: "loading" } },
      failure: { on: { FETCH: "loading", RETRY: "loading" } },
    },
  };
  let current = definition.initial;
  function send(event) {
    const target = definition.states[current]?.on?.[event];
    if (!target) {
      throw new Error(
        `Invalid transition: cannot send "${event}" from state "${current}"`,
      );
    }
    current = target;
    return current;
  }

  console.log("start:", current); // idle
  console.log("FETCH ->", send("FETCH")); // loading
  console.log("ERROR ->", send("ERROR")); // failure
  try {
    send("SUCCESS");
  } catch (err) {
    console.log("SUCCESS from failure throws:", err.message);
  }
  console.log("RETRY ->", send("RETRY")); // loading
}

// --- Builder ---
console.log("\n=== Builder (SQL query) ===");
{
  function createQueryBuilder() {
    const state = { columns: null, table: null, conditions: [], order: null, limitValue: null };
    const builder = {
      select: (...columns) => ((state.columns = columns), builder),
      from: (table) => ((state.table = table), builder),
      where: (condition) => (state.conditions.push(condition), builder),
      orderBy: (column, direction = "ASC") => ((state.order = { column, direction }), builder),
      limit: (n) => ((state.limitValue = n), builder),
      build: () => {
        if (!state.table) throw new Error("Query must call .from(table) before build()");
        let sql = `SELECT ${state.columns ? state.columns.join(", ") : "*"} FROM ${state.table}`;
        if (state.conditions.length) sql += ` WHERE ${state.conditions.join(" AND ")}`;
        if (state.order) sql += ` ORDER BY ${state.order.column} ${state.order.direction}`;
        if (state.limitValue != null) sql += ` LIMIT ${state.limitValue}`;
        return sql;
      },
    };
    return builder;
  }

  const sql = createQueryBuilder()
    .select("id", "name")
    .from("users")
    .where("age > 18")
    .where("active = true")
    .orderBy("name")
    .limit(10)
    .build();
  console.log(sql);
}

// --- Chain of Responsibility / Middleware (onion model) ---
console.log("\n=== Chain of Responsibility / Middleware ===");
{
  function createPipeline() {
    const middlewares = [];
    const pipeline = {
      use(middleware) {
        middlewares.push(middleware);
        return pipeline;
      },
      async run(ctx) {
        let lastIndex = -1;
        async function dispatch(index) {
          if (index <= lastIndex) throw new Error("next() called multiple times");
          lastIndex = index;
          const middleware = middlewares[index];
          if (!middleware) return;
          await middleware(ctx, () => dispatch(index + 1));
        }
        await dispatch(0);
        return ctx;
      },
    };
    return pipeline;
  }

  const pipeline = createPipeline();
  pipeline.use(async (ctx, next) => {
    ctx.log.push("auth:before");
    await next();
    ctx.log.push("auth:after");
  });
  pipeline.use(async (ctx, next) => {
    ctx.log.push("logger:before");
    await next();
    ctx.log.push("logger:after");
  });
  pipeline.use(async (ctx) => {
    ctx.log.push("handler"); // doesn't call next() — chain stops here
  });

  const ctx = await pipeline.run({ log: [] });
  console.log(ctx.log.join(" -> "));
}
