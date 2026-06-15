#pragma once

#include <array>
#include <cstddef>
#include <stdexcept>
#include <unordered_map>
#include <vector>

// Topic 24 (Advanced 04): Code-Level Optimization Techniques
//
// Five exercises: branchless arithmetic primitives, bit-manipulation tricks
// (population count + bit reversal), single-pass ("loop fusion") summary
// statistics, memoization with call counting, and a fixed-size branchless
// sorting network. Free functions/classes are declared here and defined in
// exercise.cpp. Stub bodies throw std::logic_error("not implemented").

// --- branchlessAbs / branchlessMin / branchlessMax: branchless arithmetic -----------------------
//
// Implement WITHOUT `if`, `?:`, `std::abs`, `std::min`, or `std::max` --
// using only bitwise operators (`^`, `&`, `~`), arithmetic (`+`, `-`),
// comparisons (`<`), and casts. The goal is code whose control flow doesn't
// depend on the *values* being compared -- a compiler can lower it to
// branch-free instructions (e.g. `cmov`), avoiding branch-misprediction
// stalls in hot loops.
//
// branchlessAbs(x): the absolute value of `x`. As with std::abs, INT_MIN is
// a special case: -INT_MIN is not representable as int, so
// branchlessAbs(INT_MIN) == INT_MIN (the bit pattern is unchanged) -- same
// "wraps back to itself" behavior as std::abs on two's-complement hardware.
// Hint: in C++20, right-shifting a negative int is a well-defined arithmetic
// (sign-extending) shift, and converting an out-of-range value to/from an
// unsigned type wraps modulo 2^N (also well-defined) -- combine these to
// avoid signed-overflow UB entirely.
//
// branchlessMin(a, b) / branchlessMax(a, b): the smaller/larger of `a` and
// `b` (ties return either, since they're equal).
//
// Example: branchlessAbs(5) == 5; branchlessAbs(-5) == 5;
// branchlessAbs(INT_MIN) == INT_MIN (the one exception to "always
// non-negative"). branchlessMin(3, -2) == -2; branchlessMax(3, -2) == 3;
// branchlessMin(7, 7) == 7.
int branchlessAbs(int x);
int branchlessMin(int a, int b);
int branchlessMax(int a, int b);

// --- countSetBits / reverseBits: bit-manipulation tricks ------------------------------------------
//
// countSetBits(x): the number of 1 bits in `x` (population count / Hamming
// weight), computed via Kernighan's trick -- repeatedly clear the lowest set
// bit with `x &= x - 1` -- which runs in O(popcount) iterations rather than
// O(32) (no need to test every bit position).
//
// reverseBits(x): `x` with its 32 bits in reversed order (bit 0 <-> bit 31,
// bit 1 <-> bit 30, ...). Used in algorithms like FFT (bit-reversal
// permutation) where reordering by reversed index avoids a separate sort/
// scatter pass.
//
// Example: countSetBits(0) == 0; countSetBits(0xFFFFFFFFu) == 32;
// countSetBits(0x12345678u) == 13 (popcount of 0001 0010 0011 0100 0101
// 0110 0111 1000).
// reverseBits(0u) == 0u; reverseBits(1u) == 0x80000000u;
// reverseBits(0x80000000u) == 1u; reverseBits(reverseBits(x)) == x for any x.
unsigned int countSetBits(unsigned int x);
unsigned int reverseBits(unsigned int x);

// --- computeStats: single-pass ("loop fusion") summary statistics ---------------------------------
//
// Returns sum, mean, min, max, and population variance of `data`, computed
// in ONE pass over the vector (not one pass per statistic) -- "loop fusion":
// combining several traversals that each touch every element into a single
// traversal, so the data streams through cache once instead of
// `numStats` times.
//
// variance is the POPULATION variance: mean of squared deviations from the
// mean, i.e. (1/n) * sum((x_i - mean)^2) -- equivalently
// (1/n) * sum(x_i^2) - mean^2, computable from running sums of x and x^2
// without a second pass.
//
// Throws std::invalid_argument if `data` is empty (mean/variance undefined).
//
// Example: computeStats({2, 4, 4, 4, 5, 5, 7, 9}) == {sum: 40, mean: 5,
// min: 2, max: 9, variance: 4} (a classic textbook example: every value's
// squared deviation from the mean 5 is 9, 1, 1, 1, 0, 0, 4, 16 -- average
// 32/8 == 4). computeStats({5}) == {sum: 5, mean: 5, min: 5, max: 5,
// variance: 0}.
struct Stats {
    double sum;
    double mean;
    double min;
    double max;
    double variance;
};

Stats computeStats(const std::vector<double>& data);

// --- StepCounter: memoization with call counting --------------------------------------------------
//
// countWays(n) returns the number of ways to climb `n` stairs taking 1, 2,
// or 3 steps at a time (the "tribonacci-like" recurrence
// ways(n) = ways(n-1) + ways(n-2) + ways(n-3) for n >= 3, with ways(0) ==
// ways(1) == 1, ways(2) == 2, and ways(k) == 0 for k < 0). Naive recursion
// is exponential; memoize each distinct `n`'s result within the instance so
// it's computed at most once.
//
// computeCount() returns how many DISTINCT values of n have actually been
// computed (cache misses) across all countWays() calls on this instance so
// far -- looking up an already-memoized value (a cache hit, whether it's the
// top-level argument or a recursive sub-call) must NOT increase it. Starts
// at 0. Negative n (which always returns 0 by definition, never computed/
// memoized) never increases it either.
//
// Example: StepCounter sc; sc.countWays(10) == 274, and afterward
// sc.computeCount() == 11 (n=0..10, each computed exactly once -- the
// recursive calls for smaller n are sub-computations of computing n=10).
// A second sc.countWays(10) call returns 274 again without changing
// computeCount() (still 11). sc.countWays(3) == 4; sc.countWays(0) == 1.
class StepCounter {
public:
    long long countWays(int n);
    size_t computeCount() const;

private:
    std::unordered_map<int, long long> memo_;
    size_t computeCount_ = 0;
};

// --- sortNetwork4: branchless fixed-size sorting network -------------------------------------------
//
// Returns `values` sorted in ascending order, using a fixed SORTING
// NETWORK -- a hardcoded sequence of compare-exchange ("compare 2 elements,
// put the smaller first") operations whose pattern does NOT depend on the
// input values (only their comparisons do) -- no std::sort, no loops whose
// trip count depends on the data. This is the technique behind
// branch-predictor-friendly fixed-size sorts (e.g. sorting 4 SIMD lanes).
//
// The minimal network for 4 elements uses 5 compare-exchanges, applied to
// these (0-based) index pairs in order: (0,1), (2,3), (0,2), (1,3), (1,2).
// Each compare-exchange(i, j) ensures result[i] <= result[j] afterward
// (swapping if needed) -- implement it with branchlessMin/branchlessMax.
//
// Example: sortNetwork4({4, 3, 2, 1}) == {1, 2, 3, 4};
// sortNetwork4({2, 2, 1, 1}) == {1, 1, 2, 2};
// sortNetwork4({1, 2, 3, 4}) == {1, 2, 3, 4} (already sorted).
std::array<int, 4> sortNetwork4(std::array<int, 4> values);
