# 06. Functions

## Declarations vs expressions vs arrows

```js
function add(a, b) { return a + b; }       // hoisted (Topic 03)

const addExpr = function (a, b) { return a + b; };

const addArrow = (a, b) => a + b;          // implicit return
```

Arrow functions are common for short callbacks and have different `this`
binding rules (intermediate roadmap).

## Returning functions / recursion

Functions are values — they can be returned from other functions
(closures, [Topic 03](../03-scope-hoisting-and-declarations/notes.md)) and
can call themselves (recursion):

```js
function makeAdder(x) {
  return (y) => x + y;
}
makeAdder(5)(3); // 8

function factorial(n) {
  if (n <= 1) return 1;
  return n * factorial(n - 1);
}
factorial(5); // 120
```

Every recursive function needs a **base case** that stops the recursion.

## Default and rest parameters

```js
function greet(name, greeting = "Hello") {
  return `${greeting}, ${name}!`;
}
greet("Ada");        // "Hello, Ada!"
greet("Ada", "Hi");  // "Hi, Ada!"

function multiplyAll(...numbers) { // numbers is a real array
  return numbers.reduce((p, n) => p * n, 1);
}
multiplyAll(2, 3, 4); // 24
```

## `fn.length`

A function's `.length` is its number of declared (non-default, non-rest)
parameters — useful for generic helpers like `curry`:

```js
((a, b, c) => {}).length; // 3
```

## Implicit `undefined` return

A function with no `return` (or a bare `return;`) evaluates to `undefined`.

## Further Reading (MDN)

- [Functions](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Functions)
- [Arrow function expressions](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Functions/Arrow_functions)
- [Default parameters](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Functions/Default_parameters)
- [Rest parameters](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Functions/rest_parameters)
- [`Function.prototype.length`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Function/length)
