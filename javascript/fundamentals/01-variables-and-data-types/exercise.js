/**
 * Return a precise lowercase type tag for `value`, using
 * `Object.prototype.toString.call(value)`.
 *
 * Examples: "array", "object", "null", "undefined", "number", "string",
 * "boolean", "function", "date", "regexp", "map", "set".
 *
 * @param {*} value
 * @returns {string}
 */
export function preciseTypeOf(value) {
  throw new Error("Not implemented");
}

/**
 * Recursively compare `a` and `b` for deep equality.
 *
 * - Primitives are compared with `Object.is` (so `NaN` equals `NaN`, but
 *   `+0` and `-0` differ).
 * - Arrays are equal if they have the same length and all elements are
 *   deeply equal, in order.
 * - Plain objects are equal if they have the same set of keys and every
 *   value is deeply equal.
 * - An array is never equal to a non-array object.
 *
 * @param {*} a
 * @param {*} b
 * @returns {boolean}
 */
export function deepEqual(a, b) {
  throw new Error("Not implemented");
}

/**
 * Count how many elements of `values` are truthy.
 *
 * @param {Array<*>} values
 * @returns {number}
 */
export function countTruthy(values) {
  throw new Error("Not implemented");
}

/**
 * Convert every element of `values` to a number (via `Number()`) and sum
 * the results. If ANY element converts to `NaN`, return `null` instead of
 * a number.
 *
 * safeNumberSum(['1', '2', '3'])   -> 6
 * safeNumberSum(['1', 'abc', '3']) -> null
 * safeNumberSum([])                -> 0
 *
 * @param {Array<*>} values
 * @returns {number|null}
 */
export function safeNumberSum(values) {
  throw new Error("Not implemented");
}

/**
 * Recursively clone arrays and plain objects. Primitives are returned
 * as-is. The clone must not share array/object references with the
 * original at any depth — mutating the clone must not affect the
 * original, and vice versa.
 *
 * @param {*} value
 * @returns {*}
 */
export function cloneDeep(value) {
  throw new Error("Not implemented");
}
