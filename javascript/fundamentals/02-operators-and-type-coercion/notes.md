# 02. Operators & Type Coercion

## Arithmetic

```js
5 % 2   // 1  — remainder
5 ** 2  // 25 — exponentiation
```

## `==` vs `===`

`===` compares value and type, no conversion. `==` coerces first, following
rules that are easy to get wrong.

```js
1 == "1"          // true
0 == false        // true
null == undefined // true  — only equal to each other
null === undefined // false
NaN == NaN        // false — NaN is never equal to anything
```

**Always use `===`/`!==`.**

## Short-circuiting: `&&` / `||`

Return one of their *operands*, not necessarily a boolean. Stop evaluating
once the result is known:

```js
"" || "default"      // "default"
"hello" || "default" // "hello"
0 && sideEffect()     // 0 — sideEffect() never runs
```

## Nullish coalescing `??` and optional chaining `?.`

`??` only falls back for `null`/`undefined` — not other falsy values:

```js
0 ?? "default"        // 0
null ?? "default"     // "default"
```

`?.` short-circuits to `undefined` instead of throwing:

```js
const user = { profile: { name: "Ada" } };
user.address?.city    // undefined, no error
settings?.save()      // undefined — save() never called if settings is null/undefined
```

## Ternary

`condition ? a : b` is an expression. Chains beyond two or three branches
should become `if`/`switch` (see [Topic 04](../04-control-flow/notes.md)).

```js
const label = score >= 90 ? "A" : score >= 80 ? "B" : "C";
```

## Bitwise operators

Operate on the 32-bit integer representation of numbers.

```js
5 & 3   // 1   — AND
5 | 2   // 7   — OR
5 ^ 1   // 4   — XOR
~5      // -6  — NOT
1 << 3  // 8   — left shift  (multiply by 2^n)
8 >> 2  // 2   — right shift (divide by 2^n, floor)
```

Common use: packing several booleans into a single integer "bitmask",
where bit `i` (`1 << i`) represents flag `i`.

## Further Reading (MDN)

- [Expressions and operators](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Expressions_and_operators)
- [Equality comparisons and sameness](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Equality_comparisons_and_sameness)
- [Bitwise operators](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators#bitwise_operators)
- [Nullish coalescing (`??`)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Nullish_coalescing)
- [Optional chaining (`?.`)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Optional_chaining)
