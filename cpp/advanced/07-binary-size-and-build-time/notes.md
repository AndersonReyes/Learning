# Advanced 07: Binary Size & Build Time

The final advanced topic. advanced/05 covered *runtime* performance
(profiling, benchmarking); this topic covers two other costs: how big the
final binary is, and how long it takes to produce it. Both are
**graph/scheduling problems** in disguise -- reading `nm`/`size`/`objdump`
output and a build's timing log is really reasoning about an `#include`
graph, a dependency DAG, and a symbol table. This topic's exercises build a
small "build-graph & binary-size analysis toolkit" that embodies that
reasoning directly.

## Binary size (ch. 28)

A linked binary's size comes from several sections:

- **`.text`** -- executable code.
- **`.data`** / **`.rodata`** -- initialized (mutable / read-only) globals.
- **`.bss`** -- zero-initialized globals (takes no space ON DISK, but counts
  at runtime).
- Debug info (`.debug_*`) and the symbol table -- often LARGER than the code
  itself in a debug build; `strip` removes them.

### What inflates `.text`

- **Templates and `inline` functions** -- each translation unit (TU) that
  instantiates `std::vector<MyType>` (or calls an `inline`/header-defined
  function) emits its OWN copy of that code. More TUs => more copies, before
  the linker dedups them.
- **Heavy inlining** (`-O3`, aggressive `__attribute__((always_inline))`) --
  trades size for speed by duplicating callee bodies at call sites.
- **Exceptions and RTTI** -- unwind tables (`.eh_frame`) and `typeinfo`/
  `typeid`/`dynamic_cast` metadata add size even on paths that never throw or
  downcast. `-fno-exceptions -fno-rtti` shrink this, at the cost of losing
  both features (common for embedded/firmware builds).

### COMDAT / vague linkage: how duplicates get removed

