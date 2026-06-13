# Advanced 02. Design Patterns in JavaScript

Design patterns are reusable solution SHAPES for common problems — not
libraries to import. JS's first-class functions, closures, and dynamic
objects make many classic OOP patterns simpler than in Java/C++ (no
interfaces/abstract classes needed). MDN has no dedicated "design patterns"
guide — these patterns are language-agnostic; see Further Reading for the
underlying JS features.

## Module pattern

Already covered by [ES Modules](../../intermediate/07-es-modules-in-depth) —
`export`/`import` give file-level encapsulation (private-by-default,
explicit exports) directly, replacing the older IIFE-based module pattern:

```js
// old IIFE-based "module" — closures hide private state
const counter = (() => {
  let count = 0; // private
  return {
    increment: () => ++count,
    get: () => count,
  };
})();
```

## Singleton

Ensure only one instance exists, with lazy creation:

```js
let instance = null;
export function getConfig() {
  if (!instance) {
    instance = loadConfigFromDisk(); // expensive, done once
  }
  return instance;
}
```

- In JS, often unnecessary — a module's top-level state IS already a
  singleton (modules are cached/evaluated once). Reach for an explicit
  singleton only when initialization needs to be LAZY (deferred until first
  use) or resettable (e.g. for tests).
- Gotcha: global mutable singletons make testing hard (shared state between
  tests) — provide a reset/teardown hook, or prefer dependency injection.

## Factory

Centralize object creation, hiding the concrete type from the caller:

```js
function createShape(type, ...args) {
  switch (type) {
    case "circle": return { type, radius: args[0], area: () => Math.PI * args[0] ** 2 };
    case "square": return { type, side: args[0], area: () => args[0] ** 2 };
    default: throw new Error(`Unknown shape: ${type}`);
  }
}
```

- Useful when construction logic is non-trivial or the concrete type depends
  on runtime input (config, API response `type` field, etc.).

## Observer / Pub-Sub

Decouple "things that happen" from "things that react" — a registry of
event names to handler lists:

```js
const bus = createEventBus();
const unsubscribe = bus.on("user:created", (user) => console.log(user));
bus.emit("user:created", { id: 1 });
unsubscribe();
```

Gotchas implemented in [`exercise.js`](./exercise.js)'s `createEventBus`:

- **Snapshot handlers at emit time** — a handler that subscribes/unsubscribes
  DURING an `emit` shouldn't affect the handlers called in that SAME `emit`
  cycle (avoids "modifying a collection while iterating" bugs).
- **Isolate handler errors** — one handler throwing shouldn't prevent
  sibling handlers from running; collect errors and report them together
  (e.g. via `AggregateError`) after all handlers have run.
- **`once`** — auto-unsubscribe after the first invocation.

## Command (with undo/redo)

Encapsulate "an action + how to reverse it" as an object, enabling undo
stacks, queuing, logging:

```js
const command = {
  execute: () => { /* do the thing */ },
  undo: () => { /* reverse the thing */ },
};
history.execute(command);
history.undo(); // calls command.undo()
history.redo(); // re-calls command.execute()
```

Gotcha implemented in `createCommandHistory`: executing a NEW command after
one or more `undo()`s must clear the redo stack — otherwise `redo()` could
reapply a command whose preconditions no longer hold (the classic "undo,
then do something different, then redo" bug).

## Builder

Construct a complex object/string step by step via chained calls, deferring
the final assembly to a `.build()`:

```js
const sql = createQueryBuilder()
  .select("id", "name")
  .from("users")
  .where("active = true")
  .orderBy("name")
  .limit(10)
  .build();
```

- Each chain method returns `this`/the builder, enabling fluent syntax.
- Optional pieces (WHERE/ORDER BY/LIMIT) are only included if their builder
  method was called; required pieces (FROM) should make `.build()` throw if
  missing — fail at build time, not at use time.

## State (finite state machines)

Model an object whose allowed operations depend on its CURRENT state, with
explicit transition rules instead of scattered `if`/`switch` flag-checking:

```js
const machine = createStateMachine({
  initial: "idle",
  states: {
    idle:    { on: { FETCH: "loading" } },
    loading: { on: { SUCCESS: "success", ERROR: "failure" } },
    success: { on: { FETCH: "loading" } },
    failure: { on: { FETCH: "loading", RETRY: "loading" } },
  },
});
machine.send("FETCH"); // -> "loading"
machine.send("BOGUS"); // throws — no such transition from "loading"
```

- Centralizing transitions in a table makes invalid states/transitions
  IMPOSSIBLE rather than something to remember to check everywhere.
- `can(event)` lets UI code (e.g. disable a button) check validity without
  triggering the transition.

## Strategy

Swap an algorithm at runtime by passing it as a value — JS functions are
already first-class, so "strategy" is often just a parameter:

```js
function sortBy(items, strategy) {
  return [...items].sort(strategy);
}
sortBy(users, (a, b) => a.age - b.age);   // strategy: by age
sortBy(users, (a, b) => a.name.localeCompare(b.name)); // strategy: by name
```

- This is the same shape as the higher-order functions from
  [Higher-Order Functions & FP](../../intermediate/04-higher-order-functions-and-fp)
  — "strategy pattern" is just naming that shape when the swappable function
  represents a full algorithm/policy.

## Chain of Responsibility / Middleware (the "onion" model)

A request passes through a chain of handlers, each able to act before/after
delegating to the next — used by Express/Koa-style middleware:

```js
const pipeline = createPipeline();
pipeline.use(async (ctx, next) => {
  ctx.log.push("auth:before");
  await next();
  ctx.log.push("auth:after");
});
pipeline.use(async (ctx, next) => {
  ctx.log.push("handler");
  // doesn't call next() — nothing further runs
});
await pipeline.run({ log: [] });
// log: ["auth:before", "handler", "auth:after"]
```

Gotchas implemented in `createPipeline`:

- Each middleware receives `next` as a function it can `await` — calling it
  resumes the chain; NOT calling it short-circuits remaining middleware
  (but already-started outer middleware still "unwind" — their code AFTER
  `await next()` still runs).
- Calling `next()` more than once is a bug (double-execution of downstream
  middleware) — guard against it by throwing.

## Further Reading (MDN)

- [Object-oriented programming](https://developer.mozilla.org/en-US/docs/Learn/JavaScript/Objects/Object-oriented_programming)
- [Closures](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Closures) (underlies module/singleton/observer state)
- [Using classes](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Using_classes)
- [`AggregateError`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/AggregateError)
