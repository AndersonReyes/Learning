# 01. Variables, Data Types & Type Conversion

## Declarations

- `const` — default. Can't reassign the binding; object/array contents are still mutable.
- `let` — reassignable, block-scoped.
- `var` — function-scoped, avoid. See [Topic 03](../03-scope-hoisting-and-declarations/notes.md).

```js
const point = { x: 1, y: 2 };
point.x = 99; // OK — mutating contents
point = {};   // TypeError — reassigning the binding
```

## Primitive types

`string`, `number`, `boolean`, `undefined`, `null`, `bigint`, `symbol`.
Everything else (objects, arrays, functions, dates, ...) is `object`.

## `typeof` quirks

```js
typeof null          // "object"  — historical bug
typeof []             // "object"  — arrays are objects
typeof function(){}  // "function"
```

Use `Array.isArray(value)` for arrays, `value === null` for null.

## Precise type tags

`Object.prototype.toString.call(value)` returns `"[object Type]"` for any
value — the only reliable way to distinguish arrays, dates, regexes, maps,
sets, etc. from plain objects:

```js
Object.prototype.toString.call([])          // "[object Array]"
Object.prototype.toString.call(null)        // "[object Null]"
Object.prototype.toString.call(new Date())  // "[object Date]"
Object.prototype.toString.call(/x/)         // "[object RegExp]"
```

## Explicit conversion

```js
Number("42")     // 42
Number("42px")   // NaN
Number(null)     // 0
Number(undefined) // NaN
Number([])       // 0
Number([1, 2])   // NaN

String(null)     // "null"
Boolean(0)       // false
```

## Implicit coercion

```js
"5" + 1   // "51" — concatenation
"5" - 1   // 4    — numeric subtraction
true + 1  // 2
```

## Truthy / falsy

Falsy: `false 0 -0 0n "" null undefined NaN`. Everything else — including
`[]` and `{}` — is truthy.

## Equality edge cases

- `NaN !== NaN` (and `NaN == NaN` is also `false`). Use `Number.isNaN(x)`.
- `Object.is(a, b)` distinguishes `+0`/`-0` and treats `NaN` as equal to
  itself — useful for implementing deep equality.

## Reference vs. value

Primitives are copied by value. Objects and arrays are copied by
**reference** — copying the variable copies the pointer, not the contents:

```js
let a = 5;
let b = a;
b = 10;
a; // 5 — unaffected

const obj1 = { x: 1 };
const obj2 = obj1;
obj2.x = 99;
obj1.x;        // 99 — same object
obj1 === obj2; // true — same reference

const obj3 = { x: 1 };
obj1 === obj3; // false — different objects, even with equal contents
```

An independent copy needs a shallow spread (`{ ...obj }`,
[Topic 08](../08-objects-and-destructuring/notes.md)) or, for nested data, a
recursive clone (`cloneDeep` in this topic's exercises).

## Further Reading (MDN)

- [Grammar and types](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Grammar_and_types)
- [`Object.prototype.toString`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/toString)
- [`Object.is`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/is)
