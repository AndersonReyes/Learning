---
description: Scaffold a new JavaScript learning topic (notes, examples, exercise, tests)
---

Create a new topic for the JavaScript learning track: $ARGUMENTS

Follow `CLAUDE.md` exactly:

1. Determine the tier (`fundamentals`, `intermediate`, or `advanced`) and the
   `NN-topic-name` slug from `javascript/ROADMAP.md` — use the next unbuilt
   topic in roadmap order unless the user specified one. Note its mapped MDN
   Guide chapter.
2. Create `javascript/<tier>/<NN>-<topic-name>/` with four files, in this order:
   - `notes.md` — terse, direct, code-first explanation ending with a
     "Further Reading (MDN)" section linking the topic's MDN chapter and
     relevant reference pages.
   - `exercise.js` — 5 hard, exported function stubs with JSDoc (params,
     return type, behavior, example I/O), each body
     `throw new Error("Not implemented")`.
   - `exercise.test.js` — `node:test` + `node:assert/strict` spec covering
     edge cases. Hand-verify every expected value before writing it — this
     file is the only answer key.
   - `examples.js` — runnable demo of the `notes.md` concepts; only add
     demos for concepts not already covered by an earlier topic's
     `examples.js`.
3. Verify from `javascript/`:
   - `node <tier>/<NN-topic-name>/examples.js` runs with no errors.
   - `node --test <tier>/<NN-topic-name>/exercise.test.js` — ALL tests FAIL
     ("Not implemented"). This is the expected starting state.
   - `npm test` discovers the new topic's suite.
4. Update `javascript/ROADMAP.md` to mark the topic as built out.
