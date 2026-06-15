#pragma once

#include <iterator>
#include <stdexcept>
#include <string>
#include <type_traits>
#include <utility>
#include <vector>

// Topic 12 (Intermediate 02): Class Templates, CTAD, SFINAE & Concepts
//
// Class templates -- like function templates -- must be fully defined where
// they're used, so all five exercises live here in exercise.h.
// exercise.cpp has nothing to add (same reasoning as intermediate/01). Stub
// bodies `throw std::logic_error("not implemented")`, caught at runtime by
// TEST_MAIN().

// --- Stack<T>: class template basics -----------------------------------------------------
//
// A LIFO stack backed by std::vector<T>.
//   push(value)   -- pushes value onto the top
//   pop()         -- removes AND returns the top element (precondition: !empty())
//   top() const   -- const reference to the top element (precondition: !empty())
//   empty() const -- true if the stack has no elements
//   size() const  -- number of elements
//
// Example: after push(1), push(2), push(3): size() == 3, top() == 3,
// pop() == 3 (size() becomes 2), pop() == 2, top() == 1.
template <typename T>
class Stack {
public:
    void push(T value) {
        (void)value;
        throw std::logic_error("not implemented");
    }

    T pop() { throw std::logic_error("not implemented"); }

    const T& top() const { throw std::logic_error("not implemented"); }

    bool empty() const { throw std::logic_error("not implemented"); }

    size_t size() const { throw std::logic_error("not implemented"); }

private:
    std::vector<T> data_;
};

// --- Sequence<T>: CTAD via an explicit deduction guide ------------------------------------
//
// A read-only wrapper around std::vector<T>, constructed from a pair of
// iterators. T is NOT one of the constructor's parameter types (the
// constructor is templated on Iter alone), so implicit CTAD cannot deduce T
// -- the explicit deduction guide below extracts T from
// std::iterator_traits<Iter>::value_type, e.g. for `std::vector<int> v`,
// `Sequence seq(v.begin(), v.end())` deduces `Sequence<int>` via that guide.
//
//   size() const   -- number of elements
//   at(i) const    -- element at index i (precondition: i < size())
//   sum() const    -- sum of all elements via operator+= starting from T{}
//                     (T{} is 0 for numeric types, "" for std::string)
//
// Example: Sequence seq(v.begin(), v.end()) for v == {1,2,3,4} gives
// seq.size() == 4, seq.at(0) == 1, seq.sum() == 10.
template <typename T>
class Sequence {
public:
    template <typename Iter>
    Sequence(Iter first, Iter last) {
        (void)first;
        (void)last;
        throw std::logic_error("not implemented");
    }

    size_t size() const { throw std::logic_error("not implemented"); }

    const T& at(size_t index) const {
        (void)index;
        throw std::logic_error("not implemented");
    }

    T sum() const { throw std::logic_error("not implemented"); }

private:
    std::vector<T> data_;
};

template <typename Iter>
Sequence(Iter, Iter) -> Sequence<typename std::iterator_traits<Iter>::value_type>;

// --- doubleValue: SFINAE via std::enable_if_t ---------------------------------------------
//
// Two overloads selected by std::is_arithmetic_v<T>:
//   - arithmetic T (int, double, ...): returns x * 2
//   - non-arithmetic T (std::string, ...): returns x + x (self-concatenation)
// Exactly one overload is in the candidate set for any given T -- the other
// is removed from overload resolution by SFINAE ("Substitution Failure Is
// Not An Error"): std::enable_if_t<false, T> has no member `type`, so that
// overload's return type is ill-formed and it's silently dropped, not a
// compile error.
//
// Example: doubleValue(5) == 10; doubleValue(2.5) == 5.0;
// doubleValue(std::string("ab")) == "abab".
template <typename T>
std::enable_if_t<std::is_arithmetic_v<T>, T> doubleValue(T x) {
    (void)x;
    throw std::logic_error("not implemented");
}

template <typename T>
std::enable_if_t<!std::is_arithmetic_v<T>, T> doubleValue(T x) {
    (void)x;
    throw std::logic_error("not implemented");
}

// --- findMax: C++20 concept-constrained template -------------------------------------------
//
// Comparable<T> requires only that `a < b` be a valid expression for two
// const T&. findMax(values) returns the largest element of values by `<`.
// Precondition: !values.empty().
//
// Example: findMax(std::vector<int>{3,1,4,1,5}) == 5;
// findMax(std::vector<std::string>{"b","a","c"}) == "c".
template <typename T>
concept Comparable = requires(const T& a, const T& b) {
    a < b;
};

template <Comparable T>
T findMax(const std::vector<T>& values) {
    (void)values;
    throw std::logic_error("not implemented");
}

// --- Optional<T>: class template + member function template ---------------------------------
//
// A minimal optional-value wrapper (a hand-rolled, simplified relative of
// std::optional<T>, covered properly in intermediate/07).
//   Optional()              -- empty
//   Optional(value)         -- holds `value`
//   hasValue() const        -- true if holding a value
//   value() const           -- the held value (precondition: hasValue())
//   valueOr(default) const  -- the held value, or `default` if empty
//   map(f) const            -- if hasValue(), returns Optional<R>(f(value()))
//                               where R = std::invoke_result_t<Func, T>;
//                               otherwise an empty Optional<R>. f's return
//                               type R may differ from T -- map can change
//                               the contained type.
//
// Example: Optional<int>(5).map([](int x){ return x * 2; }) holds 10.
// Optional<int>().map(...) is empty. Optional<int>(5).valueOr(0) == 5;
// Optional<int>().valueOr(-1) == -1.
template <typename T>
class Optional {
public:
    Optional() : hasValue_(false), value_() {}
    explicit Optional(T value) : hasValue_(true), value_(std::move(value)) {}

    bool hasValue() const { throw std::logic_error("not implemented"); }

    const T& value() const { throw std::logic_error("not implemented"); }

    T valueOr(T defaultValue) const {
        (void)defaultValue;
        throw std::logic_error("not implemented");
    }

    template <typename Func>
    auto map(Func f) const -> Optional<std::invoke_result_t<Func, T>> {
        (void)f;
        throw std::logic_error("not implemented");
    }

private:
    bool hasValue_;
    T value_;
};
