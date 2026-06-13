/**
 * Group array elements into an object keyed by `keyFn(element)`. Each key
 * maps to an array of elements that produced that key, in original order.
 *
 * groupBy([1, 2, 3, 4], n => n % 2 === 0 ? "even" : "odd")
 *   -> { odd: [1, 3], even: [2, 4] }
 *
 * @param {Array<*>} array
 * @param {(item: *) => string} keyFn
 * @returns {Object<string, Array<*>>}
 */
export function groupBy(array, keyFn) {
  throw new Error("Not implemented");
}

/**
 * Split `array` into chunks of length `size`. The last chunk may be
 * shorter if `array.length` is not a multiple of `size`.
 *
 * chunk([1, 2, 3, 4, 5], 2) -> [[1, 2], [3, 4], [5]]
 * chunk([], 3)              -> []
 *
 * @param {Array<*>} array
 * @param {number} size
 * @returns {Array<Array<*>>}
 */
export function chunk(array, size) {
  throw new Error("Not implemented");
}

/**
 * Pair up elements from `arrayA` and `arrayB` by index. The result length
 * is the length of the shorter array.
 *
 * zip([1, 2, 3], ["a", "b", "c"]) -> [[1, "a"], [2, "b"], [3, "c"]]
 * zip([1, 2], ["a", "b", "c"])    -> [[1, "a"], [2, "b"]]
 *
 * @param {Array<*>} arrayA
 * @param {Array<*>} arrayB
 * @returns {Array<Array<*>>}
 */
export function zip(arrayA, arrayB) {
  throw new Error("Not implemented");
}

/**
 * Return the unique values present in both `arrayA` and `arrayB`,
 * preserving the order they first appear in `arrayA`.
 *
 * intersection([1, 2, 2, 3], [2, 3, 4]) -> [2, 3]
 * intersection([1, 2], [3, 4])          -> []
 *
 * @param {Array<*>} arrayA
 * @param {Array<*>} arrayB
 * @returns {Array<*>}
 */
export function intersection(arrayA, arrayB) {
  throw new Error("Not implemented");
}

/**
 * Return a new array of `items` sorted ascending by each property named in
 * `keys`, in priority order (earlier keys take precedence; later keys
 * break ties). Does not mutate `items`.
 *
 * sortByMultipleKeys(
 *   [{ last: "Smith", first: "Bob" }, { last: "Adams", first: "Zoe" }, { last: "Smith", first: "Ann" }],
 *   ["last", "first"]
 * ) -> [
 *   { last: "Adams", first: "Zoe" },
 *   { last: "Smith", first: "Ann" },
 *   { last: "Smith", first: "Bob" },
 * ]
 *
 * @param {Array<Object>} items
 * @param {string[]} keys
 * @returns {Array<Object>}
 */
export function sortByMultipleKeys(items, keys) {
  throw new Error("Not implemented");
}
