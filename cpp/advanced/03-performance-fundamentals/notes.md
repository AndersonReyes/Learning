# Advanced 03: Performance Fundamentals: Architecture & Memory Hierarchy

Writing fast C++ starts with understanding the **memory hierarchy** the CPU
sits on top of -- most "optimization" is really about avoiding waits for
memory.

## The memory hierarchy

Each level is faster, smaller, and more expensive (per byte) than the one
below it. Approximate latencies (in CPU cycles, on a modern desktop CPU):

| Level         | Typical size | Latency        |
|---------------|--------------|-----------------|
| Register      | ~32 x 8B     | 0 (immediate)   |
| L1 cache      | 32-64 KB     | ~4 cycles       |
| L2 cache      | 256 KB-1 MB  | ~12 cycles      |
| L3 cache      | a few MB-tens of MB | ~40 cycles |
| Main memory (RAM) | GBs      | ~200 cycles     |
| SSD/disk      | TBs          | ~10,000-100,000+ cycles |

A cache **miss** that goes all the way to RAM can cost 50x what an L1 hit
costs. Code that's "doing less work" in terms of instruction count can still
be *slower* than code that's "doing more work" but accessing memory in a
cache-friendly pattern -- the CPU spends most of its time waiting, not
computing.

## Locality of reference

Caches work because real programs exhibit:

- **Temporal locality** -- if you access an address now, you're likely to
  access it again soon (e.g. a loop counter, a hot variable).
- **Spatial locality** -- if you access an address now, you're likely to
  access a *nearby* address soon (e.g. iterating over an array).

Caches exploit spatial locality by fetching memory in **cache lines** (fixed-size
blocks, commonly 64 bytes) -- touching one byte loads its whole line. Iterate
arrays in the order they're laid out in memory (row-major for C++'s
`std::vector<std::vector<T>>` or a flat array) to get this for free.

## Direct-mapped caches

A cache is organized into **lines** (slots), each holding one block of main
memory plus a **tag** identifying *which* block. The simplest mapping from
memory address to cache line is **direct-mapped**: each memory block maps to
exactly *one* possible line (as opposed to set-associative or fully
associative caches, which allow a block to live in several/any line).

For a byte address `a`, a cache with `numLines` lines of `lineSize` bytes
each:

```
blockNumber = a / lineSize         // which fixed-size block of memory
lineIndex   = blockNumber % numLines  // which cache line it must use
tag         = blockNumber / numLines  // which of the many blocks sharing
                                       // that line this one is
```

`access(a)`:
- If `lineIndex`'s line is valid and its stored tag matches `tag`: **hit**.
- Otherwise: **miss** -- load the block into that line (storing `tag`,
  marking valid), **evicting** whatever was there.

Direct-mapped caches are simple and fast to check (one comparison) but suffer
**conflict misses**: two frequently-used addresses that happen to map to the
same line repeatedly evict each other, even if the rest of the cache is
empty. This is why array sizes that are exact powers of two can sometimes be
*slower* -- every column of a matrix can land on the same line index.

## AoS vs SoA: data layout for cache utilization

**Array of Structs (AoS)**: one array, each element a struct with all of an
entity's fields:

```cpp
struct ParticleAoS { double x, y, z, vx, vy, vz, mass; };
std::vector<ParticleAoS> particles;
```

Reading just `mass` for every particle still pulls every `x/y/z/vx/vy/vz`
field into cache too -- wasted bandwidth, fewer useful values per cache line.

**Struct of Arrays (SoA)**: one array *per field*, all particles' values for
that field contiguous:

```cpp
struct ParticlesSoA {
    std::vector<double> x, y, z, vx, vy, vz, mass;
};
```

Now a loop over `mass` (or `vx`, `vy`, `vz` for a kinetic-energy
calculation) streams through *only* the bytes it needs -- every cache line
fetched is fully useful, and the access pattern is ideal for
auto-vectorization (SIMD instructions process several contiguous `double`s
per instruction). The tradeoff: SoA makes "give me everything about particle
`i`" require touching 7 different arrays instead of one struct -- AoS is
better when code processes whole entities; SoA is better for
field-at-a-time/bulk numeric code.

## Alignment and padding

