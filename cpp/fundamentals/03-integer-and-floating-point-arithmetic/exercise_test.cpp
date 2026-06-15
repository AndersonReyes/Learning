// Spec for exercise.h/exercise.cpp. No test framework yet (that's built in
// fundamentals/06) -- just assert(). Compile and run:
//
//   g++ -std=c++20 -Wall -Wextra -o /tmp/test exercise_test.cpp exercise.cpp && /tmp/test
//
// Every assert() must pass. If one fails, the program aborts and prints the
// failing file:line -- fix exercise.cpp and re-run.

#include "exercise.h"

#include <cassert>
#include <cmath>
#include <iostream>
#include <limits>

static void testWillAddOverflow() {
    assert(willAddOverflow(std::numeric_limits<int>::max(), 1) == true);
    assert(willAddOverflow(1, 2) == false);
    assert(willAddOverflow(std::numeric_limits<int>::max(), 0) == false);
    assert(willAddOverflow(std::numeric_limits<int>::min(), -1) == true);
    assert(willAddOverflow(std::numeric_limits<int>::min(), 0) == false);
    assert(willAddOverflow(std::numeric_limits<int>::max(), -1) == false);
    assert(willAddOverflow(std::numeric_limits<int>::min(), 1) == false);
    assert(willAddOverflow(-5, -3) == false);
    assert(willAddOverflow(std::numeric_limits<int>::min(),
                            std::numeric_limits<int>::min()) == true);
    assert(willAddOverflow(std::numeric_limits<int>::max(),
                            std::numeric_limits<int>::max()) == true);
}

static void testSafeDivide() {
    assert(safeDivide(7, 2) == std::optional<int>(3));
    assert(safeDivide(-7, 2) == std::optional<int>(-3));
    assert(safeDivide(10, 2) == std::optional<int>(5));
    assert(safeDivide(0, 5) == std::optional<int>(0));
    assert(safeDivide(10, 0) == std::nullopt);
    assert(safeDivide(std::numeric_limits<int>::min(), -1) == std::nullopt);
    assert(safeDivide(std::numeric_limits<int>::min(), 1) ==
           std::optional<int>(std::numeric_limits<int>::min()));
}

static void testSaturatingAdd() {
    assert(saturatingAdd(5u, 10u) == 15u);
    assert(saturatingAdd(0u, 0u) == 0u);
    assert(saturatingAdd(std::numeric_limits<unsigned int>::max(), 1u) ==
           std::numeric_limits<unsigned int>::max());
    assert(saturatingAdd(std::numeric_limits<unsigned int>::max(), 0u) ==
           std::numeric_limits<unsigned int>::max());
    assert(saturatingAdd(std::numeric_limits<unsigned int>::max(),
                          std::numeric_limits<unsigned int>::max()) ==
           std::numeric_limits<unsigned int>::max());
    assert(saturatingAdd(4000000000u, 1000000000u) ==
           std::numeric_limits<unsigned int>::max());
    assert(saturatingAdd(2000000000u, 2000000000u) == 4000000000u);
}

static void testClassifyFloat() {
    assert(classifyFloat(std::numeric_limits<double>::quiet_NaN()) == "nan");
    assert(classifyFloat(std::numeric_limits<double>::infinity()) == "+inf");
    assert(classifyFloat(-std::numeric_limits<double>::infinity()) == "-inf");
    assert(classifyFloat(0.0) == "+zero");
    assert(classifyFloat(-0.0) == "-zero");
    assert(classifyFloat(3.14) == "normal");
    assert(classifyFloat(-3.14) == "normal");
    assert(classifyFloat(1.0) == "normal");
}

static void testKahanSum() {
    assert(kahanSum({}) == 0.0);
    assert(kahanSum({1.0, 2.0, 3.0, 4.0}) == 10.0);

    std::vector<double> many(10000, 0.1);
    double kahan = kahanSum(many);

    double naive = 0.0;
    for (double v : many) {
        naive += v;
    }

    // Exact mathematical answer is 1000.0. Kahan summation should land
    // noticeably closer to it than plain left-to-right summation.
    assert(std::abs(kahan - 1000.0) < std::abs(naive - 1000.0));
    assert(std::abs(kahan - 1000.0) < 1e-9);
}

int main() {
    testWillAddOverflow();
    testSafeDivide();
    testSaturatingAdd();
    testClassifyFloat();
    testKahanSum();
    std::cout << "All tests passed!\n";
    return 0;
}
