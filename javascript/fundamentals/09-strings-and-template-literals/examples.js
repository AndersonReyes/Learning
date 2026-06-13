// Run with: node examples.js

// --- Strings are immutable ---
const greeting = "hello";
console.log("toUpperCase:", greeting.toUpperCase());
console.log("original unchanged:", greeting);

// --- Common methods ---
const text = "  Hello, World!  ";
console.log("trim:", JSON.stringify(text.trim()));
console.log("includes 'World':", text.includes("World"));
console.log("indexOf 'World':", text.indexOf("World"));
console.log("slice(2, 7):", text.slice(2, 7));
console.log("slice(-1):", JSON.stringify(text.slice(-1)));
console.log("split:", "a,b,c".split(","));
console.log("join:", ["a", "b", "c"].join("-"));
console.log("replace:", text.replace("World", "JS"));
console.log("replaceAll:", text.replaceAll("l", "L"));
console.log("padStart:", "5".padStart(3, "0"));
console.log("padEnd:", "5".padEnd(3, "0"));

// --- Template literals ---
const name = "Ada";
const age = 30;
console.log(`${name} is ${age} years old.`);
console.log(`Next year, ${name} will be ${age + 1}.`);

const message = `Line one
Line two`;
console.log("multi-line:", message);

const items = ["a", "b"];
console.log(`Items: ${items.map((item) => `[${item}]`).join(", ")}`);

// --- Char codes ---
console.log("'a'.charCodeAt(0):", "a".charCodeAt(0));
console.log("String.fromCharCode(97):", String.fromCharCode(97));

// shift each letter by 1 (no wraparound here)
const shifted = "abc"
  .split("")
  .map((ch) => String.fromCharCode(ch.charCodeAt(0) + 1))
  .join("");
console.log("shift 'abc' by 1:", shifted);

// --- Regular expressions ---
console.log("non-alphanumeric test:", /[^a-z0-9]/gi.test("a-b"));
console.log("replace all dashes:", "a-b-c".replace(/-/g, "_"));
console.log("split on whitespace:", "hello   world".split(/\s+/));
console.log(
  "replace with function:",
  "{{name}}".replace(/\{\{(\w+)\}\}/, (match, key) => key.toUpperCase()),
);
