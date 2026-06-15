// Spec for exercise.h/exercise.cpp. No test framework yet (this topic builds
// one, cpp/testing.h, but exercise_test.cpp here still uses plain assert()).
// Compile and run:
//
//   g++ -std=c++20 -Wall -Wextra -o /tmp/test exercise_test.cpp exercise.cpp && /tmp/test
//
// Every assert() must pass. If one fails, the program aborts and prints the
// failing file:line -- fix exercise.cpp and re-run.

#include "exercise.h"

#include <cassert>
#include <iostream>
#include <stdexcept>
#include <vector>

static int square(int x) { return x * x; }
static int negateInt(int x) { return -x; }

static void testFibMemo() {
    std::vector<long long> memo1(2, -1);
    assert(fibMemo(1, memo1) == 1);

    std::vector<long long> memo10(11, -1);
    assert(fibMemo(10, memo10) == 55);
    assert(memo10[10] == 55);

    std::vector<long long> memo50(51, -1);
    assert(fibMemo(50, memo50) == 12586269025LL);
    assert(memo50[50] == 12586269025LL);
    assert(memo50[1] == 1);  // intermediate values get filled in along the way

    std::vector<long long> memo0(1, -1);
    assert(fibMemo(0, memo0) == 0);
}

static void testMapWithFnPtr() {
    assert((mapWithFnPtr({1, 2, 3}, square) == std::vector<int>{1, 4, 9}));
    assert((mapWithFnPtr({1, -2, 3}, negateInt) == std::vector<int>{-1, 2, -3}));
    assert(mapWithFnPtr({}, square).empty());
}

static void testComposeFns() {
    auto addOne = [](int x) { return x + 1; };
    auto timesTwo = [](int x) { return x * 2; };

    auto h = composeFns(timesTwo, addOne);
    assert(h(3) == 8);  // timesTwo(addOne(3)) == timesTwo(4) == 8

    auto h2 = composeFns(addOne, timesTwo);
    assert(h2(3) == 7);  // addOne(timesTwo(3)) == addOne(6) == 7

    auto h3 = composeFns(h, addOne);  // compose an already-composed function
    assert(h3(3) == 10);              // h(addOne(3)) == h(4) == 10
    assert(h(3) == 8);                // h itself is unchanged
}

static void testMakeCounter() {
    auto c1 = makeCounter(10, 3);
    assert(c1() == 10);
    assert(c1() == 13);

    auto c2 = makeCounter(0, 1);  // independent state from c1
    assert(c2() == 0);

    assert(c1() == 16);
    assert(c2() == 1);
}

static void testRunTests() {
    std::vector<std::pair<std::string, std::function<void()>>> tests = {
        {"pass1", [] {}},
        {"throws-runtime", [] { throw std::runtime_error("boom"); }},
        {"pass2", [] { int x = 1 + 1; (void)x; }},
        {"throws-logic", [] { throw std::logic_error("not implemented"); }},
    };

    auto results = runTests(tests);
    assert(results.size() == 4);
    assert((results[0] == TestResult{"pass1", true, ""}));
    assert(results[1].name == "throws-runtime");
    assert(results[1].passed == false);
    assert(results[1].message == "boom");
    assert((results[2] == TestResult{"pass2", true, ""}));
    assert(results[3].name == "throws-logic");
    assert(results[3].passed == false);
    assert(results[3].message == "not implemented");
}

int main() {
    testFibMemo();
    testMapWithFnPtr();
    testComposeFns();
    testMakeCounter();
    testRunTests();
    std::cout << "All tests passed!\n";
    return 0;
}
