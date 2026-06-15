#pragma once

#include <stdexcept>
#include <string>
#include <type_traits>
#include <utility>

// Topic 11 (Intermediate 01): Function Templates & Compile-Time Utilities
//
// Function templates must be fully defined where they're used, so unlike
// every previous topic, the definitions (not just declarations) live here in
// exercise.h -- exercise.cpp has nothing to add. Stub bodies still
// `throw std::logic_error("not implemented")`, caught at runtime by
// TEST_MAIN() exactly like every topic since fundamentals/07 (exercise_test.cpp
// never forces compile-time evaluation of these stubs, e.g. via static_assert,
// so a throwing body is fine even on a `constexpr` function -- see power<N>).

// --- clampValue: single type parameter, deduced from all three arguments ------------------
//
// Returns `value` restricted to the closed range [low, high]: `low` if
// value < low, `high` if value > high, otherwise `value` itself.
// Precondition: low <= high. Works for any T with operator< (int, double,
// std::string, ...).
//
// Example: clampValue(5, 0, 10) == 5; clampValue(-3, 0, 10) == 0; clampValue(15, 0, 10) == 10.
template <typename T>
T clampValue(const T& value, const T& low, const T& high) {
    (void)value;
    (void)low;
    (void)high;
    throw std::logic_error("not implemented");
}

// --- addValues: two type parameters, return type deduced via decltype ----------------
//
// Returns a + b. T and U are deduced independently from the two arguments
// (they need not match), and the return type is `decltype(a + b)` -- e.g.
// addValues(2, 3.5) returns a double (5.5), even though T=int and U=double.
template <typename T, typename U>
auto addValues(T a, U b) -> decltype(std::declval<T>() + std::declval<U>()) {
    (void)a;
    (void)b;
    throw std::logic_error("not implemented");
}

// --- power: non-type template parameter + constexpr -----------------------------------
//
// Returns base^N, where N (>= 0) is a non-type template parameter (a
// compile-time constant, not a function argument). power<0>(x) == 1 for any
// x, including x == 0 (0^0 == 1 by convention). `constexpr` means a fully
// implemented power<N> CAN be evaluated at compile time (e.g. in
// static_assert or as an array bound) when called with a constant `base`.
//
// Example: power<3>(2) == 8; power<0>(100) == 1; power<10>(2) == 1024.
template <int N>
constexpr long long power(long long base) {
    (void)base;
    throw std::logic_error("not implemented");
}

// --- sumAll: variadic template + fold expression ---------------------------------------
//
// Returns the sum of all arguments, each converted to long long via
// static_cast (truncating toward zero) before summing. sumAll() with no
// arguments returns 0.
//
// Example: sumAll(1, 2, 3) == 6; sumAll(1, 2.9, 3) == 6 (2.9 -> 2);
// sumAll() == 0.
template <typename... Args>
long long sumAll(Args... args) {
    (void)sizeof...(args);
    throw std::logic_error("not implemented");
}

// --- typeCategory: if constexpr + type traits ------------------------------------------
//
// Classifies `value` at compile time via `if constexpr` and <type_traits>,
// returning:
//   - "integral:even" / "integral:odd"               if T is an integral type
//   - "floating-point:whole" / "floating-point:fractional" if T is floating-point
//   - "other"                                          otherwise
//
// `if constexpr` discards the branches that don't apply to T at compile
// time -- e.g. `value % 2` (used only in the integral branch) never needs to
// compile for T = std::string.
template <typename T>
std::string typeCategory(const T& value) {
    (void)value;
    throw std::logic_error("not implemented");
}
