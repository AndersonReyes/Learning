// Run with: node examples.js

// --- Lexical scope chain ---
const outer = "outer";

function a() {
  const middle = "middle";
  function b() {
    console.log("lexical chain:", outer, middle); // both found by walking up the chain
  }
  b();
}
a();

// --- What a closure actually is ---
function makeGreeter(name) {
  return () => `Hello, ${name}!`;
}
const greetAda = makeGreeter("Ada");
console.log("greetAda():", greetAda()); // "Hello, Ada!" — `name` still alive, scope long gone

// --- Closures capture LIVE references, not snapshots ---
function makeLogger() {
  let message = "first";
  const log = () => message;
  message = "second"; // changed AFTER `log` was created
  return log;
}
console.log("makeLogger()():", makeLogger()()); // "second" — not "first"

// Two closures sharing the same live binding:
function makeCounter() {
  let count = 0;
  return {
    increment: () => ++count,
    read: () => count,
  };
}
const sharedCounter = makeCounter();
sharedCounter.increment();
sharedCounter.increment();
console.log("sharedCounter.read():", sharedCounter.read()); // 2 — both methods share `count`

// --- Module / factory pattern for private state ---
function createAccount(balance) {
  function deposit(amount) {
    balance += amount;
    return balance;
  }
  function withdraw(amount) {
    if (amount > balance) throw new Error("insufficient funds");
    balance -= amount;
    return balance;
  }
  return { deposit, withdraw, getBalance: () => balance };
}

const acct = createAccount(100);
console.log("acct.deposit(50):", acct.deposit(50)); // 150
console.log("acct.balance (direct access):", acct.balance); // undefined — private
console.log("acct.getBalance():", acct.getBalance()); // 150

// Separate instances don't share state:
const acct2 = createAccount(0);
console.log("acct2.getBalance():", acct2.getBalance()); // 0 — independent from `acct`

// --- Memoization via closures ---
function memoize(fn) {
  const cache = new Map();
  return (...args) => {
    const key = JSON.stringify(args);
    if (!cache.has(key)) {
      console.log(`  computing for ${key}`);
      cache.set(key, fn(...args));
    }
    return cache.get(key);
  };
}

const slowSquare = (n) => n * n;
const fastSquare = memoize(slowSquare);
console.log("fastSquare(5):", fastSquare(5)); // logs "computing...", then 25
console.log("fastSquare(5):", fastSquare(5)); // no log — served from cache

// Independent memoized wrappers don't share a cache:
const anotherFastSquare = memoize(slowSquare);
console.log("anotherFastSquare(5):", anotherFastSquare(5)); // logs "computing..." again

// --- Gotcha: closures keep referenced variables alive ---
// `bigData` stays reachable as long as `getFirstByte` (the closure) is
// reachable, even though only one byte is ever used.
function makeAccessor() {
  const bigData = new Array(1000).fill(0);
  return () => bigData[0];
}
const getFirstByte = makeAccessor();
console.log("getFirstByte():", getFirstByte()); // 0 — `bigData` can't be GC'd while this closure exists