Symbols that are LEGAL to define identically in multiple TUs --template
instantiations, `inline` functions, `inline` variables-- are emitted into
special **COMDAT sections** (ELF: `SHF_GROUP`, "vague linkage" in Itanium ABI
terms). The linker keeps exactly ONE copy across all TUs that defined the
"same" symbol, discarding the rest. This is precisely what this topic's
`linkedBinarySize` models: non-template symbols are summed unconditionally
(every TU's copy is a distinct definition the linker must keep), but
template-instantiation symbols are deduplicated by name -- one copy survives
no matter how many TUs instantiated it.

**The catch -- One Definition Rule (ODR) violations**: the linker's
deduplication assumes every TU's copy of `vector<int>::push_back` is
byte-for-byte identical. If two TUs compiled the SAME template instantiation
with different flags/macros (e.g. one with `NDEBUG`, one without, changing
an `assert` inside), they're NOT identical -- but the linker still picks
ONE arbitrarily (silent, UB). `linkedBinarySize` is stricter than a real
linker here: it THROWS on a same-name-different-size mismatch, surfacing the
ODR violation a real linker would silently hide.

### Flags & tools

| Flag/tool | Effect |
|---|---|
| `-Os` | Optimize for size (subset of `-O2`, skips size-increasing passes). |
| `-ffunction-sections -fdata-sections` + `-Wl,--gc-sections` | Put each function/global in its own section so the linker can discard unreferenced ones (dead-code elimination across TU boundaries). |
| `-fvisibility=hidden` | Hide symbols not part of the public ABI -- smaller dynamic symbol table, enables more cross-TU inlining/elimination. |
| `strip` | Remove debug info and symbol tables from a built binary. |
| `nm` | List a binary/object's symbols (and their sizes with `-S`). |
| `size` | Print `.text`/`.data`/`.bss` totals. |
| `objdump -d` | Disassemble -- see what a function actually compiled to. |
| [Bloaty McBloat](https://github.com/google/bloaty) | Attribute binary size back to source files/symbols/templates. |

## Build time (ch. 29)

### Where the time goes

- **Preprocessing** -- `#include` is a textual copy-paste. `#include
  <vector>` can pull in thousands of lines of transitively-included headers,
  reparsed from scratch in EVERY TU that includes it (directly or
  transitively). `transitiveIncludeCount` computes exactly this: how much
  text a given TU's compile must parse because of its `#include` graph.
- **Template instantiation** -- the compiler re-instantiates `vector<int>`
  (etc.) in every TU that uses it, only for the linker to throw most copies
  away later (see COMDAT above) -- wasted compile work.
- **Optimization & codegen** -- scales with `-O` level and function size
  after inlining.
- **Linking** -- normally fast, but LTO moves a large chunk of optimization
  work INTO the link step (see below).

### Incremental builds: what needs to be recompiled

When a header changes, every TU that includes it (directly or transitively)
must be recompiled -- the REVERSE of the `#include` graph.
`filesToRebuild` computes this rebuild set. A header included by hundreds of
`.cpp` files turns a one-line edit into a full rebuild; this is the #1
reason to minimize what headers expose (forward-declare instead of
`#include`, prefer `.cpp`-local includes, split "leaf" headers from
"umbrella" headers).

### Parallel builds: the critical path

`make -j`/`ninja` run independent compile jobs in parallel, but the WALL-CLOCK
time is bounded below by the **critical path** through the dependency DAG --
the longest chain of "must finish before this can start" steps, regardless
of how many cores are free. `criticalPathBuildTime` computes this: it's
"Amdahl's law for builds" -- adding more workers helps only up to the point
where the critical path itself becomes the bottleneck.

Given a fixed number of workers, `groupLoadsGreedy` models the OTHER half of
the problem: distributing a batch of independent compile jobs across `N`
workers to minimize the SLOWEST worker's finish time (LPT -- Longest
Processing Time first -- is a simple, provably-good greedy heuristic, used
by `ninja`'s and distributed-build schedulers' job assignment).

### Techniques that reduce build time

- **Precompiled headers (PCH)** -- parse a common header prefix (e.g. STL +
  project-wide headers) ONCE into a binary blob, reuse it for every TU that
  shares that prefix.
- **Unity builds** -- `#include` many `.cpp` files into one TU so shared
  headers are parsed once instead of N times. Trade-off: loses
  intra-TU parallelism and can surface name collisions/`static`/anonymous-
  namespace conflicts that were previously isolated per-TU.
- **LTO / ThinLTO** -- Link-Time Optimization defers cross-TU inlining/
  dead-code-elimination to link time, for better runtime perf at the cost of
  a much heavier link step. ThinLTO parallelizes by summarizing each TU
  separately and merging summaries, mitigating the single-giant-link-job
  problem of full LTO.
- **`ccache`** -- caches object files keyed by preprocessed source + compiler
  flags; an unchanged TU's compile becomes a cache hit.
- **Distributed compilation** (`distcc`, Icecream) -- ship preprocessed TUs
  to other machines' compilers, parallelizing across a cluster instead of
  just cores. Still bounded by `criticalPathBuildTime` and improved by
  `groupLoadsGreedy`-style load balancing across the available machines.
- **Include-What-You-Use (IWYU)** -- tooling that flags headers a TU
  `#include`s but doesn't need, and includes it's missing but relies on
  transitively. Trimming unnecessary includes shrinks
  `transitiveIncludeCount` for every TU and `filesToRebuild` for every
  header.
- **C++20 modules** -- replace textual `#include` with a binary module
  interface, parsed once and `import`ed many times -- eliminates the
  repeated-reparsing cost that PCH/unity builds work around at the source
  level.

## This topic's toolkit

| Exercise | Models |
|---|---|
| `transitiveIncludeCount` | Forward `#include`-graph reachability -- how much a TU's compile must parse. |
| `filesToRebuild` | Reverse `#include`-graph reachability -- the incremental rebuild set after a header changes. |
| `criticalPathBuildTime` | DAG critical path / makespan -- the fastest possible full build with unlimited parallelism. |
| `groupLoadsGreedy` | LPT greedy scheduling -- balancing compile jobs across `N` parallel workers. |
| `linkedBinarySize` | COMDAT/vague-linkage deduplication of template instantiations, with ODR-violation detection. |

## Further Reading

- [MCPP ch. 28 -- Binary Size](https://federico-busato.github.io/Modern-CPP-Programming/htmls/28.Binary_Size.html)
- [MCPP ch. 29 -- Build Time](https://federico-busato.github.io/Modern-CPP-Programming/htmls/29.Build_time.html)
- [cppreference: One Definition Rule](https://en.cppreference.com/w/cpp/language/definition)
- [cppreference: C++20 Modules](https://en.cppreference.com/w/cpp/language/modules)
- [Wikipedia: COMDAT](https://en.wikipedia.org/wiki/COMDAT)
- [Wikipedia: Longest-processing-time-first scheduling](https://en.wikipedia.org/wiki/Longest-processing-time-first_scheduling)
- [Wikipedia: Critical path method](https://en.wikipedia.org/wiki/Critical_path_method)
- [`ccache`](https://ccache.dev/)
- [Google `Bloaty McBloat`](https://github.com/google/bloaty)
