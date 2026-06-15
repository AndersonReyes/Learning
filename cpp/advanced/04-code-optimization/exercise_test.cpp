#include "exercise.h"

#include <algorithm>
#include <array>
#include <cmath>
#include <limits>
#include <stdexcept>
#include <vector>

#include "../../testing.h"

// --- branchlessAbs / branchlessMin / branchlessMax ------------------------------------------------

TEST(BranchlessArithmetic) {
    CHECK_EQ(branchlessAbs(5), 5);
    CHECK_EQ(branchlessAbs(-5), 5);
    CHECK_EQ(branchlessAbs(0), 0);
    CHECK_EQ(branchlessAbs(std::numeric_limits<int>::max()), std::numeric_limits<int>::max());
    // -INT_MIN isn't representable as int -- the bit pattern is unchanged,
    // same as std::abs on two's-complement hardware.
    CHECK_EQ(branchlessAbs(std::numeric_limits<int>::min()), std::numeric_limits<int>::min());

    CHECK_EQ(branchlessMin(3, -2), -2);
    CHECK_EQ(branchlessMax(3, -2), 3);
    CHECK_EQ(branchlessMin(7, 7), 7);
    CHECK_EQ(branchlessMax(7, 7), 7);
    CHECK_EQ(branchlessMin(-10, -20), -20);
    CHECK_EQ(branchlessMax(-10, -20), -10);
    CHECK_EQ(branchlessMin(std::numeric_limits<int>::min(), std::numeric_limits<int>::max()),
             std::numeric_limits<int>::min());
    CHECK_EQ(branchlessMax(std::numeric_limits<int>::min(), std::numeric_limits<int>::max()),
             std::numeric_limits<int>::max());
}

// --- countSetBits / reverseBits ------------------------------------------------------------------

TEST(BitManipulationTricks) {
    CHECK_EQ(countSetBits(0u), 0u);
    CHECK_EQ(countSetBits(1u), 1u);
    CHECK_EQ(countSetBits(0xFFFFFFFFu), 32u);
    CHECK_EQ(countSetBits(0x80000000u), 1u);
    CHECK_EQ(countSetBits(0xB6u), 5u);         // 0xB6 == 0b10110110 -> five 1 bits
    CHECK_EQ(countSetBits(0x12345678u), 13u);  // popcount of 0001 0010 0011 0100 0101 0110 0111 1000

    CHECK_EQ(reverseBits(0u), 0u);
    CHECK_EQ(reverseBits(0xFFFFFFFFu), 0xFFFFFFFFu);
    CHECK_EQ(reverseBits(1u), 0x80000000u);
    CHECK_EQ(reverseBits(0x80000000u), 1u);

    for (unsigned int x : {0x12345678u, 0xDEADBEEFu, 12345u, 1u, 0u}) {
        CHECK_EQ(reverseBits(reverseBits(x)), x);
    }
}

// --- computeStats ----------------------------------------------------------------------------------

TEST(ComputeStatsSinglePass) {
    Stats s1 = computeStats({2, 4, 4, 4, 5, 5, 7, 9});
    CHECK(std::abs(s1.sum - 40.0) < 1e-9);
    CHECK(std::abs(s1.mean - 5.0) < 1e-9);
    CHECK(std::abs(s1.min - 2.0) < 1e-9);
    CHECK(std::abs(s1.max - 9.0) < 1e-9);
    CHECK(std::abs(s1.variance - 4.0) < 1e-9);

    Stats s2 = computeStats({5});
    CHECK(std::abs(s2.sum - 5.0) < 1e-9);
    CHECK(std::abs(s2.mean - 5.0) < 1e-9);
    CHECK(std::abs(s2.min - 5.0) < 1e-9);
    CHECK(std::abs(s2.max - 5.0) < 1e-9);
    CHECK(std::abs(s2.variance - 0.0) < 1e-9);

    Stats s3 = computeStats({-1, -2, -3});
    CHECK(std::abs(s3.sum - (-6.0)) < 1e-9);
    CHECK(std::abs(s3.mean - (-2.0)) < 1e-9);
    CHECK(std::abs(s3.min - (-3.0)) < 1e-9);
    CHECK(std::abs(s3.max - (-1.0)) < 1e-9);
    CHECK(std::abs(s3.variance - (2.0 / 3.0)) < 1e-9);

    bool threw = false;
    try {
        computeStats({});
    } catch (const std::invalid_argument&) {
        threw = true;
    }
    CHECK(threw);
}

// --- StepCounter -----------------------------------------------------------------------------------

TEST(MemoizedStepCounter) {
    StepCounter sc;
    CHECK_EQ(sc.computeCount(), static_cast<size_t>(0));

    CHECK_EQ(sc.countWays(0), 1LL);
    CHECK_EQ(sc.countWays(1), 1LL);
    CHECK_EQ(sc.countWays(2), 2LL);
    CHECK_EQ(sc.countWays(3), 4LL);

    StepCounter sc2;
    CHECK_EQ(sc2.countWays(10), 274LL);
    CHECK_EQ(sc2.computeCount(), static_cast<size_t>(11));  // n=0..10, each computed once

    // Repeating the same call is a cache hit -- no new computations.
    CHECK_EQ(sc2.countWays(10), 274LL);
    CHECK_EQ(sc2.computeCount(), static_cast<size_t>(11));

    // Negative n is always 0 by definition and never memoized/counted.
    CHECK_EQ(sc2.countWays(-1), 0LL);
    CHECK_EQ(sc2.computeCount(), static_cast<size_t>(11));

    // A larger n reuses the cached 0..10 and computes only 11..15 (5 new values).
    CHECK_EQ(sc2.countWays(15), 5768LL);
    CHECK_EQ(sc2.computeCount(), static_cast<size_t>(16));
}

// --- sortNetwork4 ----------------------------------------------------------------------------------

TEST(SortNetwork4) {
    CHECK(sortNetwork4({4, 3, 2, 1}) == (std::array<int, 4>{1, 2, 3, 4}));
    CHECK(sortNetwork4({1, 2, 3, 4}) == (std::array<int, 4>{1, 2, 3, 4}));
    CHECK(sortNetwork4({2, 2, 1, 1}) == (std::array<int, 4>{1, 1, 2, 2}));
    CHECK(sortNetwork4({3, 1, 2, 4}) == (std::array<int, 4>{1, 2, 3, 4}));
    CHECK(sortNetwork4({-5, 10, 0, -1}) == (std::array<int, 4>{-5, -1, 0, 10}));

    // Every permutation of {1,2,3,4} must sort to the same result.
    std::array<int, 4> perm = {1, 2, 3, 4};
    const std::array<int, 4> expected = {1, 2, 3, 4};
    do {
        CHECK(sortNetwork4(perm) == expected);
    } while (std::next_permutation(perm.begin(), perm.end()));
}

TEST_MAIN()
