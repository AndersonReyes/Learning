#pragma once

#include <numeric>
#include <stdexcept>
#include <string>
#include <type_traits>
#include <utility>
#include <vector>

// Topic 21 (Advanced 01): Move Semantics, Value Categories & Type Deduction
//
// Five exercises on std::move, perfect forwarding, type deduction, and the
// Rule of 5. Function templates are fully defined here (template
// definitions must be visible at the point of instantiation -- see
// intermediate/01); IntBuffer's member functions are declared here and
// defined in exercise.cpp. Stub bodies throw
// std::logic_error("not implemented").

// --- Tracked: shared test infrastructure (not one of the 5 exercises) ---------------------------
//
// Counts every construction/assignment/destruction it undergoes, via static
// counters. swapViaMove and rotateLeft are tested by checking these counters
// stay at 0 for copyCtor/copyAssign -- proving the implementation moves
// elements instead of copying them. Call Tracked::reset() before the
// operation under test.
struct Tracked {
    int value;

    inline static int copyCtor = 0;
    inline static int moveCtor = 0;
    inline static int copyAssign = 0;
    inline static int moveAssign = 0;

    explicit Tracked(int v) : value(v) {}
    Tracked(const Tracked& other) : value(other.value) { ++copyCtor; }
    Tracked(Tracked&& other) : value(other.value) {
        other.value = -1;
        ++moveCtor;
    }
    Tracked& operator=(const Tracked& other) {
        value = other.value;
        ++copyAssign;
        return *this;
    }
    Tracked& operator=(Tracked&& other) {
        value = other.value;
        other.value = -1;
        ++moveAssign;
        return *this;
    }

    static void reset() { copyCtor = moveCtor = copyAssign = moveAssign = 0; }

    bool operator==(const Tracked& other) const { return value == other.value; }
};

// --- swapViaMove: std::move, the three-move swap ----------------------------------------------
//
// Swaps `a` and `b` using exactly std::move three times (one move
// construction, two move assignments) -- the same shape as std::swap's
// generic implementation, but written by hand: `T temp = std::move(a); a =
// std::move(b); b = std::move(temp);`. Must not invoke T's copy constructor
// or copy assignment operator at all.
//
// Example: with Tracked a(1), b(2): after swapViaMove(a, b), a.value == 2,
// b.value == 1, and Tracked::copyCtor == 0 && Tracked::copyAssign == 0 &&
// Tracked::moveCtor == 1 && Tracked::moveAssign == 2.
template <typename T>
void swapViaMove(T& a, T& b) {
    (void)a;
    (void)b;
    throw std::logic_error("not implemented");
}

// --- rotateLeft: the juggling algorithm, O(1) extra space via std::move -------------------------
//
// Rotates `v` left by `d` positions in place, using O(1) extra space (one
// temporary element) via the juggling/cycle algorithm: let g = gcd(v.size(),
// d); for each of the g cycles, save v[i] into a temporary (via
// std::move), follow the cycle v[j] = std::move(v[j+d mod n]) until it
// returns to i, then write the temporary into the final slot. `d` is
// normalized mod v.size() first (negative `d` rotates right). A no-op if
// v.empty() or d % v.size() == 0.
//
// Must use std::move for every element relocation -- never T's copy
// constructor or copy assignment.
//
// Example: rotateLeft({1,2,3,4,5,6,7}, 3) -> {4,5,6,7,1,2,3}
// rotateLeft({1,2,3,4,5,6,7}, 0) -> unchanged
// rotateLeft({1,2,3,4,5,6,7}, 10) -> same as d=3 (10 % 7 == 3) -> {4,5,6,7,1,2,3}
// rotateLeft({1,2}, -1) -> {2,1}                 // negative d -> right rotation
//
// With 8 Tracked elements, rotateLeft(v, 3) (gcd(8,3)==1, one cycle of
// length 8) performs exactly Tracked::moveCtor == 1 (one temporary) and
// Tracked::moveAssign == 8, with copyCtor == copyAssign == 0.
// With 6 Tracked elements, rotateLeft(v, 2) (gcd(6,2)==2, two cycles of
// length 3) performs Tracked::moveCtor == 2 and Tracked::moveAssign == 6.
template <typename T>
void rotateLeft(std::vector<T>& v, long long d) {
    (void)v;
    (void)d;
    throw std::logic_error("not implemented");
}

