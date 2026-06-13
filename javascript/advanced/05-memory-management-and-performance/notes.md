# Advanced 05. Memory Management & Performance

> Adapted scope: GC internals/timing aren't deterministically testable in
> `node:test`. This topic's `exercise.js` focuses on OBSERVABLE caching/reuse
> patterns (LRU cache, object pool, bounded memoization) that are correct
> regardless of when the GC actually runs. `WeakMap`/`WeakRef` are covered
> here conceptually and via `WeakMap` (which IS deterministic).

## How JS garbage collection works (conceptually)

- V8 uses **mark-and-sweep**: starting from "roots" (global object, currently
  executing call stack), it marks every object REACHABLE by following
  references. Anything not marked is garbage, and its memory is reclaimed.
- You don't free memory manually — an object becomes eligible for collection
  the moment NOTHING reachable references it anymore.
- **Reference counting alone isn't used** (it can't handle cycles — two
  objects referencing each other but unreachable from roots are still
  collected under mark-and-sweep).

## Common memory leak patterns

- **Forgotten timers/intervals**: `setInterval` callbacks (and anything they
  close over) stay alive until `clearInterval` — a classic leak source for
  long-running processes.
- **Unremoved event listeners**: `emitter.on(...)` keeps the handler (and its
  closure) alive for the emitter's lifetime. Always pair with `off`/
  `removeListener` or use `once`.
- **Unbounded caches/collections**: a `Map`/array that only grows (e.g.
  caching every request by URL forever) grows without limit — see "Bounded
  memoization" below.
- **Closures capturing more than needed**: a closure retains its ENTIRE
  enclosing scope. A long-lived callback that closes over a large object but
  only needs one field still keeps the whole object alive.
- **Global variables**: anything attached to `globalThis` lives for the
  process's entire lifetime.

## `WeakMap` / `WeakSet`

- Like `Map`/`Set`, but keys (WeakMap) / members (WeakSet) must be objects
  (or symbols, not registered symbols) and are held WEAKLY — if nothing else
  references a key, the entry can be garbage-collected WITHOUT you removing
  it.
- NOT enumerable — no `.keys()`, `.size`, `for...of`. (You can't iterate
  "what's still alive" because that would itself be observable GC timing.)
- **Use case**: associate metadata with an object FOR AS LONG AS the object
  is alive, without keeping it alive yourself and without manual cleanup.
  ```js
  const cache = new WeakMap();
  function compute(obj) {
    if (cache.has(obj)) return cache.get(obj);
    const result = expensive(obj);
    cache.set(obj, result);
    return result;
  }
  ```
  When `obj` becomes unreachable elsewhere, its cache entry can be collected
  too — no leak, no manual `cache.delete(obj)` needed.
- `cache.set(primitiveValue, x)` throws `TypeError` — primitives can't be
  weak keys (they're not garbage-collected the same way objects are).

## `WeakRef` & `FinalizationRegistry` (conceptual — non-deterministic)

```js
const ref = new WeakRef(someObject);
ref.deref(); // someObject, or `undefined` if it's been collected

const registry = new FinalizationRegistry((heldValue) => {
  console.log(`cleaned up: ${heldValue}`);
});
registry.register(someObject, "someObject's label");
```

- `WeakRef.deref()` returns the referenced object, or `undefined` once it's
  been collected — but WHEN that happens is up to the engine.
- `FinalizationRegistry` callbacks run at an UNSPECIFIED time after the
  object is collected — possibly never (e.g. if the process exits first).
- **Never use these for program CORRECTNESS** (e.g. releasing a file handle,
  decrementing a counter that must be accurate). Only for OPTIONAL
  cleanup/diagnostics (e.g. clearing a cache entry as a memory optimization —
  the cache must still work correctly if the callback never fires).

## Performance patterns

### LRU (Least Recently Used) cache

Bound a cache to `capacity` entries; when full, evict the entry that hasn't
been accessed in the longest time. A `Map` works well — re-inserting a key
moves it to the end (most-recently-used); the first key in iteration order is
the least-recently-used.

### Object pool

For objects that are expensive to create and frequently
created/destroyed (e.g. buffers, connections), REUSE instances instead of
allocating new ones each time: `acquire()` returns a free instance (creating
one if none are free), `release(obj)` resets it and returns it to the pool.
Reduces GC churn from allocation/collection cycles.

### Bounded memoization

Plain memoization (`cache.set(key, result)` forever) is a memory leak for
caches with unbounded key spaces. Bound it:
- By SIZE (LRU eviction), or
- By TIME (TTL — a cached result expires after `N` ms and is recomputed).

### Batching

Many small operations (DB writes, network calls, DOM updates) are often
cheaper done as one larger operation. Accumulate items and flush either when
a size threshold is hit OR after a time window — whichever comes first.

## Measuring performance

- `performance.now()` — high-resolution timestamp (milliseconds, sub-ms
  precision), unaffected by system clock changes — prefer over `Date.now()`
  for measuring DURATIONS.
- `console.time(label)` / `console.timeEnd(label)` — quick wall-clock
  measurements during development.
- Avoid premature optimization — profile (`node --prof`, or
  `node --inspect` + Chrome DevTools) to find ACTUAL hot paths before
  rewriting code for speed.

## Further Reading (MDN)

- [Memory management](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Memory_management)
- [`WeakMap`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WeakMap)
- [`WeakRef`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WeakRef)
- [`FinalizationRegistry`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/FinalizationRegistry)
- [`Performance.now()`](https://developer.mozilla.org/en-US/docs/Web/API/Performance/now)
