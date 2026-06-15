# Intermediate 08: Containers, Iterators, Algorithms & Ranges

The rest of the STL "big picture": container categories beyond `std::vector`,
the iterator abstraction that lets `<algorithm>` work generically across all
of them, and C++20 `<ranges>` as a lazy, composable layer on top.

## Container categories

- **Sequence containers** -- ordered by position, not key: `std::vector`
  (contiguous, default choice), `std::deque` (double-ended queue, O(1)
  push/pop at both ends, NOT contiguous), `std::list` (doubly-linked,
  O(1) insert/erase anywhere but O(n) access), `std::array` (fixed-size,
  stack-allocated).
- **Associative containers** -- sorted by key, O(log n) operations:
  `std::map`/`std::set` (unique keys), `std::multimap`/`std::multiset`
  (duplicate keys allowed). Backed by a balanced tree -- iteration order is
  key order.
- **Unordered associative containers** -- hash-based, O(1) average:
  `std::unordered_map`/`std::unordered_set` (+ `multi` variants). No
  ordering guarantee; iteration order can change after insertion (rehashing).

```cpp
std::unordered_map<std::string, int> counts;
for (const auto& w : words) counts[w]++;   // operator[] inserts 0 if absent, then ++

for (const auto& [word, n] : counts) { ... }  // structured bindings over pairs
```

Pick `unordered_map` for pure lookup/counting (no ordering needed);
`map`/`set` when you need sorted iteration or `lower_bound`/`upper_bound`.

## `<deque>`: O(1) at both ends

