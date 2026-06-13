// Run with: node examples.js

// --- The iterator protocol ---
const iterator = {
  i: 0,
  next() {
    this.i++;
    return this.i <= 3
      ? { value: this.i, done: false }
      : { value: undefined, done: true };
  },
};
console.log("iterator.next():", iterator.next()); // { value: 1, done: false }
console.log("iterator.next():", iterator.next()); // { value: 2, done: false }
console.log("iterator.next():", iterator.next()); // { value: 3, done: false }
console.log("iterator.next():", iterator.next()); // { value: undefined, done: true }

// --- The iterable protocol: [Symbol.iterator] unlocks for...of, spread, etc. ---
const range = {
  from: 1,
  to: 3,
  [Symbol.iterator]() {
    let current = this.from;
    const last = this.to;
    return {
      next() {
        return current <= last
          ? { value: current++, done: false }
          : { value: undefined, done: true };
      },
    };
  },
};

console.log("[...range]:", [...range]); // [1, 2, 3]
console.log("[...range] again:", [...range]); // [1, 2, 3] — fresh iterator each time
console.log("Array.from(range):", Array.from(range)); // [1, 2, 3]
const [first, second] = range; // destructuring uses [Symbol.iterator] too
console.log("destructured first, second:", first, second); // 1, 2

for (const n of range) console.log("for...of range:", n); // 1, 2, 3

// --- Generator functions (function*) ---
function* countTo3() {
  yield 1;
  yield 2;
  yield 3;
}

const gen = countTo3(); // body hasn't run yet
console.log("gen.next():", gen.next()); // { value: 1, done: false }
console.log("gen.next():", gen.next()); // { value: 2, done: false }
console.log("gen.next():", gen.next()); // { value: 3, done: false }
console.log("gen.next():", gen.next()); // { value: undefined, done: true }

// A generator object is both an iterator and an iterable.
function* abc() {
  yield "a";
  yield "b";
  yield "c";
}
console.log("[...abc()]:", [...abc()]); // ["a", "b", "c"]
for (const c of abc()) console.log("for...of abc():", c); // "a", "b", "c"

// --- Gotcha: a generator object is single-use ---
const drained = abc();
console.log("[...drained] (1st time):", [...drained]); // ["a", "b", "c"]
console.log("[...drained] (2nd time):", [...drained]); // [] — same object, already done

// Contrast: `range` above is a hand-written iterable — its
// [Symbol.iterator]() builds fresh state every call, so it can be
// re-iterated. Calling abc() again gives a NEW generator object, which
// works — but re-using the SAME exhausted generator object does not.
console.log("[...abc()] (new generator object):", [...abc()]); // ["a", "b", "c"]

// --- Infinite generators ---
function* naturalNumbers() {
  let n = 1;
  while (true) yield n++;
}

// SAFE: for...of with break
const collected = [];
for (const n of naturalNumbers()) {
  if (n > 5) break;
  collected.push(n);
}
console.log("for...of with break:", collected); // [1, 2, 3, 4, 5]

// SAFE: a `take`-style helper that stops pulling after n items.
function* take(iterable, n) {
  if (n <= 0) return;
  let count = 0;
  for (const item of iterable) {
    yield item;
    if (++count >= n) return;
  }
}
console.log("[...take(naturalNumbers(), 5)]:", [...take(naturalNumbers(), 5)]); // [1, 2, 3, 4, 5]

// NEVER: [...naturalNumbers()] or Array.from(naturalNumbers()) — would hang forever.

// --- yield* delegation ---
function* letters() {
  yield "a";
  yield "b";
}
function* combined() {
  yield* letters(); // yields "a", then "b"
  yield "c";
  yield* [1, 2]; // works on any iterable, not just generators
}
console.log("[...combined()]:", [...combined()]); // ["a", "b", "c", 1, 2]

// --- A custom iterable class (fresh iterator per call -> repeatable) ---
class Range {
  constructor(start, end, step = 1) {
    if (step === 0) throw new Error("step cannot be 0");
    this.start = start;
    this.end = end;
    this.step = step;
  }

  [Symbol.iterator]() {
    let current = this.start;
    const { end, step } = this;
    return {
      next() {
        const done = step > 0 ? current >= end : current <= end;
        if (done) return { value: undefined, done: true };
        const value = current;
        current += step;
        return { value, done: false };
      },
    };
  }
}

console.log("[...new Range(0, 10, 2)]:", [...new Range(0, 10, 2)]); // [0, 2, 4, 6, 8]
console.log("[...new Range(5, 0, -1)]:", [...new Range(5, 0, -1)]); // [5, 4, 3, 2, 1]

const repeatable = new Range(0, 3);
console.log("repeatable, 1st pass:", [...repeatable]); // [0, 1, 2]
console.log("repeatable, 2nd pass (independent):", [...repeatable]); // [0, 1, 2]

// --- chunkIterable: lazy/iterable counterpart to fundamentals/07's chunk ---
function* chunkIterable(iterable, size) {
  if (size <= 0) throw new Error("size must be > 0");
  let current = [];
  for (const item of iterable) {
    current.push(item);
    if (current.length === size) {
      yield current;
      current = [];
    }
  }
  if (current.length > 0) yield current;
}

console.log("[...chunkIterable([1,2,3,4,5], 2)]:", [...chunkIterable([1, 2, 3, 4, 5], 2)]); // [[1,2],[3,4],[5]]
console.log(
  "[...take(chunkIterable(naturalNumbers(), 2), 3)]:",
  [...take(chunkIterable(naturalNumbers(), 2), 3)],
); // [[1,2],[3,4],[5,6]] — chunks an infinite source lazily

// --- zipIterables: N-ary/lazy counterpart to fundamentals/07's 2-array zip ---
function* zipIterables(...iterables) {
  if (iterables.length === 0) return;
  const iterators = iterables.map((it) => it[Symbol.iterator]());
  while (true) {
    const results = iterators.map((it) => it.next());
    if (results.some((r) => r.done)) return;
    yield results.map((r) => r.value);
  }
}

console.log(
  "[...zipIterables([1,2,3], ['a','b','c'])]:",
  [...zipIterables([1, 2, 3], ["a", "b", "c"])],
); // [[1,'a'],[2,'b'],[3,'c']]
console.log(
  "[...zipIterables(['a','b','c'], naturalNumbers())]:",
  [...zipIterables(["a", "b", "c"], naturalNumbers())],
); // [['a',1],['b',2],['c',3]] — stops because the array is exhausted, naturalNumbers() not drained
