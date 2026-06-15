# Advanced 04: Code-Level Optimization Techniques

Once the memory-hierarchy fundamentals (advanced/03) are in place, the next
layer is **code-level techniques**: rewriting *how* a computation is
expressed -- without changing *what* it computes -- so the compiler and CPU
do less, or more predictable, work. Always measure before and after
(advanced/05) -- these techniques can be no-ops or even regressions if the
compiler already does them, or if the "hot path" they target isn't actually
hot.

## Branchless programming (avoiding branch misprediction)

Modern CPUs **speculatively execute** past conditional branches, predicting
which way an `if` will go. A misprediction costs ~10-20 cycles to discard the
wrong-path work and restart -- expensive if it happens often and
unpredictably (e.g. branching on essentially-random data).

**Branchless** code expresses a choice so its result doesn't depend on
*control flow* -- only on *data* -- letting the compiler emit a conditional
move (`cmov`) or bitwise select instead of a jump:

```cpp
// Branching: a misprediction here stalls the pipeline.
int min(int a, int b) { return a < b ? a : b; }

// Branchless: `a < b` produces 0 or 1 (no jump); -(0 or 1) is 0x00000000 or
// 0xFFFFFFFF, used as a bitmask to select a or b.
int branchlessMin(int a, int b) { return b ^ ((a ^ b) & -(int)(a < b)); }
```

Why it works: if `a < b` (mask = `-1` = all-1-bits), `(a^b) & mask == a^b`,
so the result is `b ^ (a^b) == a`. If `a >= b` (mask = `0`), the result is
`b ^ 0 == b`. Either way, no jump depends on the comparison's *outcome* --
only the arithmetic result does.

