#pragma once

#include <optional>
#include <string>
#include <vector>

// Returns true if `a + b` would overflow `int` (i.e. the mathematical
// result is outside [INT_MIN, INT_MAX]), WITHOUT computing `a + b` itself
// (which would be undefined behavior if it overflows). Examples:
//   willAddOverflow(1, 2)                    -> false
//   willAddOverflow(INT_MAX, 1)              -> true
//   willAddOverflow(INT_MIN, -1)             -> true
bool willAddOverflow(int a, int b);

// Computes a / b, returning std::nullopt for the two cases where integer
// division is invalid or overflows:
//   - b == 0 (division by zero, UB for integers)
//   - a == INT_MIN && b == -1 (the mathematical result, -INT_MIN, is one
//     more than INT_MAX and isn't representable as int)
// Otherwise returns a / b (truncated toward zero). Examples:
//   safeDivide(7, 2)        -> 3
//   safeDivide(-7, 2)       -> -3
//   safeDivide(10, 0)       -> std::nullopt
//   safeDivide(INT_MIN, -1) -> std::nullopt
std::optional<int> safeDivide(int a, int b);

// Returns a + b, clamped to UINT_MAX if the addition would overflow
// `unsigned int`. Implement using unsigned wraparound (well-defined) to
// detect the overflow, not by checking bounds beforehand. Examples:
//   saturatingAdd(5u, 10u)                 -> 15u
//   saturatingAdd(UINT_MAX, 1u)            -> UINT_MAX
//   saturatingAdd(4000000000u, 1000000000u) -> UINT_MAX
unsigned int saturatingAdd(unsigned int a, unsigned int b);

// Classifies a double using <cmath>, returning one of:
//   "nan"    - x is NaN (std::isnan)
//   "+inf"   - x is positive infinity
//   "-inf"   - x is negative infinity
//   "+zero"  - x is +0.0
//   "-zero"  - x is -0.0 (distinguish via std::signbit; +0.0 == -0.0 but
//              they have different bit patterns)
//   "normal" - anything else
std::string classifyFloat(double x);

// Sums `values` using Kahan compensated summation, which tracks and corrects
// for the rounding error of each addition -- far more accurate than a plain
// running sum for long lists of similar-magnitude values. Returns 0.0 for an
// empty vector. Example:
//   kahanSum(std::vector<double>(10000, 0.1)) -> very close to 1000.0,
//   noticeably closer than a naive `for (double v : values) sum += v;`
double kahanSum(const std::vector<double>& values);
