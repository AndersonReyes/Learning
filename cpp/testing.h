// Minimal header-only test framework shared by every topic's
// exercise_test.cpp. Zero external dependencies: a TEST() registers a
// function via a static initializer, CHECK()/CHECK_EQ() record failures
// without aborting, and TEST_MAIN() expands to a main() that runs every
// registered test and reports PASS/FAIL per test plus a summary line.
#pragma once

#include <cstdio>
#include <functional>
#include <string>
#include <vector>

namespace testing {

struct Test {
    std::string name;
    std::function<void()> fn;
};

inline std::vector<Test>& registry() {
    static std::vector<Test> tests;
    return tests;
}

struct Registrar {
    Registrar(const char* name, std::function<void()> fn) {
        registry().push_back({name, std::move(fn)});
    }
};

inline int& failure_count() {
    static int n = 0;
    return n;
}

inline void check(bool cond, const char* expr, const char* file, int line) {
    if (!cond) {
        std::fprintf(stderr, "  FAIL %s:%d: CHECK(%s)\n", file, line, expr);
        ++failure_count();
    }
}

template <typename A, typename B>
void check_eq(const A& a, const B& b, const char* expr_a, const char* expr_b,
               const char* file, int line) {
    if (!(a == b)) {
        std::fprintf(stderr, "  FAIL %s:%d: CHECK_EQ(%s, %s)\n", file, line, expr_a, expr_b);
        ++failure_count();
    }
}

} // namespace testing

#define TEST(name)                                                            \
    void name();                                                              \
    static ::testing::Registrar registrar_##name(#name, name);               \
    void name()

#define CHECK(cond) ::testing::check((cond), #cond, __FILE__, __LINE__)
#define CHECK_EQ(a, b) ::testing::check_eq((a), (b), #a, #b, __FILE__, __LINE__)

// Expands to a main() that runs every TEST() registered in this translation
// unit, printing PASS/FAIL per test and a final "<passed>/<total>" summary.
// Exits 0 only if every test passed. Call exactly once per exercise_test.cpp.
#define TEST_MAIN()                                                           \
    int main() {                                                              \
        int passed = 0;                                                      \
        int total = 0;                                                       \
        for (auto& t : ::testing::registry()) {                              \
            int before = ::testing::failure_count();                         \
            t.fn();                                                          \
            ++total;                                                         \
            bool ok = (::testing::failure_count() == before);                \
            if (ok) ++passed;                                                \
            std::printf("%s %s\n", ok ? "PASS" : "FAIL", t.name.c_str());    \
        }                                                                     \
        std::printf("\n%d/%d tests passed\n", passed, total);                \
        return passed == total ? 0 : 1;                                      \
    }
