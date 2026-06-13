/**
 * Return whether `n` is a prime number, using trial division up to
 * `sqrt(n)`. Numbers less than 2 are not prime.
 *
 * @param {number} n
 * @returns {boolean}
 */
export function isPrime(n) {
  throw new Error("Not implemented");
}

/**
 * Return all prime numbers from 2 up to and including `n`, using the
 * Sieve of Eratosthenes.
 *
 * primesUpTo(10) -> [2, 3, 5, 7]
 * primesUpTo(1)  -> []
 *
 * @param {number} n
 * @returns {number[]}
 */
export function primesUpTo(n) {
  throw new Error("Not implemented");
}

/**
 * Transpose a 2D array (matrix): rows become columns and vice versa.
 * Assume `matrix` is rectangular (every row has the same length).
 *
 * transpose([[1, 2, 3], [4, 5, 6]]) -> [[1, 4], [2, 5], [3, 6]]
 *
 * @param {Array<Array<*>>} matrix
 * @returns {Array<Array<*>>}
 */
export function transpose(matrix) {
  throw new Error("Not implemented");
}

/**
 * Run-length encode a string: each maximal run of the same character is
 * replaced with the character followed by the run's length (even if 1).
 *
 * runLengthEncode("aaabbc") -> "a3b2c1"
 * runLengthEncode("abc")    -> "a1b1c1"
 * runLengthEncode("")       -> ""
 *
 * @param {string} str
 * @returns {string}
 */
export function runLengthEncode(str) {
  throw new Error("Not implemented");
}

/**
 * Return the length of the longest run of consecutive equal elements.
 *
 * longestConsecutiveRun([1, 1, 2, 2, 2, 3]) -> 3
 * longestConsecutiveRun([1, 2, 3])          -> 1
 * longestConsecutiveRun([])                 -> 0
 *
 * @param {Array<*>} arr
 * @returns {number}
 */
export function longestConsecutiveRun(arr) {
  throw new Error("Not implemented");
}
