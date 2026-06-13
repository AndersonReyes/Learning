# 03. Scope, Hoisting & `var`/`let`/`const`

## Function scope vs block scope

`var` is function-scoped — visible anywhere in the enclosing function,
regardless of `{}` nesting. `let`/`const` are block-scoped — visible only
within the nearest enclosing `{ ... }`.

```js
function example() {
  if (true) {
    var fromVar = "leaks out";
    let fromLet = "stays inside";
  }
  console.log(fromVar);  // "leaks out"
  console.log(fromLet);  // ReferenceError
}
```

## Hoisting

- **Function declarations** are fully hoisted — callable before their
  definition in the source.
- **`var`** is hoisted as `undefined` until its assignment line runs.
- **`let`/`const`** are hoisted but uninitialized — the Temporal Dead Zone.

```js
console.log(hoistedFn()); // works
function hoistedFn() { return "hello"; }

console.log(hoistedVar); // undefined
var hoistedVar = "value";
```

## Temporal Dead Zone (TDZ)

Accessing a `let`/`const` before its declaration throws `ReferenceError`:

```js
console.log(value); // ReferenceError
let value = 10;
```

## Closures

An inner function "remembers" variables from its enclosing scope, even
after the outer function returns. This is the mechanism behind private
state, memoization, and currying:

```js
function createCounter(start = 0) {
  let count = start;
  return {
    increment() { return ++count; },
    getValue() { return count; },
  };
}

const counter = createCounter();
counter.increment(); // 1
counter.getValue();  // 1
```

`count` is only reachable through the returned methods — there's no other
way to access it from outside.

### Closures and loops

`var` is function-scoped, so every iteration shares **one** binding; `let`
creates a **fresh** binding per iteration. This matters as soon as a loop body
creates a closure (e.g. a callback):

```js
for (var i = 0; i < 3; i++) {
  setTimeout(() => console.log(i), 0); // 3, 3, 3 — all see the final value
}

for (let j = 0; j < 3; j++) {
  setTimeout(() => console.log(j), 0); // 0, 1, 2 — each callback gets its own j
}
```

## Further Reading (MDN)

- [Grammar and types — Declarations](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Grammar_and_types#declarations)
- [`let`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/let) /
  [`const`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/const) /
  [`var`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/var)
- [Hoisting](https://developer.mozilla.org/en-US/docs/Glossary/Hoisting)
- [Closures](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Closures)
