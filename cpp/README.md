# C++

**Status: in progress.** Curriculum is settled (see [`ROADMAP.md`](./ROADMAP.md)).
`fundamentals/01-setup-and-hello-world` is built; see `ROADMAP.md` for status
of the rest.

## How this track works

Same shape as `javascript/` and `go/`: numbered topic folders under
`fundamentals/`, `intermediate/`, and `advanced/`, each with `notes.md`,
`examples.cpp`, `exercise.h` (+ `exercise.cpp` where needed), and
`exercise_test.cpp`. `exercise_test.cpp` is the spec/answer key — there are
no separate solution files. See [`ROADMAP.md`](./ROADMAP.md) for the full
curriculum, per-topic file pattern, and "Adding a new topic" workflow.

The curriculum is built around Federico Busato's
[**Modern C++ Programming**](https://github.com/federico-busato/Modern-CPP-Programming)
course — each topic maps to one or more of its 29 chapters, linked from that
topic's "Further Reading" section alongside cppreference.com.

## Building and testing

No external dependencies (no vendored test framework, no package manager).
Everything compiles with a plain `g++`/`clang++` invocation:

```sh
# Run a topic's examples
g++ -std=c++20 -Wall -Wextra -o /tmp/ex fundamentals/01-setup-and-hello-world/examples.cpp && /tmp/ex

# Run a topic's exercise tests (spec)
g++ -std=c++20 -Wall -Wextra -o /tmp/test fundamentals/01-setup-and-hello-world/exercise_test.cpp fundamentals/01-setup-and-hello-world/exercise.cpp && /tmp/test
```

There's no pre-built test framework — `exercise_test.cpp` for
`fundamentals/01`–`05` is a plain `assert()`-based `main()`. `fundamentals/06`
(Functions, Lambdas & the Preprocessor) builds `cpp/testing.h`, a small
`TEST`/`CHECK`/`TEST_MAIN` framework, as one of its own exercises; every
topic from `fundamentals/07` onward uses it. See "Testing strategy" in
[`ROADMAP.md`](./ROADMAP.md).

CMake is introduced as its own topic (`intermediate/06`) and used from then
on where a multi-file build is useful, and throughout the capstone.

## Capstone

Once the core curriculum is far enough along, `cpp/capstone-ray-tracer/`
builds a CPU ray tracer across three phases, following the
[_Ray Tracing in One Weekend_ series](https://github.com/RayTracing/raytracing.github.io).
Unlike the topics above, it's one growing CMake project with no test suite —
each phase is "done" when it builds and renders the expected image. See the
"Capstone" section of [`ROADMAP.md`](./ROADMAP.md).
