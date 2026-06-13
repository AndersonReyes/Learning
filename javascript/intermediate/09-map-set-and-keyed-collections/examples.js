// Run with: node examples.js

// --- Map vs plain object ---
const m = new Map();
m.set("a", 1);
m.set(42, "num key");
const objKey = { id: 1 };
m.set(objKey, "object key");

console.log("m.get('a'):", m.get("a"));
console.log("m.size:", m.size);
console.log("m.has(42):", m.has(42));
m.delete("a");
console.log("after delete, m.has('a'):", m.has("a"));

// Map is iterable directly
for (const [key, value] of m) {
  console.log("entry:", key, "->", value);
}
console.log("spread [...m]:", [...m]);

// --- Set: unique values, O(1) lookup ---
const s = new Set([1, 2, 2, 3, 1]);
console.log("Set dedups on construction:", s, "size:", s.size);
s.add(4);
console.log("s.has(2):", s.has(2));
s.delete(1);
console.log("after delete(1):", [...s]);

const allowed = new Set(["a", "b", "c"]);
console.log("allowed.has('b') (O(1)):", allowed.has("b"));

// --- Iterating: keys(), values(), entries() ---
const m2 = new Map([
  ["a", 1],
  ["b", 2],
]);
console.log("m2.keys():", [...m2.keys()]);
console.log("m2.values():", [...m2.values()]);
console.log("m2.entries():", [...m2.entries()]);

const s2 = new Set(["x", "y"]);
console.log("s2.keys() (alias for values):", [...s2.keys()]);
console.log("s2.entries() ([value, value] pairs):", [...s2.entries()]);

m2.forEach((value, key) => console.log("m2.forEach:", key, "=", value));
s2.forEach((value, key) => console.log("s2.forEach value === key:", value === key));

// --- WeakMap / WeakSet ---
const wm = new WeakMap();
const account = {};
wm.set(account, "metadata");
console.log("wm.get(account):", wm.get(account));

try {
  wm.set("string-key", "x");
} catch (err) {
  console.log("wm.set('string', ...) threw:", err.constructor.name);
}

const visited = new WeakSet();
const node = {};
visited.add(node);
console.log("visited.has(node):", visited.has(node));

// --- Gotcha: SameValueZero equality ---
const sameValueSet = new Set();
sameValueSet.add(NaN);
sameValueSet.add(NaN);
console.log("Set with two NaNs, size:", sameValueSet.size);

sameValueSet.add(0);
sameValueSet.add(-0);
console.log("after adding 0 and -0, size:", sameValueSet.size);

// --- Gotcha: object keys are compared by reference ---
const refMap = new Map();
refMap.set({ id: 1 }, "first");
refMap.set({ id: 1 }, "second");
console.log("refMap.size (two distinct object keys):", refMap.size);

const sharedKey = { id: 1 };
refMap.set(sharedKey, "third");
console.log("refMap.get(sharedKey):", refMap.get(sharedKey));
console.log("refMap.get({ id: 1 }) (different ref):", refMap.get({ id: 1 }));
