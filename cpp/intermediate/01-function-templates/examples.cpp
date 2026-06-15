#include <iostream>
#include <string>
#include <type_traits>
#include <utility>

// Topic 11 (Intermediate 01): Function Templates & Compile-Time Utilities
//
// Different illustrative examples than exercise.h's clampValue/addValues/
// power/sumAll/typeCategory -- same concepts, different functions, so the
// exercises stay unspoiled.

// --- Basic function template + argument deduction --------------------------------------

template <typename T>
const T& maxOf(const T& a, const T& b) {
    return (a < b) ? b : a;
}

// --- Trailing return type via decltype + std::declval ------------------------------------

template <typename T, typename U>
auto multiply(T a, U b) -> decltype(std::declval<T>() * std::declval<U>()) {
    return a * b;
}

// --- Non-type template parameter + constexpr (with a real static_assert) -----------------

template <int N>
constexpr long long factorial() {
    if constexpr (N <= 1) {
        return 1;
    } else {
        return N * factorial<N - 1>();
    }
}

// Evaluated entirely at compile time -- no runtime cost, and unlike the
// throwing stubs in exercise.h, this fully-implemented constexpr function CAN
// be checked with static_assert.
static_assert(factorial<0>() == 1);
static_assert(factorial<5>() == 120);
static_assert(factorial<10>() == 3628800);

// --- Variadic templates + fold expressions -------------------------------------------------

// Right fold over &&: true only if every argument is truthy.
template <typename... Args>
bool allTrue(Args... args) {
    return (args && ...);
}

// Right fold over ||: true if any argument is truthy.
template <typename... Args>
bool anyTrue(Args... args) {
    return (args || ...);
}

// Fold expression using the comma operator to print every argument.
template <typename... Args>
void printAll(const Args&... args) {
    ((std::cout << args << ' '), ...);
    std::cout << '\n';
}

// --- if constexpr + <type_traits> -----------------------------------------------------------

template <typename T>
std::string describeNumeric(T value) {
    if constexpr (std::is_integral_v<T>) {
        return "integral: " + std::to_string(value);
    } else if constexpr (std::is_floating_point_v<T>) {
        return "floating-point: " + std::to_string(value);
    } else {
        return "non-numeric";
    }
}

int main() {
    std::cout << "-- maxOf (argument deduction) --\n";
    std::cout << "maxOf(3, 7) = " << maxOf(3, 7) << "\n";
    std::cout << "maxOf(3.5, 2.5) = " << maxOf(3.5, 2.5) << "\n";
    std::cout << "maxOf(string(\"abc\"), string(\"abd\")) = "
              << maxOf(std::string("abc"), std::string("abd")) << "\n";

    std::cout << "\n-- multiply (decltype/declval trailing return type) --\n";
    std::cout << "multiply(2, 3) = " << multiply(2, 3) << "\n";
    std::cout << "multiply(2, 3.5) = " << multiply(2, 3.5) << "\n";
    std::cout << "multiply(2.5, 4) = " << multiply(2.5, 4) << "\n";

    std::cout << "\n-- factorial<N> (constexpr, verified via static_assert above) --\n";
    std::cout << "factorial<5>() = " << factorial<5>() << "\n";
    std::cout << "factorial<10>() = " << factorial<10>() << "\n";

    std::cout << "\n-- fold expressions --\n";
    std::cout << std::boolalpha;
    std::cout << "allTrue(true, true, true) = " << allTrue(true, true, true) << "\n";
    std::cout << "allTrue(true, false, true) = " << allTrue(true, false, true) << "\n";
    std::cout << "anyTrue(false, false, true) = " << anyTrue(false, false, true) << "\n";
    std::cout << "anyTrue(false, false, false) = " << anyTrue(false, false, false) << "\n";
    std::cout << "printAll(1, \"two\", 3.0): ";
    printAll(1, "two", 3.0);

    std::cout << "\n-- if constexpr + <type_traits> --\n";
    std::cout << "describeNumeric(42) = " << describeNumeric(42) << "\n";
    std::cout << "describeNumeric(3.14) = " << describeNumeric(3.14) << "\n";
    std::cout << "describeNumeric(string(\"hi\")) = " << describeNumeric(std::string("hi")) << "\n";
}