A `std::deque` ("double-ended queue") supports O(1) `push_back`,
`push_front`, `pop_back`, `pop_front`, plus O(1) random access via
`operator[]` -- but, unlike `vector`, is NOT stored as one contiguous block
(it's a sequence of fixed-size chunks), so pointers/iterators to its
elements are less stable across mutations and there's no `.data()`.

**Monotonic deque** -- a classic technique for sliding-window-maximum-style
problems: maintain a deque of *indices* such that `nums[deque[i]]` is
strictly decreasing. For each new index `i`:

1. Pop front indices that have fallen out of the window.
2. Pop back indices whose value <= `nums[i]` (they can never be the max
   again, since `i` is later AND >= them).
3. Push `i`.
4. `nums[deque.front()]` is the current window's maximum.

Each index is pushed and popped at most once -> O(n) total, vs. O(n*k) for
recomputing the max of every window.

## `<algorithm>`: custom comparators

Most `<algorithm>` functions take an optional comparator (a callable
`(a, b) -> bool` meaning "a should come before b") as their last argument.

```cpp
std::vector<std::pair<std::string, int>> items = ...;
std::sort(items.begin(), items.end(), [](const auto& a, const auto& b) {
    if (a.second != b.second) return a.second > b.second;  // freq descending
    return a.first < b.first;                              // then name ascending
});
```

**Gotcha**: a comparator must define a *strict weak ordering* --
`cmp(a, a)` must be `false`. `return a.second >= b.second` would violate
this (equal elements would compare "less than" themselves), causing UB
(usually a crash or infinite loop in `std::sort`'s implementation). Always
use `>`/`<`, never `>=`/`<=`, for the "primary" comparison, falling through
to a secondary key only on exact equality.

`std::pair`'s `operator<` already does this correctly for two-key sorts:
`std::sort(intervals.begin(), intervals.end())` on
`vector<pair<int,int>>` sorts by `.first`, breaking ties by `.second` --
exactly lexicographic pair comparison, no comparator needed.

## The iterator abstraction

An iterator is a generalized pointer: `*it` reads, `++it` advances, `it ==
end` checks termination. `<algorithm>`/`<numeric>` functions are written
generically against iterators, not containers -- `std::accumulate(first,
last, init)` works identically whether `first`/`last` come from a `vector`,
a `list`, a `set`, or a hand-rolled type.

**Iterator categories** (each a superset of the previous):
- *Input* -- single-pass, read-only (`*it`, `++it`).
- *Forward* -- input + multi-pass (can copy the iterator and traverse twice).
- *Bidirectional* -- forward + `--it`.
- *Random access* -- bidirectional + `it + n`, `it[n]`, `it1 - it2` in O(1).

`std::vector`'s iterators are random-access; `std::list`'s are
bidirectional; `std::forward_list`'s are forward-only. Algorithms that need
more (e.g. `std::sort` needs random access) won't compile for containers
that provide less.

### Writing your own forward iterator

To make a custom type work with range-based `for` and `<algorithm>`, its
iterator type needs:

```cpp
class Iterator {
public:
    using iterator_category = std::forward_iterator_tag;
    using value_type = int;
    using difference_type = std::ptrdiff_t;
    using pointer = const int*;
    using reference = int;

    int operator*() const { ... }
    Iterator& operator++() { ... }            // prefix
    bool operator==(const Iterator&) const { ... }
    bool operator!=(const Iterator&) const { ... }
};
```

The five `using` aliases are `std::iterator_traits` boilerplate -- generic
code (including `std::vector`'s iterator-range constructor and
`std::accumulate`) introspects them. `iterator_category` documents which
algorithms the type supports.

**The `end()` sentinel problem**: `end()` must compare equal to *any*
iterator that has advanced "past" the valid range -- not just one with an
identical internal state. If a custom iterator advances by a stride that
doesn't evenly divide the container's size, the "natural" past-the-end index
(`size()`, `size()+1`, ... depending on stride) won't match a single fixed
`end()` value unless `operator==` treats "index >= size" as one collapsed
"done" state:

```cpp
bool operator==(const Iterator& other) const {
    bool thisDone = index_ >= data_->size();
    bool otherDone = other.index_ >= other.data_->size();
    if (thisDone && otherDone) return true;   // both "done" -> equal,
    return index_ == other.index_;            // regardless of exact index
}
```

A container type (`IntStrideView` here) typically just owns a pointer/
reference to the underlying data plus a `begin()`/`end()` pair -- it's a
*view*, not a copy. Like `std::string_view` (topic 07), the viewed data must
outlive the view.

## C++20 `<ranges>`: lazy, composable views

`<ranges>` adds *views* -- lazy, non-owning adaptors over a range, composed
with `|` (pipe), evaluated element-by-element on demand (nothing is
materialized until iterated):

```cpp
#include <ranges>

for (int x : std::views::iota(2, 20) | std::views::filter(isPrime)) {
    // x: 2, 3, 5, 7, 11, 13, 17, 19 -- computed lazily, one at a time
}
```

- `std::views::iota(a, b)` -- lazy range of `a, a+1, ..., b-1` (like
  Python's `range`). `iota(a)` (one argument) is unbounded.
- `std::views::filter(pred)` -- only elements where `pred(x)` is true.
- `std::views::transform(fn)` -- lazily applies `fn` to each element.
- `std::views::take(n)` / `std::views::drop(n)` -- first/skip `n` elements.
- `std::views::reverse` -- reverse iteration (requires bidirectional range).

Pipelines read left-to-right as a sequence of transformations:
`data | std::views::filter(p) | std::views::transform(f)` -- "filter `data`
by `p`, then transform the result by `f`".

**Materializing**: a view is not a container -- to get a `std::vector`,
iterate it (range-based `for`, pushing into a vector) or use
`std::ranges::copy(view, std::back_inserter(result))`. (C++23 adds
`std::ranges::to<std::vector>()` for this directly; not available in C++20.)

**Laziness gotcha**: views capture their *predicate/transform callables and
underlying range by reference or value* depending on how they're
constructed -- a view over a temporary or a reference that goes out of scope
before the view is consumed is a dangling-reference bug, the same lifetime
hazard as `string_view`.

## Further Reading

- [Modern C++ Programming, ch. 20: Iterators, Containers & Algorithms](https://federico-busato.github.io/Modern-CPP-Programming/htmls/20.Iterators_Containers_Alg.html)
- [cppreference: Containers library](https://en.cppreference.com/w/cpp/container)
- [cppreference: std::deque](https://en.cppreference.com/w/cpp/container/deque)
- [cppreference: std::unordered_map](https://en.cppreference.com/w/cpp/container/unordered_map)
- [cppreference: Iterator library](https://en.cppreference.com/w/cpp/iterator)
- [cppreference: LegacyForwardIterator](https://en.cppreference.com/w/cpp/named_req/ForwardIterator)
- [cppreference: <algorithm>](https://en.cppreference.com/w/cpp/algorithm)
- [cppreference: <numeric> (std::accumulate)](https://en.cppreference.com/w/cpp/header/numeric)
- [cppreference: Ranges library](https://en.cppreference.com/w/cpp/ranges)
- [cppreference: std::views::iota](https://en.cppreference.com/w/cpp/ranges/iota_view)
- [cppreference: std::views::filter](https://en.cppreference.com/w/cpp/ranges/filter_view)
