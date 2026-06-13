// Run with: node examples.js

const numbers = [1, 2, 3, 4, 5, 6];

// --- map ---
const doubled = numbers.map((n) => n * 2);
console.log("doubled:", doubled);
console.log("original numbers unchanged:", numbers);

// --- filter ---
const evens = numbers.filter((n) => n % 2 === 0);
console.log("evens:", evens);

// --- reduce ---
const sum = numbers.reduce((acc, n) => acc + n, 0);
console.log("sum:", sum);

const product = numbers.reduce((acc, n) => acc * n, 1);
console.log("product:", product);

// --- find ---
const users = [
  { id: 1, name: "Ada" },
  { id: 2, name: "Grace" },
];
console.log("find id 2:", users.find((u) => u.id === 2));
console.log("find id 99:", users.find((u) => u.id === 99));

// --- includes / some / every ---
console.log("includes 2:", numbers.includes(2));
console.log("some > 5:", numbers.some((n) => n > 5));
console.log("every > 0:", numbers.every((n) => n > 0));

// --- flat ---
console.log("flat:", [1, [2, 3], [4]].flat());

// --- slice (no mutation) ---
console.log("slice(1, 3):", numbers.slice(1, 3));
console.log("slice(-2):", numbers.slice(-2));
console.log("original numbers unchanged:", numbers);

// --- sort (mutates, needs a comparator for numbers) ---
const messy = [10, 2, 1, 20];
console.log("default sort (lexicographic):", [...messy].sort());
console.log("numeric ascending:", [...messy].sort((a, b) => a - b));
console.log("numeric descending:", [...messy].sort((a, b) => b - a));

// --- mutating methods (push/pop/shift/unshift) ---
const stack = [1, 2, 3];
stack.push(4);
console.log("after push:", stack);
stack.pop();
console.log("after pop:", stack);
stack.unshift(0);
console.log("after unshift:", stack);
stack.shift();
console.log("after shift:", stack);

// --- deduplicating with Set ---
const withDuplicates = [1, 2, 2, 3, 1];
const unique = [...new Set(withDuplicates)];
console.log("unique:", unique);
