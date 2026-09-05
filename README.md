# Learning

Personal learning roadmaps, notes, examples, and exercises.

## A note on how this works

I built this around the way I like to learn: minimal hand-holding. Each track
starts with a roadmap that defines what to research without supplying the
answers. Some tracks also provide notes and runnable examples for context,
then exercises with a test suite as the spec instead of a solutions file.
The fun part is researching, exploring, and arriving at your own understanding
or implementation.

Where exercises exist, they skew hard. I came at this as a senior engineer and
skipped basic syntax drills, so the difficulty reflects that starting point
rather than a beginner ramp-up.

If any of this is useful to you, take it and make it your own! Clone it,
adapt the exercises to your level, swap in your own topics — hopefully it's
a fun reference, or at least a good starting point for a version that fits
how you learn.

**Full disclosure:** this repository is AI-assisted. I direct the scope,
structure, difficulty, and definition of done, then review and refine the
result. Different tracks may use different coding agents. Treat the primary
specifications and official documentation linked from each roadmap as the
authority.

## Roadmap format

Roadmaps are research plans, not generated guides or answer keys. A language
roadmap uses this structure:

- Ordered phases based on prerequisites.
- A table in each phase with `#`, `Topic`, and `Research target` columns.
- A research target that defines the topic's boundaries and important edge
  cases without explaining the answers.
- Completion criteria describing what should be understood at the end of each
  phase.
- Authoritative sources grouped by phase, favoring language specifications,
  official API documentation, standards, and primary project documentation.
- A short research method for turning each row into personal notes or code.

Roadmaps do not use status checkboxes. They do not include generated solutions
or require exercises by default. A track may add notes, examples, tests, or
capstones after its roadmap has been reviewed. Capstones may evolve alongside
the roadmap; they do not need to wait until every phase is complete.

## Creating or reworking a roadmap

1. Define the audience, prerequisites, goal, and intended depth.
2. Set the boundary of the track. Move general system design, algorithms, and
   unrelated architecture into their own modules.
3. Inventory candidate topics and combine concepts that share one mental model
   or are best researched together.
4. Order phases so language semantics precede libraries, runtime internals, and
   advanced application patterns.
5. Give each topic a concise research target covering rules, tradeoffs, edge
   cases, and relevant implementation behavior.
6. Add primary specifications and official API references for every phase.
7. Review the roadmap for gaps, duplication, misplaced topics, and oversized
   phases before generating any supporting material.
8. Add notes, exercises, or capstones only when they serve the track's learning
   goal. Prefer capstones when one evolving project can integrate the topics.

- [`go/`](./go) — Go research roadmap for experienced engineers, covering
  language semantics, types and generics, the standard library, concurrency,
  runtime and tooling, and idiomatic patterns. Capstones evolve alongside the
  research instead of separate topic exercises. See
  [`go/README.md`](./go/README.md) and [`go/ROADMAP.md`](./go/ROADMAP.md).
- [`java/`](./java) — Java research roadmap for experienced engineers,
  covering language and core APIs, generics and collections, modern Java, JVM
  internals, concurrency, and a later patterns deep dive. See
  [`java/ROADMAP.md`](./java/ROADMAP.md).
- [`rust/`](./rust) — Rust Edition 2024 research roadmap for experienced
  engineers, covering ownership, traits, standard-library boundaries,
  concurrency and async, unsafe Rust, runtime behavior, tooling, and idiomatic
  API design. See
  [`rust/README.md`](./rust/README.md) and
  [`rust/ROADMAP.md`](./rust/ROADMAP.md).
