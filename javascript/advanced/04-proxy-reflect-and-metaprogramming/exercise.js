/**
 * Create a plain object wrapped in a `Proxy` that enforces a type schema on
 * assignment.
 *
 * `schema` maps property names to an expected `typeof` result (e.g.
 * `"string"`, `"number"`, `"boolean"`, `"object"`).
 *
 * - Setting a property that's in `schema`: if `typeof value` matches the
 *   schema's type, the assignment succeeds normally. Otherwise throw a
 *   `TypeError` whose message mentions both the property name and the
 *   expected type — and the property's existing value is left UNCHANGED.
 * - Setting a property NOT in `schema`: always allowed, no type check.
 * - Reading any property behaves normally (returns `undefined` if unset).
 *
 * @param {Record<string, string>} schema
 * @returns {Record<string, any>}
 *
 * @example
 * const user = createValidatedObject({ name: "string", age: "number" });
 * user.name = "Ada";
 * user.age = 30; // ok
 * user.age = "thirty"; // throws TypeError mentioning "age" and "number"
 * user.age; // still 30
 * user.extra = { anything: true }; // ok — "extra" has no schema entry
 */
export function createValidatedObject(schema) {
  throw new Error("Not implemented");
}

/**
 * Wrap an array in a `Proxy` that supports Python-style negative indexing,
 * while passing everything else (positive indices, `.length`, array
 * methods like `.map`) through to the underlying array unchanged.
 *
 * - `proxy[-1]` reads/writes the LAST element (`arr[arr.length - 1]`).
 * - `proxy[-2]` reads/writes the SECOND-TO-LAST element, etc.
 * - `proxy[-n]` for `n > arr.length` reads as `undefined` (no special
 *   bounds checking beyond the underlying array's normal behavior).
 * - Non-negative-integer keys (`0`, `1`, `"length"`, `"map"`, ...) behave
 *   exactly as they would on the plain array.
 *
 * @param {any[]} arr
 * @returns {any[]}
 *
 * @example
 * const a = createNegativeIndexArray([10, 20, 30]);
 * a[0];     // 10
 * a[-1];    // 30
 * a[-2];    // 20
 * a.length; // 3
 * a.map((x) => x * 2); // [20, 40, 60]
 * a[-1] = 99;
 * a[2]; // 99
 */
export function createNegativeIndexArray(arr) {
  throw new Error("Not implemented");
}

/**
 * Recursively wrap `obj` in `Proxy`es that make it (and all nested
 * objects/arrays reachable from it) read-only.
 *
 * - Reading a property returns its value; if that value is itself a
 *   non-null object/array, it is ALSO wrapped (recursively, lazily — only
 *   when accessed).
 * - Any `set` (assignment) at ANY depth throws a `TypeError` whose message
 *   mentions the property name, and does NOT modify the underlying object.
 * - Any `delete` at ANY depth throws a `TypeError` whose message mentions
 *   the property name, and does NOT modify the underlying object.
 *
 * @param {object} obj
 * @returns {object}
 *
 * @example
 * const frozen = deepFreeze({ a: 1, b: { c: 2 } });
 * frozen.a;       // 1
 * frozen.b.c;     // 2
 * frozen.a = 5;   // throws TypeError, frozen.a is still 1
 * frozen.b.c = 5; // throws TypeError, frozen.b.c is still 2
 * delete frozen.a; // throws TypeError
 */
export function deepFreeze(obj) {
  throw new Error("Not implemented");
}

/**
 * Wrap `target` in a `Proxy` that calls `onChange` whenever a property's
 * value actually CHANGES (using `!==`).
 *
 * - `onChange` is called with a single object:
 *   `{ property, oldValue, newValue }`.
 * - Setting a property to the SAME value it already has (`!==` is `false`)
 *   does NOT call `onChange`.
 * - Setting a previously-unset property: `oldValue` is `undefined`.
 * - Reading properties behaves normally (returns the current value).
 *
 * @param {Record<string, any>} target
 * @param {(change: { property: string | symbol, oldValue: any, newValue: any }) => void} onChange
 * @returns {Record<string, any>}
 *
 * @example
 * const changes = [];
 * const state = createObservable({}, (c) => changes.push(c));
 * state.count = 1; // changes: [{ property: "count", oldValue: undefined, newValue: 1 }]
 * state.count = 1; // no new entry — value didn't change
 * state.count = 2; // changes: [..., { property: "count", oldValue: 1, newValue: 2 }]
 */
export function createObservable(target, onChange) {
  throw new Error("Not implemented");
}

/**
 * Wrap `target` in a `Proxy` that logs every METHOD CALL to `log`.
 *
 * - Accessing a property whose value is a function returns a wrapper
 *   function. Calling the wrapper:
 *   - Invokes the original method with `this` bound to `target` (NOT the
 *     proxy) and the same arguments.
 *   - On success: pushes `{ method, args, result }` onto `log` (`args` is
 *     an array) and returns `result`.
 *   - On throw: pushes `{ method, args, error }` onto `log` and RE-THROWS
 *     the same error.
 * - Accessing a property whose value is NOT a function returns it unchanged
 *   (not logged).
 *
 * @param {Record<string, any>} target
 * @param {Array<{ method: string | symbol, args: any[], result?: any, error?: any }>} log
 * @returns {Record<string, any>}
 *
 * @example
 * const log = [];
 * const calc = createMethodLogger({
 *   add(a, b) { return a + b; },
 *   fail() { throw new Error("boom"); },
 * }, log);
 * calc.add(2, 3); // 5
 * // log: [{ method: "add", args: [2, 3], result: 5 }]
 * try { calc.fail(); } catch {}
 * // log: [..., { method: "fail", args: [], error: Error("boom") }]
 */
export function createMethodLogger(target, log) {
  throw new Error("Not implemented");
}
