# 04. Higher-Order Functions & Functional Programming

## Functions as first-class values

Functions in JS are values: assign them to variables, store them in arrays
or object properties, pass them as arguments, return them from other
functions.

```js
const ops = {
  double: (x) => x * 2,
  square: (x) => x * x,
};
ops["double"](5); // 10

function applyTwice(fn, x) {
  return fn(fn(x));
}
applyTwice(ops.double, 3); // 12

function makeMultiplier(factor) {
  return (x) => x * factor; // returns a function
}
makeMultiplier(3)(4); // 12
```

A **higher-order function (HOF)** takes a function as an argument, returns a
function, or both. `map`/`filter`/`reduce` ([fundamentals/07](../../fundamentals/07-arrays-and-array-methods/notes.md))
are HOFs; so are `compose`/`curry` ([fundamentals/06](../../fundamentals/06-functions/notes.md))
and everything below.

## `pipe`: left-to-right composition

[fundamentals/06](../../fundamentals/06-functions/notes.md) covers `compose`
(right-to-left: `compose(f, g, h)(x) === f(g(h(x)))`). `pipe` is the mirror —
**left-to-right**, often easier to read as a left-to-right data flow:

```js
const pipe = (...fns) => (x) => fns.reduce((acc, fn) => fn(acc), x);

const double = (x) => x * 2;
const increment = (x) => x + 1;

pipe(double, increment)(3); // increment(double(3)) = 7
compose(double, increment)(3); // double(increment(3)) = 8 (for comparison)
```

Only the **first** function receives the original argument(s) (it can be
variadic); every subsequent function receives the single return value of the
previous step. With zero functions, `pipe()` is the identity function.

## Partial application vs. currying

Both fix some of a function's arguments ahead of time, but differ in shape:

- **Partial application**: fix *some* arguments now, get back a function
  taking the *rest* — call it once, with however many args remain. `.bind`
  does this ([intermediate/02](../02-this-and-function-context/notes.md)):
  ```js
  const add3 = (a, b, c) => a + b + c;
  const add5and = add3.bind(null, 2, 3); // fixes a=2, b=3
  add5and(10); // 15 — one call with the remaining args
  ```
- **Currying**: transform a function so it takes its arguments **one at a
  time** (or in groups), each call returning a new function until all
  arguments are supplied. See `curry` in
  [fundamentals/06](../../fundamentals/06-functions/notes.md) — `add3(1)(2)(3)`.

Currying is partial application taken to its strict one-argument-per-call
extreme; partial application is the general "fix a prefix of arguments" idea.

## Debouncing vs. throttling

Both limit how often a function runs in response to rapid repeated calls
(e.g. keystrokes, scroll/resize events) — but with different guarantees.

**Debounce**: wait for a quiet period. Each call **resets** the timer; `fn`
only runs after `delayMs` pass with *no further calls*. Good for
"search-as-you-type" (wait until the user stops typing).

```js
function debounce(fn, delayMs) {
  let timeoutId;
  return (...args) => {
    clearTimeout(timeoutId); // GOTCHA: must clear the previous timer,
    timeoutId = setTimeout(() => fn(...args), delayMs); // or every call schedules
  };                                                      // an extra, stale invocation
}
```

**Throttle**: guarantee at most one invocation per `intervalMs`, regardless
of how many calls arrive. Good for scroll/resize handlers (run at a steady
rate no matter how fast events fire). A "trailing" throttle also fires once
more after the interval if calls arrived during it, using the latest args —
so the final state is never missed.

| | Debounce | Throttle |
|---|---|---|
| Trigger | after calls *stop* | at a steady rate *during* calls |
| Timer reset | every call resets it | fixed cadence, not reset |
| Use case | search input, autosave | scroll, resize, mousemove |

**Gotcha**: both rely on `setTimeout`/timer IDs captured in a closure
([intermediate/01](../01-closures-and-lexical-scope/notes.md)). Forgetting
`clearTimeout` in `debounce` means *every* call schedules its own eventual
`fn` invocation — `fn` fires multiple times instead of once after the quiet
period.

## Referential transparency & shared mutable state

An expression is **referentially transparent** if it can be replaced by its
value without changing the program's behavior — true only for pure functions
([fundamentals/06](../../fundamentals/06-functions/notes.md)) with no
dependency on external mutable state:

```js
const double = (x) => x * 2;
double(5); // always 5 -> 10, anywhere, forever — replaceable by `10`

let factor = 2;
const scale = (x) => x * factor; // NOT referentially transparent
scale(5); // depends on `factor` at call time — result can change
```

FP style favors passing all needed data as arguments and returning new
values instead of closing over and mutating shared variables
([intermediate/01](../01-closures-and-lexical-scope/notes.md) covers closures
capturing *live* bindings — exactly the mechanism that makes shared mutable
state easy to reach for and easy to get wrong). Benefits: easier to test
(no setup/teardown of external state), easier to reason about (call order
doesn't matter), safe to memoize ([intermediate/01](../01-closures-and-lexical-scope/notes.md)
— memoization assumes the same inputs always produce the same output).

## Transducers (single-pass map/filter)

Chaining `array.map(...).filter(...)` builds an intermediate array at each
step. A **transducer**-style pipeline applies all the transformations inside
a *single* `reduce`, with no intermediate arrays:

```js
function transduce(transformers, reducerFn, initialValue, array) {
  const step = (acc, value) => {
    // run `value` through each map/filter in order; bail out (skip) on filter rejection
    for (const t of transformers) {
      if (t.type === "map") value = t.fn(value);
      else if (t.type === "filter" && !t.fn(value)) return acc; // skip — don't reduce this value
    }
    return reducerFn(acc, value);
  };
  return array.reduce(step, initialValue);
}
```

## Further Reading (MDN)

- [Functions](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Functions)
- [`Function`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Function)
- [`Function.prototype.bind`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Function/bind)
- [`Array.prototype.reduce`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/reduce)
- [`setTimeout`](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)
- [`clearTimeout`](https://developer.mozilla.org/en-US/docs/Web/API/Window/clearTimeout)
