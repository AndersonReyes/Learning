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

## Further Reading (MDN)

- [Grammar and types — Declarations](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Grammar_and_types#declarations)
- [`let`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/let) /
  [`const`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/const) /
  [`var`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/var)
- [Hoisting](https://developer.mozilla.org/en-US/docs/Glossary/Hoisting)
- [Closures](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Closures)
