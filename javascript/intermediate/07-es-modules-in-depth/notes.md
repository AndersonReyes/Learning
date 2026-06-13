# 07. ES Modules (`import`/`export`) in Depth

## Named exports

Export multiple bindings by name — either inline or via an `export {}` list:

```js
// math.js
export const PI = 3.14159;
export function square(x) { return x * x; }

const E = 2.71828;
export { E };
```

Import named exports with matching names (rename with `as`):

```js
import { PI, square, E as EULER } from "./math.js";
```

## Default export

One default export per module — the "main" thing the module provides. Import
it with **any** name (no curly braces, no matching name required):

```js
// logger.js
export default function log(msg) { console.log(msg); }
```
```js
import log from "./logger.js";       // any local name
import myLogger from "./logger.js";  // same binding, different name
```

A module can have **both** named exports and a default export, but only
**one** default:

```js
// api.js
export const VERSION = "1.0";
export default function request(url) { /* ... */ }
```
```js
import request, { VERSION } from "./api.js";
```

## Re-exports

Forward another module's exports without importing them into the current
scope — useful for "barrel" files that aggregate a package's public API.

```js
// barrel.js
export * from "./math.js";              // re-export ALL named exports
export { square as sq } from "./math.js"; // re-export one, renamed
export { default as log } from "./logger.js"; // re-export a default as named
```

`export * from` only re-exports **named** exports — the source module's
default export is NOT included. Conflicting names from multiple `export *`
sources are NOT re-exported (silently omitted) unless explicitly
disambiguated with a named `export ... from`.

## Namespace imports

`import * as ns` collects all named exports (and `default`, if present) into
one namespace object:

```js
import * as math from "./math.js";
math.PI;       // 3.14159
math.square(4); // 16
math.default;  // the default export, if math.js has one
```

`ns` is a **live, read-only binding object** — properties update if the
source module's exported bindings change (see "Modules are singletons"
below), but you cannot reassign or add properties to `ns` itself:

```js
math.PI = 4; // TypeError: Cannot assign to read only property
```

## Dynamic `import()`

`import()` is a function-like operator (not a regular function) that returns
a **Promise resolving to the module's namespace object** — same shape as a
`import * as ns` namespace import. Use it for conditional or lazy loading
(code-splitting, avoiding loading unused modules):

```js
async function loadFeature(enabled) {
  if (!enabled) return;
  const mod = await import("./feature.js");
  mod.run();           // named export
  mod.default();        // default export, if any
}
```

Unlike static `import`, the specifier can be a runtime expression
(`import(`./locales/${lang}.js`)`), and `import()` can appear anywhere
(inside functions, conditionals, top-level `await`).

## Modules are singletons

A module's top-level code runs **once**, on the first import. Every importer
gets the **same module instance** — including shared mutable state.

```js
// store.js
export let count = 0;
export function increment() { count++; }
```
```js
// a.js
import { increment } from "./store.js";
increment(); // count becomes 1 in the shared store.js instance
```
```js
// b.js
import { count } from "./store.js";
console.log(count); // 1 — sees a.js's mutation, same module instance
```

This is why modules are a natural fit for singletons (shared config,
caches, connection pools) — no manual "only create one instance" bookkeeping
needed; the module system does it.

Note: a `let`/`var` exported binding is **live** — importers see updates.
But importers cannot themselves reassign an imported binding (`count = 5`
in `b.js` is a `SyntaxError`); only the exporting module can mutate it.

## Circular dependency gotcha

If module A imports from B, and B imports from A, one of them gets
evaluated first — and at that point, the other module's exports may not be
initialized yet.

```js
// a.js
import { b } from "./b.js";
export const a = "A";
console.log(b); // undefined if b.js is still being evaluated
```
```js
// b.js
import { a } from "./a.js";
export const b = "B";
console.log(a); // depends on evaluation order
```

**Fix**: don't access circular imports at module top-level. Defer access
into a function body — by the time the function runs, both modules have
finished evaluating:

```js
// a.js
import { b } from "./b.js";
export const a = "A";
export function useB() {
  console.log(b); // "B" — safe, called after both modules finished loading
}
```

Function declarations (`export function`) are hoisted and don't suffer this
problem the same way — circular dependencies on **functions** (not
values computed at module top-level) tend to "just work."

## File extensions are required

Node ESM requires explicit file extensions in relative import specifiers:

```js
import { square } from "./math.js"; // OK
import { square } from "./math";    // Error: Cannot find module
```

(CommonJS `require` and bundler-based setups often allow omitting the
extension — ESM in Node does not.)

## Further Reading (MDN)

- [Modules](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Modules)
- [`export`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/export)
- [`import`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/import)
- [`import()`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/import)
