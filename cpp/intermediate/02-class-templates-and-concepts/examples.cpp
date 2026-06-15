#include <iostream>
#include <string>
#include <type_traits>
#include <utility>

// Topic 12 (Intermediate 02): Class Templates, CTAD, SFINAE & Concepts
//
// Different illustrative examples than exercise.h's
// Stack/Sequence/doubleValue/findMax/Optional -- same concepts, different
// types/functions, so the exercises stay unspoiled.

// --- Class template + implicit CTAD (two independent type parameters) --------------------

template <typename T, typename U>
class Pair {
public:
    Pair(T first, U second) : first_(std::move(first)), second_(std::move(second)) {}

    const T& first() const { return first_; }
    const U& second() const { return second_; }

    // Member function template: Func's type is deduced independently of
    // T/U, and the result type R = invoke_result_t<Func, T> may differ from T.
    template <typename Func>
    auto applyToFirst(Func f) const -> Pair<std::invoke_result_t<Func, T>, U> {
        return Pair<std::invoke_result_t<Func, T>, U>(f(first_), second_);
    }

private:
    T first_;
    U second_;
};

// --- Explicit deduction guide: deduce T from a callable's return type ----------------------

template <typename T>
class Lazy {
public:
    template <typename Func>
    explicit Lazy(Func f) : value_(f()) {}

    const T& get() const { return value_; }

private:
    T value_;
};

// T isn't a parameter type of the constructor (Func is) -- the implicit
// guide can't deduce it, so this explicit guide computes T from Func's
// return type.
template <typename Func>
Lazy(Func) -> Lazy<std::invoke_result_t<Func>>;

// --- SFINAE: enable_if overloads on integral vs. floating-point ----------------------------

template <typename T>
std::enable_if_t<std::is_integral_v<T>, std::string> describeNumber(T value) {
    return "integer " + std::to_string(value);
}

template <typename T>
std::enable_if_t<std::is_floating_point_v<T>, std::string> describeNumber(T value) {
    return "float " + std::to_string(value);
}

// --- Concept: Printable ----------------------------------------------------------------------

template <typename T>
concept Printable = requires(const T& t, std::ostream& os) {
    os << t;
};

template <Printable T>
void printLabeled(const std::string& label, const T& value) {
    std::cout << label << ": " << value << "\n";
}

int main() {
    std::cout << "-- Pair (class template + implicit CTAD) --\n";
    Pair p(1, std::string("one"));  // CTAD -> Pair<int, std::string>
    std::cout << "p.first() = " << p.first() << ", p.second() = " << p.second() << "\n";

    auto doubled = p.applyToFirst([](int x) { return x * 2; });
    std::cout << "p.applyToFirst(x*2).first() = " << doubled.first()
              << ", .second() = " << doubled.second() << "\n";

    std::cout << "\n-- Lazy (explicit deduction guide from a callable's return type) --\n";
    Lazy lazy([] { return 42; });  // CTAD -> Lazy<int>, via the explicit guide
    std::cout << "lazy.get() = " << lazy.get() << "\n";

    std::cout << "\n-- describeNumber (SFINAE: integral vs. floating-point) --\n";
    std::cout << "describeNumber(7) = " << describeNumber(7) << "\n";
    std::cout << "describeNumber(3.5) = " << describeNumber(3.5) << "\n";

    std::cout << "\n-- printLabeled (Printable concept) --\n";
    printLabeled("count", 42);
    printLabeled("name", std::string("Ada"));
    printLabeled("pi", 3.14159);
}
