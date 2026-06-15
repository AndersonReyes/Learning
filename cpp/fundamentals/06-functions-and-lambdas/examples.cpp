// Run with:
//   g++ -std=c++20 -Wall -Wextra -o /tmp/ex examples.cpp && /tmp/ex

#include <functional>
#include <iostream>
#include <stdexcept>
#include <string>
#include <vector>

// --- Preprocessor: function-like macros -----------------------------------------
#define SQUARE(x) ((x) * (x))     // properly parenthesized
#define BAD_SQUARE(x) x * x        // NOT parenthesized -- demonstrated below
#define STRINGIFY(x) #x

// --- Functions: default arguments -------------------------------------------------
std::string greet(const std::string& name, const std::string& greeting = "Hello") {
    return greeting + ", " + name + "!";
}

// --- Mutual recursion: needs a forward declaration --------------------------------
bool isOdd(int n);  // forward declaration -- isEven calls this before it's defined

bool isEven(int n) {
    if (n == 0) return true;
    return isOdd(n - 1);
}

bool isOdd(int n) {
    if (n == 0) return false;
    return isEven(n - 1);
}

// --- Function pointers ---------------------------------------------------------------
int square(int x) { return x * x; }
int cube(int x) { return x * x * x; }

int applyToFive(int (*fn)(int)) {
    return fn(5);
}

int main() {
    // --- default arguments ---
    std::cout << greet("Ada") << "\n";
    std::cout << greet("Ada", "Hi") << "\n";

    // --- mutual recursion ---
    std::cout << "\nisEven(10) = " << std::boolalpha << isEven(10) << "\n";
    std::cout << "isOdd(10) = " << isOdd(10) << "\n";

    // --- function pointers ---
    std::cout << "\napplyToFive(square) = " << applyToFive(square) << "\n";
    std::cout << "applyToFive(cube) = " << applyToFive(cube) << "\n";

    int (*fn)(int) = square;  // function name decays to a function pointer
    std::cout << "fn(6) = " << fn(6) << "\n";

    // --- lambdas: capture by value vs by reference ---
    int threshold = 10;
    auto aboveThreshold = [threshold](int x) { return x > threshold; };  // copy of threshold
    std::cout << "\naboveThreshold(15) = " << aboveThreshold(15) << "\n";
    threshold = 100;  // does NOT affect aboveThreshold -- it captured a copy
    std::cout << "aboveThreshold(15) after threshold=100 -> " << aboveThreshold(15) << "\n";

    int total = 0;
    auto accumulate = [&total](int x) { total += x; };  // reference: mutates total
    accumulate(5);
    accumulate(10);
    std::cout << "total after accumulate(5), accumulate(10) = " << total << "\n";

    // --- mutable lambda: closure with its own evolving state ---
    auto counter = [n = 0]() mutable { return n++; };
    std::cout << "\ncounter() = " << counter() << "\n";
    std::cout << "counter() = " << counter() << "\n";
    std::cout << "counter() = " << counter() << "\n";

    auto counter2 = [n = 100]() mutable { return n++; };  // independent state
    std::cout << "counter2() = " << counter2() << "\n";
    std::cout << "counter() = " << counter()
              << " (counter's own state, unaffected by counter2)\n";

    // --- std::function: holding different kinds of callables ---
    std::function<int(int)> op = square;  // function pointer
    std::cout << "\nop(4) via function pointer = " << op(4) << "\n";
    op = [](int x) { return x * 2; };  // capture-less lambda
    std::cout << "op(4) via lambda = " << op(4) << "\n";
    int factor = 3;
    op = [factor](int x) { return x * factor; };  // lambda with a capture
    std::cout << "op(4) via lambda with capture = " << op(4) << "\n";

    // --- registry pattern: named thunks run with per-test try/catch -----------------
    // cpp/testing.h (built in this topic's exercises) packages exactly this
    // pattern into TEST/CHECK/TEST_MAIN macros, used from fundamentals/07 on.
    std::vector<std::pair<std::string, std::function<void()>>> checks = {
        {"1 + 1 == 2", [] { if (1 + 1 != 2) throw std::runtime_error("math is broken"); }},
        {"always fails", [] { throw std::runtime_error("intentional failure"); }},
    };
    std::cout << "\nrunning checks:\n";
    for (const auto& [name, thunk] : checks) {
        try {
            thunk();
            std::cout << "  [PASS] " << name << "\n";
        } catch (const std::exception& e) {
            std::cout << "  [FAIL] " << name << ": " << e.what() << "\n";
        }
    }

    // --- preprocessor macros ---
    int a = 2, b = 3;
    std::cout << "\nSQUARE(a + b) = " << SQUARE(a + b) << "\n";      // ((2+3)*(2+3)) = 25
    std::cout << "BAD_SQUARE(a + b) = " << BAD_SQUARE(a + b) << "\n"; // 2+3*2+3 = 11 (wrong!)
    std::cout << "STRINGIFY(1 + 2) = " << STRINGIFY(1 + 2) << "\n";   // the text "1 + 2"
    std::cout << "this line is " << __FILE__ << ":" << __LINE__ << "\n";

    return 0;
}
