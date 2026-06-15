#include <algorithm>
#include <vector>

#include "exercise.h"
#include "../../testing.h"

TEST(RotateLeftHandlesWraparoundAndEmpty) {
    std::vector<int> v = {1, 2, 3, 4, 5};
    rotateLeft(v, 2);
    CHECK(v == (std::vector<int>{3, 4, 5, 1, 2}));

    std::vector<int> v2 = {1, 2, 3};
    rotateLeft(v2, 3);  // k % 3 == 0 -- unchanged
    CHECK(v2 == (std::vector<int>{1, 2, 3}));

    std::vector<int> v3 = {1, 2, 3, 4, 5};
    rotateLeft(v3, 7);  // 7 % 5 == 2 -- same as rotating by 2
    CHECK(v3 == (std::vector<int>{3, 4, 5, 1, 2}));

    std::vector<int> v4 = {};
    rotateLeft(v4, 5);  // empty: must not divide by zero
    CHECK(v4.empty());
}

TEST(IsPalindromeHandlesEdgeCases) {
    CHECK(isPalindrome({1, 2, 3, 2, 1}));
    CHECK(!isPalindrome({1, 2, 3}));
    CHECK(isPalindrome({}));   // empty: vacuously true, no v.size() - 1 underflow
    CHECK(isPalindrome({7}));  // single element
    CHECK(isPalindrome({1, 1}));
    CHECK(!isPalindrome({1, 2}));
}

TEST(PartitionByPivotGroupsElementsAndPreservesMultiset) {
    std::vector<int> v = {5, 1, 4, 2, 8, 0, 3};
    std::vector<int> sortedBefore = v;
    std::sort(sortedBefore.begin(), sortedBefore.end());

    size_t boundary = partitionByPivot(v, 4);
    CHECK_EQ(boundary, size_t(4));
    for (size_t i = 0; i < boundary; ++i) CHECK(v[i] < 4);
    for (size_t i = boundary; i < v.size(); ++i) CHECK(v[i] >= 4);

    std::vector<int> sortedAfter = v;
    std::sort(sortedAfter.begin(), sortedAfter.end());
    CHECK(sortedAfter == sortedBefore);

    std::vector<int> allLow = {1, 2, 3};
    CHECK_EQ(partitionByPivot(allLow, 10), size_t(3));

    std::vector<int> allHigh = {10, 20, 30};
    CHECK_EQ(partitionByPivot(allHigh, 5), size_t(0));

    std::vector<int> empty;
    CHECK_EQ(partitionByPivot(empty, 5), size_t(0));
}

TEST(MergeSortedMergesAndPreservesDuplicates) {
    CHECK(mergeSorted({1, 3, 5, 7}, {2, 4, 6}) == (std::vector<int>{1, 2, 3, 4, 5, 6, 7}));
    CHECK(mergeSorted({}, {1, 2, 3}) == (std::vector<int>{1, 2, 3}));
    CHECK(mergeSorted({1, 2, 3}, {}) == (std::vector<int>{1, 2, 3}));
    CHECK(mergeSorted({1, 2, 2, 5}, {2, 3}) == (std::vector<int>{1, 2, 2, 2, 3, 5}));
    CHECK(mergeSorted({}, {}) == std::vector<int>{});
}

TEST(FindDuplicateUsesFloydsCycleDetection) {
    CHECK_EQ(findDuplicate({1, 3, 4, 2, 2}), 2);
    CHECK_EQ(findDuplicate({1, 2, 3, 4, 4}), 4);
    CHECK_EQ(findDuplicate({2, 2, 1}), 2);
    CHECK_EQ(findDuplicate({3, 1, 3, 3, 2}), 3);  // duplicated 3 times
}

TEST_MAIN()
