/**
 * Custom error thrown by `timeout()` when `ms` elapses before
 * `promiseOrFn` settles.
 */
export class TimeoutError extends Error {
  constructor(message) {
    super(message);
    this.name = "TimeoutError";
  }
}

/**
 * Reimplementation of `Promise.all` — do NOT use the real `Promise.all`,
 * `Promise.allSettled`, `Promise.any`, or `Promise.race`.
 *
 * Returns a promise that:
 * - Resolves with an array of results in the SAME ORDER as `promises`, once
 *   every input has resolved. Non-promise values are treated as already
 *   resolved.
 * - Rejects immediately with the reason of the FIRST input to reject
 *   (fail-fast) — other in-flight promises are not awaited.
 * - Resolves to `[]` for an empty input array.
 *
 * myPromiseAll([1, Promise.resolve(2), 3]); // -> [1, 2, 3]
 * myPromiseAll([Promise.resolve(1), Promise.reject("boom")]); // rejects "boom"
 * myPromiseAll([]); // -> []
 *
 * @param {Array<*|Promise<*>>} promises
 * @returns {Promise<Array<*>>}
 */
export function myPromiseAll(promises) {
  throw new Error("Not implemented");
}

/**
 * Reimplementation of `Promise.race` — do NOT use the real `Promise.race`.
 *
 * Returns a promise that settles (fulfills or rejects) with the outcome of
 * whichever input in `promises` settles FIRST, in either direction.
 * Non-promise values are treated as already resolved (and thus settle
 * immediately).
 *
 * myPromiseRace([delay(10).then(() => "slow"), delay(5).then(() => "fast")]);
 * // -> "fast"
 * myPromiseRace([Promise.reject("fast fail"), delay(10).then(() => "slow")]);
 * // rejects "fast fail"
 *
 * @param {Array<*|Promise<*>>} promises
 * @returns {Promise<*>}
 */
export function myPromiseRace(promises) {
  throw new Error("Not implemented");
}

/**
 * Run zero-arg functions ONE AT A TIME, in order — each is awaited before
 * the next is called. Returns an array of their results in order.
 *
 * If any function throws or returns a rejected promise, `sequence` rejects
 * immediately with that reason and does NOT call the remaining functions.
 *
 * await sequence([
 *   () => 1,
 *   () => Promise.resolve(2),
 *   async () => 3,
 * ]); // -> [1, 2, 3], called one at a time
 *
 * await sequence([() => 1, () => { throw new Error("boom"); }, () => 3]);
 * // rejects "boom" — third function never called
 *
 * @param {Array<() => (*|Promise<*>)>} asyncFns
 * @returns {Promise<Array<*>>}
 */
export async function sequence(asyncFns) {
  throw new Error("Not implemented");
}

/**
 * Concurrency-limited async map. Calls `mapperFn(item, index)` for every
 * item in `items`, running at most `limit` calls concurrently.
 *
 * - Returns a promise resolving to an array of results in the SAME ORDER as
 *   `items`, regardless of the order in which calls complete.
 * - `limit === Infinity` (or `limit >= items.length`) behaves like full
 *   concurrency (all calls start immediately).
 * - `limit === 1` behaves like `sequence` (one at a time).
 * - If any call rejects, `asyncPool` rejects with that reason. In-flight
 *   calls are allowed to continue running to completion (no cancellation).
 *
 * await asyncPool(2, [1, 2, 3, 4], (n) => delay(10).then(() => n * 2));
 * // -> [2, 4, 6, 8], at most 2 in flight at once
 *
 * @param {number} limit
 * @param {Array<*>} items
 * @param {(item: *, index: number) => (*|Promise<*>)} mapperFn
 * @returns {Promise<Array<*>>}
 */
export function asyncPool(limit, items, mapperFn) {
  throw new Error("Not implemented");
}

/**
 * Race `promiseOrFn` against a timeout of `ms` milliseconds.
 *
 * `promiseOrFn` is either a promise, or a zero-arg function returning a
 * promise (or plain value) — called immediately to obtain the promise.
 *
 * - If `promiseOrFn` settles before `ms` elapses, the returned promise
 *   settles the same way (same value or rejection reason).
 * - If `ms` elapses first, the returned promise rejects with a
 *   `TimeoutError`.
 * - The internal timer is cleared once `promiseOrFn` settles, so it never
 *   keeps the process/event loop alive unnecessarily.
 *
 * await timeout(delay(5).then(() => "ok"), 50); // -> "ok"
 * await timeout(delay(50).then(() => "ok"), 5); // rejects TimeoutError
 *
 * @param {Promise<*>|(() => (*|Promise<*>))} promiseOrFn
 * @param {number} ms
 * @returns {Promise<*>}
 */
export function timeout(promiseOrFn, ms) {
  throw new Error("Not implemented");
}
