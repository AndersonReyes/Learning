# Advanced 05: Compiler Optimization, Profiling & Benchmarking

advanced/03-04 covered techniques for writing fast code. This topic covers
**how to know if it worked**: compiler optimization levels, profilers that
find where time actually goes, and the statistics needed to interpret
benchmark results correctly (the "benchmark statistics toolkit" this topic's
exercises build).

## Compiler optimization levels

`g++`/`clang++` flags controlling how aggressively the compiler transforms
code (inlining, loop unrolling, vectorization, dead-code elimination,
constant folding, ...):

| Flag | Effect |
|------|--------|
| `-O0` (default) | No optimization -- fastest to compile, best for debugging (variables/lines map directly to source). |
| `-O1` | Basic optimizations, modest compile-time cost. |
| `-O2` | Most optimizations that don't trade size for speed significantly -- the standard "release build" flag. |
| `-O3` | Adds aggressive vectorization/inlining -- can increase code size; occasionally slower if it hurts instruction-cache locality. |
| `-Ofast` | `-O3` plus relaxes strict standards compliance (e.g. `-ffast-math`, which breaks IEEE 754 semantics like NaN/Inf handling and reordering of floating-point operations) -- use only when that tradeoff is acceptable. |
| `-flto` | Link-Time Optimization -- optimizes across translation-unit boundaries (e.g. inlining a function defined in another `.cpp` file), not just within one file. |
| `-march=native` | Generate code using all instruction set extensions the *build machine's* CPU supports (AVX2, etc.) -- the binary may not run on older CPUs. |

**Always benchmark with the same flags you'll ship with.** Code that's
"optimized" under `-O0` (e.g. manual loop unrolling) is often *redundant*
under `-O2` -- the compiler already does it, and the hand-unrolled version
can even be slower (larger code, worse instruction-cache behavior). advanced/04's
examples.cpp's row/column traversal demo shows this directly: at `-O0` the
"cache-friendly" pattern can lose to overhead noise; at `-O2` it wins as
expected.

**Profile-Guided Optimization (PGO)**: compile with `-fprofile-generate`, run
the program on representative workloads (producing `.gcda` profile data),
then recompile with `-fprofile-use` -- the compiler uses real branch/call
frequencies to make better inlining and layout decisions than static
heuristics alone.

## Profilers: finding where time actually goes

**Don't guess where the bottleneck is -- measure.** Common tools:

- **`perf`** (Linux): samples the running program's instruction pointer at a
  fixed rate (statistical/sampling profiler) with very low overhead.
  `perf record ./prog && perf report` shows which functions consumed the
  most CPU time. `perf stat` reports hardware counters (cache misses, branch
  mispredictions, instructions-per-cycle) -- directly measuring the
  advanced/03 cache-hierarchy and advanced/04 branch-prediction effects.
- **`gprof`**: an older instrumenting profiler (compile with `-pg`) --
  higher overhead, but reports call counts and call graphs.
- **Valgrind's `callgrind`**: simulates the CPU/cache to give exact
  (deterministic, reproducible) instruction and cache-access counts --
  much slower to run than `perf`, but reproducible across machines/runs.
- **Sanitizers** (`-fsanitize=address,undefined`, from intermediate/06) are
  *correctness* tools, not performance tools -- but a program that's
  UB-clean is a prerequisite for trusting its benchmark numbers (UB can make
  the optimizer do something different from what you measured).

## Benchmark methodology

A single timed run is close to useless -- system noise (OS scheduling,
thermal throttling, other processes, cache state from a *previous*
iteration) means run-to-run variance can dwarf the effect you're trying to
measure. Sound methodology:

1. **Warm up** -- run the code a few times before timing, so caches/branch
   predictors/CPU frequency scaling reach steady state. Discard these
   warm-up iterations from your samples.
2. **Repeat** -- run many iterations (tens to thousands, depending on how
   fast each iteration is), recording each as a separate SAMPLE.
3. **Prevent dead-code elimination** -- if the optimizer can prove a
   computed result is never used, it deletes the computation entirely
   (you'd be benchmarking an empty loop). Libraries like Google Benchmark
   provide `DoNotOptimize(result)`; a simple DIY equivalent is writing the
   result to a `volatile` variable or accumulating it into a value that's
   printed/returned at the end.
4. **Summarize, don't eyeball** -- with samples in hand, the toolkit this
   topic builds:
   - **mean/median/stddev** (`summarize`) -- median is more robust to
     occasional outlier spikes than mean; stddev quantifies how noisy the
     measurement is (a high stddev relative to the mean means you need more
     samples or better isolation).
   - **percentiles** (`percentile`) -- for latency-sensitive code, the *tail*
     (p95/p99) often matters more than the average: a server's user-facing
     latency is dominated by its slow requests, not its median one.
   - **trimmed mean** (`trimmedMean`) -- drop the most extreme
     high/low samples (a fixed fraction from each end) before averaging,
     reducing the influence of rare scheduling-noise spikes without
     discarding the bulk of the data the way taking only the median does.
   - **significance testing** (`isSignificantSpeedup`) -- "after" being
     numerically lower than "before" isn't enough; if their distributions'
     confidence intervals overlap, the difference could be noise. Compare
     `before`/`after` sample sets, not single numbers.

## Amdahl's law vs Gustafson's law

Both relate **parallel speedup** to the fraction `p` of work that can be
parallelized and the number of processors `n` -- but they answer different
questions:

- **Amdahl's law** -- "I have a FIXED problem; how much faster does it run on
  `n` processors?" `speedup = 1 / ((1-p) + p/n)`. The sequential fraction
  `1-p` caps the speedup: even infinite processors can't beat
  `1 / (1-p)` (if 10% of the work is inherently sequential, speedup never
  exceeds 10x).
- **Gustafson's law** -- "I have `n` processors; if I use them to solve a
  PROPORTIONALLY LARGER problem in the same time, how much MORE work gets
  done?" `speedup = (1-p) + p*n`. This is often more realistic for scaling
  up workloads (more data, finer simulations) rather than running a
  fixed-size task faster.

Both are *predictions* assuming perfect parallelization of the parallel
portion (no synchronization overhead, perfect load balancing) -- real
speedups from `std::thread`/`std::async` (advanced/02) are usually lower;
measure and compare against the prediction to see how much overhead your
actual parallelization adds.

## Further Reading

- [MCPP ch. 25 -- Compiler Optimizations, Profiling, and Benchmarking](https://federico-busato.github.io/Modern-CPP-Programming/htmls/25.Compiler_Optimization_Profiling_Benchmarking.html)
- [cppreference: `<chrono>`](https://en.cppreference.com/w/cpp/header/chrono)
- [Google Benchmark (microbenchmarking library)](https://github.com/google/benchmark)
- [`perf` Examples (Brendan Gregg)](https://www.brendangregg.com/perf.html)
- [Wikipedia: Amdahl's law](https://en.wikipedia.org/wiki/Amdahl%27s_law)
- [Wikipedia: Gustafson's law](https://en.wikipedia.org/wiki/Gustafson%27s_law)
- [Wikipedia: Percentile (linear interpolation methods)](https://en.wikipedia.org/wiki/Percentile)
