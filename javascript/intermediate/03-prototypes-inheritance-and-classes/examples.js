// Run with: node examples.js

// --- Every object has a prototype; property lookup walks the chain ---
const animal = {
  speak() {
    return `${this.name} makes a sound`;
  },
};

const dog = Object.create(animal); // dog's prototype is `animal`
dog.name = "Rex";

console.log("dog.speak():", dog.speak());
console.log("getPrototypeOf(dog) === animal:", Object.getPrototypeOf(dog) === animal);
console.log("dog.hasOwnProperty('name'):", dog.hasOwnProperty("name"));
console.log("dog.hasOwnProperty('speak'):", dog.hasOwnProperty("speak"));

// --- class is sugar over prototypes ---
class Animal {
  constructor(name) {
    this.name = name; // own property, per-instance
  }
  speak() {
    // lives on Animal.prototype — shared, not copied
    return `${this.name} makes a sound`;
  }
}

const a1 = new Animal("Rex");
const a2 = new Animal("Milo");
console.log("a1.speak():", a1.speak());
console.log("getPrototypeOf(a1) === Animal.prototype:", Object.getPrototypeOf(a1) === Animal.prototype);
console.log("a1.hasOwnProperty('name'):", a1.hasOwnProperty("name"));
console.log("a1.hasOwnProperty('speak'):", a1.hasOwnProperty("speak"));
console.log("a1.speak === a2.speak (shared method):", a1.speak === a2.speak);

// --- extends / super — inheritance & polymorphism ---
class Dog extends Animal {
  constructor(name, breed) {
    super(name); // must call super() before using `this`
    this.breed = breed;
  }
  speak() {
    // override — polymorphism
    return `${this.name} barks`;
  }
  parentSpeak() {
    return super.speak(); // explicitly call the overridden method
  }
}

const d = new Dog("Fido", "Lab");
console.log("d.speak():", d.speak());
console.log("d.parentSpeak():", d.parentSpeak());
console.log("d instanceof Dog:", d instanceof Dog);
console.log("d instanceof Animal:", d instanceof Animal);

// Polymorphism: same method call, different behavior per actual type
for (const creature of [a1, d]) {
  console.log(`polymorphic speak (${creature.constructor.name}):`, creature.speak());
}

// --- Static methods & properties ---
class Counter {
  static count = 0;
  constructor(name) {
    this.name = name;
    Counter.count++;
  }
  static describe() {
    return `${Counter.count} counters created`;
  }
}

new Counter("a");
new Counter("b");
console.log("Counter.describe():", Counter.describe());
console.log("Counter.count:", Counter.count);
console.log("instance.count is undefined:", new Counter("c").count);

// --- Private fields (#field) vs convention (_field) ---
class Account {
  #balance = 0; // truly private — only accessible inside the class
  _label = "account"; // convention only — reachable, but "please don't touch"

  constructor(balance) {
    this.#balance = balance;
  }
  deposit(amount) {
    this.#balance += amount;
    return this.#balance;
  }
  get balance() {
    return this.#balance;
  }
}

const acct = new Account(100);
console.log("acct.balance (via getter):", acct.balance);
console.log("acct.deposit(50):", acct.deposit(50));
console.log("acct._label (convention, accessible):", acct._label);
console.log("'#balance' in Object.keys(acct):", Object.keys(acct).includes("#balance"));
// acct.#balance;          // SyntaxError if uncommented — not valid outside the class

// --- Mixins — composing behavior (single inheritance workaround) ---
const SerializableMixin = {
  serialize() {
    return JSON.stringify(this);
  },
};

class Point {
  constructor(x, y) {
    this.x = x;
    this.y = y;
  }
}

Object.assign(Point.prototype, SerializableMixin);

const p = new Point(1, 2);
console.log("p.serialize():", p.serialize());

// Another unrelated class can mix in the same behavior
class Config {
  constructor(settings) {
    Object.assign(this, settings);
  }
}
Object.assign(Config.prototype, SerializableMixin);
console.log("new Config({theme:'dark'}).serialize():", new Config({ theme: "dark" }).serialize());

// --- Gotcha: instanceof walks the whole prototype chain ---
class A {}
class B extends A {}
const b = new B();
console.log("b instanceof B:", b instanceof B);
console.log("b instanceof A:", b instanceof A); // true — A.prototype is in b's chain

// --- Gotcha: Object.create(null) — no prototype at all ---
const bare = Object.create(null);
bare.x = 1;
console.log("Object.getPrototypeOf(bare):", Object.getPrototypeOf(bare));
console.log("bare.toString:", bare.toString); // undefined — no Object.prototype in chain
console.log("bare.hasOwnProperty:", bare.hasOwnProperty); // undefined — even this is gone

// Safe way to check own properties on a null-prototype object:
console.log(
  "safe hasOwnProperty check:",
  Object.prototype.hasOwnProperty.call(bare, "x"),
);
