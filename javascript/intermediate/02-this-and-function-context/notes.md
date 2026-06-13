# 02. `this` Keyword & Function Context

## `this` depends on the call-site, not the definition site

`this` is determined by **how a function is called** — not where it's
written. The same function can get a different `this` each time it's called:

```js
function whoAmI() {
  return this;
}

whoAmI();             // call-site: plain call
obj.whoAmI = whoAmI;
obj.whoAmI();         // call-site: method call -> different `this`
```

Arrow functions are the one exception (see below) — they ignore the
call-site entirely.

## Plain function call -> `this` is `undefined` (strict mode)

ES modules are always strict mode (see
[fundamentals/08](../../fundamentals/08-objects-and-destructuring/notes.md#objectfreeze--shallow-immutability)).
In strict mode, calling a plain function leaves `this` as `undefined`:

```js
function showThis() {
  return this;
}
showThis(); // undefined (strict mode)
```

In non-strict (sloppy) mode `this` would default to the global object —
this old behavior is why `this` in a careless plain call can silently point
at `globalThis` in scripts. ES modules don't have this problem.

## Method call -> `this` is the object before the dot

```js
const counter = {
  count: 0,
  increment() {
    this.count += 1;
    return this.count;
  },
};

counter.increment(); // 1 -- `this` is `counter`
```

Only the **call-site** matters — the function doesn't "belong" to the
object in any binding sense:

```js
const increment = counter.increment;
increment(); // this is undefined here -- TypeError reading this.count
```

## Arrow functions: `this` is lexical, not call-site

Arrow functions have **no own `this`**. They capture `this` from the
enclosing scope at the point they're *defined*, and keep it forever —
`.call`/`.apply`/`.bind` cannot change it.

```js
const obj = {
  value: 42,
  regular: function () {
    return this.value;
  },
  arrow: () => {
    return this.value; // `this` is whatever `this` is OUTSIDE obj -- NOT obj
  },
};

obj.regular(); // 42
obj.arrow();   // undefined -- arrow captured module-level `this` (undefined in ES modules)
```

**Classic gotcha**: an arrow function as an object property does NOT get the
object as `this`. Use a regular method (shorthand `method() {}`) when you
need `this` to be the object.

Arrows ARE the right tool *inside* a method, to preserve the enclosing
`this` for nested callbacks:

```js
const timer = {
  seconds: 0,
  start() {
    setInterval(() => {
      this.seconds += 1; // arrow captures `this` from start() -> timer
    }, 1000);
  },
};
```

If `start` used a regular `function` for the `setInterval` callback, `this`
inside it would be `undefined` (plain call) -- not `timer`.

## `call` and `apply` -- explicit `this`, called immediately

Both invoke the function immediately with a given `this`; they differ only
in how extra arguments are passed:

```js
function introduce(greeting) {
  return `${greeting}, I'm ${this.name}`;
}

const ada = { name: "Ada" };

introduce.call(ada, "Hi");      // "Hi, I'm Ada"  -- args listed individually
introduce.apply(ada, ["Hi"]);   // "Hi, I'm Ada"  -- args passed as an array
```

Mnemonic: **A**pply takes an **A**rray.

## `bind` -- returns a new function with `this` permanently fixed

Unlike `call`/`apply`, `bind` does NOT call the function -- it returns a new
function with `this` (and optionally leading arguments) locked in:

```js
const greetAda = introduce.bind(ada);
greetAda("Hi"); // "Hi, I'm Ada" -- callable later, `this` is fixed

const greetAdaHi = introduce.bind(ada, "Hi"); // partial application
greetAdaHi(); // "Hi, I'm Ada"
```

A bound function's `this` cannot be changed again, even with another
`.call`/`.apply`/`.bind`.

## The "losing `this`" gotcha -- extracting a method as a callback

Passing `obj.method` around detaches it from `obj`. By the time it's
*called*, the call-site is just a plain function call:

```js
const counter = {
  count: 0,
  increment() {
    this.count += 1;
  },
};

setTimeout(counter.increment, 0); // `this` is undefined inside increment -- TypeError
[1].forEach(counter.increment);   // same problem
```

**Fixes**:

```js
setTimeout(counter.increment.bind(counter), 0); // bind `this`
setTimeout(() => counter.increment(), 0);       // arrow wrapper -- looks up
                                                 // `counter` at call time
```

## Class methods are NOT auto-bound

Methods defined in a `class` body behave like the method-call case above --
they only have the right `this` when called as `instance.method()`:

```js
class Counter {
  count = 0;
  increment() {
    this.count += 1;
    return this.count;
  }
}

const c = new Counter();
c.increment();              // works -- `this` is `c`

const fn = c.increment;
fn();                        // `this` is undefined -- TypeError

button.addEventListener("click", c.increment); // same gotcha
```

**Fixes**: same as above -- `c.increment.bind(c)`, an arrow wrapper, or bind
in the constructor (`this.increment = this.increment.bind(this)`), or use an
arrow function as a class field (`increment = () => { ... }`, which captures
`this` lexically since it's defined in the constructor's scope).

## Further Reading (MDN)

- [`this`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/this)
- [`Function.prototype.call()`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Function/call)
- [`Function.prototype.apply()`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Function/apply)
- [`Function.prototype.bind()`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Function/bind)
- [Arrow function expressions](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Functions/Arrow_functions)
