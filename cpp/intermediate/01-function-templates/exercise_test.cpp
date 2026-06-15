#include <string>

#include "exercise.h"
#include "../../testing.h"

TEST(ClampWithIntegers) {
    CHECK_EQ(clampValue(5, 0, 10), 5);
    CHECK_EQ(clampValue(-3, 0, 10), 0);
    CHECK_EQ(clampValue(15, 0, 10), 10);
    CHECK_EQ(clampValue(0, 0, 10), 0);    // boundary: value == low
    CHECK_EQ(clampValue(10, 0, 10), 10);  // boundary: value == high
}

TEST(ClampWithDoublesAndStrings) {
    CHECK_EQ(clampValue(0.5, 0.0, 1.0), 0.5);
    CHECK_EQ(clampValue(2.5, 0.0, 1.0), 1.0);
    CHECK_EQ(clampValue(-0.5, 0.0, 1.0), 0.0);

    CHECK_EQ(clampValue(std::string("m"), std::string("a"), std::string("z")), std::string("m"));
    CHECK_EQ(clampValue(std::string("zzz"), std::string("a"), std::string("z")), std::string("z"));
    CHECK_EQ(clampValue(std::string("0"), std::string("a"), std::string("z")), std::string("a"));
}

TEST(AddValuesDeducesReturnTypeFromOperandTypes) {
    CHECK_EQ(addValues(2, 3), 5);        // int + int -> int
    CHECK_EQ(addValues(2, 3.5), 5.5);    // int + double -> double
    CHECK_EQ(addValues(2.5, 3), 5.5);    // double + int -> double
    CHECK_EQ(addValues(2.5, 2.5), 5.0);  // double + double -> double
}

TEST(PowerWithNonTypeTemplateParameter) {
    CHECK_EQ(power<0>(100), 1LL);
    CHECK_EQ(power<0>(0), 1LL);  // 0^0 == 1 by convention
    CHECK_EQ(power<1>(7), 7LL);
    CHECK_EQ(power<2>(5), 25LL);
    CHECK_EQ(power<10>(2), 1024LL);
    CHECK_EQ(power<3>(-2), -8LL);  // negative base, odd exponent
}

TEST(SumAllVariadicFoldExpression) {
    CHECK_EQ(sumAll(), 0LL);
    CHECK_EQ(sumAll(1, 2, 3), 6LL);
    CHECK_EQ(sumAll(1, 2.9, 3), 6LL);     // 2.9 truncates to 2 via static_cast<long long>
    CHECK_EQ(sumAll(-5), -5LL);
    CHECK_EQ(sumAll(1.9, 1.9, 1.9), 3LL);  // each truncates to 1 -> 1+1+1
}

TEST(TypeCategoryUsesIfConstexprAndTypeTraits) {
    CHECK_EQ(typeCategory(4), std::string("integral:even"));
    CHECK_EQ(typeCategory(7), std::string("integral:odd"));
    CHECK_EQ(typeCategory(4.0), std::string("floating-point:whole"));
    CHECK_EQ(typeCategory(4.5), std::string("floating-point:fractional"));
    CHECK_EQ(typeCategory(std::string("hi")), std::string("other"));
}

TEST_MAIN()
