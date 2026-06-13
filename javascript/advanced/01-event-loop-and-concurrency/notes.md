# Advanced 01. Event Loop, Microtasks & Concurrency Model

JS is **single-threaded**: one call stack, one thing executing at a time.
"Concurrency" comes from the **event loop** interleaving callbacks between
synchronous bursts of code — not from parallel threads.

## The model

```
Call stack  → runs synchronous code to completion (a "task")
Microtask queue → Promise .then/.catch/.finally callbacks, queueMicrotask()
Macrotask queue → setTimeout/setInterval, I/O callbacks, UI events
```

Order of operations:

1. Run the current task (top-level script, or a callback) until the call
   stack is empty.
2. Drain the **entire** microtask queue — including microtasks queued BY
   microtasks that ran during this drain (it loops until empty).
3. Run the **next** macrotask (one, not all).
4. Repeat from step 2.

```js
console.log("1 sync");
setTimeout(() => console.log("2 macrotask"), 0);
Promise.resolve().then(() => console.log("3 microtask"));
queueMicrotask(() => console.log("4 microtask"));
console.log("5 sync");
// Output: 1 sync, 5 sync, 3 microtask, 4 microtask, 2 macrotask
```

- **Microtasks always run before the next macrotask**, even
  `setTimeout(fn, 0)`.
- A microtask that queues another microtask delays macrotasks further — an
  infinite chain of `.then()` calls can **starve** the macrotask queue
  ("microtask starvation").
- Node-specific: `process.nextTick()` callbacks run BEFORE other microtasks
  (even before promise callbacks), after the current operation completes.

## async/await is sugar over microtasks

```js
async function f() {
  console.log("a");
  await null; // suspends here, resumes as a MICROTASK
  console.log("b");
}
f();
console.log("c");
// Output: a, c, b
```

- Everything after an `await` runs in a microtask (even `await
  <already-resolved-value>`).
- `await` does NOT block the thread — other code runs while "waiting."

## Concurrency control patterns

Because async operations interleave, two common needs:

1. **Rate-limit how often a function runs** (debounce/throttle) — e.g. don't
   fire a search API request on every keystroke.
2. **Limit how many async operations run at once** (concurrency pool) — e.g.
   don't fetch 1000 URLs simultaneously and exhaust sockets/memory.

### Debounce vs throttle

- **Debounce**: wait until activity STOPS for `wait` ms, then fire once with
  the latest args. Resets the timer on every call. Good for "search as you
  type" (wait until the user pauses).
- **Throttle**: fire at most once per `wait` ms. Typically fires immediately
  on the first call (leading edge) and, if more calls arrive during the
  window, fires once more after the window with the latest args (trailing
  edge). Good for scroll/resize handlers (steady rate, not silence-based).

Both are built on `setTimeout`/`clearTimeout`. Gotcha: a plain arrow-function
wrapper loses `this` — use a `function` expression for the returned wrapper
if `fn` relies on `this`, and `fn.apply(this, args)` to forward it.

### Concurrency-limited async pool

Running `items.map(iteratorFn)` + `Promise.all` starts ALL operations
immediately — fine for 5 items, a problem for 5000 (too many open
connections, rate limits). An async pool starts only `limit` operations at a
time, starting the next one as soon as a slot frees up — while still
returning results in the ORIGINAL order (track by index, not completion
order).

### Retry with exponential backoff

Transient failures (network blips, rate limits) are often worth retrying —
but retrying immediately in a tight loop can make things worse. Exponential
backoff waits `base * 2^attempt` ms between attempts (`base, base*2,
base*4, ...`), spreading retries out and giving the failing
resource time to recover.

### Async mutex (mutual exclusion)

Even single-threaded JS can have race conditions across `await` boundaries —
if two async operations both read-modify-write shared state with an `await`
in between, they can interleave and corrupt it. An async mutex serializes
access: `runExclusive(fn)` queues `fn` to run only after all previously
queued tasks have settled, guaranteeing no two `fn`s run with overlapping
"critical sections" — implemented by chaining promises (`queue =
queue.then(() => fn())`), no actual locks/threads involved.

## Further Reading (MDN)

- [The event loop](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Event_loop)
- [Using promises](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Using_promises)
- [`queueMicrotask()`](https://developer.mozilla.org/en-US/docs/Web/API/Window/queueMicrotask)
- [`setTimeout()`](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)
- [`Promise.all()`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Promise/all)
