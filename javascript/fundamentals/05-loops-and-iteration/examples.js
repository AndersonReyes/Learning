// Run with: node examples.js

// --- for ---
for (let i = 0; i < 5; i++) {
  console.log("for i:", i);
}

// --- while ---
let n = 3;
while (n > 0) {
  console.log("while n:", n);
  n--;
}

// --- do...while (always runs at least once) ---
let attempts = 0;
do {
  attempts++;
} while (attempts < 3);
console.log("attempts:", attempts);

// --- for...of over a string ---
for (const char of "abc") {
  console.log("char:", char);
}

// --- for...of over an array ---
for (const item of [10, 20, 30]) {
  console.log("item:", item);
}

// --- for...in over an object's keys ---
const point = { x: 1, y: 2 };
for (const key in point) {
  console.log("key/value:", key, point[key]);
}

// --- break and continue ---
console.log("break/continue demo:");
for (const value of [1, 2, 3, 4, 5]) {
  if (value === 3) continue; // skip 3
  if (value === 5) break; // stop before 5
  console.log("  value:", value);
}

// --- nested loops: flatten one level ---
const groups = [[1, 2], [3, 4], [5]];
const flat = [];
for (const group of groups) {
  for (const item of group) {
    flat.push(item);
  }
}
console.log("flat:", flat);
