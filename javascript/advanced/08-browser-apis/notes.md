# Advanced 08. Browser APIs (DOM, Fetch, Events)

> Adapted scope: there's no DOM in Node (no `document`, `window`, elements).
> This topic covers Web APIs that ARE available as **globals in Node 22**:
> `EventTarget`/`CustomEvent`, `URL`/`URLSearchParams`, `fetch` (exercised via
> injected fetch functions — no real network calls), and `structuredClone`.
> For DOM/element manipulation and CSS, see the `html/` and `css/` tracks —
> those cover the visual DOM with runnable browser examples.

## `EventTarget` / `CustomEvent` — the platform event system

`EventTarget` is the base "pub/sub" interface used throughout the web
platform (DOM nodes, `XMLHttpRequest`, `AbortSignal`, etc.) and available as
a standalone class in Node:

```js
const target = new EventTarget();

target.addEventListener("greet", (event) => {
  console.log("hello", event.detail);
});

target.dispatchEvent(new CustomEvent("greet", { detail: "world" }));
// "hello world"
```

- **`CustomEvent`** carries arbitrary data in `event.detail` (a plain `Event`
  has no payload).
- **`addEventListener(type, listener, options)`** — `options.once: true`
  auto-removes the listener after it fires once.
- **`removeEventListener(type, listener)`** requires the *same function
  reference* passed to `addEventListener` — an inline arrow function can
  never be removed. Store a reference if you need to unsubscribe.

```js
const handler = (e) => console.log(e.detail);
target.addEventListener("x", handler);
target.removeEventListener("x", handler); // works -- same reference

target.addEventListener("y", (e) => console.log(e.detail));
// no way to remove this listener -- never stored
```

## `URL` — parsing and building URLs

```js
const url = new URL("/users/1?active=true", "https://api.example.com/v1/");
url.href;     // "https://api.example.com/users/1?active=true"
url.origin;   // "https://api.example.com"
url.pathname; // "/users/1"
url.search;   // "?active=true"
url.hostname; // "api.example.com"
```

**Relative resolution gotcha**: a relative path *replaces* the last path
segment of the base, it doesn't append:

```js
new URL("users", "https://api.example.com/v1").href;
// "https://api.example.com/users"  -- "v1" was REPLACED, not joined!

new URL("users", "https://api.example.com/v1/").href;
// "https://api.example.com/v1/users" -- trailing slash on base matters
```

To reliably "join" a base + path, build `url.pathname` explicitly rather
than relying on relative-URL resolution.

## `URLSearchParams` — query strings

```js
const params = new URLSearchParams("a=1&b=2&b=3");
params.get("a");      // "1" (first match only)
params.getAll("b");   // ["2", "3"] (all matches, for repeated keys)
params.has("c");      // false

params.append("c", "x");
params.toString();    // "a=1&b=2&b=3&c=x"
```

- Repeated keys (`b=2&b=3`) are how query strings represent arrays — there's
  no native array syntax.
- `URLSearchParams.toString()` encodes spaces as `+` (the
  `application/x-www-form-urlencoded` convention), not `%20`.
- A `URL` object's `.searchParams` is a live `URLSearchParams` — mutating it
  updates `url.search`/`url.href` automatically.

## `fetch` — making requests (and testing without a network)

`fetch(url)` returns a `Promise<Response>`. Key `Response` properties:
`.ok` (true for 2xx status), `.status`, `.json()` / `.text()` (also
Promises).

```js
async function getJSON(url) {
  const response = await fetch(url);
  if (!response.ok) throw new Error(`HTTP ${response.status}`);
  return response.json();
}
```

**Testing without a real network**: write code that accepts `fetch` (or a
fetch-like function) as a parameter, and pass a fake implementation in tests
that returns objects shaped like `Response` (`{ ok, status, json() }`). This
is **dependency injection** — the same pattern as injecting a clock or
random source.

## `structuredClone` — deep copying

```js
const original = { nested: { arr: [1, 2, 3] }, map: new Map([["x", 1]]) };
const clone = structuredClone(original);

clone.nested.arr.push(4);
original.nested.arr; // [1, 2, 3] -- unaffected, fully independent copy
clone.map instanceof Map; // true -- Map/Set/Date/RegExp/typed arrays survive
```

Unlike `JSON.parse(JSON.stringify(x))`, `structuredClone`:
- Preserves `Map`, `Set`, `Date`, `RegExp`, typed arrays, `ArrayBuffer`.
- Handles circular references.
- Does **NOT** clone functions, DOM nodes, or class instances with
  prototypes other than plain objects/arrays (throws `DataCloneError`).

## `AbortController` / `AbortSignal` — cancellation

```js
const controller = new AbortController();
fetch(url, { signal: controller.signal });
controller.abort(); // aborts the in-flight fetch, rejects with AbortError
```

A standard cancellation token, accepted by `fetch` and other async Web APIs.
Useful for timeouts (abort after N ms) or cancel buttons.

## Gotchas

- **DOM events vs `EventTarget`**: DOM elements extend `EventTarget`, so
  everything above applies to `addEventListener`/`dispatchEvent` on real
  elements too — but elements/`document`/`window` don't exist in Node.
- **Listener identity**: `removeEventListener` and `off`-style APIs need the
  *exact function reference* used to subscribe.
- **URL relative resolution**: don't assume `new URL(path, base)` "appends" —
  it follows browser link-resolution rules (replaces the last segment unless
  the base ends with `/`).
- **`URLSearchParams` encodes spaces as `+`**, not `%20` — both decode to a
  space, but don't `assert.equal` against a string with `%20` literally.
- **`structuredClone` throws on non-cloneable values** — functions, class
  instances (other than built-ins like `Map`/`Date`), and DOM nodes.

## Further Reading (MDN)

- [Web APIs](https://developer.mozilla.org/en-US/docs/Web/API)
- [`EventTarget`](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget)
- [`CustomEvent`](https://developer.mozilla.org/en-US/docs/Web/API/CustomEvent)
- [`URL`](https://developer.mozilla.org/en-US/docs/Web/API/URL)
- [`URLSearchParams`](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams)
- [`fetch`](https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API)
- [`structuredClone`](https://developer.mozilla.org/en-US/docs/Web/API/structuredClone)
- [`AbortController`](https://developer.mozilla.org/en-US/docs/Web/API/AbortController)
