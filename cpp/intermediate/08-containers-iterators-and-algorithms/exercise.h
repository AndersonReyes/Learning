#pragma once

#include <cstddef>
#include <iterator>
#include <stdexcept>
#include <string>
#include <utility>
#include <vector>

// Topic 20 (Intermediate 08): Containers, Iterators, Algorithms & Ranges
//
// Five exercises spanning the rest of the STL: associative containers
// (<unordered_map>), <algorithm> (sort with custom comparators), <deque> for
// an O(n) sliding-window algorithm, writing your OWN forward iterator, and
// C++20 <ranges> views/pipelines. Stub bodies throw
// std::logic_error("not implemented").

// --- topKFrequent: <unordered_map> + std::sort with a custom comparator --------------------------
//
// Returns the `k` most frequent words in `words`, ordered by descending
// frequency; ties broken by ascending lexicographical order. If `k` exceeds
// the number of distinct words, returns ALL distinct words (so the result
// has min(k, distinct words) elements). `k` is assumed >= 0.
//
// Example: topKFrequent({"apple","banana","apple","cherry","banana","apple","date"}, 2)
//   counts: apple=3, banana=2, cherry=1, date=1
//   == {"apple", "banana"}
// topKFrequent({...same...}, 3) == {"apple", "banana", "cherry"}  // 1-1 tie -> "cherry" < "date"
// topKFrequent({...same...}, 10) == {"apple", "banana", "cherry", "date"}  // k > distinct
std::vector<std::string> topKFrequent(const std::vector<std::string>& words, int k);

// --- mergeIntervals: std::sort on std::pair, then a linear merge --------------------------------
//
// `intervals` is a vector of closed [start, end] intervals (start <= end),
// in arbitrary order. Returns the minimal set of closed intervals covering
// the same points, sorted by start. Two intervals merge if they overlap OR
// touch (i.e. `b.start <= a.end`).
//
// Example: mergeIntervals({{8,10},{1,3},{2,6},{15,18}}) == {{1,6},{8,10},{15,18}}
// mergeIntervals({{1,4},{4,5}}) == {{1,5}}              // touching intervals merge
// mergeIntervals({{1,10},{2,5},{6,8}}) == {{1,10}}      // nested intervals absorbed
// mergeIntervals({}) == {}
// mergeIntervals({{5,5}}) == {{5,5}}
std::vector<std::pair<int, int>> mergeIntervals(std::vector<std::pair<int, int>> intervals);

// --- slidingWindowMaximum: <deque>-based O(n) monotonic-deque algorithm -------------------------
//
// Returns, for each contiguous window of `k` consecutive elements in `nums`
// (windows starting at indices 0 through nums.size()-k inclusive), the
// maximum value in that window -- in O(n) total via a std::deque<size_t> of
// indices kept in decreasing order of nums[index] (a "monotonic deque"):
// pop expired-front indices, pop back indices whose value <= nums[i], push
// i, then the window max is nums[deque.front()].
// Precondition: 1 <= k <= nums.size().
//
// Example: slidingWindowMaximum({1,3,-1,-3,5,3,6,7}, 3) == {3,3,5,5,6,7}
// slidingWindowMaximum({1,3,-1,-3,5,3,6,7}, 1) == {1,3,-1,-3,5,3,6,7}  // k=1: every element
// slidingWindowMaximum({1,3,-1,-3,5,3,6,7}, 8) == {7}                  // k==size: one window
std::vector<int> slidingWindowMaximum(const std::vector<int>& nums, int k);

// --- IntStrideView: writing your own forward iterator --------------------------------------------
//
// A read-only, non-owning view over a std::vector<int> that iterates every
// `stride`-th element starting at index 0 (indices 0, stride, 2*stride,
// ...) without copying the underlying data. begin()/end() return a custom
// forward-iterator type (operator*, operator++, operator==, operator!=) so
// IntStrideView works in range-based for loops and with <algorithm>/
// <numeric> functions like std::accumulate -- e.g.
// std::accumulate(view.begin(), view.end(), 0).
//
// `end()` must compare equal to any iterator whose index has advanced past
// data.size() -- not just one with an identical index -- since `stride`
// need not evenly divide data.size() (operator== should treat "index >=
// size" as a single "done" state).
//
// Precondition: stride >= 1. The vector referenced by `data` must outlive
// the IntStrideView (and any iterators obtained from it).
//
// Example: data = {10,20,30,40,50,60,70}
//   IntStrideView(data, 3) yields 10, 40, 70
//   IntStrideView(data, 1) yields the whole vector
//   IntStrideView(data, 10) yields just 10 (stride >= size -> one element)
//   IntStrideView({}, 3) yields nothing (begin() == end())
class IntStrideView {
public:
    IntStrideView(const std::vector<int>& data, size_t stride);

    class Iterator {
    public:
        using iterator_category = std::forward_iterator_tag;
        using value_type = int;
        using difference_type = std::ptrdiff_t;
        using pointer = const int*;
        using reference = int;

        Iterator(const std::vector<int>* data, size_t index, size_t stride);

        int operator*() const;
        Iterator& operator++();
        bool operator==(const Iterator& other) const;
        bool operator!=(const Iterator& other) const;

    private:
        const std::vector<int>* data_;
        size_t index_;
        size_t stride_;
    };

    Iterator begin() const;
    Iterator end() const;

private:
    const std::vector<int>* data_;
    size_t stride_;
};

// --- primesUpTo: C++20 <ranges> views::iota | views::filter pipeline ----------------------------
//
// Returns all prime numbers <= n, in ascending order, generated via a
// std::views::iota(2, n + 1) | std::views::filter(isPrime) pipeline (an
// isPrime helper is an implementation detail, not part of this header) --
// materialized into a std::vector<int> by iterating the resulting view.
//
// Example: primesUpTo(20) == {2,3,5,7,11,13,17,19}
// primesUpTo(2) == {2}
// primesUpTo(1) == {}   // and primesUpTo(0), primesUpTo(-5)
std::vector<int> primesUpTo(int n);
