# 06. Promises & Async/Await

## The 3 states

A `Promise` is an object representing a value that may not exist yet:

- **pending** — initial state, neither fulfilled nor rejected.
- **fulfilled** — operation succeeded, has a resulting value.
- **rejected** — operation failed, has a reason (usually an `Error`).

Once **settled** (fulfilled or rejected), a promise's state and value are
**final and immutable** — calling `resolve`/`reject` again does nothing, and
`.then()`/`.catch()` callbacks added later still fire with that same outcome:

```js
const p = new Promise((resolve, reject) => {
  resolve("done");
  reject(new Error("ignored")); // no-op — already settled
});
p.then((v) => console.log(v)); // "done"
```

## `async`/`await` is sugar over `.then()`

An `async function` **always returns a Promise** — if the body returns a
plain value, it's wrapped; if it throws, the promise rejects:

```js
async function f() {
  return 42;
}
f(); // Promise<fulfilled: 42> — NOT 42 directly
f().then((v) => console.log(v)); // 42

async function g() {
  throw new Error("boom");
}
g().catch((e) => console.log(e.message)); // "boom"
```

`await` pauses the `async function` until the awaited promise settles, then
either returns the fulfilled value or throws the rejection reason. These two
are equivalent:

```js
// .then() chain
function getUserThen(id) {
  return fetchUser(id).then((user) => fetchPosts(user.id));
}

// async/await
async function getUserAwait(id) {
  const user = await fetchUser(id);
  return fetchPosts(user.id);
}
```

`await` only works **inside an `async function`** — or at the top level of an
ES module (top-level await). Using it elsewhere is a syntax error.

## `Promise.all` / `allSettled` / `race` / `any`

Given an iterable of promises (or plain values — non-promises are treated as
already-fulfilled):

- **`Promise.all(promises)`** — resolves with an array of all results, in
  order, once EVERY promise fulfills. **Fails fast**: rejects immediately with
  the FIRST rejection reason, without waiting for the others.
- **`Promise.allSettled(promises)`** — waits for ALL to settle, **never
  rejects**. Resolves with an array of
  `{ status: "fulfilled", value }` or `{ status: "rejected", reason }` per
  input, in order.
- **`Promise.race(promises)`** — settles as soon as the FIRST promise settles,
  **whether it wins or loses** (fulfillment or rejection — whichever happens
  first wins).
- **`Promise.any(promises)`** — settles as soon as the FIRST promise
  **fulfills**, ignoring rejections along the way. Only rejects if ALL inputs
  reject (with an `AggregateError` containing all reasons).

```js
const ok = Promise.resolve("ok");
const fail = Promise.reject(new Error("fail"));

await Promise.all([ok, fail]).catch((e) => e.message); // "fail" — fails fast
await Promise.allSettled([ok, fail]);
// [{status:"fulfilled", value:"ok"}, {status:"rejected", reason: Error("fail")}]
await Promise.race([ok, fail]); // "ok" (or "fail" if fail settles first)
await Promise.any([fail, ok]); // "ok" — first FULFILLED, rejection ignored
```

An empty array: `Promise.all([])` resolves to `[]`; `Promise.any([])` rejects
with `AggregateError` (no promise can ever fulfill).

## Sequential vs concurrent execution

`await` inside a `for...of` loop runs each iteration **one at a time** — total
time is the SUM of each delay:

```js
// Sequential: ~30ms total (10ms + 10ms + 10ms)
for (const ms of [10, 10, 10]) {
  await delay(ms);
}
```

`Promise.all(array.map(asyncFn))` starts ALL calls immediately (synchronously,
during `.map()`), THEN awaits them together — total time is the MAX of the
delays, since they overlap:

```js
// Concurrent: ~10ms total (all three run in parallel)
await Promise.all([10, 10, 10].map((ms) => delay(ms)));
```

Use sequential when each step depends on the previous result, or when
concurrent requests would overwhelm a resource (rate limits, DB connections).
Use concurrent when steps are independent.

## Error handling

`try/catch` around `await` catches rejections just like thrown exceptions:

```js
async function load() {
  try {
    return await fetchData();
  } catch (err) {
    console.error("failed:", err.message);
    return null;
  }
}
```

**Unhandled promise rejections**: a promise chain that is never `await`ed and
has no `.catch()` produces an unhandled rejection — Node logs a warning (and
in strict configurations can crash the process):

```js
fetchData().then((d) => use(d)); // if fetchData() rejects, this is UNHANDLED

// fix: add .catch(), or await it inside a try/catch
fetchData()
  .then((d) => use(d))
  .catch((err) => console.error(err));
```

## `AbortController` for cancellation

`AbortController` provides a `signal` that async operations can observe to
stop early. Calling `.abort()` transitions `signal.aborted` to `true` and
fires the signal's `"abort"` event — `fetch` and many APIs accept `{ signal }`
and reject with an `AbortError`/`DOMException` when aborted:

```js
const controller = new AbortController();
const { signal } = controller;

setTimeout(() => controller.abort(), 50); // cancel after 50ms

try {
  await fetch("https://example.com", { signal });
} catch (err) {
  if (err.name === "AbortError") console.log("cancelled");
}
```

For custom async functions (no built-in `signal` support), check
`signal.aborted` manually or reject via an `"abort"` listener:

```js
function abortableDelay(ms, signal) {
  return new Promise((resolve, reject) => {
    if (signal.aborted) return reject(new Error("aborted"));
    const id = setTimeout(resolve, ms);
    signal.addEventListener("abort", () => {
      clearTimeout(id);
      reject(new Error("aborted"));
    });
  });
}
```

## Further Reading (MDN)

- [Using promises](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Using_promises)
- [`Promise.all()`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Promise/all)
- [`Promise.allSettled()`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Promise/allSettled)
- [`Promise.race()`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Promise/race)
- [`Promise.any()`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Promise/any)
- [`async function`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/async_function)
- [`await`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/await)
- [`AbortController`](https://developer.mozilla.org/en-US/docs/Web/API/AbortController)
