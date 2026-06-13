// Run with: node examples.js

// --- `this` depends on the call-site, not the definition site ---
function whoAmI() {
  return this;
}
const holder = { whoAmI };
console.log("plain call this:", whoAmI()); // undefined (strict mode)
console.log("method call this === holder:", holder.whoAmI() === holder); // true

// --- Plain function call -> `this` is undefined (strict mode) ---
function showThis() {
  return this;
}
console.log("showThis():", showThis()); // undefined

// --- Method call -> `this` is the object before the dot ---
const counter = {
  count: 0,
  increment() {
    this.count += 1;
    return this.count;
  },
};
console.log("counter.increment():", counter.increment()); // 1
console.log("counter.increment():", counter.increment()); // 2

// Detaching the method loses `this`
const increment = counter.increment;
try {
  increment(); // `this` is undefined here
} catch (err) {
  console.log("detached increment() threw:", err.constructor.name);
}

// --- Arrow functions: `this` is lexical, not call-site ---
const obj = {
  value: 42,
  regular: function () {
    return this.value;
  },
  arrow: () => {
    return this; // `this` is whatever `this` is OUTSIDE obj
  },
};
console.log("obj.regular():", obj.regular()); // 42
console.log("obj.arrow():", obj.arrow()); // undefined -- captured module-level `this`

// this.value (instead of just this) would THROW -- module-level `this` is
// undefined, so reading a property off it is a TypeError, not `undefined`
const throwingArrow = () => this.value;
try {
  throwingArrow();
} catch (err) {
  console.log("this.value on module-level this threw:", err.constructor.name);
}

// Arrows preserve enclosing `this` for nested callbacks
const timer = {
  seconds: 0,
  tick() {
    const tickOnce = () => {
      this.seconds += 1; // arrow captures `this` from tick() -> timer
    };
    tickOnce();
    tickOnce();
    return this.seconds;
  },
};
console.log("timer.tick():", timer.tick()); // 2

// --- call and apply: explicit `this`, called immediately ---
function introduce(greeting) {
  return `${greeting}, I'm ${this.name}`;
}
const ada = { name: "Ada" };
console.log("introduce.call(ada, 'Hi'):", introduce.call(ada, "Hi"));
console.log("introduce.apply(ada, ['Hi']):", introduce.apply(ada, ["Hi"]));

// --- bind: returns a new function with `this` permanently fixed ---
const greetAda = introduce.bind(ada);
console.log("greetAda('Hi'):", greetAda("Hi"));

const greetAdaHi = introduce.bind(ada, "Hi"); // partial application
console.log("greetAdaHi():", greetAdaHi());

// --- The "losing this" gotcha: extracting a method as a callback ---
const detachedIncrement = counter.increment;
try {
  [1].forEach(detachedIncrement); // `this` is undefined inside increment
} catch (err) {
  console.log("forEach(detachedIncrement) threw:", err.constructor.name);
}

// Fix 1: bind
const boundIncrement = counter.increment.bind(counter);
console.log("boundIncrement():", boundIncrement()); // 3 (counter.count was 2)

// Fix 2: arrow wrapper -- looks up `counter` at call time
const wrappedIncrement = () => counter.increment();
console.log("wrappedIncrement():", wrappedIncrement()); // 4

// --- Class methods are NOT auto-bound ---
class Counter {
  count = 0;
  increment() {
    this.count += 1;
    return this.count;
  }
}
const c = new Counter();
console.log("c.increment():", c.increment()); // 1 -- `this` is `c`

const fn = c.increment;
try {
  fn(); // `this` is undefined -- TypeError
} catch (err) {
  console.log("detached class method threw:", err.constructor.name);
}

// Fix: bind in constructor or extract a pre-bound reference
const boundFn = c.increment.bind(c);
console.log("boundFn():", boundFn()); // 2
