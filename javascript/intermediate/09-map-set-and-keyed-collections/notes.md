# 09. Map, Set & Keyed Collections

## Map vs plain object

`Map` is a key/value collection purpose-built for dynamic data:

```js
const m = new Map();
m.set("a", 1);
m.set(42, "num key");
m.set({ id: 1 }, "object key"); // any value can be a key

m.get("a");   // 1
m.size;       // 3 (real property, not a method)
m.has(42);    // true
m.delete("a");
```

Differences from a plain object:

| | `Map` | Object |
|---|---|---|
| Key types | any value (objects, functions, `NaN`, ...) | strings/symbols only (others coerced to string) |
| Key order | insertion order, guaranteed | mostly insertion order, but integer-like keys sort numerically first |
| Size | `.size` | `Object.keys(obj).length` |
| Prototype pollution | none — no inherited keys like `toString` | inherited keys can collide (`"toString" in {}` -> `true`) |
| Iteration | iterable directly | needs `Object.entries`/`keys`/`values` |
| Performance | optimized for frequent add/remove | optimized for static, known-shape records |

```js
// Map is iterable directly:
for (const [key, value] of m) {
  console.log(key, value);
}
// spread works too:
[...m]; // [["a", 1], [42, "num key"], [{id:1}, "object key"]]
```

Use a plain object for fixed-shape records (`{ name, age }`); use `Map` when
keys are dynamic, non-string, or you need guaranteed order and a `.size`.

## Set: unique values, O(1) lookup

`Set` stores unique values, insertion order preserved:

```js
const s = new Set([1, 2, 2, 3, 1]);
s.size;        // 3 -> Set(3) {1, 2, 3}
s.add(4);
s.has(2);      // true
s.delete(1);
[...s];        // [2, 3, 4]
```

`.has()` on a `Set` is O(1) (hash-based). `Array.prototype.includes()` is
O(n) — it scans the whole array. For repeated membership checks, convert to
a `Set` first:

```js
const allowed = new Set(["a", "b", "c"]);
allowed.has("b"); // O(1)

const arr = ["a", "b", "c"];
arr.includes("b"); // O(n) — fine for one-off checks, slow in a loop
```

## Iterating: keys(), values(), entries()

Both `Map` and `Set` expose the same three iterator methods:

```js
const m = new Map([["a", 1], ["b", 2]]);
[...m.keys()];    // ["a", "b"]
[...m.values()];  // [1, 2]
[...m.entries()]; // [["a", 1], ["b", 2]] -- same as [...m]

const s = new Set(["x", "y"]);
[...s.keys()];    // ["x", "y"]   -- alias for values()
[...s.values()];  // ["x", "y"]
[...s.entries()]; // [["x","x"], ["y","y"]] -- [value, value] pairs (kept for Map-like consistency)
```

`Map` and `Set` both support `.forEach((value, key, collection) => ...)` —
for a `Set`, `value === key` on every call.

## WeakMap / WeakSet: keys are weakly held

`WeakMap` keys (and `WeakSet` values) **must be objects** — primitives throw:

```js
const wm = new WeakMap();
const obj = {};
wm.set(obj, "metadata");
wm.get(obj); // "metadata"

wm.set("string", "x"); // TypeError: Invalid value used as weak map key
```

Key differences from `Map`/`Set`:

- **Not enumerable, not iterable** — no `.keys()`, `.values()`, `.entries()`,
  `.size`, `.forEach()`, no `for...of`. You can only interact with a key you
  already have a reference to.
- **Garbage-collected automatically** — if `obj` has no other references
  anywhere in the program, its entry is removed from the `WeakMap` and both
  can be freed. A regular `Map` would keep `obj` alive forever as a key,
  leaking memory.

Common use: attach private data to an object without modifying it and
without leaking memory if the object is later discarded:

```js
const privateData = new WeakMap();
class Account {
  constructor(balance) {
    privateData.set(this, { balance });
  }
  getBalance() {
    return privateData.get(this).balance;
  }
}
// when an Account instance is no longer referenced anywhere,
// its entry in `privateData` is collected too.
```

`WeakSet` is the `Set` analog — track "has this object been seen" without
preventing collection:

```js
const visited = new WeakSet();
visited.add(someNode);
visited.has(someNode); // true
```

## Gotcha: key/value equality is SameValueZero

`Map` keys and `Set` values are compared using **SameValueZero** — like
`===`, except `NaN` is considered equal to itself:

```js
const s = new Set();
s.add(NaN);
s.add(NaN);
s.size; // 1 -- NaN === NaN is false, but SameValueZero treats them as equal

s.add(0);
s.add(-0);
s.size; // still 1 -- 0 and -0 are SameValueZero-equal (unlike Object.is)
```

Object keys are compared **by reference**, not deep equality — two
structurally identical objects are different keys:

```js
const m = new Map();
m.set({ id: 1 }, "first");
m.set({ id: 1 }, "second"); // different object -> different key
m.size; // 2

const key = { id: 1 };
m.set(key, "third");
m.get(key); // "third" -- same reference required
m.get({ id: 1 }); // undefined -- different object, even though it "looks" the same
```

## Further Reading (MDN)

- [Keyed collections](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Keyed_collections)
- [`Map`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Map)
- [`Set`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Set)
- [`WeakMap`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WeakMap)
- [`WeakSet`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WeakSet)
- [Equality comparisons and sameness](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Equality_comparisons_and_sameness)
