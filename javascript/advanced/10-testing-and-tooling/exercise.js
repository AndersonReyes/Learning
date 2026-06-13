/**
 * Create a small assertion library. Each method does nothing on success and
 * throws an `Error` with a descriptive message on failure.
 *
 * Returns `{ equal, deepEqual, ok, throws }`:
 * - `equal(actual, expected, message?)`: passes if `Object.is(actual,
 *   expected)` (so `NaN` equals `NaN`, but `0` !== `-0`). On failure, throws
 *   `message` if given, else an `Error` whose message mentions both values.
 * - `deepEqual(actual, expected, message?)`: passes if `actual` and
 *   `expected` are recursively structurally equal (same keys/values for
 *   objects, same elements for arrays, primitives compared with
 *   `Object.is`). On failure, throws `message` if given, else a descriptive
 *   `Error`.
 * - `ok(value, message?)`: passes if `value` is truthy. On failure, throws
 *   `message` if given, else a descriptive `Error`.
 * - `throws(fn, message?)`: passes if calling `fn()` throws (any error). On
 *   failure (fn did NOT throw), throws `message` if given, else a
 *   descriptive `Error`.
 *
 * @returns {{
 *   equal: (actual: any, expected: any, message?: string) => void,
 *   deepEqual: (actual: any, expected: any, message?: string) => void,
 *   ok: (value: any, message?: string) => void,
 *   throws: (fn: () => void, message?: string) => void,
 * }}
 */
export function createAssert() {
  throw new Error("Not implemented");
}

/**
 * Create a mock function that records its calls and results.
 *
 * The returned function `mock(...args)`:
 * - Records `args` (an array) in `mock.calls`, in call order.
 * - Invokes the current implementation (initially `implementation`, or a
 *   no-op returning `undefined` if `implementation` is omitted) with `args`
 *   and `this`.
 * - Records `{ type: "return", value }` or `{ type: "throw", value: error }`
 *   in `mock.results` for each call, matching whether the implementation
 *   returned or threw.
 * - Returns the implementation's return value, or re-throws its error.
 *
 * Also has:
 * - `mock.mockReturnValue(value)`: subsequent calls return `value`
 *   (replaces the current implementation).
 * - `mock.mockImplementation(fn)`: subsequent calls invoke `fn(...args)`.
 * - `mock.reset()`: clears `mock.calls` and `mock.results` (does NOT change
 *   the current implementation).
 *
 * @param {(...args: any[]) => any} [implementation]
 * @returns {((...args: any[]) => any) & {
 *   calls: any[][],
 *   results: Array<{ type: "return" | "throw", value: any }>,
 *   mockReturnValue: (value: any) => void,
 *   mockImplementation: (fn: (...args: any[]) => any) => void,
 *   reset: () => void,
 * }}
 */
export function createMockFn(implementation) {
  throw new Error("Not implemented");
}

/**
 * Replace `obj[methodName]` with a mock function (see `createMockFn`) that,
 * by default, calls through to the ORIGINAL method (with `this` bound to
 * `obj`) and returns/throws whatever it does.
 *
 * The returned spy has all of `createMockFn`'s properties/methods, plus:
 * - `spy.restore()`: puts the original method back on `obj`.
 *
 * @param {object} obj
 * @param {string} methodName
 * @returns {ReturnType<typeof createMockFn> & { restore: () => void }}
 *
 * @example
 * const obj = { greet(name) { return `Hello, ${name}!`; } };
 * const spy = spyOn(obj, "greet");
 * obj.greet("Ada"); // "Hello, Ada!" -- still calls the original
 * spy.calls; // [["Ada"]]
 * spy.restore();
 */
export function spyOn(obj, methodName) {
  throw new Error("Not implemented");
}

/**
 * Create a mini test runner.
 *
 * Returns `{ test(name, fn), run() }`:
 * - `test(name, fn)`: registers a test. `fn` may be sync or async (return a
 *   Promise).
 * - `run()`: runs all registered tests IN REGISTRATION ORDER, awaiting each
 *   before starting the next. A test "fails" if `fn()` throws or its
 *   returned Promise rejects; one test failing does NOT stop the others
 *   from running. Returns a Promise resolving to:
 *   `{ total, passed, failed, failures: [{ name, error }, ...] }`, where
 *   `failures` is in the order those tests were run.
 *
 * @returns {{
 *   test: (name: string, fn: () => any) => void,
 *   run: () => Promise<{ total: number, passed: number, failed: number, failures: Array<{ name: string, error: Error }> }>,
 * }}
 */
export function createTestRunner() {
  throw new Error("Not implemented");
}

/**
 * Create a snapshot matcher for "record on first run, compare on later
 * runs" style testing.
 *
 * Returns `{ match(name, value), snapshots }`:
 * - `match(name, value)`: if no snapshot named `name` has been recorded
 *   yet, records `value` (compared/stored via `JSON.stringify`) and
 *   returns. If a snapshot named `name` already exists, compares its
 *   `JSON.stringify` against the new `value`'s; if they differ, throws an
 *   `Error` whose message includes `name`. If they match, returns
 *   normally.
 * - `snapshots` (getter): returns a plain object mapping each recorded
 *   `name` to its (JSON-round-tripped) value.
 *
 * @returns {{
 *   match: (name: string, value: any) => void,
 *   readonly snapshots: Record<string, any>,
 * }}
 *
 * @example
 * const matcher = createSnapshotMatcher();
 * matcher.match("user", { id: 1, name: "Ada" }); // records
 * matcher.match("user", { id: 1, name: "Ada" }); // matches, no throw
 * matcher.match("user", { id: 1, name: "Bob" }); // throws -- mismatch
 */
export function createSnapshotMatcher() {
  throw new Error("Not implemented");
}
