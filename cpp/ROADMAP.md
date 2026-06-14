# C++ Roadmap

**Status: planning.** The phase breakdown below is a rough outline. Build
system, project layout, and "Further Reading" reference docs are not decided
yet — see "Open TODOs" at the bottom.

## Shape of this track

This track is **one cumulative project**, not a set of independent topics:
a ray tracer built incrementally across three phases, following the
[_Ray Tracing in One Weekend_ series](https://github.com/RayTracing/raytracing.github.io)
(Shirley et al. — three short books, each a direct sequel to the last).
Each phase's code extends the previous phase's code. There's no
`exercise.test.js`/`exercise_test.go`-style spec; "done" for a phase means
the program builds and renders the expected output image, checked by running
it and inspecting the result.

## Phase 1 — foundational ray tracer

Covers the first book. Builds a single-threaded CPU ray tracer that outputs
a pixel image, starting from nothing and ending with a recognizable rendered
scene:

- Image output (PPM format), pixel grid, viewport/camera basics
- 3D vector math (point/direction/color all modeled as a vector type)
- Rays, ray-sphere intersection, surface normals
- Antialiasing via per-pixel multisampling
- Diffuse (matte) materials and recursive ray bounces
- Reflective (metal) materials, including fuzzy reflection
- Refractive (glass/dielectric) materials
- A positionable camera: field of view, orientation, depth-of-field/defocus
  blur
- Final scene: many randomly-placed spheres with mixed materials, rendered
  together

**C++ concepts introduced:** project/build setup, classes and operator
overloading (vector/color/ray types), header organization, inheritance and
virtual functions (a common "hittable" interface, a common "material"
interface), `shared_ptr` for polymorphic ownership of scene objects,
recursion (ray bounce depth + base case), `const`-correctness, `<random>`
for sampling.

## Phase 2 — performance, textures, lighting, volumes

Covers the second book. Extends the Phase 1 ray tracer with features needed
for more complex, realistic scenes:

- Motion blur (time-dependent ray generation)
- Bounding volume hierarchy (BVH) for fast ray-object intersection at scale
- Textures: solid colors, procedural patterns (e.g. checkerboards, noise),
  and image-based textures
- Rectangular/quad primitives and emissive (light-source) materials
- Instancing: translating and rotating objects without duplicating geometry
- Participating media (fog/smoke-style volumes)

**C++ concepts introduced:** recursive tree data structures (BVH
construction and traversal), `std::vector` and STL algorithms (e.g. sorting
for BVH construction), function objects/lambdas as comparators, file I/O
(loading image textures from disk), deeper class composition.

## Phase 3 — physically-based sampling

Covers the third book. Reworks the renderer's sampling strategy for more
accurate, less noisy images:

- Monte Carlo integration fundamentals (estimating an integral via random
  sampling)
- Importance sampling (sampling proportional to where the integrand is
  large)
- Probability density functions (PDFs) attached to materials/scattering
- Sampling light sources directly, rather than relying on chance ray hits
- Mixture densities that combine multiple sampling strategies

**C++ concepts introduced:** numerical/probabilistic code organization,
abstract base classes for swappable sampling strategies (strategy pattern),
refactoring and extending the Phase 1–2 codebase without breaking it.

## Open TODOs

- [ ] Pick a primary C++ reference for each phase's "Further Reading"
      section (candidates to evaluate later: cppreference.com, _A Tour of
      C++_, learncpp.com — **not decided**, deferred per request)
- [ ] Decide build system and project layout (CMake vs. plain
      compiler invocations; one project for all phases vs. one per phase)
- [ ] Decide `notes.md` granularity (one per phase vs. one per major
      concept/chapter)
- [ ] Write Phase 1 `notes.md` and initial project scaffold once the above
      are settled
