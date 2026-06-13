// Run with: node examples.js

// --- Function declaration ---
function add(a, b) {
  return a + b;
}
console.log("add(2, 3):", add(2, 3));

// --- Function expression vs arrow function ---
const addExpr = function (a, b) {
  return a + b;
};
const addArrow = (a, b) => a + b;
console.log("addExpr(2, 3):", addExpr(2, 3));
console.log("addArrow(2, 3):", addArrow(2, 3));

// --- Returning a function from a function (closures again) ---
function makeAdder(x) {
  return (y) => x + y;
}
const add5 = makeAdder(5);
const add10 = makeAdder(10);
console.log("add5(3):", add5(3));
console.log("add10(3):", add10(3));

// --- Recursion ---
function factorial(n) {
  if (n <= 1) return 1; // base case
  return n * factorial(n - 1);
}
console.log("factorial(5):", factorial(5));

// --- fn.length: number of declared (non-default, non-rest) params ---
function example(a, b, c = 1, ...rest) {}
console.log("example.length:", example.length); // 2 (c and rest don't count)

// --- Default parameters ---
function greet(name, greeting = "Hello") {
  return `${greeting}, ${name}!`;
}
console.log("greet('Ada'):", greet("Ada"));
console.log("greet('Ada', 'Hi'):", greet("Ada", "Hi"));
console.log("greet('Ada', undefined):", greet("Ada", undefined));

// --- Rest parameters ---
function multiplyAll(...numbers) {
  console.log("  numbers is an array:", Array.isArray(numbers), numbers);
  return numbers.reduce((product, n) => product * n, 1);
}
console.log("multiplyAll(2, 3, 4):", multiplyAll(2, 3, 4));
console.log("multiplyAll():", multiplyAll());

// --- Implicit undefined return ---
function logMessage(msg) {
  console.log("logMessage says:", msg);
}
const result = logMessage("hi");
console.log("result of logMessage:", result);
