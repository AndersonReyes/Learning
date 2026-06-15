#include <numeric>
#include <string>
#include <utility>
#include <vector>

#include "exercise.h"
#include "../../testing.h"

TEST(TopKFrequentBreaksTiesLexicographically) {
    std::vector<std::string> words = {"apple", "banana", "apple", "cherry",
                                       "banana", "apple",  "date"};
    // counts: apple=3, banana=2, cherry=1, date=1
    CHECK(topKFrequent(words, 2) == (std::vector<std::string>{"apple", "banana"}));
    CHECK(topKFrequent(words, 3) == (std::vector<std::string>{"apple", "banana", "cherry"}));
    CHECK(topKFrequent(words, 10) ==
          (std::vector<std::string>{"apple", "banana", "cherry", "date"}));
    CHECK(topKFrequent(words, 0) == std::vector<std::string>{});
    CHECK(topKFrequent({}, 3) == std::vector<std::string>{});

    std::vector<std::string> single = {"x", "x", "x"};
    CHECK(topKFrequent(single, 1) == (std::vector<std::string>{"x"}));
}

TEST(MergeIntervalsMergesOverlappingAndTouching) {
    CHECK(mergeIntervals({{8, 10}, {1, 3}, {2, 6}, {15, 18}}) ==
          (std::vector<std::pair<int, int>>{{1, 6}, {8, 10}, {15, 18}}));
    CHECK(mergeIntervals({{1, 4}, {4, 5}}) == (std::vector<std::pair<int, int>>{{1, 5}}));
    CHECK(mergeIntervals({{1, 10}, {2, 5}, {6, 8}}) ==
          (std::vector<std::pair<int, int>>{{1, 10}}));
    CHECK(mergeIntervals({}) == (std::vector<std::pair<int, int>>{}));
    CHECK(mergeIntervals({{5, 5}}) == (std::vector<std::pair<int, int>>{{5, 5}}));
}

TEST(SlidingWindowMaximumHandlesAllWindowSizes) {
    std::vector<int> nums = {1, 3, -1, -3, 5, 3, 6, 7};
    CHECK(slidingWindowMaximum(nums, 3) == (std::vector<int>{3, 3, 5, 5, 6, 7}));
    CHECK(slidingWindowMaximum(nums, 1) == nums);
    CHECK(slidingWindowMaximum(nums, 8) == (std::vector<int>{7}));
}

TEST(IntStrideViewIteratesEveryStrideElement) {
    std::vector<int> data = {10, 20, 30, 40, 50, 60, 70};

    IntStrideView byThree(data, 3);
    CHECK((std::vector<int>(byThree.begin(), byThree.end())) == (std::vector<int>{10, 40, 70}));
    CHECK(std::accumulate(byThree.begin(), byThree.end(), 0) == 120);

    IntStrideView byOne(data, 1);
    CHECK((std::vector<int>(byOne.begin(), byOne.end())) == data);

    IntStrideView byTen(data, 10);
    CHECK((std::vector<int>(byTen.begin(), byTen.end())) == (std::vector<int>{10}));

    std::vector<int> empty;
    IntStrideView byThreeEmpty(empty, 3);
    CHECK(byThreeEmpty.begin() == byThreeEmpty.end());
    CHECK((std::vector<int>(byThreeEmpty.begin(), byThreeEmpty.end())) == std::vector<int>{});
}

TEST(PrimesUpToFiltersWithRangesPipeline) {
    CHECK(primesUpTo(20) == (std::vector<int>{2, 3, 5, 7, 11, 13, 17, 19}));
    CHECK(primesUpTo(2) == (std::vector<int>{2}));
    CHECK(primesUpTo(1) == std::vector<int>{});
    CHECK(primesUpTo(0) == std::vector<int>{});
    CHECK(primesUpTo(-5) == std::vector<int>{});
}

TEST_MAIN()
