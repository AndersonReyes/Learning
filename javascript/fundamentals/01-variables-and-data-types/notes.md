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

## Further Reading (MDN)

- [Grammar and types](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Grammar_and_types)
- [Data structures](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Data_structures)
- [`Object.prototype.toString`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/toString)
- [`Object.is`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/is)
