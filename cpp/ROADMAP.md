# C++ Roadmap

**Status: in progress.** Curriculum and reference are settled (see below).
`fundamentals/01-setup-and-hello-world`, `fundamentals/02-types-and-operators`,
`fundamentals/03-integer-and-floating-point-arithmetic`,
`fundamentals/04-control-flow-and-entities`,
`fundamentals/05-pointers-references-and-memory`,
`fundamentals/06-functions-and-lambdas` (which also built `cpp/testing.h`,
used by every topic from `fundamentals/07` on), and
`fundamentals/07-classes-and-raii` are built. Next topic to build:
`fundamentals/08-polymorphism-and-operator-overloading`.

## Reference

Primary reference for this track is Federico Busato's
[**Modern C++ Programming**](https://github.com/federico-busato/Modern-CPP-Programming)
course (CC BY 4.0 course content, MIT-licensed code samples; 29 chapters,
used at the University of Verona). Each topic below maps to one or more of
its chapters, and each topic's `notes.md` links the matching chapter's HTML
slides (`https://federico-busato.github.io/Modern-CPP-Programming/htmls/NN.Chapter.html`)
plus relevant [cppreference.com](https://en.cppreference.com/) pages under
"Further Reading" — the same role MDN plays for the JS track and
go.dev/RFCs play for the Go track.

## Per-topic structure

Each topic is a numbered folder, e.g. `fundamentals/02-types-and-operators/`,
with 4 files (same role as the JS/Go pattern, adapted for C++):

- **`notes.md`** — concept explanation in the same terse style as JS/Go:
  syntax, rules, gotchas, short snippets. Ends with "Further Reading" linking
  the matching Modern C++ Programming chapter(s) and cppreference pages.
- **`examples.cpp`** — single translation unit, compiled and run directly
  (`g++ -std=c++20 -Wall -Wextra -o /tmp/ex examples.cpp && /tmp/ex`).
  Demonstrates the topic's concepts with `std::cout`/`printf`. No exercises
  here.
- **`exercise.h`** + `exercise.cpp` — 5 function/class stubs with doc
  comments (signature, behavior, example I/O).
- **`exercise_test.cpp`** — this IS the spec/answer key (no separate
  solution files). How it's written depends on the testing strategy below.

Plain compiler invocations (no CMake) until `intermediate/06`, which covers
CMake directly — from then on, topics that benefit from a multi-file build
may include a `CMakeLists.txt`. The capstone (below) uses CMake throughout.

## Testing strategy: build the framework, don't start with one

There's no pre-built test framework — building one is part of the
curriculum, once the language features it needs have been covered:

- **Fundamentals 1–5**: `exercise_test.cpp` is a plain `main()` using
  `<cassert>`. Each check is `assert(expr == expected)`; stub bodies return a
  default/sentinel value (`""`, `{}`, `0`, ...) so a failing assertion points
  at a specific line — not `throw`, since exception handling isn't covered
  until `advanced/02`. `main()` prints an "all tests passed" message and
  returns 0 only if every `assert` survives. Compiled with
  `g++ -std=c++20 -Wall -Wextra -o /tmp/test exercise_test.cpp exercise.cpp && /tmp/test`.
- **`fundamentals/06` (Functions, Lambdas & the Preprocessor)** still uses
  plain `<cassert>` for its own `exercise_test.cpp` (its `runTests` exercise
  *is* the core logic of a test runner: run named thunks, catch
  `std::exception` per thunk, record pass/fail). It also builds
  [`cpp/testing.h`](./testing.h): a small header-only `TEST(name) { ... }` /
  `CHECK(cond)` / `TEST_MAIN()` framework — macros (`#`/`##`,
  `__FILE__`/`__LINE__`) for registration and diagnostics, and
  `std::function`/lambdas to hold each test body in a registry that
  `TEST_MAIN()` iterates, catching exceptions per test so one failing test
  can't abort the others.
- **`fundamentals/07` onward** (all of Intermediate/Advanced):
  `exercise_test.cpp` uses `cpp/testing.h` from `fundamentals/06` (included
  as `../../testing.h`). Stub bodies switch to `throw
  std::logic_error("not implemented")`, caught by `TEST_MAIN()` and reported
  as a clean FAIL. Compiled with
  `g++ -std=c++20 -Wall -Wextra -o /tmp/test exercise_test.cpp exercise.cpp && /tmp/test`.
- **`intermediate/01` (Function Templates)** revisits `cpp/testing.h`,
  adding a templated `CHECK_EQ(a, b)` that prints both sides on failure —
  applying templates to the learner's own tooling.

## Fundamentals

| # | Topic | Folder | Reference | Status |
|---|-------|--------|-----------|--------|
| 1 | Setup, Compilation & Hello World | [`fundamentals/01-setup-and-hello-world`](./fundamentals/01-setup-and-hello-world) | MCPP ch. 2 (Preparation) | done |
| 2 | Type System, Fundamental Types & Operators | [`fundamentals/02-types-and-operators`](./fundamentals/02-types-and-operators) | MCPP ch. 3 | done |
| 3 | Integer & Floating-Point Arithmetic | [`fundamentals/03-integer-and-floating-point-arithmetic`](./fundamentals/03-integer-and-floating-point-arithmetic) | MCPP ch. 4–5 | done |
| 4 | Control Flow, Enums, Structs & Namespaces | [`fundamentals/04-control-flow-and-entities`](./fundamentals/04-control-flow-and-entities) | MCPP ch. 6 | done |
| 5 | Pointers, References, Memory & `const` | [`fundamentals/05-pointers-references-and-memory`](./fundamentals/05-pointers-references-and-memory) | MCPP ch. 7 | done |
| 6 | Functions, Lambdas & the Preprocessor (+ build `cpp/testing.h`) | [`fundamentals/06-functions-and-lambdas`](./fundamentals/06-functions-and-lambdas) | MCPP ch. 8 | done |
| 7 | Classes, Constructors, Destructors & RAII | [`fundamentals/07-classes-and-raii`](./fundamentals/07-classes-and-raii) | MCPP ch. 9 | done |
| 8 | Polymorphism & Operator Overloading | `fundamentals/08-polymorphism-and-operator-overloading` | MCPP ch. 10 | planned |

## Intermediate

| # | Topic | Folder | Reference | Status |
|---|-------|--------|-----------|--------|
| 1 | Function Templates & Compile-Time Utilities | `intermediate/01-function-templates` | MCPP ch. 11 | planned |
| 2 | Class Templates, CTAD, SFINAE & Concepts | `intermediate/02-class-templates-and-concepts` | MCPP ch. 12 | planned |
| 3 | Translation Units, Linkage & the ODR | `intermediate/03-translation-units-and-odr` | MCPP ch. 13 | planned |
| 4 | Multi-File Projects, `#include`, Modules & Libraries | `intermediate/04-modules-and-libraries` | MCPP ch. 14 | planned |
| 5 | Project Organization & Code Conventions | `intermediate/05-code-conventions` | MCPP ch. 15–16 | planned |
| 6 | Debugging, Sanitizers, Testing & CMake | `intermediate/06-debugging-testing-and-cmake` | MCPP ch. 17–18 | planned |
| 7 | Standard Library Utilities (`string`, `optional`, `variant`, `<random>`, filesystem) | `intermediate/07-standard-library-utilities` | MCPP ch. 19 | planned |
| 8 | Containers, Iterators, Algorithms & Ranges | `intermediate/08-containers-iterators-and-algorithms` | MCPP ch. 20 | planned |

## Advanced

| # | Topic | Folder | Reference | Status |
|---|-------|--------|-----------|--------|
| 1 | Move Semantics, Value Categories & Type Deduction | `advanced/01-move-semantics-and-type-deduction` | MCPP ch. 21 | planned |
| 2 | Error Handling, Smart Pointers & Concurrency | `advanced/02-error-handling-smart-pointers-and-concurrency` | MCPP ch. 22 | planned |
| 3 | Performance Fundamentals: Architecture & Memory Hierarchy | `advanced/03-performance-fundamentals` | MCPP ch. 23 | planned |
| 4 | Code-Level Optimization Techniques | `advanced/04-code-optimization` | MCPP ch. 24 | planned |
| 5 | Compiler Optimization, Profiling & Benchmarking | `advanced/05-profiling-and-benchmarking` | MCPP ch. 25 | planned |
| 6 | Software Design Principles, Idioms & Patterns | `advanced/06-software-design-and-idioms` | MCPP ch. 26–27 | planned |
| 7 | Binary Size & Build Time | `advanced/07-binary-size-and-build-time` | MCPP ch. 28–29 | planned |

## Exercise difficulty

Same bar as JS/Go: **hard, challenging problems**, hand-verified before
writing `exercise_test.cpp` (temporarily implement a reference solution,
compile + run the test binary, confirm all tests pass, then revert to
stubs). Since C++ exercises often hinge on subtle language rules (UB,
overload resolution, template deduction, object lifetime), tests should
exercise those edge cases directly, not just "happy path" I/O.

## Adding a new topic

1. Pick the next `planned` topic from the tables above, in order
   (Fundamentals → Intermediate → Advanced), noting its MCPP chapter(s).
2. Write, in this order: `notes.md` → `exercise.h` (+ `exercise.cpp`) →
   `exercise_test.cpp` → `examples.cpp`.
3. Verify:
   - `g++ -std=c++20 -Wall -Wextra -o /tmp/ex <topic>/examples.cpp && /tmp/ex`
     runs cleanly.
   - The exercise test binary builds, and **every check currently fails**
     (sentinel-returning stubs trip an `assert` in fundamentals 1–5; `throw
     std::logic_error("not implemented")` is caught as a FAIL from
     fundamentals 6 onward) — the expected starting state for a learner.
4. Update this file: mark the topic `done` and turn its folder cell into a
   link.

## Capstone: Ray Tracing in One Weekend

Once the core curriculum (or a substantial chunk of it — Fundamentals
through at least `intermediate/04`) is built, the cumulative project from
the previous version of this roadmap moves here as a capstone: a CPU ray
tracer built across three phases, following the
[_Ray Tracing in One Weekend_ series](https://github.com/RayTracing/raytracing.github.io).
Unlike the topics above, this is **one growing CMake project** with no test
suite — "done" for each phase means the program builds and renders the
expected image, checked by running it and viewing the output (same
"build it and verify by running" workflow as the JS capstone). Lives at
`cpp/capstone-ray-tracer/`.

### Phase 1 — foundational ray tracer

- Image output (PPM format), pixel grid, viewport/camera basics
- 3D vector math (point/direction/color as one vector type, with operator
  overloading)
- Rays, ray-sphere intersection, surface normals
- Antialiasing via per-pixel multisampling
- Diffuse (matte), reflective (metal, with fuzz), and refractive
  (dielectric) materials via recursive ray bounces
- A positionable camera: field of view, orientation, depth-of-field/defocus
  blur
- Final scene: many randomly-placed spheres with mixed materials

Applies fundamentals + intermediate concepts directly: classes/operator
overloading (vectors, colors, rays), inheritance and virtual functions
(`hittable`/`material` interfaces), `shared_ptr` for polymorphic ownership,
recursion with a depth limit, `const`-correctness, `<random>`.

### Phase 2 — performance, textures, lighting, volumes

- Motion blur (time-dependent ray generation)
- Bounding volume hierarchy (BVH) for fast ray-object intersection at scale
- Solid, procedural (e.g. checkerboard/noise), and image-based textures
- Rectangular/quad primitives and emissive (light-source) materials
- Instancing: translating/rotating objects without duplicating geometry
- Participating media (fog/smoke-style volumes)

Adds: recursive tree structures (BVH), `std::vector` + STL algorithms (BVH
construction), function objects/lambdas as comparators, image file I/O.

### Phase 3 — physically-based sampling

- Monte Carlo integration fundamentals
- Importance sampling and probability density functions (PDFs) for
  scattering
- Sampling light sources directly instead of relying on chance ray hits
- Mixture densities combining multiple sampling strategies

Adds: numerical/probabilistic code organization, abstract base classes for
swappable sampling strategies (strategy pattern), refactoring the Phase 1–2
codebase without breaking it.
