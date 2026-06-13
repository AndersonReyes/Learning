// Run with: node examples.js

// --- Declarations ---
const fixed = "I cannot be reassigned";
let mutable = "I can be reassigned";
mutable = "see?";
console.log(fixed, "|", mutable);

// const prevents reassigning the *binding*, not mutation of the value
const point = { x: 1, y: 2 };
point.x = 99;
console.log("mutated const object:", point);

// --- typeof and its quirks ---
console.log("typeof 'hi':", typeof "hi");
console.log("typeof 42:", typeof 42);
console.log("typeof true:", typeof true);
console.log("typeof undefined:", typeof undefined);
console.log("typeof Symbol():", typeof Symbol());
console.log("typeof 10n:", typeof 10n);
console.log("typeof {}:", typeof {});
console.log("typeof []:", typeof []); // "object" — surprising!
console.log("typeof null:", typeof null); // "object" — historical bug
console.log("typeof function(){}:", typeof function () {});

console.log("Array.isArray([]):", Array.isArray([]));
console.log("Array.isArray({}):", Array.isArray({}));

// --- Precise type tags ---
console.log("toString.call([]):", Object.prototype.toString.call([]));
console.log("toString.call(null):", Object.prototype.toString.call(null));
console.log("toString.call(new Date()):", Object.prototype.toString.call(new Date()));
console.log("toString.call(/x/):", Object.prototype.toString.call(/x/));

// --- NaN and Object.is ---
console.log("NaN === NaN:", NaN === NaN);
console.log("Number.isNaN(NaN):", Number.isNaN(NaN));
console.log("Object.is(NaN, NaN):", Object.is(NaN, NaN));
console.log("Object.is(0, -0):", Object.is(0, -0));

// --- Explicit conversion ---
console.log("Number('42'):", Number("42"));
console.log("Number('42px'):", Number("42px"));
console.log("Number(''):", Number(""));
console.log("Number(true):", Number(true));
console.log("Number(null):", Number(null));
console.log("Number(undefined):", Number(undefined));
console.log("Number([]):", Number([]));
console.log("Number([1]):", Number([1]));
console.log("Number([1, 2]):", Number([1, 2]));

console.log("String(42):", String(42));
console.log("String(null):", String(null));
console.log("String(undefined):", String(undefined));

console.log("Boolean(0):", Boolean(0));
console.log("Boolean('hello'):", Boolean("hello"));

// --- Implicit conversion (coercion) ---
console.log("'5' + 1:", "5" + 1);
console.log("'5' - 1:", "5" - 1);
console.log("'5' * '2':", "5" * "2");
console.log("true + 1:", true + 1);

// --- Truthy / falsy ---
const falsyValues = [false, 0, -0, 0n, "", null, undefined, NaN];
for (const value of falsyValues) {
  console.log(`Boolean(${String(value)}):`, Boolean(value));
}

console.log("Boolean([]) (empty array is truthy):", Boolean([]));
console.log("Boolean('0') (non-empty string is truthy):", Boolean("0"));

// --- Template literals (preview) ---
const name = "Ada";
const age = 30;
console.log(`${name} is ${age} years old.`);
