/**
 * A range of numbers from `start` (inclusive) to `end` (EXCLUSIVE), stepping
 * by `step`. Implements `[Symbol.iterator]()` so instances work with
 * spread, `for...of`, `Array.from`, and destructuring.
 *
 * Supports a negative `step` for descending ranges. Each call to
 * `[Symbol.iterator]()` returns a FRESH iterator with its own position, so
 * an instance can be iterated multiple times independently (two separate
 * `for...of` loops over the same `Range` both see the full sequence).
 *
 * Throws if `step === 0` (would never terminate).
 *
 * [...new Range(0, 10, 2)]  -> [0, 2, 4, 6, 8]
 * [...new Range(5, 0, -1)]  -> [5, 4, 3, 2, 1]
 * [...new Range(0, 5)]      -> [0, 1, 2, 3, 4]   (step defaults to 1)
 * [...new Range(0, 0)]      -> []
 *
 * const r = new Range(0, 3);
 * [...r]; -> [0, 1, 2]
 * [...r]; -> [0, 1, 2]  (iterating again works — fresh iterator each time)
 *
 * @param {number} start
 * @param {number} end
 * @param {number} [step=1]
 */
export class Range {
  constructor(start, end, step = 1) {
    throw new Error("Not implemented");
  }

  [Symbol.iterator]() {
    throw new Error("Not implemented");
  }
}

/**
 * An infinite generator yielding 1, 2, 3, 4, ... forever. Never sets
 * `done: true` — only safe to consume lazily (e.g. `for...of` with `break`,
 * or via `take`, exercise 3).
 *
 * const gen = naturalNumbers();
 * gen.next(); -> { value: 1, done: false }
 * gen.next(); -> { value: 2, done: false }
 * gen.next(); -> { value: 3, done: false }
 *
 * @returns {Generator<number, void, void>}
 */
export function* naturalNumbers() {
  throw new Error("Not implemented");
}

/**
 * Yield up to `n` items from `iterable`, then stop. Does not call `.next()`
 * on `iterable` more than `n` times — safe to use with infinite iterables
 * (e.g. `naturalNumbers()`). If `iterable` has fewer than `n` items, yields
 * all of them and stops early. `n <= 0` yields nothing (and does not pull
 * from `iterable` at all).
 *
 * [...take(naturalNumbers(), 5)] -> [1, 2, 3, 4, 5]
 * [...take([1, 2], 5)]           -> [1, 2]
 * [...take([1, 2, 3], 0)]        -> []
 *
 * @param {Iterable<*>} iterable
 * @param {number} n
 * @returns {Generator<*, void, void>}
 */
export function* take(iterable, n) {
  throw new Error("Not implemented");
}

/**
 * Lazily split `iterable` into chunks of up to `size` consecutive items.
 * The last chunk may have fewer than `size` items. Throws if `size <= 0`.
 *
 * This is the LAZY/iterable counterpart to
 * `fundamentals/07-arrays-and-array-methods`'s array-based `chunk` (which
 * eagerly takes a whole array and returns a whole array of arrays).
 * `chunkIterable` works on ANY iterable — including infinite ones, when
 * combined with `take` — because it only pulls items as they're yielded.
 *
 * [...chunkIterable([1, 2, 3, 4, 5], 2)] -> [[1, 2], [3, 4], [5]]
 * [...chunkIterable([], 3)]              -> []
 * [...take(chunkIterable(naturalNumbers(), 2), 3)] -> [[1, 2], [3, 4], [5, 6]]
 *
 * @param {Iterable<*>} iterable
 * @param {number} size
 * @returns {Generator<Array<*>, void, void>}
 */
export function* chunkIterable(iterable, size) {
  throw new Error("Not implemented");
}

/**
 * Lazily pair up one item from EACH of `...iterables` per yield, stopping as
 * soon as ANY iterable is exhausted (result length = shortest input length).
 *
 * This is the N-ARY/lazy counterpart to
 * `fundamentals/07-arrays-and-array-methods`'s `zip` (which takes exactly 2
 * arrays and returns a whole array of pairs). `zipIterables` accepts any
 * number of iterables — including infinite ones — and never pulls more than
 * one extra item past the shortest input, so it's safe to mix in
 * `naturalNumbers()`.
 *
 * [...zipIterables([1, 2, 3], ["a", "b", "c"])] -> [[1, "a"], [2, "b"], [3, "c"]]
 * [...zipIterables(['a','b','c'], naturalNumbers())] -> [['a',1], ['b',2], ['c',3]]
 * [...zipIterables([1, 2], ["a", "b", "c"], [true, false])] -> [[1, "a", true], [2, "b", false]]
 *
 * @param {...Iterable<*>} iterables
 * @returns {Generator<Array<*>, void, void>}
 */
export function* zipIterables(...iterables) {
  throw new Error("Not implemented");
}
