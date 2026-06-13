# 10. Enumerability & Property Descriptors

## Property descriptors

Every object property has a descriptor with attributes controlling how it
behaves. Two kinds:

- **Data descriptor**: `{ value, writable, enumerable, configurable }`
- **Accessor descriptor**: `{ get, set, enumerable, configurable }`

```js
const obj = { a: 1 };
Object.getOwnPropertyDescriptor(obj, "a");
// { value: 1, writable: true, enumerable: true, configurable: true }

Object.getOwnPropertyDescriptors(obj); // all own descriptors, keyed by prop name
```

Properties created via assignment (`obj.a = 1`) or object literals default to
`writable: true, enumerable: true, configurable: true`. Properties created via
`Object.defineProperty` default **everything to `false`** unless specified:

```js
const obj2 = {};
Object.defineProperty(obj2, "hidden", { value: 1 });
Object.getOwnPropertyDescriptor(obj2, "hidden");
// { value: 1, writable: false, enumerable: false, configurable: false }

Object.keys(obj2);        // [] — hidden not enumerable
obj2.hidden;               // 1 — still readable
obj2.hidden = 2;           // silently fails (non-writable, non-strict) or
                           // throws TypeError (strict mode — ES modules)
```

When **redefining an existing property**, unspecified attributes keep their
current values (not reset to `false`).

## What each attribute controls

| Attribute | `true` | `false` |
|---|---|---|
| `writable` | value can be reassigned | assignment throws (strict) / no-ops |
| `enumerable` | shows in `for...in`, `Object.keys`, spread, `JSON.stringify` | hidden from those, but still own/accessible |
| `configurable` | property can be deleted or redefined | `delete` throws; most redefinitions throw |

`configurable: false` is the strictest — it also blocks switching between
data and accessor descriptors, and (if `writable: false`) blocks changing
`writable` back to `true`. Note `writable` CAN still go `true -> false` even
when non-configurable.

## Enumerating properties — pick the right tool

| Method | Own only? | Enumerable only? | Includes symbols? |
|---|---|---|---|
| `Object.keys(obj)` / `Object.values` / `Object.entries` | yes | yes | no |
| `for...in` | **no** (walks prototype chain) | yes | no |
| `Object.getOwnPropertyNames(obj)` | yes | no (all) | no |
| `Object.getOwnPropertySymbols(obj)` | yes | no (all) | yes (only) |
| `Reflect.ownKeys(obj)` | yes | no (all) | yes |
| `JSON.stringify` / spread `{...obj}` | yes | yes | no |

```js
const proto = { fromProto: 1 };
const obj = Object.create(proto);
obj.own = 2;
Object.defineProperty(obj, "hidden", { value: 3, enumerable: false });

Object.keys(obj);              // ["own"]
for (const k in obj) { /* "own", "fromProto" */ }
Object.getOwnPropertyNames(obj); // ["own", "hidden"]
```

## Own vs. inherited

```js
obj.hasOwnProperty("own");     // true
obj.hasOwnProperty("fromProto"); // false — inherited
"fromProto" in obj;            // true — `in` checks the whole prototype chain
Object.prototype.propertyIsEnumerable.call(obj, "hidden"); // false (own, but not enumerable)
Object.prototype.propertyIsEnumerable.call(obj, "fromProto"); // false (not OWN, regardless of enumerable)
```

`hasOwnProperty`/`propertyIsEnumerable` are called via
`Object.prototype.x.call(obj, ...)` (or `Object.hasOwn(obj, key)` — ES2022)
when `obj` might be `Object.create(null)` (no inherited methods) or might
have shadowed `hasOwnProperty` itself.

## Freeze / seal / prevent extensions

| | new props | delete/configure existing | write existing values |
|---|---|---|---|
| `Object.preventExtensions` | blocked | allowed | allowed |
| `Object.seal` | blocked | blocked (`configurable: false` on all) | allowed (if `writable`) |
| `Object.freeze` | blocked | blocked | blocked (`writable: false` too) |

Each has an `is*` check: `Object.isExtensible`, `Object.isSealed`,
`Object.isFrozen`. All are **shallow** — nested objects are unaffected (see
[Topic 08](../../fundamentals/08-objects-and-destructuring/notes.md#objectfreeze--shallow-immutability)
for the "deep freeze" idea, implemented as an exercise here).

## Symbol-keyed properties

Symbols are valid property keys but invisible to the "enumerable own keys"
APIs (`Object.keys`, `for...in`, `JSON.stringify`, spread) — only
`Object.getOwnPropertySymbols` and `Reflect.ownKeys` see them. This makes
symbols useful for "hidden" metadata that coexists with normal data without
colliding with string keys or showing up in casual iteration. (Full
metaprogramming with symbols — `Symbol.iterator`, well-known symbols, `Proxy`
— is its own topic later in the roadmap.)

## Further Reading (MDN)

- [Enumerability and ownership of properties](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Enumerability_and_ownership_of_properties)
- [`Object.defineProperty`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/defineProperty)
- [`Object.getOwnPropertyDescriptor`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/getOwnPropertyDescriptor)
- [`Object.freeze`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/freeze) /
  [`Object.seal`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/seal) /
  [`Object.preventExtensions`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/preventExtensions)
- [`Reflect.ownKeys`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/ownKeys)
- [`Object.hasOwn`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/hasOwn)
