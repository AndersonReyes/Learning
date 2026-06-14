# C++ — Ray Tracing in One Weekend

**Status: planning.** This track isn't built out yet — see
[`ROADMAP.md`](./ROADMAP.md) for the phase outline and open TODOs (build
system, project layout, and reference docs are all still TBD).

## How this track works

Unlike `javascript/` and `go/` (independent numbered topics, each with its
own `node:test`/`go test` exercise spec), this track is **one cumulative
project**: a ray tracer built up incrementally across three phases, based on
the [_Ray Tracing in One Weekend_ series](https://github.com/RayTracing/raytracing.github.io).
Each phase's code builds directly on the previous phase's.

There's no test suite here. Each phase's notes will explain the concepts and
walk through the code for that phase; a phase is "done" when the program
builds and renders the expected output image — checked by running it and
viewing the result, similar to the capstone project's "build it and verify
by running" workflow.

See [`ROADMAP.md`](./ROADMAP.md) for the phase-by-phase breakdown.
