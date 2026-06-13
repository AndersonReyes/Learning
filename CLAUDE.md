# Learning Repo — Project Memory

This repo holds self-study tracks. Currently: `javascript/` (fundamentals done,
intermediate/advanced roadmapped). Future language tracks should follow the
same top-level pattern: a dedicated `<language>/` directory with its own
README, ROADMAP, package manifest, and numbered topic folders.

## JavaScript track (`javascript/`)

Full curriculum lives in `javascript/ROADMAP.md` (fundamentals built out,
intermediate/advanced planned), each topic mapped to an MDN Guide chapter
(https://github.com/mdn/content/tree/main/files/en-us/web/javascript/guide).

### Per-topic structure

Each topic is a numbered folder, e.g. `fundamentals/06-functions/`, with
exactly 4 files:

- **`notes.md`** — concept explanation. Terse and direct: syntax, rules,
  gotchas, short code snippets. No "why this matters" filler prose — but the
  notes must be **self-contained**: include the gotchas and good practices
  needed to use the concept well (e.g. reference vs. value, closures-in-loops,
  guard clauses, pure functions), not just bare syntax. Ends with a
  "Further Reading (MDN)" section linking precisely to the page(s) that cover
  what THIS topic discusses — prefer specific reference pages over broad guide
  chapters that include material the topic doesn't cover.
- **`examples.js`** — runnable via `node examples.js`. Demonstrates every
  concept from `notes.md` with `console.log`. No exercises here.
- **`exercise.js`** — 5 exported function stubs with JSDoc (params, return
  type, behavior, example I/O). Every stub body is
  `throw new Error("Not implemented")`.
- **`exercise.test.js`** — `node:test` + `node:assert/strict` suite. This IS
  the spec/answer key — there are no separate solution files anywhere.
  Cover edge cases thoroughly; the learner implements `exercise.js` until the
  tests pass.

### Exercise difficulty

Exercises should be **hard, challenging algorithmic problems** — not basic
syntax drills — even if every exercise in a topic ends up hard. Before writing
the test file, hand-verify all expected outputs since there's no reference
implementation to check against.

### Writing style

Short and direct. No filler words. Prefer bullet points and code over prose.
"Terse" means cutting prose, not substance — don't drop important gotchas or
good practices for the sake of brevity.

### Adding a new topic

1. Pick the next topic from `javascript/ROADMAP.md` (fundamentals →
   intermediate → advanced order), noting its mapped MDN chapter.
2. Write, in this order: `notes.md` → `exercise.js` → `exercise.test.js` →
   `examples.js`. Only add new-concept demos to `examples.js` if the
   exercises introduce an API not already covered by an earlier topic's
   `examples.js`.
3. Verify:
   - `node <topic>/examples.js` runs with no errors.
   - `node --test <topic>/exercise.test.js` — ALL tests FAIL (stubs throw
     "Not implemented"). This is the expected starting state for a learner.
   - `npm test` from `javascript/` picks up the new topic's tests.
4. Update `javascript/ROADMAP.md` to mark the topic as built out.

### package.json

`javascript/package.json`'s test script is `node --test` (no path argument —
`node --test fundamentals` does NOT recurse in Node 22; omitting the path
gives recursive `**/*.test.js` discovery from `javascript/`).
