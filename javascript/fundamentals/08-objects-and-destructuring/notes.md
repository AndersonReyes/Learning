# 08. Objects & Destructuring

## Object literals & property access

```js
const person = {
  first: "Ada",
  last: "Lovelace",
  greet() {
    return `Hi, I'm ${this.first}`;
  },
};

person.first;    // "Ada"      — dot notation
person["last"];  // "Lovelace" — bracket notation, needed for dynamic/non-identifier keys

const key = "first";
person[key];     // "Ada"
```

## Shorthand and computed properties

```js
const first = "Ada", last = "Lovelace";
const person = { first, last };  // shorthand for { first: first, last: last }

const field = "email";
const record = { [field]: "ada@example.com" }; // computed key -> { email: "..." }
```

## Spreading objects

```js
const merged = { ...defaults, ...options }; // later keys override earlier ones
const updated = { ...person, last: "Byron" }; // copy with one field changed
```

Spread is shallow — nested objects/arrays are shared by reference between the
original and the copy (see [Topic 01](../01-variables-and-data-types/notes.md#reference-vs-value)).

## Destructuring

```js
const { first, last } = person;
const { first: givenName } = person;  // rename
const { nickname = "n/a" } = person;  // default if missing/undefined

function getFullName({ first, last }) {  // destructure in parameters
  return `${first} ${last}`;
}

const { position: { x, y } } = shape;  // nested

const { theme, ...rest } = options;  // rest collects remaining keys
```

## Iterating

```js
Object.keys(person);    // ["first", "last"]
Object.values(person);  // ["Ada", "Lovelace"]
Object.entries(person); // [["first", "Ada"], ["last", "Lovelace"]]

for (const [key, value] of Object.entries(person)) {
  console.log(key, value);
}
```

## `Object.freeze` — shallow immutability

```js
const obj = Object.freeze({ a: 1, nested: { b: 2 } });
Object.isFrozen(obj);        // true
obj.a = 99;                  // throws TypeError (strict mode — ES modules are always strict)
obj.nested.b = 99;           // allowed — freeze is shallow
Object.isFrozen(obj.nested); // false
```

To freeze nested structures too, recursively freeze each nested
object/array before freezing the parent ("deep freeze").

## Further Reading (MDN)

- [Working with objects](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Working_with_objects)
- [Grammar and types — Destructuring](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Grammar_and_types#destructuring)
- [Destructuring assignment (reference)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Destructuring_assignment)
- [Spread syntax (`...`)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Spread_syntax)
- [`Object.entries`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/entries)
- [`Object.freeze`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/freeze)
