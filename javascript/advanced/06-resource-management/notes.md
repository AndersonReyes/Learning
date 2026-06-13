# Advanced 06. Resource Management

> Adapted scope: the `using` / `await using` declaration syntax (and the
> `DisposableStack` / `AsyncDisposableStack` / `SuppressedError` globals) are
> **NOT supported in Node 22** (verified — `typeof DisposableStack` is
> `"undefined"`, and `using` parses as a plain identifier, not a
> declaration keyword). The well-known symbols `Symbol.dispose` and
> `Symbol.asyncDispose` themselves DO exist. This topic's code uses the
> manual `Symbol.dispose`/`Symbol.asyncDispose` + `try`/`finally` pattern
> that `using` desugars to — the syntax below is documented for when you use
> a newer runtime, but isn't executed.

## The problem: deterministic cleanup

Resources (file handles, DB connections, locks, timers) need cleanup whether
the code that uses them succeeds, returns early, or throws. The manual
pattern is `try`/`finally`:

```js
const file = openFile("data.txt");
try {
  process(file);
} finally {
  file.close(); // always runs
}
```

This gets unwieldy with MULTIPLE resources (nested `try`/`finally` for each)
or when a resource is only conditionally acquired.

## `Symbol.dispose` / `Symbol.asyncDispose` (available in Node 22)

A "Disposable" object implements `[Symbol.dispose]()` (sync cleanup); an
"AsyncDisposable" implements `[Symbol.asyncDispose]()` (returns a `Promise`,
for cleanup that's itself async — e.g. closing a network connection).

```js
function openResource(name) {
  console.log(`open ${name}`);
  return {
    name,
    [Symbol.dispose]() {
      console.log(`close ${name}`);
    },
  };
}

function withResource(acquire, use) {
  const resource = acquire();
  try {
    return use(resource);
  } finally {
    resource[Symbol.dispose](); // always runs, even if use() throws
  }
}
```

## `using` / `await using` (NOT supported in Node 22 — conceptual)

In a runtime that supports the Explicit Resource Management proposal, the
`withResource` helper above becomes unnecessary — `using` calls
`[Symbol.dispose]()` automatically at the end of the enclosing block:

```js
// Conceptual — NOT run in this repo (Node 22 doesn't parse `using`)
function process() {
  using file = openResource("data.txt");
  doWork(file);
  // file[Symbol.dispose]() called here automatically,
  // even if doWork() throws or this function returns early.
}
```

`await using` is the async form, calling `[Symbol.asyncDispose]()` (falling
back to `[Symbol.dispose]()` if only that's defined) and `await`-ing it:

```js
// Conceptual
async function process() {
  await using conn = await openConnection();
  await conn.query("...");
  // await conn[Symbol.asyncDispose]() called here automatically
}
```

Multiple `using` declarations in the same block are disposed in **REVERSE
(LIFO) order** — last declared, first disposed. This matters when resources
depend on each other (e.g. a transaction must be rolled back BEFORE its
connection is closed).

## `DisposableStack` / `AsyncDisposableStack` (NOT available in Node 22 — conceptual)

A `DisposableStack` collects multiple disposables and disposes them all (in
LIFO order) with one call — useful for "acquire N resources, clean up all of
them" without nested `try`/`finally`:

```js
// Conceptual — NOT available in Node 22
function process() {
  using stack = new DisposableStack();
  const a = stack.use(openResource("a"));
  const b = stack.use(openResource("b"));
  doWork(a, b);
  // stack disposes b, then a, automatically
}
```

`exercise.js`'s `createDisposableStack()` implements a manual subset of this
(`use`, `dispose`, `disposed`) using `Symbol.dispose` + `AggregateError` (in
place of the unavailable `SuppressedError`) for reporting multiple disposal
failures.

## `SuppressedError` (NOT available in Node 22 — conceptual)

If BOTH the body of a `using` block AND a resource's `[Symbol.dispose]()`
throw, the spec wraps them in a `SuppressedError` (the dispose error becomes
`.error`, the original becomes `.suppressed`) so neither is silently lost.
This repo's manual `createDisposableStack` instead collects ALL disposal
errors into an `AggregateError` (covered in
[Advanced 02](../02-design-patterns)).

## Gotchas

- **Partial acquisition failure**: if acquiring resource 2 of 3 throws,
  resource 1 must STILL be disposed before the error propagates — easy to
  forget with manual `try`/`finally` chains (see `acquireAll` in
  `exercise.js`).
- **LIFO disposal order**: dispose resources in the REVERSE order they were
  acquired — a resource acquired later may depend on one acquired earlier
  (e.g. a cursor depends on its connection).
- **Lazy resources**: if a resource is created on-demand and never actually
  used, disposing it should be a no-op — don't create something just to
  immediately destroy it (see `createLazyResource`).
- **Async disposal needs `await`**: `[Symbol.asyncDispose]()` returns a
  `Promise` — forgetting to `await` it in a `finally` block means cleanup
  may not have finished before the function returns.

## Further Reading (MDN)

- [Resource management](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Resource_management)
- [`Symbol.dispose`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Symbol/dispose)
- [`Symbol.asyncDispose`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Symbol/asyncDispose)
- [`AggregateError`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/AggregateError) (used in `createDisposableStack`)
