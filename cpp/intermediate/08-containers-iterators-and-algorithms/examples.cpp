#include <algorithm>
#include <deque>
#include <iostream>
#include <iterator>
#include <map>
#include <numeric>
#include <ranges>
#include <set>
#include <string>
#include <unordered_map>
#include <vector>

// --- Container categories: unordered_map, map, set, deque ------------------

void containerDemo() {
    std::cout << "-- container categories --\n";

    // unordered_map: O(1) average lookup/insert, no ordering guarantee.
    std::unordered_map<std::string, int> wordCounts;
    for (const std::string w : {"a", "b", "a", "c", "b", "a"}) wordCounts[w]++;
    std::cout << "unordered_map counts: a=" << wordCounts["a"] << " b=" << wordCounts["b"]
              << " c=" << wordCounts["c"] << "\n";

    // map: sorted by key, ordered iteration "for free".
    std::map<std::string, int> sortedCounts(wordCounts.begin(), wordCounts.end());
    std::cout << "map (sorted by key): ";
    for (const auto& [word, count] : sortedCounts) std::cout << word << "=" << count << " ";
    std::cout << "\n";

    // set: sorted, unique elements.
    std::set<int> uniqueValues = {5, 3, 1, 3, 5, 2};
    std::cout << "set (sorted, unique): ";
    for (int v : uniqueValues) std::cout << v << " ";
    std::cout << "\n";

    // deque: O(1) push/pop at both ends.
    std::deque<int> dq = {2, 3, 4};
    dq.push_front(1);
    dq.push_back(5);
    std::cout << "deque after push_front(1), push_back(5): ";
    for (int v : dq) std::cout << v << " ";
    std::cout << "\n\n";
}

// --- <algorithm>: custom comparators ----------------------------------------

struct Person {
    std::string name;
    int age;
};

void algorithmDemo() {
    std::cout << "-- std::sort with custom comparators --\n";

    std::vector<Person> people = {{"Alice", 30}, {"Bob", 25}, {"Carol", 25}, {"Dave", 40}};

    // Sort by age ascending, ties broken by name ascending -- a strict weak
    // ordering: equal ages fall through to the name comparison, never `>=`.
    std::sort(people.begin(), people.end(), [](const Person& a, const Person& b) {
        if (a.age != b.age) return a.age < b.age;
        return a.name < b.name;
    });

    for (const Person& p : people) std::cout << p.name << "(" << p.age << ") ";
    std::cout << "\n\n";
}

// --- <deque>: sliding window MINIMUM via a monotonic deque -------------------

std::vector<int> slidingWindowMinimum(const std::vector<int>& nums, size_t k) {
    std::deque<size_t> indices;  // values strictly INCREASING (mirror of the maximum version)
    std::vector<int> result;
    for (size_t i = 0; i < nums.size(); ++i) {
        while (!indices.empty() && i >= k && indices.front() <= i - k) indices.pop_front();
        while (!indices.empty() && nums[indices.back()] >= nums[i]) indices.pop_back();
        indices.push_back(i);
        if (i + 1 >= k) result.push_back(nums[indices.front()]);
    }
    return result;
}

void monotonicDequeDemo() {
    std::cout << "-- monotonic deque: sliding window minimum --\n";
    std::vector<int> nums = {4, 2, 12, 3, 8, 1, 5};
    std::vector<int> mins = slidingWindowMinimum(nums, 3);
    std::cout << "window-of-3 minimums: ";
    for (int m : mins) std::cout << m << " ";
    std::cout << "\n\n";
}

// --- Writing a custom forward iterator: a Python-style integer range --------

class IntRange {
public:
    IntRange(int start, int stop, int step = 1) : start_(start), stop_(stop), step_(step) {}

    class Iterator {
    public:
        using iterator_category = std::forward_iterator_tag;
        using value_type = int;
        using difference_type = std::ptrdiff_t;
        using pointer = const int*;
        using reference = int;

        Iterator(int value, int step) : value_(value), step_(step) {}

        int operator*() const { return value_; }
        Iterator& operator++() {
            value_ += step_;
            return *this;
        }
        bool operator==(const Iterator& other) const { return value_ == other.value_; }
        bool operator!=(const Iterator& other) const { return !(*this == other); }

    private:
        int value_;
        int step_;
    };

    // For a positive step, iteration stops at the first value >= stop_; for
    // a negative step, at the first value <= stop_ -- both are expressed as
    // "round stop_ up/down to the next reachable value from start_", so
    // end() compares equal to the iterator that would be produced by
    // incrementing exactly that many times.
    Iterator begin() const { return Iterator(start_, step_); }
    Iterator end() const {
        int span = stop_ - start_;
        int steps = (span + step_ - (step_ > 0 ? 1 : -1)) / step_;
        if (steps < 0) steps = 0;
        return Iterator(start_ + steps * step_, step_);
    }

private:
    int start_, stop_, step_;
};

void customIteratorDemo() {
    std::cout << "-- custom forward iterator: IntRange --\n";

    IntRange r(0, 10, 2);
    std::cout << "IntRange(0, 10, 2): ";
    for (int x : r) std::cout << x << " ";
    std::cout << "\n";

    // Works with <numeric>/<algorithm> like any other forward range.
    std::cout << "sum = " << std::accumulate(r.begin(), r.end(), 0) << "\n";

    std::vector<int> collected(r.begin(), r.end());
    std::cout << "collected into vector, size = " << collected.size() << "\n\n";
}

// --- C++20 <ranges>: filter | transform | take pipelines ---------------------

void rangesDemo() {
    std::cout << "-- <ranges>: filter | transform | take --\n";

    std::vector<int> nums = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12};

    // First 3 squares of the even numbers in `nums`.
    auto evenSquares = nums | std::views::filter([](int x) { return x % 2 == 0; }) |
                        std::views::transform([](int x) { return x * x; }) | std::views::take(3);

    std::cout << "first 3 even squares: ";
    for (int x : evenSquares) std::cout << x << " ";
    std::cout << "\n";

    // views::iota for an unbounded-ish counting sequence, lazily filtered.
    auto multiplesOfSeven =
        std::views::iota(1) | std::views::filter([](int x) { return x % 7 == 0; }) |
        std::views::take(4);
    std::cout << "first 4 multiples of 7: ";
    for (int x : multiplesOfSeven) std::cout << x << " ";
    std::cout << "\n\n";
}

int main() {
    containerDemo();
    algorithmDemo();
    monotonicDequeDemo();
    customIteratorDemo();
    rangesDemo();
}
