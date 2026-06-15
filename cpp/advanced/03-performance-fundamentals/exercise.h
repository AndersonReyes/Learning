#pragma once

#include <cstddef>
#include <new>
#include <stdexcept>
#include <vector>

// Topic 23 (Advanced 03): Performance Fundamentals: Architecture & Memory Hierarchy
//
// Five exercises: a direct-mapped cache simulator, a bump-pointer arena
// allocator, Array-of-Structs -> Struct-of-Arrays conversion, and two
// cache-blocking ("tiling") algorithms (matrix transpose and matrix
// multiply). Class member functions are declared here and defined in
// exercise.cpp. Stub bodies throw std::logic_error("not implemented").

// --- DirectMappedCache: simulating the memory hierarchy ------------------------------------------
//
// A direct-mapped cache simulator over a byte-addressed memory space, with
// `numLines` cache lines of `lineSize` bytes each (numLines > 0, lineSize >
// 0; need not be powers of two).
//
// For an address `a`:
//   blockNumber = a / lineSize
//   lineIndex   = blockNumber % numLines
//   tag         = blockNumber / numLines
//
// access(a):
//   - If line `lineIndex` is valid and its stored tag == `tag`: a HIT.
//   - Otherwise: a MISS. Line `lineIndex` is then loaded with `tag` (marked
//     valid), evicting whatever was there (if anything).
// Returns true on a hit, false on a miss. hits()/misses() are running
// totals across all access() calls (both start at 0).
//
// Example: DirectMappedCache c(2, 4);  // 2 lines x 4 bytes = 8-byte cache
// c.access(0)  -> false (miss; blockNumber=0, line=0, tag=0)
// c.access(4)  -> false (miss; blockNumber=1, line=1, tag=0)
// c.access(8)  -> false (miss; blockNumber=2, line=0, tag=1 != 0 -> evict)
// c.access(0)  -> false (miss; line=0 now holds tag=1, but tag=0 -> evict again)
// c.access(0)  -> true  (hit; line=0 now holds tag=0, matches)
// c.hits() == 1, c.misses() == 4
class DirectMappedCache {
public:
    DirectMappedCache(size_t numLines, size_t lineSize);
    bool access(size_t address);
    size_t hits() const;
    size_t misses() const;

private:
    struct Line {
        bool valid = false;
        size_t tag = 0;
    };
    std::vector<Line> lines_;
    size_t lineSize_ = 1;
    size_t hits_ = 0;
    size_t misses_ = 0;
};

// --- Arena: a bump-pointer allocator (contiguous, alignment-aware) -------------------------------
//
// A fixed-capacity arena ("bump") allocator backed by one contiguous
// buffer of `capacityBytes` bytes.
//
//   allocate(size, alignment) -- advances the arena's offset to the next
//     multiple of `alignment` (alignment is a power of two), then reserves
//     `size` bytes there, returning a pointer to them. Throws
//     std::bad_alloc -- and leaves bytesUsed() UNCHANGED -- if the
//     (alignment-padded) allocation would exceed capacity().
//   reset() -- rewinds the arena to empty (bytesUsed() == 0); the buffer's
//     contents and addresses are otherwise untouched, so the next
//     allocate() call reuses memory from the start.
//   bytesUsed() const -- current offset (including alignment padding
//     already consumed). Starts at 0.
//   capacity() const -- capacityBytes, fixed at construction.
//
// Example: Arena a(24);
// void* p1 = a.allocate(8, 8);  // offset 0 -> 8;  bytesUsed() == 8
// void* p2 = a.allocate(3, 1);  // offset 8 -> 11; bytesUsed() == 11
// void* p3 = a.allocate(8, 8);  // pad 11 -> 16, then -> 24; bytesUsed() == 24
// a.allocate(1, 1);              // would exceed 24 -> throws std::bad_alloc;
//                                 // bytesUsed() still 24
// a.reset();                      // bytesUsed() == 0
// a.allocate(8, 8) == p1          // bump pointer restarted from the same buffer
class Arena {
public:
    explicit Arena(size_t capacityBytes);
    void* allocate(size_t size, size_t alignment);
    void reset();
    size_t bytesUsed() const;
    size_t capacity() const;

private:
    std::vector<unsigned char> buffer_;
    size_t offset_ = 0;
};

