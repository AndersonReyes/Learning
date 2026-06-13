# 07. Arrays & Array Methods

## Basics

```js
const numbers = [1, 2, 3];
numbers.length;              // 3
numbers[0];                  // 1
numbers[numbers.length - 1]; // 3 — last element

numbers.push(4);    // [1, 2, 3, 4] — mutates
numbers.pop();      // [1, 2, 3]    — mutates
numbers.unshift(0); // [0, 1, 2, 3] — mutates
numbers.shift();    // [1, 2, 3]    — mutates
```

Non-mutating (return new array/value): `map`, `filter`, `reduce`, `find`, `slice`,
`flat`, `concat`, spread `[...arr]`.
Mutating: `push`, `pop`, `shift`, `unshift`, `splice`, `sort`, `reverse`.

## map / filter / reduce

```js
[1, 2, 3].map((n) => n * 2);             // [2, 4, 6]
[1, 2, 3, 4].filter((n) => n % 2 === 0); // [2, 4]
[1, 2, 3, 4].reduce((acc, n) => acc + n, 0); // 10
```

`reduce` always needs an explicit initial value — without one it uses the
first element as the initial accumulator, which throws on `[]`.

## find / includes / some / every

```js
[{ id: 1 }, { id: 2 }].find((u) => u.id === 2); // { id: 2 }
[{ id: 1 }, { id: 2 }].find((u) => u.id === 9); // undefined

[1, 2, 3].includes(2);        // true
[1, 2, 3].some((n) => n > 2); // true  — at least one
[1, 2, 3].every((n) => n > 0); // true — all
```

## slice — extract a portion, no mutation

```js
[1, 2, 3, 4, 5].slice(1, 3); // [2, 3] — start inclusive, end exclusive
[1, 2, 3, 4, 5].slice(-2);   // [4, 5] — negative = from the end
[1, 2, 3].slice();           // [1, 2, 3] — shallow copy
```

## flat

```js
[1, [2, 3], [4, [5]]].flat();  // [1, 2, 3, 4, [5]] — one level
[1, [2, [3]]].flat(Infinity);  // [1, 2, 3]         — fully flatten
```

## sort — comparator-based, mutates in place

```js
[10, 2, 1].sort();                 // [1, 10, 2] — default is lexicographic, wrong for numbers
[10, 2, 1].sort((a, b) => a - b);  // [1, 2, 10] — ascending
[10, 2, 1].sort((a, b) => b - a);  // [10, 2, 1] — descending
```

Comparator returns negative (a before b), positive (a after b), or 0 (equal).
To avoid mutating, sort a copy: `[...arr].sort(...)`.

## Deduplicating with Set

```js
[...new Set([1, 2, 2, 3, 1])]; // [1, 2, 3]
```

## Copying arrays

Assignment copies the reference, not the array (see
[Topic 01](../01-variables-and-data-types/notes.md#reference-vs-value)):

```js
const a = [1, 2, 3];
const b = a;
b.push(4);
a; // [1, 2, 3, 4] — same array, both names point to it

const copy = [...a];     // shallow copy
const copy2 = a.slice(); // also a shallow copy
```

## Further Reading (MDN)

- [`Array.prototype.map`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/map)
- [`Array.prototype.filter`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/filter)
- [`Array.prototype.reduce`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/reduce)
- [`Array.prototype.find`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/find)
- [`Array.prototype.sort`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/sort)
- [`Array.prototype.slice`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/slice)
- [`Array.prototype.flat`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/flat)
- [`Set`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Set)
