/**
 * Reimplement Function.prototype.call without using .call, .apply, or .bind.
 *
 * Invokes `fn` with `this` set to `thisArg` and `args` passed as individual
 * arguments. Returns whatever `fn` returns.
 *
 * `thisArg` handling (primitives):
 * - `null`/`undefined` -> `fn` is invoked as a plain call, so `this` inside
 *   `fn` is `undefined` (strict mode).
 * - any other primitive (string/number/boolean/symbol/bigint) -> `thisArg`
 *   is BOXED via `Object(thisArg)` before the call, so `this` inside `fn`
 *   is a wrapper object (e.g. `Number`), not the raw primitive. Use
 *   `this.valueOf()` to get the primitive back.
 * - objects/arrays/functions -> `this` inside `fn` is `thisArg` itself
 *   (same reference).
 *
 * myCall(function () { return this.name; }, { name: "Ada" }) -> "Ada"
 * myCall(function (a, b) { return a + b; }, null, 1, 2) -> 3
 *
 * @param {Function} fn
 * @param {*} thisArg
 * @param {...*} args
 * @returns {*}
 */
export function myCall(fn, thisArg, ...args) {
  throw new Error("Not implemented");
}

/**
 * Like myCall, but arguments are passed as an array (or array-like).
 * If `argsArray` is omitted or undefined, `fn` is called with no arguments.
 * Same `thisArg` handling as myCall.
 *
 * myApply(function (a, b) { return a + b; }, null, [1, 2]) -> 3
 * myApply(function () { return this.name; }, { name: "Ada" }) -> "Ada"
 *
 * @param {Function} fn
 * @param {*} thisArg
 * @param {Array<*>} [argsArray]
 * @returns {*}
 */
export function myApply(fn, thisArg, argsArray) {
  const args = argsArray === undefined ? [] : argsArray;
  return myCall(fn, thisArg, ...args);
}

/**
 * Reimplement Function.prototype.bind without using the real .bind
 * (myCall/myApply are fine).
 *
 * Returns a new function that, when called with `callArgs`, invokes `fn`
 * with `this` set to `thisArg` and arguments `[...boundArgs, ...callArgs]`.
 *
 * NOT SUPPORTED / out of scope: calling the returned function with `new`
 * (constructor usage). Only plain calls are required to work.
 *
 * const greet = myBind(function (greeting) { return `${greeting}, ${this.name}`; }, { name: "Ada" });
 * greet("Hi") -> "Hi, Ada"
 *
 * const add5 = myBind((a, b) => a + b, null, 5);
 * add5(3) -> 8
 *
 * @param {Function} fn
 * @param {*} thisArg
 * @param {...*} boundArgs
 * @returns {Function}
 */
export function myBind(fn, thisArg, ...boundArgs) {
  throw new Error("Not implemented");
}

/**
 * Mutate `obj` so each method named in `methodNames` is bound to `obj`,
 * so it can be extracted (e.g. passed to setTimeout, used as an event
 * handler) without losing `this`. Returns `obj`.
 *
 * Throws new Error(`<name> is not a function on obj`) if a name in
 * `methodNames` is not a function property of `obj`.
 *
 * const obj = { count: 0, increment() { this.count += 1; return this.count; } };
 * bindAll(obj, ["increment"]);
 * const fn = obj.increment;
 * fn() -> 1   // works even though `fn` was extracted from `obj`
 *
 * @param {object} obj
 * @param {string[]} methodNames
 * @returns {object}
 */
export function bindAll(obj, methodNames) {
  throw new Error("Not implemented");
}

/**
 * Return a chainable numeric builder wrapping `initial`.
 *
 * Methods `add(n)`, `subtract(n)`, `multiply(n)`, `divide(n)` mutate the
 * wrapped value and return the builder itself (for chaining). `value()`
 * returns the current number.
 *
 * `divide(0)` throws new Error("Division by zero").
 *
 * createChainable(10).add(5).multiply(2).value() -> 30
 * createChainable(10).subtract(4).divide(2).value() -> 3
 *
 * @param {number} initial
 * @returns {{
 *   add: (n: number) => object,
 *   subtract: (n: number) => object,
 *   multiply: (n: number) => object,
 *   divide: (n: number) => object,
 *   value: () => number,
 * }}
 */
export function createChainable(initial) {
  throw new Error("Not implemented");
}
