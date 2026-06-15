#include <string>
#include <vector>

#include "exercise.h"
#include "../../testing.h"

TEST(StackPushPopLifoOrder) {
    Stack<int> s;
    CHECK(s.empty());
    s.push(1);
    s.push(2);
    s.push(3);
    CHECK(!s.empty());
    CHECK_EQ(s.size(), size_t(3));
    CHECK_EQ(s.top(), 3);
    CHECK_EQ(s.pop(), 3);
    CHECK_EQ(s.pop(), 2);
    CHECK_EQ(s.size(), size_t(1));
    CHECK_EQ(s.top(), 1);
}

TEST(StackWithStrings) {
    Stack<std::string> s;
    s.push("a");
    s.push("b");
    s.push("c");
    CHECK_EQ(s.pop(), std::string("c"));
    CHECK_EQ(s.pop(), std::string("b"));
    CHECK_EQ(s.top(), std::string("a"));
    CHECK_EQ(s.size(), size_t(1));
}

TEST(SequenceCtadFromIteratorPair) {
    std::vector<int> v{1, 2, 3, 4};
    Sequence seq(v.begin(), v.end());  // CTAD via deduction guide -> Sequence<int>
    CHECK_EQ(seq.size(), size_t(4));
    CHECK_EQ(seq.at(0), 1);
    CHECK_EQ(seq.at(3), 4);
    CHECK_EQ(seq.sum(), 10);
}

TEST(SequenceWithStrings) {
    std::vector<std::string> v{"a", "b", "c"};
    Sequence seq(v.begin(), v.end());  // -> Sequence<std::string>
    CHECK_EQ(seq.size(), size_t(3));
    CHECK_EQ(seq.at(1), std::string("b"));
    CHECK_EQ(seq.sum(), std::string("abc"));
}

TEST(DoubleValueSfinaeArithmeticVsNonArithmetic) {
    CHECK_EQ(doubleValue(5), 10);
    CHECK_EQ(doubleValue(-3), -6);
    CHECK_EQ(doubleValue(2.5), 5.0);
    CHECK_EQ(doubleValue(std::string("ab")), std::string("abab"));
    CHECK_EQ(doubleValue(std::string("")), std::string(""));
}

TEST(FindMaxWithComparableConcept) {
    CHECK_EQ(findMax(std::vector<int>{3, 1, 4, 1, 5, 9, 2, 6}), 9);
    CHECK_EQ(findMax(std::vector<int>{-1, -2, -3}), -1);
    CHECK_EQ(findMax(std::vector<double>{1.5, 2.5, 0.5}), 2.5);
    CHECK_EQ(findMax(std::vector<std::string>{"banana", "apple", "cherry"}), std::string("cherry"));
    CHECK_EQ(findMax(std::vector<int>{42}), 42);  // single element
}

TEST(OptionalHoldsValueOrEmpty) {
    Optional<int> a(5);
    Optional<int> empty;

    CHECK(a.hasValue());
    CHECK(!empty.hasValue());
    CHECK_EQ(a.value(), 5);
    CHECK_EQ(a.valueOr(0), 5);
    CHECK_EQ(empty.valueOr(-1), -1);
}

TEST(OptionalMapTransformsOrPropagatesEmpty) {
    Optional<int> a(5);
    Optional<int> empty;

    auto doubled = a.map([](int x) { return x * 2; });
    CHECK(doubled.hasValue());
    CHECK_EQ(doubled.value(), 10);

    auto emptyDoubled = empty.map([](int x) { return x * 2; });
    CHECK(!emptyDoubled.hasValue());

    auto asString = a.map([](int x) { return std::to_string(x); });
    CHECK(asString.hasValue());
    CHECK_EQ(asString.value(), std::string("5"));
}

TEST_MAIN()
