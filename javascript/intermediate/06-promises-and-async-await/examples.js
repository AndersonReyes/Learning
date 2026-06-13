// Run with: node examples.js

const delay = (ms, value) =>
  new Promise((resolve) => setTimeout(() => resolve(value), ms));

// --- The 3 states ---
const pending = new Promise(() => {}); // never settles
console.log("pending promise:", pending); // Promise { <pending> }

const fulfilled = Promise.resolve("done");
console.log("fulfilled promise:", fulfilled); // Promise { 'done' }

const rejected = Promise.reject(new Error("nope"));
console.log(
  "rejected promise ->",
  await rejected.catch((e) => `rejected: ${e.message}`),
);

// Once settled, state/value are final — later resolve()/reject() are no-ops.
const settled = new Promise((resolve, reject) => {
  resolve("done");
  reject(new Error("ignored")); // no-op — already settled
});
console.log("settled.then ->", await settled); // "done"

// --- async/await is sugar over .then() ---
async function f() {
  return 42;
}
console.log("f() returns a Promise:", f()); // Promise { 42 } — NOT 42 directly
console.log("await f():", await f()); // 42

async function g() {
  throw new Error("boom");
}
console.log("g().catch:", await g().catch((e) => e.message)); // "boom"

// Equivalent .then() chain vs async/await:
function getUserThen(id) {
  return Promise.resolve({ id, name: "Ada" }).then((user) =>
    Promise.resolve(`${user.name}'s posts`),
  );
}
async function getUserAwait(id) {
  const user = await Promise.resolve({ id, name: "Ada" });
  return Promise.resolve(`${user.name}'s posts`);
}
console.log("getUserThen:", await getUserThen(1));
console.log("getUserAwait:", await getUserAwait(1));

// --- Promise.all / allSettled / race / any ---
const ok = () => Promise.resolve("ok");
const fail = () => Promise.reject(new Error("fail"));

console.log(
  "Promise.all (fails fast):",
  await Promise.all([ok(), Promise.resolve("ok2")]),
); // ["ok", "ok2"]
console.log(
  "Promise.all rejects with first reason:",
  await Promise.all([ok(), fail()]).catch((e) => e.message),
); // "fail"

console.log(
  "Promise.allSettled (never rejects):",
  (await Promise.allSettled([ok(), fail()])).map((r) =>
    r.status === "fulfilled" ? r : { status: r.status, reason: r.reason.message },
  ),
);
// [{status:"fulfilled", value:"ok"}, {status:"rejected", reason:"fail"}]

console.log(
  "Promise.race (first to SETTLE, win or lose):",
  await Promise.race([delay(10, "slow"), delay(5, "fast")]),
); // "fast"

console.log(
  "Promise.any (first to FULFILL, ignores rejections):",
  await Promise.any([fail(), delay(5, "ok")]),
); // "ok" — rejection ignored

console.log("Promise.all([]) ->", await Promise.all([])); // []
try {
  await Promise.any([]);
} catch (e) {
  console.log("Promise.any([]) rejects with:", e.constructor.name); // AggregateError
}

// --- Sequential vs concurrent execution ---
console.log("--- sequential (for...of + await) ---");
let start = Date.now();
for (const ms of [10, 10, 10]) {
  await delay(ms);
}
console.log(`sequential total: ~${Date.now() - start}ms`); // ~30ms — sum of delays

console.log("--- concurrent (Promise.all + map) ---");
start = Date.now();
await Promise.all([10, 10, 10].map((ms) => delay(ms)));
console.log(`concurrent total: ~${Date.now() - start}ms`); // ~10ms — max of delays

// --- Error handling: try/catch around await ---
async function load() {
  try {
    return await Promise.reject(new Error("network error"));
  } catch (err) {
    console.error("load failed:", err.message);
    return null;
  }
}
console.log("load():", await load()); // null

// --- Unhandled promise rejections ---
// A promise chain that's never awaited and has no .catch() is an UNHANDLED
// rejection — Node prints a warning. We immediately .catch() here so this
// file itself runs cleanly; the comment shows the BAD version.
function fetchData() {
  return Promise.reject(new Error("fetch failed"));
}
// BAD (commented out — would trigger an unhandledRejection):
// fetchData().then((d) => console.log(d));

// GOOD: always terminate a chain with .catch(), or await inside try/catch.
fetchData()
  .then((d) => console.log(d))
  .catch((err) => console.log("caught chain rejection:", err.message));

// --- AbortController for cancellation ---
function abortableDelay(ms, signal) {
  return new Promise((resolve, reject) => {
    if (signal.aborted) return reject(new Error("aborted"));
    const id = setTimeout(resolve, ms);
    signal.addEventListener("abort", () => {
      clearTimeout(id);
      reject(new Error("aborted"));
    });
  });
}

const controller = new AbortController();
const abortablePromise = abortableDelay(50, controller.signal);
controller.abort(); // abort immediately, before the 50ms elapses
try {
  await abortablePromise;
} catch (err) {
  console.log("abortableDelay:", err.message); // "aborted"
}

// Wait for the dangling .catch() above to log before exiting.
await delay(0);
