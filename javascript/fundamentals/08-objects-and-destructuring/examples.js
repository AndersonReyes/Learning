// Run with: node examples.js

// --- Object literals & property access ---
const person = {
  first: "Ada",
  last: "Lovelace",
  greet() {
    return `Hi, I'm ${this.first}`;
  },
};
console.log("dot notation:", person.first);
console.log("bracket notation:", person["last"]);
console.log("method call:", person.greet());

// --- Shorthand and computed properties ---
const first = "Grace";
const last = "Hopper";
const shorthand = { first, last };
console.log("shorthand:", shorthand);

const field = "email";
const record = { [field]: "ada@example.com" };
console.log("computed key:", record);

// --- Spreading objects ---
const defaults = { theme: "light", fontSize: 12 };
const options = { theme: "dark" };
const merged = { ...defaults, ...options };
console.log("merged:", merged);

const updated = { ...person, last: "Byron" };
console.log("updated (original unchanged):", updated.last, person.last);

// --- Destructuring ---
const { first: f, last: l, age = 36 } = person;
console.log("destructured with default:", f, l, age);

// --- Destructuring function parameters ---
function getFullName({ first, last }) {
  return `${first} ${last}`;
}
console.log("getFullName:", getFullName(person));

// --- Nested destructuring ---
const shape = { position: { x: 10, y: 20 }, color: "red" };
const { position: { x, y } } = shape;
console.log("nested destructure:", x, y);

// --- Rest in object destructuring ---
const { theme, ...otherOptions } = { theme: "dark", fontSize: 12, lang: "en" };
console.log("theme:", theme);
console.log("otherOptions:", otherOptions);

// --- Object.keys / values / entries ---
console.log("Object.keys:", Object.keys(person));
console.log("Object.values:", Object.values({ a: 1, b: 2 }));
for (const [key, value] of Object.entries({ a: 1, b: 2 })) {
  console.log("entry:", key, value);
}

// --- Object.freeze (shallow) ---
const frozen = Object.freeze({ a: 1, nested: { b: 2 } });
console.log("isFrozen(frozen):", Object.isFrozen(frozen));
console.log("isFrozen(frozen.nested):", Object.isFrozen(frozen.nested));

frozen.nested.b = 99; // allowed — freeze is shallow
console.log("frozen.nested.b after mutation:", frozen.nested.b);

try {
  frozen.a = 99; // throws in strict mode (ES modules are always strict)
} catch (err) {
  console.log("mutating a frozen property threw:", err.constructor.name);
}