// --- ParticleAoS / ParticlesSoA / toSoA / totalKineticEnergy: AoS -> SoA layout -------------------
//
// ParticleAoS is the "Array of Structs" layout: one struct per particle,
// all its fields adjacent in memory. ParticlesSoA is the "Struct of
// Arrays" layout: one array per FIELD, all particles' values for that
// field adjacent -- the layout that lets SIMD/vectorized code stream
// through (say) every `mass` value without touching `x`/`y`/`z`/...
// cache lines at all.
//
//   toSoA(particles) -- converts an AoS vector to the equivalent SoA form
//     (same length, same per-particle values, reorganized by field).
//   totalKineticEnergy(particles) -- sum over all particles of
//     0.5 * mass * (vx^2 + vy^2 + vz^2). 0 for an empty ParticlesSoA.
//
// Example: particles = [{x:0,y:0,z:0, vx:1,vy:0,vz:0, mass:2},
//                        {x:1,y:1,z:1, vx:0,vy:3,vz:4, mass:1}]
// toSoA(particles).vx == {1, 0}; toSoA(particles).mass == {2, 1}
// totalKineticEnergy(toSoA(particles)) == 0.5*2*(1^2) + 0.5*1*(3^2+4^2)
//                                       == 1.0 + 12.5 == 13.5
struct ParticleAoS {
    double x, y, z;
    double vx, vy, vz;
    double mass;
};

struct ParticlesSoA {
    std::vector<double> x, y, z;
    std::vector<double> vx, vy, vz;
    std::vector<double> mass;
};

ParticlesSoA toSoA(const std::vector<ParticleAoS>& particles);

double totalKineticEnergy(const ParticlesSoA& particles);

// --- transposeBlocked: cache-blocking (tiling) for 2D access patterns ----------------------------
//
// Returns the transpose of `m` (an R x C matrix: R rows, each a vector of
// the same length C), computed using blockSize x blockSize tiles -- the
// cache-blocking/tiling technique for improving spatial locality on 2D
// array traversals. The OUTPUT does not depend on blockSize, only the
// access pattern does -- blockSize need not evenly divide R or C (partial
// tiles at the edges are handled normally), and blockSize == 0 is treated
// as 1 (no blocking). Throws std::invalid_argument if `m`'s rows don't all
// have the same length. Returns an empty matrix (0 rows) if `m` is empty.
//
// Example: transposeBlocked({{1,2},{3,4},{5,6}}, 2) == {{1,3,5},{2,4,6}}
// (a 3x2 matrix transposed to 2x3) -- same result for any blockSize >= 0.
std::vector<std::vector<int>> transposeBlocked(const std::vector<std::vector<int>>& m,
                                                 size_t blockSize);

// --- multiplyBlocked: cache-blocking (tiling) for compute-bound kernels ---------------------------
//
// Returns A * B (standard matrix multiplication) computed using blockSize
// x blockSize x blockSize tiles over the i/j/k loops -- the cache-blocking
// technique for compute-bound kernels (re-using each tile of A/B from
// cache across multiple output elements before it's evicted). A is R x K
// (R rows of length K); B is K x C (K rows of length C); the result is R x
// C. The OUTPUT does not depend on blockSize -- blockSize == 0 is treated
// as 1 (no blocking). Throws std::invalid_argument if A's rows aren't all
// the same length, B's rows aren't all the same length, or A's row length
// != B's row count (i.e. A's columns != B's rows).
//
// Example: A = {{1,2,3},{4,5,6}} (2x3), B = {{7,8},{9,10},{11,12}} (3x2)
// multiplyBlocked(A, B, 1) == {{58,64},{139,154}} -- same result for any
// blockSize >= 0.
std::vector<std::vector<double>> multiplyBlocked(const std::vector<std::vector<double>>& A,
                                                   const std::vector<std::vector<double>>& B,
                                                   size_t blockSize);
