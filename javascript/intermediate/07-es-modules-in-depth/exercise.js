/**
 * Simulate the "module is a singleton" pattern: a module's top-level code
 * runs ONCE on first import, and every importer gets the SAME instance.
 *
 * Returns a `getInstance()` function that:
 * - Does NOT call `factory` until the FIRST call to `getInstance()` (lazy).
 * - Calls `factory()` exactly ONCE, caching its return value.
 * - Returns the SAME cached value on every call, even if `factory` would
 *   produce a different object each time it's invoked.
 *
 * let calls = 0;
 * const getInstance = createModule(() => { calls++; return { id: calls }; });
 * calls;              // -> 0 (factory not called yet)
 * getInstance();      // -> { id: 1 } (factory called now)
 * getInstance();      // -> the SAME { id: 1 } object (===)
 * calls;              // -> 1 (factory called only once)
 *
 * @param {() => *} factory
 * @returns {() => *} getInstance
 */
export function createModule(factory) {
  throw new Error("Not implemented");
}

/**
 * Simulate `export * from "./mod.js"` aggregation across multiple modules.
 *
 * Each argument is a plain object representing a module's named exports.
 * Returns a new object containing all keys from all namespaces.
 *
 * - If the SAME key appears in more than one namespace with DIFFERENT
 *   values (compared with `===`), throw an `Error` whose message identifies
 *   the key name and the (0-based) indices of the colliding namespaces.
 * - If the SAME key appears with the SAME value (`===`) in multiple
 *   namespaces, that's not a collision — allow it.
 *
 * mergeNamespaces({ a: 1, b: 2 }, { c: 3 });
 * // -> { a: 1, b: 2, c: 3 }
 *
 * const shared = { x: 1 };
 * mergeNamespaces({ shared }, { shared });
 * // -> { shared } — same value, no collision
 *
 * mergeNamespaces({ a: 1 }, { a: 2 });
 * // throws Error mentioning "a" and namespaces 0 and 1
 *
 * @param {...Object<string, *>} namespaces
 * @returns {Object<string, *>}
 */
export function mergeNamespaces(...namespaces) {
  throw new Error("Not implemented");
}

/**
 * Simulate a dynamic `import()`-based lazy plugin registry.
 *
 * Returns `{ register(name, loader), load(name) }`:
 * - `register(name, loader)` stores `loader` (a zero-arg function returning
 *   a value OR a Promise, simulating `import("./plugin.js")`) under `name`.
 *   Overwrites any previous registration for `name`.
 * - `load(name)` returns a Promise resolving to the loaded module:
 *   - Calls `loader()` only on the FIRST `load(name)` for that name.
 *   - Caches the RESOLVED value and returns it for all subsequent
 *     `load(name)` calls (without calling `loader` again).
 *   - If `load(name)` is called multiple times CONCURRENTLY before the
 *     first call resolves, `loader` is still invoked only ONCE total (all
 *     callers share the same in-flight load).
 *   - `load` on a `name` that was never `register`-ed rejects with an
 *     `Error`.
 *
 * const registry = createPluginRegistry();
 * let calls = 0;
 * registry.register("math", async () => { calls++; return { square: (x) => x * x }; });
 * const [a, b] = await Promise.all([registry.load("math"), registry.load("math")]);
 * a === b;  // -> true (same cached module object)
 * calls;    // -> 1 (loader called once despite concurrent load calls)
 *
 * await registry.load("missing"); // rejects Error
 *
 * @returns {{
 *   register: (name: string, loader: () => (*|Promise<*>)) => void,
 *   load: (name: string) => Promise<*>,
 * }}
 */
export function createPluginRegistry() {
  throw new Error("Not implemented");
}

/**
 * Simulate a circular-dependency-safe lazy export using an accessor
 * property: defines `key` on `target` via `Object.defineProperty` with a
 * getter that:
 * - Calls `factory()` on the FIRST read of `target[key]`, returning its
 *   result.
 * - Caches that result, and replaces the accessor with a plain data
 *   property (or an equivalent getter that just returns the cached value)
 *   so subsequent reads of `target[key]` return the cached value WITHOUT
 *   calling `factory` again.
 *
 * `factory` must be called AT MOST once, even across many reads of
 * `target[key]`. Returns `target`.
 *
 * let calls = 0;
 * const target = {};
 * defineLazyExport(target, "config", () => { calls++; return { ready: true }; });
 * calls;          // -> 0 (not computed yet)
 * target.config;  // -> { ready: true } (factory called now, calls -> 1)
 * target.config;  // -> the SAME { ready: true } object, calls still -> 1
 *
 * @param {Object} target
 * @param {string} key
 * @param {() => *} factory
 * @returns {Object} target
 */
export function defineLazyExport(target, key, factory) {
  throw new Error("Not implemented");
}

/**
 * Simulate the read-only nature of `import * as ns` namespace bindings.
 *
 * Given a plain object `moduleExports`, returns an object with the same
 * enumerable own keys/values that:
 * - Allows reading existing properties normally.
 * - THROWS a `TypeError` when assigning to an existing property.
 * - THROWS a `TypeError` when adding a new property.
 *
 * (Both must throw synchronously on plain assignment — as in real ES module
 * code, which always runs in strict mode — not fail silently.)
 *
 * Implementation note: built with `Object.freeze`, which makes the returned
 * object's own properties non-writable and non-configurable. In strict-mode
 * code (this file is an ES module, so always strict), assigning to a
 * non-writable property or adding a property to a non-extensible object
 * throws `TypeError` — matching `import * as ns` semantics without needing
 * a `Proxy`.
 *
 * const ns = createReadonlyNamespace({ PI: 3.14 });
 * ns.PI;            // -> 3.14
 * ns.PI = 4;        // throws TypeError
 * ns.EXTRA = "no";  // throws TypeError
 *
 * @param {Object<string, *>} moduleExports
 * @returns {Object<string, *>}
 */
export function createReadonlyNamespace(moduleExports) {
  throw new Error("Not implemented");
}
