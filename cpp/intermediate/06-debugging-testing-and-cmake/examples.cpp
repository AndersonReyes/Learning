#include <cassert>
#include <iostream>
#include <type_traits>

// Topic 17-18 (Intermediate 06): Debugging, Sanitizers, Testing & CMake
//
// CMakeLists.txt (sibling to this file) is this topic's main new artifact --
// see notes.md for the build/sanitizer commands and what gdb/ASan/UBSan
// catch on buggy code (not reproduced here, since this file must run
// cleanly). This file covers what's left that's demonstrable in one TU:
// assert() + NDEBUG, static_assert, and __FILE__/__LINE__/__func__.

// --- assert(): a runtime check, compiled out when NDEBUG is defined ----------------------------
//
// assert(cond) does nothing if NDEBUG is defined (typical for release
// builds: cmake's "Release" preset adds -DNDEBUG). It's for catching
// programmer errors (violated preconditions) during development -- NOT for
// validating user input, which must still be checked in release builds too.
int divide(int a, int b) {
    assert(b != 0 && "divide: b must be non-zero");
    return a / b;
}

// --- static_assert: a compile-time check, zero runtime cost --------------------------------------
//
// Evaluated by the compiler; a failing static_assert is a compile error, not
// a runtime one. Useful for checking assumptions about types/sizes that must
// hold for the code to be correct at all.
static_assert(sizeof(int) >= 4, "this code assumes at least a 32-bit int");

template <typename T>
T half(T value) {
    static_assert(std::is_arithmetic_v<T>, "half<T> requires an arithmetic type");
    return value / 2;
}

// --- __FILE__ / __LINE__ / __func__: locating where you are -------------------------------------
//
// Predefined macros (__FILE__, __LINE__) and a predefined identifier
// (__func__) that expand to the current source location -- the same
// building blocks cpp/testing.h uses to report which CHECK failed and
// where.
void logLocation() {
    std::cout << __FILE__ << ":" << __LINE__ << " in " << __func__ << "()\n";
}

int main() {
    std::cout << "-- assert() (active unless NDEBUG is defined) --\n";
    std::cout << "divide(10, 2) = " << divide(10, 2) << "\n";
#ifdef NDEBUG
    std::cout << "NDEBUG is defined -- assert() is compiled out\n";
#else
    std::cout << "NDEBUG is NOT defined -- assert() is active\n";
#endif

    std::cout << "\n-- static_assert (compile-time, zero runtime cost) --\n";
    std::cout << "half(10) = " << half(10) << "\n";

    std::cout << "\n-- __FILE__/__LINE__/__func__ --\n";
    logLocation();
}
