#include <cctype>
#include <iostream>
#include <string>

// Topic 13 (Intermediate 03): Translation Units, Linkage & the ODR
//
// Different illustrative examples than exercise.h's
// nextId/normalizeWhitespace/globalRequestCount/nearlyEqual/toRoman.
// examples.cpp is a single translation unit, so the cross-TU effects of
// linkage/inline/extern can't be OBSERVED by running it -- but the syntax
// and within-TU behavior can. exercise.cpp + exercise_test.cpp (two TUs,
// linked together) is where those effects actually matter.

// --- Function-local static: storage duration -----------------------------------------------

int requestId() {
    static int counter = 100;  // initialized once, on the first call
    return ++counter;
}

// --- Anonymous namespace: internal linkage --------------------------------------------------

namespace {
// Would be invisible to any other .cpp file, even if this were split into a
// header + .cpp -- internal linkage.
bool isVowel(char c) {
    c = static_cast<char>(std::tolower(static_cast<unsigned char>(c)));
    return c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u';
}
}  // namespace

int countVowels(const std::string& text) {
    int count = 0;
    for (char c : text) {
        if (isVowel(c)) ++count;
    }
    return count;
}

// --- inline function + inline constexpr variable ---------------------------------------------

// If this were declared in a header shared by multiple .cpp files, `inline`
// would be required to avoid "multiple definition" link errors.
inline constexpr double kTaxRate = 0.07;

inline double withTax(double price) { return price * (1.0 + kTaxRate); }

// --- extern declaration + definition in the same TU --------------------------------------------

// In a real multi-file program, the `extern` line would live in a header
// (declaration only) and the initialized line in exactly one .cpp file (the
// definition) -- see exercise.h/.cpp's globalRequestCount for the real
// split. Here both appear in one TU just to show the syntax.
extern int sharedTotal;
int sharedTotal = 0;

// --- static at namespace scope: internal linkage (pre-C++11 style) -------------------------------

// Internal linkage, like the anonymous-namespace isVowel above -- an older
// syntax for the same effect, still common in existing codebases.
static int s_callCount = 0;

void recordCall() { ++s_callCount; }

int main() {
    std::cout << "-- requestId (function-local static) --\n";
    std::cout << "requestId() = " << requestId() << "\n";
    std::cout << "requestId() = " << requestId() << "\n";
    std::cout << "requestId() = " << requestId() << "\n";

    std::cout << "\n-- countVowels (anonymous-namespace helper) --\n";
    std::cout << "countVowels(\"Hello, World!\") = " << countVowels("Hello, World!") << "\n";
    std::cout << "countVowels(\"sky\") = " << countVowels("sky") << "\n";

    std::cout << "\n-- inline function + inline constexpr --\n";
    std::cout << "withTax(100.0) = " << withTax(100.0) << " (kTaxRate = " << kTaxRate << ")\n";

    std::cout << "\n-- extern declaration + definition --\n";
    sharedTotal += 42;
    std::cout << "sharedTotal = " << sharedTotal << "\n";

    std::cout << "\n-- static (internal linkage) global --\n";
    recordCall();
    recordCall();
    recordCall();
    std::cout << "s_callCount = " << s_callCount << "\n";
}
