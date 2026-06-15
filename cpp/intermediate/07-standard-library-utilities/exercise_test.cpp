#include <optional>
#include <random>
#include <string>
#include <string_view>
#include <variant>
#include <vector>

#include "exercise.h"
#include "../../testing.h"

TEST(ParseIntAcceptsOnlyExactIntLiterals) {
    CHECK(parseInt("123") == 123);
    CHECK(parseInt("-45") == -45);
    CHECK(parseInt("0") == 0);
    CHECK(parseInt("-0") == 0);
    CHECK(parseInt("2147483647") == 2147483647);    // INT_MAX
    CHECK(parseInt("-2147483648") == -2147483648);  // INT_MIN

    CHECK(parseInt("+7") == std::nullopt);          // no leading '+'
    CHECK(parseInt("") == std::nullopt);            // empty
    CHECK(parseInt("123abc") == std::nullopt);      // not fully consumed
    CHECK(parseInt("  123") == std::nullopt);       // leading whitespace
    CHECK(parseInt("2147483648") == std::nullopt);  // overflow
    CHECK(parseInt("99999999999999999999") == std::nullopt);
}

TEST(JsonValueToStringRendersEachAlternative) {
    CHECK_EQ(jsonValueToString(std::monostate{}), std::string("null"));
    CHECK_EQ(jsonValueToString(true), std::string("true"));
    CHECK_EQ(jsonValueToString(false), std::string("false"));
    CHECK_EQ(jsonValueToString(42), std::string("42"));
    CHECK_EQ(jsonValueToString(-7), std::string("-7"));
    CHECK_EQ(jsonValueToString(3.5), std::string("3.5"));
    CHECK_EQ(jsonValueToString(100.0), std::string("100"));
    CHECK_EQ(jsonValueToString(std::string("hi")), std::string("\"hi\""));
    CHECK_EQ(jsonValueToString(std::string("")), std::string("\"\""));
}

TEST(FisherYatesShuffleIsReproducibleAndPreservesElements) {
    std::mt19937 rng(42);
    std::vector<int> result = fisherYatesShuffle({1, 2, 3, 4, 5, 6, 7, 8, 9, 10}, rng);
    CHECK(result == (std::vector<int>{2, 4, 10, 8, 7, 1, 9, 5, 6, 3}));

    std::mt19937 rng2(7);
    std::vector<int> result2 = fisherYatesShuffle({1, 2, 3, 4, 5}, rng2);
    CHECK(result2 == (std::vector<int>{3, 4, 2, 5, 1}));

    // Single element / empty: shuffling is a no-op regardless of rng state.
    std::mt19937 rng3(123);
    CHECK(fisherYatesShuffle({42}, rng3) == (std::vector<int>{42}));
    CHECK(fisherYatesShuffle({}, rng3) == std::vector<int>{});
}

TEST(NormalizedPathResolvesDotAndDotDot) {
    CHECK_EQ(normalizedPath("a/./b/../c"), std::string("a/c"));
    CHECK_EQ(normalizedPath("a/b/../../c"), std::string("c"));
    CHECK_EQ(normalizedPath("./a/b/"), std::string("a/b/"));
    CHECK_EQ(normalizedPath("/a/../../b"), std::string("/b"));
    CHECK_EQ(normalizedPath("a/b/c/../.."), std::string("a/"));
}

TEST(SplitOnWhitespaceSkipsRunsAndEdges) {
    CHECK(splitOnWhitespace("  hello   world  ") ==
          (std::vector<std::string_view>{"hello", "world"}));
    CHECK(splitOnWhitespace("") == std::vector<std::string_view>{});
    CHECK(splitOnWhitespace("   ") == std::vector<std::string_view>{});
    CHECK(splitOnWhitespace("a") == (std::vector<std::string_view>{"a"}));
    CHECK(splitOnWhitespace("a\tb\nc") == (std::vector<std::string_view>{"a", "b", "c"}));
}

TEST_MAIN()
