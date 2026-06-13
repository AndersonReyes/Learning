// Run with: node examples.js
//
// This file demonstrates real ES module mechanics by importing small helper
// modules from ./examples/. (The exercise.js functions simulate
// module-pattern CONCEPTS as plain functions — see notes.md for why.)

// --- Named exports + default export from the same module ---
import describeSquare, { PI, square, E } from "./examples/math.mjs";
console.log("named export PI:", PI);
console.log("named export square(4):", square(4));
console.log("named export E:", E);
console.log("default export describeSquare(4):", describeSquare(4));

// --- Default-only module, imported under any local name ---
import log from "./examples/logger.mjs";
import myLogger from "./examples/logger.mjs"; // same binding, different local name
console.log("log('hi'):", log("hi"));
console.log("myLogger === log:", myLogger === log); // true — same function

// --- Re-exports / barrel module ---
// barrel.mjs does:
//   export * from "./math.mjs"          (PI, square, E — NOT math's default)
//   export { square as sq } from "./math.mjs"
//   export { default as log } from "./logger.mjs"
import * as barrel from "./examples/barrel.mjs";
console.log("barrel.PI:", barrel.PI); // re-exported via `export *`
console.log("barrel.sq(5):", barrel.sq(5)); // re-exported + renamed
console.log("barrel.log('via barrel'):", barrel.log("via barrel")); // default re-exported as named
console.log("barrel.default:", barrel.default); // undefined — `export *` never re-exports a default

// --- Namespace imports: `ns` is a live, read-only binding object ---
import * as math from "./examples/math.mjs";
console.log("math namespace keys:", Object.keys(math).sort());
console.log("math.square(3):", math.square(3));
try {
  math.PI = 4; // TypeError: namespace objects are read-only
} catch (err) {
  console.log("math.PI = 4 threw:", err.constructor.name);
}

// --- Modules are singletons: shared mutable state across importers ---
// Both of these come from the SAME module instance of store.mjs — Node only
// evaluates a module's top-level code once, no matter how many places
// import it. `count` is a LIVE binding: re-reading it after `increment()`
// reflects store.mjs's internal mutation, even though `count` looks like a
// plain local identifier.
import { count, increment } from "./examples/store.mjs";
import * as storeAgain from "./examples/store.mjs";

console.log("count before increment:", count); // 0
increment(); // mutates store.mjs's internal state
console.log("count after increment (live binding):", count); // 1 — sees the mutation
console.log("storeAgain.count (same instance):", storeAgain.count); // 1 — same module instance

// --- Dynamic import(): returns a Promise<namespace object> ---
async function loadFeatureConditionally(enabled) {
  if (!enabled) {
    console.log("feature disabled — module not loaded");
    return;
  }
  const mod = await import("./examples/feature.mjs");
  console.log("dynamic import mod.run():", mod.run());
  console.log("dynamic import mod.default():", mod.default());
}
await loadFeatureConditionally(false);
await loadFeatureConditionally(true);

// --- Circular dependency gotcha ---
// circular-a.mjs imports `b` from circular-b.mjs, which imports `a` from
// circular-a.mjs. Evaluation order (A is imported first, so A starts first
// but B finishes first — see circular-b.mjs for the full walkthrough):
//  - B reads `a` at its own top level WHILE A is still mid-evaluation ->
//    `a` is in its Temporal Dead Zone -> ReferenceError (caught, exported
//    as a boolean below).
//  - By the time A's remaining top-level code runs, B has fully finished,
//    so A can safely read `b` at top level.
// Either way, DEFERRED access (inside a function, called after everything
// has loaded) always works — that's the fix.
import { useB, topLevelBValue } from "./examples/circular-a.mjs";
import { useA, topLevelAAccessThrew } from "./examples/circular-b.mjs";

console.log("B's top-level read of `a` threw (TDZ):", topLevelAAccessThrew);
console.log("A's top-level read of `b` (safe, B finished first):", topLevelBValue);
console.log("useB() [deferred access, after all modules loaded]:", useB());
console.log("useA() [deferred access, after all modules loaded]:", useA());

// --- File extensions are required in Node ESM ---
// import { square } from "./examples/math";    // Error: Cannot find module
// import { square } from "./examples/math.mjs"; // OK (already done above)
console.log("file extensions are required — see notes.md");
