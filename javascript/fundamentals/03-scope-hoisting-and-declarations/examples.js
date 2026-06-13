// Run with: node examples.js

// --- var leaks out of blocks, let does not ---
function scopeDemo() {
  if (true) {
    var fromVar = "I leak out of the if-block";
    let fromLet = "I stay inside the if-block";
    console.log("inside block, fromLet:", fromLet);
  }

  console.log("outside block, fromVar:", fromVar);

  try {
    console.log("outside block, fromLet:", fromLet);
  } catch (err) {
    console.log("outside block, fromLet threw:", err.constructor.name);
  }
}
scopeDemo();

// --- Function declarations are fully hoisted ---
console.log("calling hoistedFn before its declaration:", hoistedFn());
function hoistedFn() {
  return "hello from a hoisted function";
}

// --- var is hoisted but starts as undefined ---
console.log("hoistedVar before assignment:", hoistedVar);
var hoistedVar = "now it has a value";
console.log("hoistedVar after assignment:", hoistedVar);

// --- let/const are hoisted into the Temporal Dead Zone ---
try {
  console.log(tdzValue); // ReferenceError
} catch (err) {
  console.log("accessing tdzValue before declaration threw:", err.constructor.name);
}
let tdzValue = "now initialized";
console.log("tdzValue after declaration:", tdzValue);

// --- Closures: private state via createCounter ---
function createCounter(start = 0) {
  let count = start;
  return {
    increment() {
      count += 1;
      return count;
    },
    decrement() {
      count -= 1;
      return count;
    },
    getValue() {
      return count;
    },
  };
}

const counter = createCounter();
console.log("counter.increment():", counter.increment());
console.log("counter.increment():", counter.increment());
console.log("counter.decrement():", counter.decrement());
console.log("counter.getValue():", counter.getValue());
// `count` itself is not accessible from here — only through the methods.

// --- Closures for memoization ---
function memoize(fn) {
  const cache = new Map();
  return (arg) => {
    if (!cache.has(arg)) {
      cache.set(arg, fn(arg));
    }
    return cache.get(arg);
  };
}

let slowCalls = 0;
const slowSquare = memoize((n) => {
  slowCalls += 1;
  return n * n;
});
console.log("slowSquare(5):", slowSquare(5));
console.log("slowSquare(5) again:", slowSquare(5));
console.log("slowCalls (should be 1):", slowCalls);