**`branchlessAbs` and signed overflow**: the classic abs trick is
`(x ^ mask) - mask` where `mask = x >> 31` (all 1s if negative, all 0s if
non-negative). For `x == INT_MIN`, `-INT_MIN` overflows `int` -- but doing
the same arithmetic in `unsigned int` is well-defined (wraps modulo 2^32),
and converting the unsigned result back to `int` is well-defined in C++20
(reinterprets the bit pattern, two's complement). The "answer" for
`abs(INT_MIN)` isn't representable as `int` anyway, so wrapping back to
`INT_MIN` matches what `std::abs`/hardware does in practice.

```cpp
int branchlessAbs(int x) {
    unsigned int ux = (unsigned int)x;
    unsigned int mask = (unsigned int)(x >> 31);  // well-defined: arithmetic shift in C++20
    return (int)((ux ^ mask) - mask);             // well-defined: unsigned wraparound + reinterpret
}
```

**Caveat**: branchless tricks aren't always faster -- if the branch is highly
*predictable* (e.g. always taken), the CPU predicts it correctly almost every
time and the `if` is free, while a branchless version still does the
arithmetic on *every* call. Use branchless code for genuinely
data-dependent, unpredictable conditions in hot loops.

## Bit-manipulation tricks

**Population count (Hamming weight)** -- count of set bits. Kernighan's
trick, `x &= x - 1`, clears the *lowest* set bit each iteration (subtracting
1 flips all trailing zeros to 1 and the lowest 1 to 0; ANDing with the
original clears that run): runs in `O(popcount(x))` iterations instead of
`O(bit width)`.

```cpp
unsigned countSetBits(unsigned x) {
    unsigned count = 0;
    while (x) { x &= x - 1; ++count; }
    return count;
}
```

(`<bit>`'s `std::popcount` does this in hardware via the `POPCNT`
instruction where available -- prefer it in real code; the manual loop is for
understanding the trick.)

**Bit reversal** -- reverse bit order (bit 0 <-> bit N-1, etc.), used in FFT's
bit-reversal permutation and some hash/PRNG constructions: shift bits out of
one end and into the other, one at a time.

## Loop fusion: combining passes over the same data

Computing several aggregates (sum, min, max, sum of squares, ...) over a
`std::vector` in *separate* loops means the data streams through cache
`numStats` times. **Loop fusion** computes them all in ONE traversal:

```cpp
// Three passes -- data re-read from RAM/cache 3x for large vectors.
double sum = std::accumulate(v.begin(), v.end(), 0.0);
double mn = *std::min_element(v.begin(), v.end());
double mx = *std::max_element(v.begin(), v.end());

// Fused -- one pass.
double sum = 0, sumSq = 0, mn = v[0], mx = v[0];
for (double x : v) {
    sum += x; sumSq += x * x;
    if (x < mn) mn = x;
    if (x > mx) mx = x;
}
```

**Numerical caveat**: `variance = E[x^2] - E[x]^2` (the fused formula above)
is a single pass but can suffer **catastrophic cancellation** when the mean
is large relative to the spread (subtracting two large nearly-equal numbers
loses precision). **Welford's online algorithm** computes variance in one
pass *without* this issue, at the cost of a per-element division -- the
right tradeoff depends on the data's scale.

## Memoization: trading memory for avoided recomputation

A naive recursive function whose calls overlap (e.g. `f(n) = f(n-1) + f(n-2)
+ f(n-3)`) recomputes the same sub-results exponentially many times.
**Memoization** caches each input's result the first time it's computed, so
later calls (including recursive sub-calls with the same argument) are O(1)
lookups:

```cpp
std::unordered_map<int, long long> memo;
long long f(int n) {
    if (n < 0) return 0;
    if (auto it = memo.find(n); it != memo.end()) return it->second;
    long long result = (n < 2) ? 1 : f(n-1) + f(n-2) + f(n-3);
    memo[n] = result;
    return result;
}
```

This turns an O(3^n)-ish call tree into O(n) *distinct* computations (each
`n` computed once, in any order dictated by the recursion, but never twice).
This is the foundation of **dynamic programming**: memoized recursion
("top-down DP") and iterative table-filling ("bottom-up DP") compute the same
results -- top-down only computes the subproblems actually needed.

## Sorting networks: branch-predictor-friendly fixed-size sorts

A **sorting network** for `N` elements is a fixed sequence of
*compare-exchange* operations (compare two elements, put the smaller one
first) whose *positions* don't depend on the data -- only the swap decisions
do. For small, fixed `N` (e.g. sorting 4 SIMD lanes, or a small fixed-size
"top-K" buffer), a sorting network beats `std::sort`:

- No loop-trip-count branches (the sequence of comparisons is always the
  same length).
- Each compare-exchange can itself be branchless (`branchlessMin`/
  `branchlessMax`), so the entire sort has *zero* data-dependent branches.

The (proven) minimal network for 4 elements uses 5 compare-exchanges, applied
to index pairs `(0,1)`, `(2,3)`, `(0,2)`, `(1,3)`, `(1,2)` in that fixed
order -- regardless of input, after these 5 steps the array is sorted.

## General principles

- **Measure first** (advanced/05) -- intuition about what's "slow" is often
  wrong; the bottleneck is frequently somewhere unexpected.
- **Algorithmic complexity dominates** for large inputs -- an O(n log n)
  algorithm beats a micro-optimized O(n^2) one well before these
  micro-techniques matter. Apply code-level optimization to algorithms that
  are already asymptotically appropriate.
- **The compiler often already does this** -- with optimizations enabled
  (`-O2`/`-O3`), GCC/Clang routinely turn simple `?:`/`if` into `cmov`,
  unroll small fixed-trip-count loops, and vectorize simple reductions.
  Hand-written "optimized" code can be *redundant* or even *slower* than the
  straightforward version once the optimizer runs -- write clear code first,
  profile, and apply these techniques only where they measurably help.

## Further Reading

- [MCPP ch. 24 -- Code Optimization](https://federico-busato.github.io/Modern-CPP-Programming/htmls/24.Code_Optimization.html)
- [cppreference: `std::popcount`](https://en.cppreference.com/w/cpp/numeric/popcount)
- [cppreference: bit manipulation (`<bit>`)](https://en.cppreference.com/w/cpp/header/bit)
- [cppreference: arithmetic conversions / integer overflow](https://en.cppreference.com/w/cpp/language/implicit_conversion)
- [Wikipedia: Sorting network](https://en.wikipedia.org/wiki/Sorting_network)
- [Wikipedia: Algorithms for calculating variance (Welford's algorithm)](https://en.wikipedia.org/wiki/Algorithms_for_calculating_variance)
