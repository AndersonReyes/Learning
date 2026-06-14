# Learning Repo — Project Memory

This repo holds self-study tracks. Currently: `javascript/` (fundamentals
done; intermediate/advanced in progress — see `javascript/ROADMAP.md` status
column), `go/` (fundamentals/intermediate/advanced fully built — taught
through computer networking, see `go/ROADMAP.md`), `html/` and `css/` (notes
+ viewable examples, no exercises), `cpp/` (planning stage — see
`cpp/ROADMAP.md`). Future language tracks should follow the same top-level
pattern: a dedicated `<language>/` directory with its own README, ROADMAP,
package manifest (if runnable), and numbered topic folders.

## JavaScript track (`javascript/`)

Full curriculum lives in `javascript/ROADMAP.md`. Each row has a `Status`
column (`planned`/`done`) — **this is the checkpoint tracker**. A fresh
session should `grep -n planned javascript/ROADMAP.md` to find the next topic
to build, in table order (fundamentals → intermediate → advanced). Each topic
maps to an MDN Guide chapter
(https://github.com/mdn/content/tree/main/files/en-us/web/javascript/guide).

Folders: `javascript/fundamentals/NN-*`, `javascript/intermediate/NN-*`,
`javascript/advanced/NN-*` — same 4-file pattern for all tiers.

### Adapted topics

A handful of intermediate/advanced topics narrow `exercise.js`'s scope to a
testable subset because their textbook subject matter doesn't map onto
pure-Node `node:test` functions (e.g. ES Modules in depth, Memory Management,
Resource Management, Browser APIs, Testing & Tooling, TypeScript Basics). See
the "Adapted topics" section in `javascript/ROADMAP.md` for each one's scope
— `notes.md` still covers the full topic conceptually; only `exercise.js`
narrows.

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

## Go track (`go/`)

Full curriculum lives in `go/ROADMAP.md` (fundamentals, intermediate, and
advanced all fully built). Unlike the JS track, **every topic pairs a Go
language concept with a networking concept** — Go is the implementation
language for a computer-networking curriculum, basics through advanced. With
Advanced complete, the roadmap's "Capstones" section lists follow-on
projects (a network monitoring tool, a BitTorrent client, and a future
`rust/` track building a TCP/IP stack from scratch, CS144/`smoltcp`-style).

### Per-topic structure

Each topic is a numbered folder, e.g.
`fundamentals/01-go-basics-and-ip-addressing/`, with:

- **`notes.md`** — concept explanation covering BOTH the Go concept(s) and
  the networking concept(s) for the topic. Same terse style as the JS track:
  syntax, rules, gotchas, short code snippets, self-contained. Ends with a
  "Further Reading" section linking go.dev (Tour of Go / Effective Go /
  pkg.go.dev) and the relevant RFC(s).
- **`examples/main.go`** — `package main`, runnable via
  `go run ./fundamentals/NN-topic/examples`. Demonstrates the concepts from
  `notes.md` with `fmt.Println`/`fmt.Printf`, using illustrative examples
  that are deliberately NOT the exercise problems (keeps exercises unspoiled).
- **`exercise.go`** — `package <topicname>` (short, lowercase, e.g. `ipaddr`).
  5 exported function stubs with Go doc comments (signature, behavior,
  example I/O). Stub bodies return zero values / `errors.New("not
  implemented")` — NOT `panic()` (a panic during a test re-panics after
  `testing` recovers it, crashing the whole package's test binary and
  preventing other tests from reporting).
- **`exercise_test.go`** — `package <topicname>` (internal test package),
  table-driven tests using `testing`. This IS the spec/answer key — no
  separate solution files.

### Exercise difficulty

Same bar as JS: **hard, challenging algorithmic problems**, hand-verified
before writing the test file — verify by temporarily implementing a reference
solution in `exercise.go`, running `go test`, confirming all tests pass, then
reverting to stubs.

### Adding a new topic

1. Pick the next topic from `go/ROADMAP.md` (fundamentals → intermediate →
   advanced order), noting its Go + networking references.
2. Write, in this order: `notes.md` → `exercise.go` → `exercise_test.go` →
   `examples/main.go`.
3. Verify:
   - `go vet ./...` from `go/` passes.
   - `go run ./fundamentals/NN-topic/examples` runs with no errors.
   - `go test ./fundamentals/NN-topic/...` — ALL tests FAIL (stubs return
     "not implemented"). This is the expected starting state for a learner.
   - `go test ./...` from `go/` picks up the new topic's tests.
4. Update `go/ROADMAP.md` to mark the topic as built (folder link).

### go.mod

Module path `github.com/andersonreyes/learning/go`, `go 1.24`. Test command
`go test ./...` from `go/` (recursive by default, unlike `node --test`).

## HTML track (`html/`)

Single-topic track (HTML is small enough to cover in one folder):
`html/01-html-fundamentals/notes.md` + `examples/*.html`. No exercises.
`html/README.md` explains these are opened directly in a browser (no test
runner). Add more `NN-*` topic folders only if the single folder gets
unwieldy.

## CSS track (`css/`)

3 topic folders: `01-css-fundamentals`, `02-flexbox-and-grid`,
`03-responsive-and-modern-css` — each `notes.md` + `examples/*.html` with
self-contained `<style>` blocks (one example pairs `.html`+`.css` to
demonstrate `<link>`). No exercises. `css/README.md` explains the
browser-viewing workflow; `css/ROADMAP.md` tracks the 3 topics plus a TODO
for future full design projects (deferred — not part of this build-out).

## C++ track (`cpp/`)

**Status: planning.** Full curriculum lives in `cpp/ROADMAP.md` — 23 topics
across Fundamentals/Intermediate/Advanced, mapped to the 29 chapters of
Federico Busato's [Modern C++ Programming](https://github.com/federico-busato/Modern-CPP-Programming)
course (the track's primary reference, cited in each topic's "Further
Reading"). No topics are built yet; first one is
`fundamentals/01-setup-and-hello-world`.

### Per-topic structure

Same shape as JS/Go, adapted for C++ with zero external dependencies. Each
topic is a numbered folder, e.g. `fundamentals/02-types-and-operators/`,
with:

- **`notes.md`** — concept explanation, terse like JS/Go: syntax, rules,
  gotchas, short snippets. Ends with "Further Reading" linking the matching
  Modern C++ Programming chapter(s) and cppreference.com pages.
- **`examples.cpp`** — single translation unit, runnable via
  `g++ -std=c++20 -Wall -Wextra -o /tmp/ex examples.cpp && /tmp/ex`.
  Demonstrates `notes.md` concepts via `std::cout`/`printf`. No exercises.
- **`exercise.h`** + `exercise.cpp` — 5 function/class stubs with doc
  comments (signature, behavior, example I/O).
- **`exercise_test.cpp`** — this IS the spec/answer key — no separate
  solution files. See "Testing strategy" below for how it's written.

Plain compiler invocations until `intermediate/06` (Debugging, Testing &
CMake), which introduces CMake; topics after that may add a
`CMakeLists.txt` where a multi-file build helps.

### Testing strategy: build the framework, don't start with one

No pre-built test framework — building one is part of the curriculum (see
`cpp/ROADMAP.md` for the full rationale):

- **Fundamentals 1–5**: `exercise_test.cpp` is a plain `main()` using
  `<cassert>` — `assert(expr == expected)` per check. Stub bodies return a
  default/sentinel value (`""`, `{}`, `0`, ...), NOT `throw` (exceptions
  aren't covered until `advanced/02`), so a failing assert points at a
  specific line.
- **`fundamentals/06`** (Functions, Lambdas & the Preprocessor) builds
  `cpp/testing.h` — a header-only `TEST`/`CHECK`/`TEST_MAIN` framework — as
  one of its own exercises.
- **`fundamentals/07` onward**: `exercise_test.cpp` uses `cpp/testing.h`.
  Stub bodies switch to `throw std::logic_error("not implemented")`, caught
  by `TEST_MAIN()` and reported as a FAIL.
- **`intermediate/01`** (Function Templates) extends `cpp/testing.h` with a
  templated `CHECK_EQ(a, b)`.

### Exercise difficulty

Same bar as JS/Go: **hard, challenging problems**, hand-verified before
writing `exercise_test.cpp` (temporarily implement a reference solution,
build + run the test binary, confirm all pass, then revert to stubs). C++
exercises should exercise language-rule edge cases (UB, overload resolution,
template deduction, object lifetime), not just happy-path I/O.

### Adding a new topic

1. Pick the next `planned` topic from `cpp/ROADMAP.md` (Fundamentals →
   Intermediate → Advanced order), noting its Modern C++ Programming
   chapter(s).
2. Write, in this order: `notes.md` → `exercise.h` + `exercise.cpp` →
   `exercise_test.cpp` → `examples.cpp`.
3. Verify:
   - `examples.cpp` builds and runs cleanly.
   - The exercise test binary builds, and **every check currently fails**
     (per the testing strategy above) — the expected starting state for a
     learner.
4. Update `cpp/ROADMAP.md`: mark the topic `done` and link its folder.

### Capstone

Once the core curriculum (or at least Fundamentals through
`intermediate/04`) is built, `cpp/capstone-ray-tracer/` becomes a ray tracer
built across three phases following the
[_Ray Tracing in One Weekend_ series](https://github.com/RayTracing/raytracing.github.io)
— one growing CMake project, no test suite. "Done" for a phase means it
builds and renders the expected image, verified by running it — same
"build it and verify by running" workflow as the JS capstone. Full
phase breakdown is in the "Capstone" section of `cpp/ROADMAP.md`.

## Checkpointing & quota awareness

This is a large, multi-session build-out. Work one topic at a time through
its full verification (notes → exercise → test → examples, all checks pass),
then immediately update its `Status` to `done` (with folder link) in the
relevant ROADMAP.md, commit, and push. Never stop mid-topic — a clean stop
boundary is always a fully-verified, committed, pushed topic. This bounds
lost work to zero regardless of when a session ends.
