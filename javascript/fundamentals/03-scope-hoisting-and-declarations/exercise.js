/**
 * Create a bank account object using closures to keep `balance` private —
 * it must not be accessible as a property on the returned object.
 *
 * @param {number} initialBalance
 * @returns {{
 *   deposit: (amount: number) => number,
 *   withdraw: (amount: number) => number,
 *   getBalance: () => number,
 * }}
 *
 * - deposit(amount): if amount <= 0, throw new Error("Invalid amount").
 *   Otherwise add it to the balance and return the new balance.
 * - withdraw(amount): if amount <= 0, throw new Error("Invalid amount").
 *   If amount > balance, throw new Error("Insufficient funds").
 *   Otherwise subtract it from the balance and return the new balance.
 * - getBalance(): return the current balance.
 */
export function createBankAccount(initialBalance) {
  throw new Error("Not implemented");
}

/**
 * Return a wrapped version of `fn` that only ever calls `fn` once. The
 * first call's result is cached and returned for all subsequent calls
 * (regardless of arguments), and `fn` is never invoked again.
 *
 * @param {Function} fn
 * @returns {Function}
 */
export function once(fn) {
  throw new Error("Not implemented");
}

/**
 * Return a memoized version of `fn`. Results are cached by a key derived
 * from `JSON.stringify(args)` — calling the memoized function again with
 * the same arguments must return the cached result without calling `fn`
 * again.
 *
 * @param {Function} fn
 * @returns {Function}
 */
export function memoize(fn) {
  throw new Error("Not implemented");
}

/**
 * Curry a two-argument function so it can be called either as
 * `curried(a)(b)` or `curried(a, b)`, both returning `fn(a, b)`.
 *
 * @param {(a: *, b: *) => *} fn
 * @returns {(a: *, b?: *) => *}
 */
export function curry2(fn) {
  throw new Error("Not implemented");
}

/**
 * Create a counter that records the history of every value it has held,
 * starting with `start`.
 *
 * @param {number} [start=0]
 * @returns {{
 *   increment: () => number,
 *   decrement: () => number,
 *   getValue: () => number,
 *   getHistory: () => number[],
 *   reset: () => number,
 * }}
 *
 * - increment()/decrement(): update the value, append it to history, and
 *   return the new value.
 * - getHistory(): return the full history array, starting with `start`.
 * - reset(): set the value back to `start`, reset history to `[start]`,
 *   and return `start`.
 */
export function createCounterWithHistory(start = 0) {
  throw new Error("Not implemented");
}
