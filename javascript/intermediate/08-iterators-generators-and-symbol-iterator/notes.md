# 08. Iterators, Generators & `Symbol.iterator`

## The iterator protocol

An **iterator** is any object with a `.next()` method that returns
`{ value, done }`:

```js
const iterator = {
  i: 0,
  next() {
    this.i++;
    return this.i <= 3 ? { value: this.i, done: false } : { value: undefined, done: true };
  },
};
iterator.next(); // { value: 1, done: false }
iterator.next(); // { value: 2, done: false }
iterator.next(); // { value: 3, done: false }
iterator.next(); // { value: undefined, done: true }
```

Once `done: true`, further `.next()` calls conventionally keep returning
`{ value: undefined, done: true }` (don't restart).

## The iterable protocol

An **iterable** is any object with a `[Symbol.iterator]()` method that
returns an iterator. This single method is what unlocks:

- `for...of`
- spread syntax (`[...x]`, `f(...x)`)
- `Array.from(x)`
- destructuring (`const [a, b] = x`)

```js
const range = {
  from: 1,
  to: 3,
  [Symbol.iterator]() {
    let current = this.from;
    const last = this.to;
    return {
      next() {
        return current <= last
          ? { value: current++, done: false }
          : { value: undefined, done: true };
      },
    };
  },
};

[...range];        // [1, 2, 3]
[...range];        // [1, 2, 3] — works again: [Symbol.iterator] returns a FRESH iterator
for (const n of range) console.log(n); // 1, 2, 3
```

**Key design point**: `[Symbol.iterator]()` is called fresh every time
something iterates the object (every `for...of`, every spread). If it
returns a NEW iterator with its own position each time, the object supports
multiple independent iterations. If it returns the SAME (shared/exhausted)
iterator object, re-iteration breaks — see the generator gotcha below.

## Generator functions (`function*`)

A `function*` declaration. Calling it does **not** run the body — it returns
a **generator object** immediately. The body runs only as the generator is
driven via `.next()`, pausing at each `yield`:

```js
function* countTo3() {
  yield 1;
  yield 2;
  yield 3;
}

const gen = countTo3(); // body hasn't run yet
gen.next(); // runs to first `yield` -> { value: 1, done: false }
gen.next(); // { value: 2, done: false }
gen.next(); // { value: 3, done: false }
gen.next(); // body finishes -> { value: undefined, done: true }
```

A generator object is **both an iterator and an iterable**: it has `.next()`
AND `[Symbol.iterator]()`. Its `[Symbol.iterator]()` returns **itself**
(`gen[Symbol.iterator]() === gen`). This is why `for...of`, spread, etc. all
work directly on a generator object:

```js
function* abc() {
  yield "a";
  yield "b";
  yield "c";
}
[...abc()];           // ["a", "b", "c"]
for (const c of abc()) console.log(c); // "a", "b", "c"
```

## Gotcha: a generator object is single-use

Because `gen[Symbol.iterator]()` returns `gen` itself (not a fresh iterator),
once a generator object is fully drained, iterating it again yields nothing:

```js
const gen = abc();
[...gen]; // ["a", "b", "c"] — drained
[...gen]; // [] — same object, already done, NOT a fresh start
```

Contrast with the `range` object above (a hand-written iterable): each
`[Symbol.iterator]()` call builds new `current`/`last` state, so it can be
iterated repeatedly. **If you need repeatable iteration, write a class/object
whose `[Symbol.iterator]()` returns a fresh iterator each call** (or write a
*generator function* and call it again to get a new generator object —
the function itself isn't exhausted, only each generator object it produces).

## Infinite generators

`while (true) yield ...` never sets `done: true` — safe only when consumed
**lazily** or with an explicit limit:

```js
function* naturalNumbers() {
  let n = 1;
  while (true) yield n++;
}

// SAFE: for...of with break
for (const n of naturalNumbers()) {
  if (n > 5) break;
  console.log(n);
}

// SAFE: a `take`-style helper that stops pulling after n items (see exercise 3)

// NEVER DO THIS — hangs / runs out of memory:
// [...naturalNumbers()]
// Array.from(naturalNumbers())
```

Spread and `Array.from` (without a limiting second pass) try to drain the
iterable to `done: true`, which never happens.

## `yield*` — delegating to another iterable

`yield*` re-yields every value from another iterable/generator, in place:

```js
function* letters() {
  yield "a";
  yield "b";
}
function* combined() {
  yield* letters();  // yields "a", then "b"
  yield "c";
  yield* [1, 2];     // works on any iterable, not just generators
}
[...combined()]; // ["a", "b", "c", 1, 2]
```

## Further Reading (MDN)

- [Iterators and generators](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Iterators_and_generators)
- [`Symbol.iterator`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Symbol/iterator)
- [`function*`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/function*)
- [`yield`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/yield)
- [`yield*`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/yield*)
