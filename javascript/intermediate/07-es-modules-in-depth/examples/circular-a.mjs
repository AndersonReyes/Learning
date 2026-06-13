// Circular dependency gotcha (A <-> B). See circular-b.mjs for the full
// evaluation-order walkthrough.
//
// `b` is imported from circular-b.mjs. Because circular-b.mjs finishes
// evaluating fully (including `export const b = "B"`) BEFORE control
// returns here, `b` IS safely initialized by the time this module's
// remaining top-level code runs.
import { b } from "./circular-b.mjs";

export const a = "A";

// Safe here: circular-b.mjs already finished, so `b` is initialized.
export const topLevelBValue = b;

// Also safe: deferred access inside a function.
export function useB() {
  return b;
}
