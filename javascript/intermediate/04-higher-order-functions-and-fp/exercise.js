/**
 * Split `array` into two arrays: elements for which `predicate` returns
 * truthy, and elements for which it returns falsy. Relative order is
 * preserved within each group.
 *
 * partition([1, 2, 3, 4, 5], (n) => n % 2 === 0)
 *   -> [[2, 4], [1, 3, 5]]
 * partition([], (n) => n > 0)
 *   -> [[], []]
 *
 * @param {Array<*>} array
 * @param {(item: *, index: number, array: Array<*>) => boolean} predicate
 * @returns {[Array<*>, Array<*>]} [matching, nonMatching]
 */
export function partition(array, predicate) {
  throw new Error("Not implemented");
}

/**
 * Return a debounced wrapper around `fn`. Each call to the wrapper resets a
 * `delayMs` timer; `fn` is invoked (with the most recent call's arguments
 * and `this`) only after `delayMs` elapse with no further calls.
 *
 * The returned function has a `.cancel()` method that cancels any pending
 * invocation (a no-op if none is pending).
 *
 * const debounced = debounce(fn, 100);
 * debounced("a"); // schedules fn("a") for +100ms
 * debounced("b"); // cancels the "a" call, schedules fn("b") for +100ms
 * // ...100ms with no more calls...           -> fn("b") runs, fn("a") never does
 *
 * debounced("c");
 * debounced.cancel(); // "c" call never runs
 *
 * @param {Function} fn
 * @param {number} delayMs
 * @returns {((...args: *[]) => void) & { cancel: () => void }}
 */
export function debounce(fn, delayMs) {
  throw new Error("Not implemented");
}

/**
 * Return a throttled wrapper around `fn` ("trailing" throttle).
 *
 * - The first call invokes `fn` immediately (with its arguments and `this`).
 * - Further calls within `intervalMs` of the last invocation are recorded
 *   but do not call `fn` immediately.
 * - Once `intervalMs` has elapsed, if at least one call happened during the
 *   interval, `fn` is invoked once more with the arguments from the LATEST
 *   such call. This restarts the interval.
 * - If no calls happened during an interval, the next call after it again
 *   invokes `fn` immediately (back to the first-call behavior).
 *
 * const throttled = throttle(fn, 100);
 * throttled("a"); // fn("a") runs immediately
 * throttled("b"); // within 100ms — fn not called yet
 * throttled("c"); // within 100ms — fn not called yet
 * // ...100ms elapse...                       -> fn("c") runs (latest pending args)
 *
 * @param {Function} fn
 * @param {number} intervalMs
 * @returns {(...args: *[]) => void}
 */
export function throttle(fn, intervalMs) {
  throw new Error("Not implemented");
}

/**
 * Left-to-right function composition: pipe(f, g, h)(...args) is equivalent
 * to h(g(f(...args))).
 *
 * Only the FIRST function receives the original argument(s) (it may take
 * any number of arguments); every subsequent function receives the single
 * return value of the previous one.
 *
 * With zero functions, pipe() returns an identity function: pipe()(x) === x.
 * With one function, pipe(f) behaves like f.
 *
 * const double = (x) => x * 2;
 * const increment = (x) => x + 1;
 * pipe(double, increment)(3); // increment(double(3)) = 7
 * pipe(increment, double)(3); // double(increment(3)) = 8
 * pipe()(5); // 5
 *
 * @param {...Function} fns
 * @returns {Function}
 */
export function pipe(...fns) {
  throw new Error("Not implemented");
}

/**
 * Apply a list of transducer-style `transformers` to `array` in a SINGLE
 * pass (no intermediate arrays for each step), then fold the transformed
 * values with `reducerFn`/`initialValue`.
 *
 * Each transformer is one of:
 *   { type: "map", fn: (value) => newValue }
 *   { type: "filter", fn: (value) => boolean }
 *
 * Transformers run in array order for each element. If a "filter" rejects a
 * value, that value is skipped entirely (never reaches later transformers
 * or `reducerFn`).
 *
 * transduce(
 *   [{ type: "map", fn: (x) => x * 2 }, { type: "filter", fn: (x) => x > 5 }],
 *   (acc, x) => acc + x,
 *   0,
 *   [1, 2, 3, 4],
 * );
 * // map: [2, 4, 6, 8] -> filter >5: [6, 8] -> sum: 14
 *
 * transduce([], (acc, x) => acc + x, 0, [1, 2, 3]); // 6 (no transformers)
 *
 * @param {Array<{type: "map"|"filter", fn: Function}>} transformers
 * @param {(acc: *, value: *) => *} reducerFn
 * @param {*} initialValue
 * @param {Array<*>} array
 * @returns {*}
 */
export function transduce(transformers, reducerFn, initialValue, array) {
  throw new Error("Not implemented");
}
