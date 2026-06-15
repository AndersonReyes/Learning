#pragma once

// A minimal header-only test framework, built in fundamentals/06 as an
// application of that topic's concepts (function pointers, std::function,
// lambdas, closures, and the preprocessor). Used by exercise_test.cpp from
// fundamentals/07 onward.
//
// Usage:
//
//   #include "../../testing.h"
//
//   TEST(AdditionWorks) {
//       CHECK(1 + 1 == 2);
//   }
//
//   TEST(StubThrows) {
//       CHECK(someFunction() == 42);  // CHECK can also catch a thrown
//                                       // std::logic_error("not implemented")
//   }
//
//   TEST_MAIN()
//
// Each TEST(name) { ... } registers a test (by name) in a global registry at
// static-initialization time. TEST_MAIN() expands to a main() that runs every
// registered test in registration order, catching any exception derived from
// std::exception so one failing test can't abort the rest, and prints a
// [PASS]/[FAIL] line per test plus a summary.

#include <exception>
#include <functional>
#include <iostream>
#include <string>
#include <vector>

namespace testing {

// One registered test: its name and its body (a thunk -- a callable that
// takes no arguments and returns nothing).
struct Test {
    std::string name;
    std::function<void()> body;
};

// The global test registry. A function-local static (rather than a plain
// global variable) guarantees it's initialized before its first use,
// regardless of which translation unit's static Registrar runs first --
// avoiding the "static initialization order fiasco".
inline std::vector<Test>& registry() {
    static std::vector<Test> tests;
    return tests;
}

// Constructing a Registrar appends one Test to the registry. TEST(name)
// declares a static Registrar so this runs once, automatically, before
// main() -- this is how each TEST block "registers itself".
struct Registrar {
    Registrar(std::string name, std::function<void()> body) {
        registry().push_back({std::move(name), std::move(body)});
    }
};

// Thrown by CHECK() on failure. Carries a message describing the failed
// condition, including its source location.
struct CheckFailure : std::exception {
    std::string message;
    explicit CheckFailure(std::string msg) : message(std::move(msg)) {}
    const char* what() const noexcept override { return message.c_str(); }
};

}  // namespace testing

// Defines one test named `name`. Expands to a forward declaration, a static
// Registrar that registers `name` (as both a string, via stringification
// with `#`, and a function pointer -- implicitly convertible to
// std::function<void()>), and the opening of the function body (the `{ ... }`
// written after the macro is the test's body).
//
// `##` (token pasting) builds a unique identifier per test, e.g.
// TEST(AdditionWorks) declares `registrar_AdditionWorks` -- so multiple
// TEST(...) blocks in one file don't collide.
#define TEST(name)                                                   \
    void name();                                                     \
    static ::testing::Registrar registrar_##name(#name, name);       \
    void name()

// Checks that `cond` is true; if not, throws testing::CheckFailure with the
// condition's source text (via stringification, `#cond`) and its
// __FILE__:__LINE__. Wrapped in `do { ... } while (false)` so CHECK(...)
// behaves like a single statement (safe inside an unbraced `if`, etc.).
#define CHECK(cond)                                                   \
    do {                                                              \
        if (!(cond)) {                                                \
            throw ::testing::CheckFailure(                            \
                std::string(__FILE__) + ":" + std::to_string(__LINE__) + \
                ": CHECK failed: " #cond);                            \
        }                                                             \
    } while (false)

// Expands to a main() that runs every test registered via TEST(...), in
// registration order. A test that throws any exception derived from
// std::exception (including CheckFailure from a failed CHECK, or
// std::logic_error("not implemented") from an unimplemented stub) is
// reported as [FAIL] with that exception's what(); other tests still run.
// Exits 0 if every test passed, 1 otherwise.
#define TEST_MAIN()                                                   \
    int main() {                                                      \
        int failed = 0;                                               \
        for (const auto& test : ::testing::registry()) {              \
            try {                                                      \
                test.body();                                           \
                std::cout << "[PASS] " << test.name << "\n";           \
            } catch (const std::exception& e) {                        \
                std::cout << "[FAIL] " << test.name << ": " << e.what() \
                          << "\n";                                      \
                ++failed;                                               \
            }                                                          \
        }                                                              \
        if (failed == 0) {                                             \
            std::cout << "All tests passed!\n";                        \
        } else {                                                        \
            std::cout << failed << " test(s) failed.\n";                \
        }                                                               \
        return failed == 0 ? 0 : 1;                                     \
    }
