# 03. Prototypes, Inheritance & Classes (OOP in JS)

## Every object has a prototype

Objects link to another object (their **prototype**). Property lookup that
fails on the object itself walks up this chain until it finds the property or
hits `null`:

```js
const animal = {
  speak() { return `${this.name} makes a sound`; },
};

const dog = Object.create(animal); // dog's prototype is `animal`
dog.name = "Rex";

dog.speak();                      // "Rex makes a sound" — found on `animal`
Object.getPrototypeOf(dog) === animal; // true
dog.hasOwnProperty("name");       // true  — own property
dog.hasOwnProperty("speak");      // false — inherited via the chain
```

`Object.create(proto)` creates a new object with `proto` as its prototype —
no constructor, no `class`, just direct prototype linkage.

## `class` is sugar over prototypes

```js
class Animal {
  constructor(name) {
    this.name = name;       // own property, per-instance
  }
  speak() {                  // lives on Animal.prototype — SHARED, not copied
    return `${this.name} makes a sound`;
  }
}

const a = new Animal("Rex");
Object.getPrototypeOf(a) === Animal.prototype; // true
a.hasOwnProperty("name");   // true
a.hasOwnProperty("speak");  // false — one shared function on the prototype
```

Every instance shares the **same** `speak` function object — methods aren't
copied per-instance, which is why `class`/prototypes are memory-efficient
versus closures-with-methods-as-own-properties.

## `extends` / `super` — inheritance & polymorphism

```js
class Dog extends Animal {
  constructor(name, breed) {
    super(name);            // MUST call super() before using `this`
    this.breed = breed;
  }
  speak() {                  // override — polymorphism
    return `${this.name} barks`;
  }
  parentSpeak() {
    return super.speak();   // explicitly call the overridden method
  }
}

const d = new Dog("Fido", "Lab");
d.speak();              // "Fido barks" — Dog's override wins
d.parentSpeak();        // "Fido makes a sound" — Animal's original
d instanceof Dog;       // true
d instanceof Animal;    // true — extends chains the prototypes
```

`extends` sets `Dog.prototype`'s prototype to `Animal.prototype`, so lookup
falls through to the parent's methods when a subclass doesn't override them.

## Static methods & properties

`static` members live on the **class itself**, not on instances or
`.prototype`:

```js
class Animal {
  static count = 0;
  constructor(name) {
    this.name = name;
    Animal.count++;
  }
  static describe() {
    return `${Animal.count} animals created`;
  }
}

new Animal("Rex");
new Animal("Milo");
Animal.describe();   // "2 animals created"
Animal.count;        // 2
new Animal("X").count; // undefined — not on instances
```

Use `static` for factory methods, counters, and constants tied to the class
rather than any one instance.

## Private fields (`#field`) vs convention (`_field`)

```js
class Account {
  #balance = 0;            // truly private — only accessible inside the class
  _label = "account";      // convention only — "please don't touch", but reachable

  constructor(balance) { this.#balance = balance; }
  deposit(amount) { this.#balance += amount; return this.#balance; }
  get balance() { return this.#balance; }
}

const acct = new Account(100);
acct.balance;     // 100 — via getter
acct._label;      // "account" — accessible, just a convention
acct.#balance;    // SyntaxError — #fields aren't even valid syntax outside the class
```

- `#field`: enforced by the engine. Inaccessible (and not even visible via
  `Object.keys`/`for...in`) outside the class body — not even subclasses can
  reach a parent's `#field` directly.
- `_field`: a plain (enumerable, public) property. The underscore is purely a
  signal to other developers — does nothing to prevent access or mutation.

## Mixins — composing behavior (JS has single inheritance)

A class can only `extends` one parent. To share behavior across unrelated
class hierarchies, copy methods onto a prototype with `Object.assign`:

```js
const SerializableMixin = {
  serialize() { return JSON.stringify(this); },
};

class Point {
  constructor(x, y) { this.x = x; this.y = y; }
}

Object.assign(Point.prototype, SerializableMixin);

new Point(1, 2).serialize(); // '{"x":1,"y":2}'
```

Any class can mix in the same object of methods — composition instead of deep
inheritance trees.

## Gotchas

**`instanceof` walks the prototype chain** — `x instanceof C` checks whether
`C.prototype` appears anywhere in `x`'s chain, not whether `x` was literally
built by `C`'s constructor:

```js
class A {}
class B extends A {}
const b = new B();
b instanceof B; // true
b instanceof A; // true — A.prototype is in b's chain via extends
```

**`Object.create(null)` — no prototype at all**:

```js
const bare = Object.create(null);
bare.toString;          // undefined — no Object.prototype in the chain
Object.getPrototypeOf(bare); // null
bare.hasOwnProperty;    // undefined — even hasOwnProperty is gone!
Object.prototype.hasOwnProperty.call(bare, "x"); // safe way to check
```

Useful for objects used purely as dictionaries/maps, where you don't want
inherited keys (`toString`, `constructor`, etc.) to collide with data keys.

## Further Reading (MDN)

- [Inheritance and the prototype chain](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Inheritance_and_the_prototype_chain)
- [Using classes](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Using_classes)
- [`Object.create`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/create)
- [`Object.getPrototypeOf`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/getPrototypeOf)
- [Private class features](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Classes/Private_properties)
- [`static`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Classes/static)
- [`extends`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Classes/extends)
- [`super`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/super)
- [`instanceof`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/instanceof)
