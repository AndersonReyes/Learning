#pragma once

#include <functional>
#include <string>
#include <utility>
#include <vector>

// Computes the nth Fibonacci number (fibMemo(0) == 0, fibMemo(1) == 1, ...)
// recursively, using `memo` as a memoization cache: memo[i] holds the
// already-computed value for index i, or -1 if not yet computed. `memo` must
// have size >= n + 1 on entry. Mutates `memo` in place, filling in every
// index from 0 to n that gets computed along the way (so a caller can reuse
// `memo` across calls). Example:
//   std::vector<long long> memo(51, -1);
//   fibMemo(50, memo) -> 12586269025
long long fibMemo(int n, std::vector<long long>& memo);

// Applies the function pointer `fn` to every element of `values`, returning
// a new vector of the results (same order, same size). Example:
//   int square(int x) { return x * x; }
//   mapWithFnPtr({1, 2, 3}, square) -> {1, 4, 9}
std::vector<int> mapWithFnPtr(const std::vector<int>& values, int (*fn)(int));

// Returns a new function equal to f(g(x)) -- calling the result with
// argument x first evaluates g(x), then passes that result to f. Neither f
// nor g is invoked by composeFns itself; the composition only runs when the
// returned function is called (and may be called any number of times).
// Example:
//   auto addOne = [](int x) { return x + 1; };
//   auto timesTwo = [](int x) { return x * 2; };
//   auto h = composeFns(timesTwo, addOne);
//   h(3) -> timesTwo(addOne(3)) -> timesTwo(4) -> 8
std::function<int(int)> composeFns(std::function<int(int)> f,
                                    std::function<int(int)> g);

// Returns a closure that, each time it is called, returns the current
// counter value and then advances the counter by `step` for next time. The
// first call returns `start`. Each call to makeCounter() produces an
// independent counter with its own state. Example:
//   auto c = makeCounter(10, 3);
//   c() -> 10
//   c() -> 13
//   c() -> 16
std::function<long long()> makeCounter(long long start, long long step);

// The outcome of running one named test.
struct TestResult {
    std::string name;
    bool passed;
    std::string message;  // empty if passed; the caught exception's what()
                           // if not
    bool operator==(const TestResult&) const = default;
};

// Runs each (name, thunk) pair in `tests`, in order, by invoking the thunk.
// If the thunk throws an exception derived from std::exception, the
// corresponding result has passed=false and message set to that
// exception's what(); if the thunk returns normally, passed=true and
// message is empty. A throwing test does not stop later tests from running.
// Returns one TestResult per input test, in input order. Example:
//   runTests({{"ok", []{}}, {"bad", []{ throw std::runtime_error("x"); }}})
//   -> {{"ok", true, ""}, {"bad", false, "x"}}
std::vector<TestResult> runTests(
    const std::vector<std::pair<std::string, std::function<void()>>>& tests);
