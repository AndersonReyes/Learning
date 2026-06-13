// Run with: node examples.js

// --- Property descriptors ---
const obj = { a: 1 };
console.log("descriptor of 'a':", Object.getOwnPropertyDescriptor(obj, "a"));

const obj2 = {};
Object.defineProperty(obj2, "hidden", { value: 1 });
console.log("descriptor of defineProperty default:", Object.getOwnPropertyDescriptor(obj2, "hidden"));
console.log("Object.keys(obj2) (hidden not enumerable):", Object.keys(obj2));
console.log("obj2.hidden (still readable):", obj2.hidden);
try {
  obj2.hidden = 2; // strict mode: non-writable assignment throws
} catch (err) {
  console.log("obj2.hidden = 2 threw:", err.constructor.name);
}

console.log("getOwnPropertyDescriptors(obj):", Object.getOwnPropertyDescriptors(obj));

// --- Enumerating properties: pick the right tool ---
const proto = { fromProto: 1 };
const child = Object.create(proto);
child.own = 2;
Object.defineProperty(child, "hiddenOwn", { value: 3, enumerable: false });

console.log("Object.keys(child):", Object.keys(child));
const forInKeys = [];
for (const k in child) forInKeys.push(k);
console.log("for...in keys (own + inherited enumerable):", forInKeys);
console.log("Object.getOwnPropertyNames(child):", Object.getOwnPropertyNames(child));

// --- Own vs. inherited ---
console.log("child.hasOwnProperty('own'):", child.hasOwnProperty("own"));
console.log("child.hasOwnProperty('fromProto'):", child.hasOwnProperty("fromProto"));
console.log("'fromProto' in child:", "fromProto" in child);
console.log(
  "propertyIsEnumerable('hiddenOwn'):",
  Object.prototype.propertyIsEnumerable.call(child, "hiddenOwn"),
);
console.log(
  "propertyIsEnumerable('fromProto') (inherited, not own):",
  Object.prototype.propertyIsEnumerable.call(child, "fromProto"),
);
console.log("Object.hasOwn(child, 'own'):", Object.hasOwn(child, "own"));

// --- Freeze / seal / preventExtensions ---
const frozen = Object.freeze({ a: 1, nested: { b: 2 } });
console.log("Object.isFrozen(frozen):", Object.isFrozen(frozen));
console.log("Object.isFrozen(frozen.nested) (shallow):", Object.isFrozen(frozen.nested));

const sealed = Object.seal({ a: 1 });
sealed.a = 99; // OK, still writable
console.log("sealed.a after reassign:", sealed.a);
try {
  delete sealed.a;
} catch (err) {
  console.log("delete sealed.a threw:", err.constructor.name);
}

const prevented = Object.preventExtensions({ a: 1 });
try {
  prevented.b = 2;
} catch (err) {
  console.log("prevented.b = 2 threw:", err.constructor.name);
}

// --- Symbol-keyed properties ---
const sym = Symbol("meta");
const withSymbol = { a: 1, [sym]: "hidden metadata" };
console.log("Object.keys(withSymbol) (symbol invisible):", Object.keys(withSymbol));
console.log("Object.getOwnPropertySymbols(withSymbol):", Object.getOwnPropertySymbols(withSymbol));
console.log("Reflect.ownKeys(withSymbol):", Reflect.ownKeys(withSymbol));
console.log("JSON.stringify(withSymbol) (symbol omitted):", JSON.stringify(withSymbol));
