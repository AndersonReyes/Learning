#include <string>
#include <type_traits>
#include <utility>
#include <vector>

#include "exercise.h"
#include "../../testing.h"

TEST(SwapViaMoveUsesOnlyMoves) {
    Tracked::reset();
    Tracked a(1), b(2);
    swapViaMove(a, b);
    CHECK(a.value == 2);
    CHECK(b.value == 1);
    CHECK(Tracked::copyCtor == 0);
    CHECK(Tracked::copyAssign == 0);
    CHECK(Tracked::moveCtor == 1);
    CHECK(Tracked::moveAssign == 2);

    int x = 1, y = 2;
    swapViaMove(x, y);
    CHECK(x == 2);
    CHECK(y == 1);

    std::string s1 = "hello", s2 = "world";
    swapViaMove(s1, s2);
    CHECK_EQ(s1, std::string("world"));
    CHECK_EQ(s2, std::string("hello"));
}

TEST(RotateLeftHandlesVariousShifts) {
    std::vector<int> v1 = {1, 2, 3, 4, 5, 6, 7};
    rotateLeft(v1, 3);
    CHECK(v1 == (std::vector<int>{4, 5, 6, 7, 1, 2, 3}));

    std::vector<int> v2 = {1, 2, 3, 4, 5, 6, 7};
    rotateLeft(v2, 0);
    CHECK(v2 == (std::vector<int>{1, 2, 3, 4, 5, 6, 7}));

    std::vector<int> v3 = {1, 2, 3, 4, 5, 6, 7};
    rotateLeft(v3, 7);  // full rotation -> no-op
    CHECK(v3 == (std::vector<int>{1, 2, 3, 4, 5, 6, 7}));

    std::vector<int> v4 = {1, 2, 3, 4, 5, 6, 7};
    rotateLeft(v4, 10);  // 10 % 7 == 3
    CHECK(v4 == (std::vector<int>{4, 5, 6, 7, 1, 2, 3}));

    std::vector<int> v5 = {1, 2};
    rotateLeft(v5, -1);  // negative -> right rotation
    CHECK(v5 == (std::vector<int>{2, 1}));

    std::vector<int> v6;
    rotateLeft(v6, 3);
    CHECK(v6 == std::vector<int>{});
}

TEST(RotateLeftUsesMovesNotCopies) {
    std::vector<Tracked> v1;
    v1.reserve(8);
    for (int i = 0; i < 8; ++i) v1.emplace_back(i);
    Tracked::reset();
    rotateLeft(v1, 3);  // gcd(8,3) == 1 -> one cycle of length 8

    std::vector<int> values1;
    for (const auto& t : v1) values1.push_back(t.value);
    CHECK(values1 == (std::vector<int>{3, 4, 5, 6, 7, 0, 1, 2}));
    CHECK(Tracked::copyCtor == 0);
    CHECK(Tracked::copyAssign == 0);
    CHECK(Tracked::moveCtor == 1);
    CHECK(Tracked::moveAssign == 8);

    std::vector<Tracked> v2;
    v2.reserve(6);
    for (int i = 0; i < 6; ++i) v2.emplace_back(i);
    Tracked::reset();
    rotateLeft(v2, 2);  // gcd(6,2) == 2 -> two cycles of length 3

    std::vector<int> values2;
    for (const auto& t : v2) values2.push_back(t.value);
    CHECK(values2 == (std::vector<int>{2, 3, 4, 5, 0, 1}));
    CHECK(Tracked::copyCtor == 0);
    CHECK(Tracked::copyAssign == 0);
    CHECK(Tracked::moveCtor == 2);
    CHECK(Tracked::moveAssign == 6);
}

TEST(ForwardingRefKindDetectsValueCategory) {
    int x = 5;
    CHECK_EQ(forwardingRefKind(x), std::string("lvalue"));
    CHECK_EQ(forwardingRefKind(5), std::string("rvalue"));
    CHECK_EQ(forwardingRefKind(std::move(x)), std::string("rvalue"));

    const int cx = 10;
    CHECK_EQ(forwardingRefKind(cx), std::string("lvalue"));
}

TEST(FirstElementPreservesReferenceAndConstness) {
    std::vector<int> v = {10, 20, 30};
    firstElement(v) = 99;
    CHECK(v[0] == 99);

    const std::vector<int> cv = {1, 2, 3};
    CHECK(firstElement(cv) == 1);

    static_assert(std::is_same_v<decltype(firstElement(v)), int&>);
    static_assert(std::is_same_v<decltype(firstElement(cv)), const int&>);
}

TEST(IntBufferRuleOfFive) {
    IntBuffer a(5);
    for (size_t i = 0; i < 5; ++i) a[i] = static_cast<int>(i);
    CHECK(a.size() == 5);
    CHECK(a.sum() == 10);

    IntBuffer b = a;  // copy: independent storage
    b[0] = 100;
    CHECK(a[0] == 0);
    CHECK(b[0] == 100);

    IntBuffer c = std::move(a);  // move: O(1) ownership transfer
    CHECK(c.size() == 5);
    CHECK(c.sum() == 10);
    CHECK(a.size() == 0);

    IntBuffer d(3);
    d = std::move(c);  // move assignment
    CHECK(d.size() == 5);
    CHECK(d.sum() == 10);
    CHECK(c.size() == 0);

    d = std::move(d);  // self-move-assignment must be safe
    CHECK(d.size() == 5);
    CHECK(d.sum() == 10);

    IntBuffer e(4);
    for (size_t i = 0; i < 4; ++i) e[i] = static_cast<int>(i) + 1;
    e = e;  // self-copy-assignment must be safe
    CHECK(e.size() == 4);
    CHECK(e.sum() == 10);
}

TEST_MAIN()
