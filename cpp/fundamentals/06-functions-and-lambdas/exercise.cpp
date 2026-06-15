#include "exercise.h"

// Each stub below returns a placeholder value so the program compiles and
// runs, but every assert() in exercise_test.cpp fails until you replace the
// body with a real implementation.

long long fibMemo(int n, std::vector<long long>& memo) {
    (void)n;
    (void)memo;
    return 0;
}

std::vector<int> mapWithFnPtr(const std::vector<int>& values, int (*fn)(int)) {
    (void)values;
    (void)fn;
    return {};
}

std::function<int(int)> composeFns(std::function<int(int)> f,
                                    std::function<int(int)> g) {
    (void)f;
    (void)g;
    return [](int) { return 0; };
}

std::function<long long()> makeCounter(long long start, long long step) {
    (void)start;
    (void)step;
    return []() { return 0LL; };
}

std::vector<TestResult> runTests(
    const std::vector<std::pair<std::string, std::function<void()>>>& tests) {
    (void)tests;
    return {};
}
