#include <charconv>
#include <filesystem>
#include <iostream>
#include <optional>
#include <random>
#include <sstream>
#include <string>
#include <string_view>
#include <variant>
#include <vector>

// --- std::optional + std::from_chars: parse a double, safely --------------

std::optional<double> parseDouble(std::string_view s) {
    double value = 0.0;
    auto [ptr, ec] = std::from_chars(s.data(), s.data() + s.size(), value);
    if (ec != std::errc{} || ptr != s.data() + s.size()) return std::nullopt;
    return value;
}

void optionalDemo() {
    std::cout << "-- std::optional + std::from_chars --\n";

    for (std::string_view s : {"3.14", "-2.5", "1e10", "abc", "3.14xyz", ""}) {
        std::optional<double> result = parseDouble(s);
        std::cout << "parseDouble(\"" << s << "\") = ";
        if (result) {
            std::cout << *result;
        } else {
            std::cout << "nullopt";
        }
        std::cout << "\n";
    }

    // value_or for a default, has_value/operator bool for branching.
    std::optional<double> maybe = parseDouble("not a number");
    std::cout << "value_or(0.0) = " << maybe.value_or(0.0) << "\n";
    std::cout << "has_value() = " << std::boolalpha << maybe.has_value() << "\n";

    // to_chars: the inverse direction, no allocation.
    char buf[32];
    auto [ptr, ec] = std::to_chars(buf, buf + sizeof(buf), 255, /*base=*/16);
    if (ec == std::errc{}) {
        std::cout << "to_chars(255, base=16) = " << std::string_view(buf, ptr - buf) << "\n";
    }
    std::cout << "\n";
}

// --- std::variant + std::visit: a tiny shape "sum type" --------------------

struct Circle {
    double radius;
};
struct Rectangle {
    double width, height;
};
struct Triangle {
    double base, height;
};

using Shape = std::variant<Circle, Rectangle, Triangle>;

double area(const Shape& shape) {
    return std::visit(
        [](const auto& s) -> double {
            using T = std::decay_t<decltype(s)>;
            if constexpr (std::is_same_v<T, Circle>) {
                return 3.14159265358979 * s.radius * s.radius;
            } else if constexpr (std::is_same_v<T, Rectangle>) {
                return s.width * s.height;
            } else {  // Triangle
                return 0.5 * s.base * s.height;
            }
        },
        shape);
}

void variantDemo() {
    std::cout << "-- std::variant + std::visit --\n";

    std::vector<Shape> shapes = {Circle{2.0}, Rectangle{3.0, 4.0}, Triangle{6.0, 2.0}};
    for (const Shape& shape : shapes) {
        std::cout << "index=" << shape.index() << " area=" << area(shape) << "\n";
    }

    // holds_alternative / get_if for ad-hoc checks without visiting everything.
    Shape s = Rectangle{5.0, 5.0};
    if (std::holds_alternative<Rectangle>(s)) {
        const Rectangle& r = std::get<Rectangle>(s);
        std::cout << "Rectangle " << r.width << "x" << r.height << "\n";
    }
    if (const Circle* c = std::get_if<Circle>(&s)) {
        std::cout << "unreachable, but if it were a circle: r=" << c->radius << "\n";
    } else {
        std::cout << "s is not a Circle\n";
    }
    std::cout << "\n";
}

// --- <random>: engine vs. distribution reproducibility ----------------------

void randomDemo() {
    std::cout << "-- <random>: engine vs. distribution --\n";

    // The raw mt19937 sequence for a fixed seed is standardized: this will
    // print the same five numbers on every conforming implementation.
    std::mt19937 rng(1);
    std::cout << "raw rng() values: ";
    for (int i = 0; i < 5; ++i) std::cout << rng() << " ";
    std::cout << "\n";

    // uniform_int_distribution's mapping is implementation-defined: this
    // demo's values are reproducible WITHIN one toolchain, but not
    // necessarily across libstdc++/libc++/MSVC.
    std::mt19937 rng2(1);
    std::uniform_int_distribution<int> dist(1, 6);  // simulate a die
    std::cout << "uniform_int_distribution(1,6): ";
    for (int i = 0; i < 5; ++i) std::cout << dist(rng2) << " ";
    std::cout << "\n\n";
}

// --- std::filesystem::path: components and lexically_normal ----------------

void filesystemDemo() {
    std::cout << "-- std::filesystem::path --\n";

    std::filesystem::path p = "/usr/local/bin/clang++";
    std::cout << "path: " << p.generic_string() << "\n";
    std::cout << "  filename:   " << p.filename().generic_string() << "\n";
    std::cout << "  stem:       " << p.stem().generic_string() << "\n";
    std::cout << "  extension:  \"" << p.extension().generic_string() << "\"\n";
    std::cout << "  parent_path: " << p.parent_path().generic_string() << "\n";

    // operator/ joins path components with the correct separator.
    std::filesystem::path joined = std::filesystem::path("src") / "lib" / "core.cpp";
    std::cout << "joined: " << joined.generic_string() << "\n";

    // lexically_normal: purely textual, doesn't touch the filesystem.
    for (const char* raw : {"a/./b/../c", "./x/y/", "/a/../../b", "x/../.."}) {
        std::cout << "lexically_normal(\"" << raw << "\") = \""
                  << std::filesystem::path(raw).lexically_normal().generic_string() << "\"\n";
    }
    std::cout << "\n";
}

// --- std::string_view: non-owning views and lifetime ------------------------

void stringViewDemo() {
    std::cout << "-- std::string_view --\n";

    std::string sentence = "the quick brown fox";

    // substr on a string_view returns another string_view -- no allocation.
    std::string_view view = sentence;
    std::string_view first_word = view.substr(0, view.find(' '));
    std::cout << "first word: " << first_word << "\n";

    // starts_with / ends_with (C++20).
    std::cout << "starts_with(\"the\") = " << std::boolalpha << view.starts_with("the") << "\n";
    std::cout << "ends_with(\"fox\") = " << view.ends_with("fox") << "\n";

    // Lifetime gotcha: a string_view into a temporary dangles immediately.
    // std::string_view dangling = std::string("temp");  // would be UB to use
    // The line above is commented out -- it compiles but is undefined
    // behavior to dereference, since the temporary std::string is destroyed
    // at the end of the full expression.

    std::cout << "\n";
}

int main() {
    optionalDemo();
    variantDemo();
    randomDemo();
    filesystemDemo();
    stringViewDemo();
}
