/**
 * Compute n! (factorial) recursively.
 *
 * factorial(0) -> 1
 * factorial(5) -> 120
 *
 * Throw new Error("factorial is not defined for negative numbers") if
 * `n` is negative.
 *
 * @param {number} n
 * @returns {number}
 */
export function factorial(n) {
  throw new Error("Not implemented");
}

/**
 * Compute the nth Fibonacci number recursively (0-indexed):
 * fibonacci(0) -> 0, fibonacci(1) -> 1, fibonacci(2) -> 1,
 * fibonacci(3) -> 2, fibonacci(5) -> 5.
 *
 * @param {number} n
 * @returns {number}
 */
export function fibonacci(n) {
  throw new Error("Not implemented");
}

/**
 * Compose any number of single-argument functions, applying them
 * right-to-left: compose(f, g, h)(x) === f(g(h(x))).
 *
 * With no functions, compose()(x) returns x unchanged.
 *
 * @param {...Function} fns
 * @returns {Function}
 */
export function compose(...fns) {
  throw new Error("Not implemented");
}

/**
 * Curry `fn` so it can be called with any combination of argument groups,
 * collecting arguments until it has at least `fn.length` of them, then
 * calling `fn` with all collected arguments.
 *
 * const add3 = curry((a, b, c) => a + b + c);
 * add3(1)(2)(3) === 6
 * add3(1, 2)(3) === 6
 * add3(1)(2, 3) === 6
 * add3(1, 2, 3) === 6
 *
 * @param {Function} fn
 * @returns {Function}
 */
export function curry(fn) {
  throw new Error("Not implemented");
}

/**
 * Recursively flatten a nested array to any depth.
 *
 * flattenDeep([1, [2, [3, [4, [5]]]]]) -> [1, 2, 3, 4, 5]
 * flattenDeep([])                       -> []
 *
 * @param {Array<*>} arr
 * @returns {Array<*>}
 */
export function flattenDeep(arr) {
  throw new Error("Not implemented");
}
