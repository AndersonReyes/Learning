#include <iostream>
#include <string>
#include <utility>

// Topic 15-16 (Intermediate 05): Project Organization & Code Conventions
//
// No new core-language feature -- this file demonstrates conventions from
// notes.md that aren't covered by exercise.h's Task/Priority/Status design:
// a project namespace, [[nodiscard]], kPascalCase constants, guard clauses,
// structured bindings, and nullptr.

// A project-specific namespace avoids name collisions with other code --
// never write `using namespace std;` at namespace scope in a header.
namespace conventions {

// PascalCase type, PascalCase enumerators.
enum class Color { Red, Green, Blue };

// camelCase function name. The switch is exhaustive over Color's three
// enumerators, so no `default` is needed -- -Wswitch would warn if a case
// were missing.
std::string colorName(Color c) {
    switch (c) {
        case Color::Red: return "Red";
        case Color::Green: return "Green";
        case Color::Blue: return "Blue";
    }
    return "Unknown";  // unreachable for a valid Color; keeps -Wreturn-type quiet
}

// [[nodiscard]]: a pure computation with no side effect -- ignoring its
// result (a bare `clampToUnit(x);` statement) would trigger a compiler
// warning, since that call would otherwise do nothing useful at all.
[[nodiscard]] double clampToUnit(double value) {
    if (value < 0.0) return 0.0;
    if (value > 1.0) return 1.0;
    return value;
}

// kPascalCase: this repo's convention for namespace-scope constexpr values
// (see kEpsilon, kRomanNumerals in earlier topics).
constexpr double kHalf = 0.5;

}  // namespace conventions

// --- Guard clauses / early returns vs. nested if-else -------------------------------------------

std::string gradeLabel(int score) {
    if (score < 0 || score > 100) return "invalid";
    if (score >= 90) return "A";
    if (score >= 80) return "B";
    if (score >= 70) return "C";
    if (score >= 60) return "D";
    return "F";
}

int main() {
    std::cout << "-- enum class + exhaustive switch --\n";
    std::cout << "colorName(Green) = " << conventions::colorName(conventions::Color::Green)
              << "\n";

    std::cout << "\n-- [[nodiscard]] (return value used) --\n";
    double clamped = conventions::clampToUnit(1.5);
    std::cout << "clampToUnit(1.5) = " << clamped << "\n";
    std::cout << "kHalf = " << conventions::kHalf << "\n";

    std::cout << "\n-- guard clauses / early returns --\n";
    for (int score : {95, 82, 71, 60, 40, -1}) {
        std::cout << "gradeLabel(" << score << ") = " << gradeLabel(score) << "\n";
    }

    std::cout << "\n-- structured bindings --\n";
    std::pair<int, int> minMax{3, 9};
    auto [lo, hi] = minMax;
    std::cout << "lo=" << lo << " hi=" << hi << "\n";

    std::cout << "\n-- nullptr over NULL/0 --\n";
    const int* p = nullptr;
    std::cout << "p == nullptr: " << (p == nullptr) << "\n";
}
