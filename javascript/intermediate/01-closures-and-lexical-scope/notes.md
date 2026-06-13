# 01. Closures & Lexical Scope

## Lexical scope chain recap

A function's scope is determined by where it's **written**, not where it's
**called**. Each function carries a reference to the scope it was defined
in, forming a chain looked up on variable access:

```js
const outer = "outer";

function a() {
  const middle = "middle";
  function b() {
    console.log(outer, middle); // both found by walking up the chain
  }
  b();
}
```

`var`/`let`/`const` semantics and the Temporal Dead Zone are covered in
[fundamentals/03](../../fundamentals/03-scope-hoisting-and-declarations/notes.md).
For the `var` vs `let` loop-binding gotcha (each loop iteration sharing vs.
getting its own binding when a closure is created inside), see
[fundamentals/03 — Closures and loops](../../fundamentals/03-scope-hoisting-and-declarations/notes.md#closures-and-loops).

## What a closure actually is

A closure = a function + a reference to its lexical environment. The
function keeps access to variables from its enclosing scope **even after**
that outer function has returned:

```js
function makeGreeter(name) {
  return () => `Hello, ${name}!`;
}
const greetAda = makeGreeter("Ada");
greetAda(); // "Hello, Ada!" — `name` still alive, scope long gone
```

## Closures capture LIVE references, not snapshots

The closure doesn't copy the variable's value at creation time — it holds a
reference to the **binding**. If the variable changes after the closure is
created (but before it's called), the closure sees the new value:

```js
function makeLogger() {
  let message = "first";
  const log = () => console.log(message);
  message = "second"; // changed AFTER `log` was created
  return log;
}
makeLogger()(); // "second" — not "first"
```

This is why returning multiple closures over the *same* variable lets them
communicate:

```js
function makeCounter() {
  let count = 0;
  return {
    increment: () => ++count,
    read: () => count,
  };
}
const c = makeCounter();
c.increment();
c.increment();
c.read(); // 2 — both functions share the live `count` binding
```

## Module / factory pattern for private state

Closures are JS's mechanism for private fields before/without `class`
private (`#field`) syntax. A factory function declares local variables and
returns an object whose methods close over them — there is **no property**
on the returned object exposing the state directly:

```js
function createAccount(balance) {
  function deposit(amount) { balance += amount; return balance; }
  function withdraw(amount) {
    if (amount > balance) throw new Error("insufficient funds");
    balance -= amount;
    return balance;
  }
  return { deposit, withdraw, getBalance: () => balance };
}

const acct = createAccount(100);
acct.deposit(50);
acct.balance; // undefined — not accessible, only via getBalance()
```

Each call to `createAccount` creates a **fresh** `balance` binding — separate
instances don't share state.

## Memoization via closures

A memoized function closes over a cache (object/Map) that persists across
calls, keyed by the arguments:

```js
function memoize(fn) {
  const cache = new Map();
  return (...args) => {
    const key = JSON.stringify(args);
    if (!cache.has(key)) cache.set(key, fn(...args));
    return cache.get(key);
  };
}

const slowSquare = (n) => n * n;
const fastSquare = memoize(slowSquare);
fastSquare(5); // computed, cached
fastSquare(5); // returned from cache
```

Each call to `memoize` creates a new `cache` — independent memoized wrappers
of the same underlying function don't share results.

## Gotcha: closures keep referenced variables alive

A variable captured by a closure can't be garbage-collected while the
closure itself is reachable — even if the variable is large and only one
closure still needs it. Holding onto closures you no longer need (e.g. in a
long-lived array or event listener) can leak memory.

## Further Reading (MDN)

- [Closures](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Closures)
- [Closures — Lexical scoping](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Closures#lexical_scoping)
- [Closures — Emulating private methods with closures](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Closures#emulating_private_methods_with_closures)
- [Closures — Closure scope chain](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Closures#closure_scope_chain)
