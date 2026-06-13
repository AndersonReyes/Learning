/**
 * Recursively merge `source` into `target`, returning a new object. Plain
 * objects are merged key by key, recursively. Arrays and any other values
 * in `source` replace the corresponding value in `target` wholesale (they
 * are NOT merged element-by-element). Neither input is mutated.
 *
 * deepMerge({ a: 1, b: { x: 1, y: 2 } }, { b: { y: 3, z: 4 }, c: 5 })
 *   -> { a: 1, b: { x: 1, y: 3, z: 4 }, c: 5 }
 *
 * deepMerge({ a: [1, 2] }, { a: [3, 4, 5] }) -> { a: [3, 4, 5] }
 *
 * @param {object} target
 * @param {object} source
 * @returns {object}
 */
export function deepMerge(target, source) {
  throw new Error("Not implemented");
}

/**
 * Read a nested value from `obj` using a dot-separated `path`. Return
 * `undefined` if any part of the path doesn't exist.
 *
 * getPath({ a: { b: { c: 42 } } }, "a.b.c") -> 42
 * getPath({ a: { b: { c: 42 } } }, "a.x.c") -> undefined
 * getPath({ a: 1 }, "a")                    -> 1
 *
 * @param {object} obj
 * @param {string} path
 * @returns {*}
 */
export function getPath(obj, path) {
  throw new Error("Not implemented");
}

/**
 * Return a NEW object equal to `obj` but with `value` set at the
 * dot-separated `path`, creating intermediate objects as needed. `obj` and
 * all its nested objects are left unchanged.
 *
 * setPath({ a: { b: 1 } }, "a.c", 2) -> { a: { b: 1, c: 2 } }
 * setPath({}, "a.b.c", 42)           -> { a: { b: { c: 42 } } }
 *
 * @param {object} obj
 * @param {string} path
 * @param {*} value
 * @returns {object}
 */
export function setPath(obj, path, value) {
  throw new Error("Not implemented");
}

/**
 * Recursively freeze `obj` and all nested plain objects/arrays using
 * Object.freeze, then return `obj` itself (same reference).
 *
 * const frozen = deepFreeze({ a: 1, b: { c: 2 } });
 * Object.isFrozen(frozen)   -> true
 * Object.isFrozen(frozen.b) -> true
 *
 * @param {object} obj
 * @returns {object}
 */
export function deepFreeze(obj) {
  throw new Error("Not implemented");
}

/**
 * Group the keys of `obj` by the `typeof` their values.
 *
 * groupByTypeofValue({ a: 1, b: "x", c: true, d: 2, e: "y" })
 *   -> { number: ["a", "d"], string: ["b", "e"], boolean: ["c"] }
 *
 * groupByTypeofValue({}) -> {}
 *
 * @param {object} obj
 * @returns {Object<string, string[]>}
 */
export function groupByTypeofValue(obj) {
  throw new Error("Not implemented");
}