Every type has an **alignment requirement** (`alignof(T)`, a power of two):
the address of any `T` object must be a multiple of `alignof(T)`. The
compiler inserts **padding** between struct members so each satisfies its own
alignment, which is why `sizeof(struct)` can exceed the sum of its members'
sizes. Mis-sized/misaligned data can split across cache lines (extra fetch)
or, for SIMD loads on some platforms, be outright illegal.

To round an offset `off` up to the next multiple of a power-of-two
`alignment`:

```cpp
size_t aligned = (off + alignment - 1) & ~(alignment - 1);
```

This works because for power-of-two `alignment`, `alignment - 1` is a mask
of low bits (e.g. `8 - 1 == 0b111`); adding `alignment - 1` then clearing
those bits rounds up to the next multiple. `<new>` provides
`operator new(size_t, std::align_val_t)` for over-aligned heap allocations,
and `std::align` (in `<memory>`) does the equivalent pointer-bumping for a
raw buffer.

## Bump-pointer (arena) allocators

A general-purpose allocator (`malloc`/`new`) does bookkeeping (free lists,
metadata headers) on every call -- overhead, and it scatters allocations
across memory (poor locality). An **arena** (bump-pointer allocator) instead
owns one big contiguous buffer and a single `offset`:

- `allocate(size, alignment)`: round `offset` up to `alignment`, hand back
  that address, advance `offset` by `size`. O(1), no metadata, and
  consecutive allocations are adjacent in memory (great spatial locality).
- `reset()`: set `offset` back to 0 -- frees *everything* at once. No
  per-object destructors run; arenas suit short-lived batches of
  trivially-destructible data (e.g. per-frame game engine allocations, or
  per-request server scratch space).
- The tradeoff is **no individual `free`** -- you can't reclaim one object's
  space without resetting the whole arena (or building a more complex
  allocator on top).

## Cache blocking (tiling)

For algorithms over 2D data (matrix transpose, matrix multiply, image
processing), naive loop orders can have terrible locality: transposing a
large matrix by columns means each column read jumps `rowLength * sizeof(T)`
bytes between consecutive elements -- one cache line per element touched, and
lines evicted before they're reused.

**Cache blocking (tiling)** processes the data in small `blockSize x
blockSize` tiles that fit in cache, finishing all the work for one tile
before moving to the next:

```cpp
for (size_t ii = 0; ii < rows; ii += blockSize) {
    for (size_t jj = 0; jj < cols; jj += blockSize) {
        // iMax/jMax clamp the last (partial) tile to the matrix bounds
        for (size_t i = ii; i < iMax; ++i) {
            for (size_t j = jj; j < jMax; ++j) {
                // ... work on m[i][j] ...
            }
        }
    }
}
```

Crucially, **the output doesn't depend on `blockSize`** -- tiling changes
only the *order* in which the same operations happen, trading some accesses'
locality for others'. `blockSize == 0` (or `1`) means "no blocking" (process
elements one at a time in row-major order) and must produce the exact same
result as any other `blockSize`.

For **matrix multiply** specifically, blocking the `i`/`j`/`k` loops lets a
tile of `A` and `B` stay resident in cache across many output elements before
being evicted -- each element of `A`/`B` is reused `O(blockSize)` times from
cache instead of being re-fetched from RAM. As long as the innermost loop
order over `k` is preserved for each fixed `(i, j)`, floating-point summation
order -- and therefore the result -- is identical regardless of `blockSize`.

## Further Reading

- [MCPP ch. 23 -- Performance: Computer Architecture and Memory Hierarchy](https://federico-busato.github.io/Modern-CPP-Programming/htmls/23.Performance_Computer_Architecture.html)
- [cppreference: Memory layout / object representation](https://en.cppreference.com/w/cpp/language/object)
- [cppreference: `alignof`](https://en.cppreference.com/w/cpp/language/alignof)
- [cppreference: `std::align`](https://en.cppreference.com/w/cpp/memory/align)
- [cppreference: `<new>` (alignment-aware allocation)](https://en.cppreference.com/w/cpp/header/new)
- [cppreference: `std::bad_alloc`](https://en.cppreference.com/w/cpp/memory/new/bad_alloc)
