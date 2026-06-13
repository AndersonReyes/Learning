// Run with: node examples.js

const delay = (ms, value) =>
  new Promise((resolve) => setTimeout(() => resolve(value), ms));

// --- 1. Sync code runs first, then ALL microtasks, then the next macrotask ---
console.log("=== 1. Sync vs microtask vs macrotask ===");
console.log("a: sync (start)");
setTimeout(() => console.log("d: macrotask (setTimeout 0)"), 0);
Promise.resolve().then(() => console.log("b: microtask (promise.then)"));
queueMicrotask(() => console.log("c: microtask (queueMicrotask)"));
console.log("a2: sync (end)");
// expected: a, a2, b, c, d
await delay(10);

// --- 2. process.nextTick runs before Promise microtasks (Node-specific,
//     within an ordinary callback/macrotask) ---
console.log("\n=== 2. process.nextTick vs Promise microtask ===");
await new Promise((resolve) => {
  setTimeout(() => {
    Promise.resolve().then(() => console.log("b: promise microtask"));
    process.nextTick(() => console.log("a: process.nextTick"));
    setTimeout(resolve, 0);
  }, 0);
});
// expected: a, b

// --- 3. async/await is sugar over microtasks: code after `await` resumes
//     as a microtask, so sync code after the call still runs first ---
console.log("\n=== 3. async/await is microtask sugar ===");
async function asyncDemo() {
  console.log("a: async fn before await");
  await null;
  console.log("c: async fn after await (resumed as a microtask)");
}
asyncDemo();
console.log("b: sync code after calling asyncDemo()");
// expected: a, b, c
await delay(10);

// --- 4. A chain of microtasks fully drains before the next macrotask, even
//     if each microtask schedules another one ---
console.log("\n=== 4. A microtask chain drains before the next macrotask ===");
function microtaskChain(n) {
  if (n === 0) return;
  Promise.resolve().then(() => {
    console.log(`chain step ${n}`);
    microtaskChain(n - 1);
  });
}
setTimeout(() => console.log("macrotask after the microtask chain"), 0);
microtaskChain(3);
// expected: chain step 3, 2, 1, then the macrotask
await delay(10);

// --- 5. Sequential vs concurrent async operations ---
console.log("\n=== 5. Sequential vs concurrent awaiting ===");
{
  const start = Date.now();
  const a = await delay(20, "a");
  const b = await delay(20, "b");
  console.log(
    `sequential: got ${a}, ${b} after ~${Date.now() - start}ms (expect ~40ms — each await waits for the previous to finish)`,
  );
}
{
  const start = Date.now();
  const [a, b] = await Promise.all([delay(20, "a"), delay(20, "b")]);
  console.log(
    `concurrent: got ${a}, ${b} after ~${Date.now() - start}ms (expect ~20ms — both run at the same time)`,
  );
}
