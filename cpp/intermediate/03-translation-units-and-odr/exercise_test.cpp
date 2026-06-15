#include <string>

#include "exercise.h"
#include "../../testing.h"

TEST(NextIdReturnsIncreasingSequence) {
    int a = nextId();
    int b = nextId();
    int c = nextId();
    CHECK(a >= 1);
    CHECK_EQ(b, a + 1);
    CHECK_EQ(c, b + 1);
}

TEST(NormalizeWhitespaceCollapsesAndTrims) {
    CHECK_EQ(normalizeWhitespace("  hello \t world\n"), std::string("hello world"));
    CHECK_EQ(normalizeWhitespace(""), std::string(""));
    CHECK_EQ(normalizeWhitespace("   "), std::string(""));
    CHECK_EQ(normalizeWhitespace("single"), std::string("single"));
    CHECK_EQ(normalizeWhitespace("a\n\nb"), std::string("a b"));
}

TEST(ExternGlobalSharedAcrossTranslationUnits) {
    resetRequestCount();
    CHECK_EQ(getRequestCount(), 0);
    CHECK_EQ(globalRequestCount, 0);

    recordRequest();
    recordRequest();
    recordRequest();
    CHECK_EQ(getRequestCount(), 3);
    CHECK_EQ(globalRequestCount, 3);  // exercise_test.cpp sees exercise.cpp's variable directly

    resetRequestCount();
    CHECK_EQ(globalRequestCount, 0);
}

TEST(InlineNearlyEqualHandlesFloatingPointRounding) {
    CHECK(nearlyEqual(0.1 + 0.2, 0.3));
    CHECK(nearlyEqual(5.0, 5.0));
    CHECK(!nearlyEqual(1.0, 1.1));
    CHECK(!nearlyEqual(0.0, 1e-6));
}

TEST(ToRomanConvertsUsingInternalLookupTable) {
    CHECK_EQ(toRoman(1), std::string("I"));
    CHECK_EQ(toRoman(4), std::string("IV"));
    CHECK_EQ(toRoman(9), std::string("IX"));
    CHECK_EQ(toRoman(40), std::string("XL"));
    CHECK_EQ(toRoman(58), std::string("LVIII"));
    CHECK_EQ(toRoman(1994), std::string("MCMXCIV"));
    CHECK_EQ(toRoman(3999), std::string("MMMCMXCIX"));
}

TEST_MAIN()
