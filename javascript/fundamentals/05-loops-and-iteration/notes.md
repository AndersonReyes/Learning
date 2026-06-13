# 05. Loops & Iteration

## `for`

```js
for (let i = 0; i < 5; i++) {
  console.log(i); // 0, 1, 2, 3, 4
}
```

Use when you need the index (e.g. comparing `array[i]` with `array[i+1]`,
or stepping by a custom amount).

## `while` / `do...while`

```js
let n = 5;
while (n > 0) {
  console.log(n);
  n--;
}

// runs the body at least once
do { attempts++; } while (attempts < 3);
```

Use `while` when the number of iterations isn't known ahead of time.

## `for...of` — values

Works on any iterable (arrays, strings, `Map`, `Set`, ...):

```js
for (const char of "abc") { /* "a", "b", "c" */ }
for (const item of [10, 20, 30]) { /* 10, 20, 30 */ }
```

## `for...in` — object keys

```js
for (const key in { x: 1, y: 2 }) { /* "x", then "y" */ }
```

Avoid on arrays — iterates indices as strings and can pick up inherited
properties. Use `for...of` for array values.

## `break` / `continue`

```js
for (const n of [1, 2, 3, 4, 5]) {
  if (n === 3) continue; // skip
  if (n === 5) break;    // stop
  console.log(n); // 1, 2, 4
}
```

## Nesting loops

```js
const groups = [[1, 2], [3, 4], [5]];
const flat = [];
for (const group of groups) {
  for (const item of group) flat.push(item);
}
// flat = [1, 2, 3, 4, 5]
```

## Choosing a loop

- Need the index, a custom step, or to iterate backwards → `for`
- Iterate values of an array/string/iterable → `for...of`
- Unknown number of iterations, condition-based → `while` / `do...while`
- Object's own enumerable keys → `for...in` (rare — prefer
  `Object.keys`/`Object.entries`, [Topic 08](../08-objects-and-destructuring/notes.md))

A loop variable captured by a closure (e.g. a callback created inside the
loop body) behaves differently for `var` vs `let` — see
[Topic 03](../03-scope-hoisting-and-declarations/notes.md#closures-and-loops).

## Further Reading (MDN)

- [Loops and iteration](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Loops_and_iteration)
- [`for`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/for) /
  [`while`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/while) /
  [`do...while`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/do...while)
- [`for...of`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/for...of) /
  [`for...in`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/for...in)
