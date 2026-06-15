#pragma once

#include <cmath>
#include <stdexcept>
#include <string>

// Topic 13 (Intermediate 03): Translation Units, Linkage & the ODR
//
// Unlike intermediate/01-02 (function/class templates, fully defined in the
// header), this topic returns to the normal split: declarations here in
// exercise.h, definitions in exercise.cpp. Two translation units --
// exercise.cpp and exercise_test.cpp -- both #include this header and are
// linked together (g++ ... exercise_test.cpp exercise.cpp && ./test),
// making this topic's build a live demonstration of the concepts it covers.

// --- nextId: function-local static (storage duration) -------------------------------------
//
// Returns a new, strictly increasing integer ID each call: if one call
// returns N, the next call returns N+1, and so on, starting from some
// N >= 1. Implement using a function-local `static` variable -- initialized
// once (the first time control passes through its declaration) and
// retaining its value across calls for the rest of the program's lifetime.
//
// Example: three consecutive calls return a, a+1, a+2 for some a >= 1.
int nextId();

// --- normalizeWhitespace: external linkage + an internal-linkage helper ---------------------
//
// Collapses every run of whitespace (space, tab, newline, etc.) in `text`
// into a single space, and trims leading/trailing whitespace. exercise.cpp
// defines a helper `isSpaceChar` in an unnamed namespace -- internal
// linkage, invisible outside that translation unit -- which
// normalizeWhitespace (externally linked, declared here) can use freely.
//
// Example: normalizeWhitespace("  hello \t world\n") == "hello world";
// normalizeWhitespace("") == ""; normalizeWhitespace("   ") == "";
// normalizeWhitespace("single") == "single".
std::string normalizeWhitespace(const std::string& text);

// --- extern global variable + accessors: declaration vs. definition -------------------------
//
// `globalRequestCount` is DECLARED here (`extern`: "this variable exists
// somewhere, with external linkage") and DEFINED exactly once, in
// exercise.cpp, as the ODR requires. Both exercise.cpp's functions AND
// exercise_test.cpp (which #includes this header) refer to the SAME
// variable.
//
//   recordRequest()    -- increments globalRequestCount by 1
//   getRequestCount()  -- returns its current value
//   resetRequestCount() -- sets it back to 0
extern int globalRequestCount;

void recordRequest();
int getRequestCount();
void resetRequestCount();

// --- nearlyEqual + kEpsilon: inline functions/variables defined in the header ----------------
//
// Both are fully DEFINED here (not just declared). Ordinarily, defining a
// non-template function in a header #include'd by multiple translation
// units (exercise.cpp AND exercise_test.cpp both #include "exercise.h")
// would violate the ODR: each TU would get its own definition, and the
// linker would see duplicate symbols ("multiple definition of
// `nearlyEqual'"). `inline` tells the linker "these definitions are
// guaranteed identical across TUs -- merge them into one". `kEpsilon` is
// `constexpr`, which already gives it internal linkage (each TU's copy is a
// distinct, non-conflicting symbol, so no ODR violation either way); `inline`
// on a variable (C++17) additionally guarantees a single shared instance
// across TUs, which matters if its address is ever taken.
//
// nearlyEqual(a, b) returns true iff |a - b| < kEpsilon.
//
// Example: nearlyEqual(0.1 + 0.2, 0.3) == true (despite floating-point
// rounding error in 0.1 + 0.2); nearlyEqual(1.0, 1.1) == false.
inline constexpr double kEpsilon = 1e-9;

inline bool nearlyEqual(double a, double b) {
    (void)a;
    (void)b;
    throw std::logic_error("not implemented");
}

// --- toRoman: external linkage + an internal-linkage lookup table ----------------------------
//
// Converts `value` to a Roman numeral string. Precondition:
// 1 <= value <= 3999. exercise.cpp defines a lookup table of (value, symbol)
// pairs in an unnamed namespace -- internal linkage, invisible outside that
// translation unit -- which toRoman (declared here) iterates over.
//
// Example: toRoman(1) == "I"; toRoman(4) == "IV"; toRoman(9) == "IX";
// toRoman(58) == "LVIII"; toRoman(1994) == "MCMXCIV".
std::string toRoman(int value);
