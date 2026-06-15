// Run with:
//   g++ -std=c++20 -Wall -Wextra -o /tmp/ex examples.cpp && /tmp/ex

#include <iostream>
#include <string>
#include <vector>

// --- Namespaces -------------------------------------------------------------
// Nested namespace definition (C++17 shorthand for
// namespace shapes { namespace flat { ... } }).
namespace shapes::flat {

struct Circle {
    double radius;
};

double area(const Circle& c) {
    constexpr double pi = 3.14159265358979;
    return pi * c.radius * c.radius;
}

}  // namespace shapes::flat

// Anonymous namespace: internal linkage -- only visible in this .cpp file.
namespace {
int helperCallCount = 0;
}

int main() {
    // --- if/else: dangling-else gotcha --------------------------------------
    bool outer = true;
    bool inner = false;
    if (outer) {
        if (inner) {
            std::cout << "inner true\n";
        } else {
            std::cout << "else binds to the NEAREST if (inner), not outer\n";
        }
    }

    // --- switch with fallthrough ----------------------------------------------
    for (int day = 1; day <= 7; ++day) {
        std::string kind;
        switch (day) {
            case 1: case 2: case 3: case 4: case 5:
                kind = "weekday";
                break;
            case 6: case 7:
                kind = "weekend";
                break;
            default:
                kind = "invalid";
        }
        std::cout << "day " << day << " is a " << kind << "\n";
    }

    // --- Loops: range-based for, const auto& vs auto ----------------------------
    std::vector<std::string> names = {"alice", "bob", "carol"};
    std::cout << "\nnames:";
    for (const auto& name : names) {  // const ref: no copy, no mutation
        std::cout << " " << name;
    }
    std::cout << "\n";

    // do-while: body runs at least once, even if the condition is false
    // immediately.
    int countdown = 0;
    do {
        std::cout << "countdown=" << countdown << " (ran at least once)\n";
    } while (--countdown > 0);

    // --- break/continue ----------------------------------------------------------
    std::cout << "\nodd numbers below 10, skipping multiples of 5:";
    for (int i = 1; i < 10; ++i) {
        if (i % 2 == 0) continue;  // skip even numbers
        if (i == 5) continue;      // skip 5 specifically
        if (i > 8) break;          // stop early
        std::cout << " " << i;
    }
    std::cout << "\n";

    // --- enum vs enum class --------------------------------------------------------
    enum LegacyColor { Red, Green, Blue };  // unscoped: leaks Red/Green/Blue
    int legacy = Red;                        // implicit conversion to int
    std::cout << "\nLegacyColor Red as int: " << legacy << "\n";

    enum class Color { Red, Green, Blue };
    Color c = Color::Red;
    std::cout << "Color::Red as int: " << static_cast<int>(c) << "\n";

    // enum class with explicit underlying values
    enum class HttpStatus : int { Ok = 200, NotFound = 404, ServerError = 500 };
    HttpStatus status = HttpStatus::NotFound;
    std::cout << "HttpStatus::NotFound = " << static_cast<int>(status) << "\n";

    // --- structs: aggregate init, default members, = default comparison -----------
    struct Point {
        int x = 0;
        int y = 0;
        bool operator==(const Point&) const = default;
    };

    Point p1{3, 4};
    Point p2;  // x=0, y=0 via default member initializers
    std::cout << "\np1 = (" << p1.x << ", " << p1.y << "), p2 = (" << p2.x
              << ", " << p2.y << ")\n";
    std::cout << "p1 == p2: " << std::boolalpha << (p1 == p2) << "\n";

    // --- Namespaces in use ------------------------------------------------------------
    shapes::flat::Circle circle{2.0};
    std::cout << "\ncircle area: " << shapes::flat::area(circle) << "\n";

    using shapes::flat::Circle;  // import one name
    Circle another{1.0};
    std::cout << "another area: " << shapes::flat::area(another) << "\n";

    ++helperCallCount;  // from the anonymous namespace above
    std::cout << "helperCallCount: " << helperCallCount << "\n";

    return 0;
}
