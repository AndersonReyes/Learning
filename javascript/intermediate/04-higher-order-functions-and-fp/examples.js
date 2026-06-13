// Run with: node examples.js

// --- Functions as first-class values ---
const ops = {
  double: (x) => x * 2,
  square: (x) => x * x,
};
console.log("ops['double'](5):", ops["double"](5)); // 10

function applyTwice(fn, x) {
  return fn(fn(x));
}
console.log("applyTwice(ops.double, 3):", applyTwice(ops.double, 3)); // 12

function makeMultiplier(factor) {
  return (x) => x * factor; // returns a function
}
console.log("makeMultiplier(3)(4):", makeMultiplier(3)(4)); // 12

// --- pipe: left-to-right composition ---
const pipe = (...fns) => (x) => fns.reduce((acc, fn) => fn(acc), x);
const compose = (...fns) => (x) => fns.reduceRight((acc, fn) => fn(acc), x);

const double = (x) => x * 2;
const increment = (x) => x + 1;

console.log("pipe(double, increment)(3):", pipe(double, increment)(3)); // 7
console.log("compose(double, increment)(3):", compose(double, increment)(3)); // 8
console.log("pipe()(5):", pipe()(5)); // 5 — identity with no functions

// --- Partial application vs. currying ---
const add3 = (a, b, c) => a + b + c;

// Partial application: fix a prefix, get back a function for the rest.
const add5and = add3.bind(null, 2, 3); // fixes a=2, b=3
console.log("add5and(10):", add5and(10)); // 15

// Currying: one argument (or group) at a time.
function curry(fn) {
  return function curried(...args) {
    if (args.length >= fn.length) return fn(...args);
    return (...rest) => curried(...args, ...rest);
  };
}
const curriedAdd3 = curry(add3);
console.log("curriedAdd3(1)(2)(3):", curriedAdd3(1)(2)(3)); // 6
console.log("curriedAdd3(1, 2)(3):", curriedAdd3(1, 2)(3)); // 6

// --- Debouncing ---
function debounce(fn, delayMs) {
  let timeoutId;
  function debounced(...args) {
    clearTimeout(timeoutId); // GOTCHA: must clear the previous timer
    timeoutId = setTimeout(() => {
      timeoutId = undefined;
      fn(...args);
    }, delayMs);
  }
  debounced.cancel = () => {
    clearTimeout(timeoutId);
    timeoutId = undefined;
  };
  return debounced;
}

const debouncedLog = debounce((msg) => console.log("debounced fired:", msg), 50);
debouncedLog("first");
debouncedLog("second"); // resets the timer — "first" never fires
console.log("debounce: scheduled, waiting for quiet period...");
setTimeout(() => console.log("debounce: 60ms elapsed (one log above expected)"), 60);

// --- Throttling (trailing) ---
function throttle(fn, intervalMs) {
  let onCooldown = false;
  let pendingArgs = null;

  function startCooldown() {
    onCooldown = true;
    setTimeout(() => {
      if (pendingArgs) {
        const args = pendingArgs;
        pendingArgs = null;
        fn(...args);
        startCooldown();
      } else {
        onCooldown = false;
      }
    }, intervalMs);
  }

  return function throttled(...args) {
    if (!onCooldown) {
      fn(...args);
      startCooldown();
    } else {
      pendingArgs = args;
    }
  };
}

const throttledLog = throttle((msg) => console.log("throttled fired:", msg), 50);
throttledLog("a"); // fires immediately
throttledLog("b"); // suppressed
throttledLog("c"); // suppressed, overwrites "b"
console.log("throttle: 'a' should log above immediately, 'c' trailing after ~50ms");

// --- Referential transparency & shared mutable state ---
const doubleRT = (x) => x * 2;
console.log("doubleRT(5) (referentially transparent):", doubleRT(5)); // always 10

let factor = 2;
const scale = (x) => x * factor; // NOT referentially transparent
console.log("scale(5) with factor=2:", scale(5)); // 10
factor = 10;
console.log("scale(5) with factor=10:", scale(5)); // 50 — same input, different result

// --- Transducers (single-pass map/filter) ---
function transduce(transformers, reducerFn, initialValue, array) {
  const step = (acc, value) => {
    for (const t of transformers) {
      if (t.type === "map") value = t.fn(value);
      else if (t.type === "filter" && !t.fn(value)) return acc; // skip
    }
    return reducerFn(acc, value);
  };
  return array.reduce(step, initialValue);
}

const transduced = transduce(
  [
    { type: "map", fn: (x) => x * 2 },
    { type: "filter", fn: (x) => x > 5 },
  ],
  (acc, x) => acc + x,
  0,
  [1, 2, 3, 4],
);
console.log("transduce(map *2, filter >5, sum) over [1,2,3,4]:", transduced); // 14
