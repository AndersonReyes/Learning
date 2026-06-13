# 05. Error Handling

## `try` / `catch` / `finally`

```js
try {
  riskyOperation();
} catch (err) {
  console.error("failed:", err.message);
} finally {
  cleanup(); // ALWAYS runs — success, caught error, or uncaught error
}
```

- `catch` only runs if `try` throws.
- `finally` runs **no matter what** — normal completion, `return`,
  `throw`, or an error that's never caught (it runs before the error
  propagates).
- The `catch` binding is optional if you don't need the error value:

```js
try {
  JSON.parse(input);
} catch {
  return defaultConfig; // don't care what went wrong
}
```

## Gotcha: `finally`'s `return` overrides everything

If `finally` itself has a `return` (or `throw`), it **replaces** any
pending return/throw from `try`/`catch`:

```js
function f() {
  try {
    return "from try";
  } finally {
    return "from finally"; // wins
  }
}
f(); // "from finally" — "from try" is discarded
```

Avoid `return`/`throw` inside `finally` unless that's deliberately the
point (e.g. always-fail cleanup). Use `finally` for side effects
(closing files, releasing locks), not control flow.

## `throw` accepts any value — but throw `Error`s

```js
throw "oops";        // valid JS, but...
throw { code: 42 };  // ...neither has a stack trace
throw new Error("oops"); // always do this — has `.message`, `.stack`, `.name`
```

Always throw `Error` instances (or subclasses). Non-`Error` throws lose
`.stack`, making bugs hard to trace, and `catch (err)` callers can't rely
on `err.message` existing.

## Custom error classes

`extends Error` to create domain-specific error types. Two things you
**must** do:

1. Call `super(message)` (and pass `options` through for `cause`,
   see below).
2. Set `this.name = this.constructor.name` — without it, `name` stays
   `"Error"` for every subclass (it's inherited, not auto-set), which
   breaks `console.log`/`err.toString()` output and any logic switching
   on `err.name`.

```js
class AppError extends Error {
  constructor(message, options) {
    super(message, options);
    this.name = this.constructor.name; // "AppError" (or subclass name)
  }
}

class ValidationError extends AppError {}

const err = new ValidationError("bad input");
err.name;          // "ValidationError" — constructor.name resolves per-instance
err instanceof ValidationError; // true
err instanceof AppError;        // true
err instanceof Error;           // true
```

Extra fields (e.g. `code`, `field`) are just assigned in the constructor
like any class property.

## `Error` cause chaining (ES2022)

`new Error(message, { cause })` attaches a lower-level error as
`error.cause` — wrap-and-rethrow without losing the original:

```js
try {
  parseConfig(raw);
} catch (err) {
  throw new Error("failed to load config", { cause: err });
}

// later, at the top level:
catch (err) {
  console.error(err.message);       // "failed to load config"
  console.error(err.cause.message); // original parse error
}
```

Chains can nest arbitrarily deep (`err.cause.cause.cause...`) — useful
for preserving context as an error crosses layer boundaries (parser ->
service -> HTTP handler) while still seeing the root cause.

## Result type — alternative to exceptions

For **expected** failures (parsing, validation, lookups), returning a
tagged result avoids `try/catch` at every call site:

```js
function parseNumber(s) {
  const n = Number(s);
  return Number.isNaN(n)
    ? { ok: false, error: new Error(`invalid number: ${s}`) }
    : { ok: true, value: n };
}

const result = parseNumber("42");
if (result.ok) {
  console.log(result.value); // 42
} else {
  console.error(result.error.message);
}
```

- Use **exceptions** for truly exceptional/programmer-error conditions
  (bugs, invariant violations) — let them propagate and crash loudly.
- Use **Result types** for expected, recoverable failures — the caller
  is forced (by the shape) to handle both branches, and chaining many
  fallible steps doesn't require nested `try/catch`.

## Gotcha: silently swallowing errors

```js
try {
  doSomething();
} catch (err) {
  // nothing here — error vanishes, bug becomes invisible
}
```

Empty `catch` blocks hide failures until they cause confusing problems
far away. Always do ONE of:

- log it (`console.error(err)`),
- rethrow it (possibly wrapped with `cause`),
- or handle it with a deliberate fallback — and comment *why* it's safe
  to ignore.

## Further Reading (MDN)

- [Control flow and error handling — error handling](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Control_flow_and_error_handling#error_handling)
- [`try...catch`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/try...catch)
- [`throw`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/throw)
- [`Error`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Error)
- [`Error: cause`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Error/cause)
- [`Error.prototype.name`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Error/name)
