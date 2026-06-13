/**
 * Run `use(resource)` with a resource obtained from `acquire()`, guaranteeing
 * `resource[Symbol.dispose]()` is called exactly once afterward — whether
 * `use` returns normally or throws.
 *
 * - On success: returns `use(resource)`'s return value (after disposing).
 * - On `use` throwing: disposes the resource, then RE-THROWS the same error.
 *
 * @param {() => { [Symbol.dispose]: () => void, [key: string]: any }} acquire
 * @param {(resource: any) => any} use
 * @returns {any}
 *
 * @example
 * const log = [];
 * const resource = {
 *   [Symbol.dispose]() { log.push("dispose"); },
 * };
 * withResource(() => resource, (r) => { log.push("use"); return "ok"; });
 * // log: ["use", "dispose"]
 */
export function withResource(acquire, use) {
  throw new Error("Not implemented");
}

/**
 * Async version of `withResource`: `acquireAsync()` and `use(resource)` may
 * both return Promises, and the resource's cleanup is
 * `resource[Symbol.asyncDispose]()` (also a `Promise`, which is `await`ed).
 *
 * Guarantees `await resource[Symbol.asyncDispose]()` runs exactly once,
 * whether `use` resolves or rejects, before `withAsyncResource` itself
 * resolves/rejects.
 *
 * @param {() => Promise<{ [Symbol.asyncDispose]: () => Promise<void>, [key: string]: any }>} acquireAsync
 * @param {(resource: any) => Promise<any>} use
 * @returns {Promise<any>}
 */
export async function withAsyncResource(acquireAsync, use) {
  throw new Error("Not implemented");
}

/**
 * Create a manual `DisposableStack`-like object that tracks multiple
 * disposables and disposes them all at once, in REVERSE (LIFO) registration
 * order.
 *
 * Returns `{ use(resource), dispose(), disposed }`:
 * - `use(resource)` registers `resource` (which has `[Symbol.dispose]()`)
 *   and returns it unchanged. Throws an `Error` if called after `dispose()`.
 * - `dispose()`: calls `[Symbol.dispose]()` on every registered resource, in
 *   REVERSE order of registration. Marks the stack as `disposed`. A second
 *   call to `dispose()` is a no-op. If ONE OR MORE resources' `dispose`
 *   throw, ALL are still attempted, and afterward `dispose()` throws an
 *   `AggregateError` containing all thrown errors.
 * - `disposed` (getter) — `true` after `dispose()` has been called.
 *
 * @returns {{
 *   use: (resource: { [Symbol.dispose]: () => void }) => any,
 *   dispose: () => void,
 *   readonly disposed: boolean,
 * }}
 *
 * @example
 * const log = [];
 * const make = (name) => ({ [Symbol.dispose]() { log.push(`dispose:${name}`); } });
 * const stack = createDisposableStack();
 * stack.use(make("a"));
 * stack.use(make("b"));
 * stack.dispose();
 * // log: ["dispose:b", "dispose:a"]
 * stack.disposed; // true
 */
export function createDisposableStack() {
  throw new Error("Not implemented");
}

/**
 * Acquire multiple resources in order, then run `use(resources)`, disposing
 * ALL successfully-acquired resources in REVERSE order afterward.
 *
 * - Calls each function in `acquires` (in order), collecting results.
 * - If `acquires[i]()` THROWS: disposes resources `0..i-1` (in reverse
 *   order, via `[Symbol.dispose]()`), then re-throws the acquisition error.
 *   `use` is NOT called.
 * - If all acquisitions succeed: calls `use(resources)` (an array, in
 *   acquisition order). Whether `use` returns or throws, disposes ALL
 *   resources (in reverse order) afterward. Returns `use`'s result, or
 *   re-throws its error (after disposal).
 *
 * @param {Array<() => { [Symbol.dispose]: () => void, [key: string]: any }>} acquires
 * @param {(resources: any[]) => any} use
 * @returns {any}
 */
export function acquireAll(acquires, use) {
  throw new Error("Not implemented");
}

/**
 * Wrap `factory` (a zero-arg function returning a disposable resource) so
 * the resource is created LAZILY — only on the first call to `get()` — and
 * disposed only if it was ever created.
 *
 * Returns `{ get(), [Symbol.dispose]() }`:
 * - `get()`: on the first call, invokes `factory()` and caches the result.
 *   Subsequent calls return the SAME cached resource (without calling
 *   `factory()` again). Throws an `Error` if called AFTER
 *   `[Symbol.dispose]()`.
 * - `[Symbol.dispose]()`: if `factory()` was never called, this is a NO-OP.
 *   Otherwise calls the underlying resource's `[Symbol.dispose]()`. A
 *   second call is a no-op (disposes at most once).
 *
 * @param {() => { [Symbol.dispose]: () => void, [key: string]: any }} factory
 * @returns {{ get: () => any, [Symbol.dispose]: () => void }}
 *
 * @example
 * let created = 0;
 * const lazy = createLazyResource(() => {
 *   created++;
 *   return { [Symbol.dispose]() {} };
 * });
 * lazy[Symbol.dispose](); // no-op, created === 0
 *
 * const lazy2 = createLazyResource(() => { created++; return { [Symbol.dispose]() {} }; });
 * lazy2.get();
 * lazy2.get(); // same resource, created only incremented once
 */
export function createLazyResource(factory) {
  throw new Error("Not implemented");
}
