// Other half of the A <-> B circular dependency.
//
// Evaluation order when examples.js does `import ... from "./circular-a.mjs"`:
//  1. Node starts evaluating circular-a.mjs.
//  2. circular-a.mjs's `import { b } from "./circular-b.mjs"` runs FIRST,
//     so Node starts evaluating THIS module (circular-b.mjs) before
//     circular-a.mjs has produced anything.
//  3. THIS module's `import { a } from "./circular-a.mjs"` resolves to
//     circular-a.mjs's (still in-progress) module record. The `a` binding
//     exists but is uninitialized (Temporal Dead Zone) at this point.
//  4. Reading `a` at TOP LEVEL here throws a ReferenceError (TDZ) — that's
//     the circular dependency gotcha. We catch it below and export the
//     result so examples.js can show it.
//  5. `export const b = "B"` runs, `b` becomes initialized.
//  6. circular-b.mjs finishes; control returns to circular-a.mjs, which can
//     now safely see `b`.
import { a } from "./circular-a.mjs";

export const b = "B";

// BAD: at THIS point in evaluation, `a` is in its Temporal Dead Zone —
// circular-a.mjs hasn't reached `export const a = "A"` yet. Reading it here
// throws ReferenceError: Cannot access 'a' before initialization.
let topLevelAAccessThrew;
try {
  void a;
  topLevelAAccessThrew = false;
} catch {
  topLevelAAccessThrew = true;
}
export { topLevelAAccessThrew };

// GOOD: deferred access inside a function, called after all modules
// finished evaluating — `a` is safely initialized by then.
export function useA() {
  return a;
}
