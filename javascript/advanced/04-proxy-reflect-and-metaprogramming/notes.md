# Advanced 04. Proxy, Reflect & Metaprogramming

## `Proxy`

```js
const proxy = new Proxy(target, handler);
```

- `target` — the object being wrapped.
- `handler` — an object of TRAPS (functions) that intercept fundamental
  operations on `proxy`. Any operation without a corresponding trap falls
  through to `target`'s default behavior.
- `proxy !== target` — the proxy is a DIFFERENT object (different identity),
  but operations on it are forwarded to/intercept `target`.

### Common traps

| Trap | Intercepts | Must return |
|---|---|---|
| `get(target, prop, receiver)` | `proxy.prop`, `proxy[prop]` | the value |
| `set(target, prop, value, receiver)` | `proxy.prop = v` | `boolean` (success) |
| `has(target, prop)` | `prop in proxy` | `boolean` |
| `deleteProperty(target, prop)` | `delete proxy.prop` | `boolean` (success) |
| `ownKeys(target)` | `Object.keys`, `for...in`, spread | array of keys |
| `getPrototypeOf(target)` | `Object.getPrototypeOf`, `instanceof` | an object or `null` |
| `defineProperty(target, prop, desc)` | `Object.defineProperty` | `boolean` |
| `apply(target, thisArg, args)` | calling `proxy(...)` (target must be callable) | the return value |
| `construct(target, args, newTarget)` | `new proxy(...)` (target must be a constructor) | an object |

## `Reflect`

Static methods that mirror every trap with the DEFAULT behavior —
`Reflect.get`, `Reflect.set`, `Reflect.has`, `Reflect.deleteProperty`,
`Reflect.ownKeys`, `Reflect.apply`, `Reflect.construct`, etc.

- Inside a trap, call the matching `Reflect.x(...)` to fall back to default
  behavior for cases you don't want to customize.
- **`Reflect.get(target, prop, receiver)` / `Reflect.set(..., receiver)`** —
  the `receiver` argument matters when `target` has INHERITED getters/setters:
  it's the `this` used for the accessor. Passing the proxy's `receiver`
  through (instead of using `target.prop` / `target.prop = v` directly) keeps
  inherited accessors working correctly when the proxy sits in a prototype
  chain.
- `Reflect.apply(fn, thisArg, args)` === `fn.apply(thisArg, args)` but works
  even if `fn.apply` has been overwritten/deleted.

```js
const handler = {
  get(target, prop, receiver) {
    console.log(`reading "${String(prop)}"`);
    return Reflect.get(target, prop, receiver); // default behavior
  },
};
```

## Invariants

Traps can't lie about certain things — violating an invariant throws a
`TypeError`:

- If a property on `target` is non-configurable AND non-writable, `get` MUST
  return its actual value.
- `has` can't report a non-configurable own property of `target` as absent.
- `deleteProperty` can't report success (`true`) for a non-configurable
  property.

## Revocable proxies

```js
const { proxy, revoke } = Proxy.revocable(target, handler);
proxy.x; // works
revoke();
proxy.x; // throws TypeError — proxy is "dead"
```

Useful for revoking access to an object (e.g. after a session ends) without
needing the original object to cooperate.

## Well-known symbols (the other half of "metaprogramming")

- **`Symbol.iterator`** — makes an object iterable with `for...of` /
  spread. Covered in
  [Intermediate 08](../../intermediate/08-iterators-generators-and-symbol-iterator).
- **`Symbol.toPrimitive(hint)`** — customizes coercion. `hint` is
  `"number"`, `"string"`, or `"default"`.
  ```js
  const money = {
    [Symbol.toPrimitive](hint) {
      if (hint === "string") return "$10.00";
      return 10; // "number" and "default"
    },
  };
  +money; // 10
  `${money}`; // "$10.00"
  money + ""; // "10" (default hint, then string-concatenated)
  ```
- **`Symbol.toStringTag`** — customizes `Object.prototype.toString.call(x)`:
  ```js
  class Matrix {
    get [Symbol.toStringTag]() { return "Matrix"; }
  }
  Object.prototype.toString.call(new Matrix()); // "[object Matrix]"
  ```
- **`Symbol.hasInstance`** — customizes `instanceof`:
  ```js
  const Even = { [Symbol.hasInstance](x) { return typeof x === "number" && x % 2 === 0; } };
  4 instanceof Even; // true
  3 instanceof Even; // false
  ```

## Gotchas

- **Identity**: `proxy !== target` and `proxy !== new Proxy(target, handler)`
  (a fresh wrapper each time) — don't rely on `===` to detect "is this my
  object", compare the underlying `target` instead if needed.
- **`this` inside intercepted methods**: calling `proxy.method()` invokes
  `method` with `this === proxy` (not `target`) by default, so the method's
  internal property accesses re-enter the proxy's traps too. Use
  `Reflect.apply(fn, target, args)` inside a `get` trap if you want the
  ORIGINAL object as `this` instead.
- **Forgetting a trap ≠ removing behavior** — an `omitted` trap falls back to
  default (`Reflect.x`) behavior on `target`; to actually BLOCK an operation
  you must define the trap and return `false` / throw.
- **Performance**: every trapped operation adds overhead — avoid wrapping
  objects accessed in hot loops if it's avoidable.
- **Arrays behind a Proxy**: `length` updates automatically when you set/
  delete numeric indices via the default `set`/`deleteProperty` behavior —
  but a custom `set` trap that doesn't delegate to `Reflect.set` can leave
  `length` out of sync.

## Further Reading (MDN)

- [Meta programming](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Meta_programming)
- [`Proxy`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Proxy)
- [`Reflect`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect)
- [`Symbol.toPrimitive`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Symbol/toPrimitive)
- [`Symbol.hasInstance`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Symbol/hasInstance)
- [`Symbol.toStringTag`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Symbol/toStringTag)