// --- forwardingRefKind: forwarding-reference (T&&) deduction -----------------------------------
//
// Given a forwarding reference parameter `T&& x`, returns "lvalue" if the
// argument was an lvalue (T deduced as a reference type, e.g. `int&`) or
// "rvalue" if the argument was an rvalue (T deduced as a non-reference
// type, e.g. `int`) -- per std::is_lvalue_reference_v<T>. This is the
// deduction std::forward relies on to "remember" the original argument's
// value category.
//
// Example: int x = 5;
// forwardingRefKind(x) == "lvalue"          // T deduced as int&
// forwardingRefKind(5) == "rvalue"          // T deduced as int
// forwardingRefKind(std::move(x)) == "rvalue"
// const int cx = 10; forwardingRefKind(cx) == "lvalue"  // T deduced as const int&
template <typename T>
std::string forwardingRefKind(T&& x) {
    (void)x;
    throw std::logic_error("not implemented");
}

// --- firstElement: decltype on the return type preserves reference-ness -------------------------
//
// Returns a reference to c[0] -- NOT a copy. The trailing return type
// `decltype(c[0])` (rather than plain `auto`, which would deduce a
// by-value `int` and strip the reference) preserves c's reference and
// const-ness: for a non-const lvalue Container, the result is `int&` and
// can be assigned through; for a const Container, it's `const int&`.
// (`decltype(auto)` as the return type, deducing from the `return`
// statement, achieves the same thing for a non-trailing declaration.)
//
// Example: std::vector<int> v = {10,20,30};
// firstElement(v) = 99;       // v[0] is now 99 -- mutates through the reference
// const std::vector<int> cv = {1,2,3};
// firstElement(cv) == 1;      // reads through a const int&
template <typename Container>
auto firstElement(Container&& c) -> decltype(c[0]) {
    (void)c;
    throw std::logic_error("not implemented");
}

// --- IntBuffer: Rule of 5 ------------------------------------------------------------------------
//
// An RAII heap-allocated array of `size` ints, zero-initialized.
//   IntBuffer(size)        -- allocates `size` ints, all 0
//   copy ctor/assignment   -- deep copy (independent storage)
//   move ctor/assignment   -- O(1): takes ownership of the source's
//                             storage, leaving the source with size() == 0
//                             and no allocation (safe to destroy)
//   Both assignment operators must be safe for self-assignment
//   (`a = a;` and `a = std::move(a);` must leave `a` valid and unchanged).
//   ~IntBuffer()           -- frees the storage (already implemented below;
//                             relies on data_'s default member initializer
//                             so it's correct even for a moved-from buffer)
//   size() const           -- number of ints
//   operator[](i)           -- element access (precondition: i < size())
//   sum() const            -- sum of all elements
//
// (Real code would mark the move constructor/assignment `noexcept` --
// omitted here only so the stub bodies can `throw` like every other
// exercise in this track.)
//
// Example: IntBuffer a(5); fill a[0..4] with 0..4; a.sum() == 10.
// IntBuffer b = a; b[0] = 100; -- a[0] is still 0 (deep copy).
// IntBuffer c = std::move(a); -- c.sum() == 10, a.size() == 0.
class IntBuffer {
public:
    explicit IntBuffer(size_t size);
    IntBuffer(const IntBuffer& other);
    IntBuffer(IntBuffer&& other);
    IntBuffer& operator=(const IntBuffer& other);
    IntBuffer& operator=(IntBuffer&& other);
    ~IntBuffer();

    size_t size() const;
    int& operator[](size_t i);
    const int& operator[](size_t i) const;
    long sum() const;

private:
    int* data_ = nullptr;
    size_t size_ = 0;
};
