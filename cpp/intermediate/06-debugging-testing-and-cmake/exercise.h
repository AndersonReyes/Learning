#pragma once

#include <cstddef>
#include <stdexcept>
#include <vector>

// Topic 17-18 (Intermediate 06): Debugging, Sanitizers, Testing & CMake
//
// This topic adds CMakeLists.txt (sibling to this file) -- the first in this
// track. Build with:
//
//   cmake -S . -B build && cmake --build build && ./build/exercise_test
//
// or, with AddressSanitizer + UndefinedBehaviorSanitizer enabled:
//
//   cmake -S . -B build-asan -DENABLE_SANITIZERS=ON && cmake --build build-asan && ./build-asan/exercise_test
//
// The plain g++ invocation from earlier topics still works too:
//
//   g++ -std=c++20 -Wall -Wextra -o test exercise_test.cpp exercise.cpp && ./test
//
// All five exercises operate on std::vector<int> by index. operator[] does
// NOT bounds-check -- an off-by-one is silent undefined behavior without
// tooling, but is caught immediately by -fsanitize=address (out-of-bounds
// access) or -fsanitize=undefined (e.g. division/modulo by zero, signed
// overflow). See notes.md for what each sanitizer catches.
//
// Stub bodies throw std::logic_error("not implemented").

// --- rotateLeft: in-place left rotation ---------------------------------------------------------
//
// Rotates v left by k positions, in place. k may be >= v.size() (use
// k % v.size()) or 0. An empty vector is left unchanged for any k -- note
// that k % v.size() would be division-by-zero if v is empty, so that case
// must be handled before the modulo.
//
// Example: rotateLeft({1,2,3,4,5}, 2) leaves v == {3,4,5,1,2}.
// rotateLeft({1,2,3}, 3) leaves v == {1,2,3} (k % 3 == 0).
// rotateLeft({}, 5) leaves v == {} (no-op).
void rotateLeft(std::vector<int>& v, size_t k);

// --- isPalindrome: two-pointer scan -------------------------------------------------------------
//
// True iff v reads the same forwards and backwards. An empty vector and a
// single-element vector are both palindromes. Computing v.size() - 1 when
// v is empty would underflow (size_t is unsigned) -- handle the empty case
// first.
//
// Example: isPalindrome({1,2,3,2,1}) == true; isPalindrome({1,2,3}) == false;
// isPalindrome({}) == true; isPalindrome({7}) == true.
bool isPalindrome(const std::vector<int>& v);

// --- partitionByPivot: reorder around a pivot value ----------------------------------------------
//
// Reorders v in place so every element < pivot comes before every element >=
// pivot (the relative order WITHIN each group is unspecified). Returns the
// number of elements < pivot (i.e. the boundary index between the two
// groups). The multiset of elements is unchanged.
//
// Example: partitionByPivot({5,1,4,2,8,0,3}, 4) returns 4, and afterwards v's
// first 4 elements are some permutation of {1,2,0,3} and the last 3 are some
// permutation of {5,4,8}.
size_t partitionByPivot(std::vector<int>& v, int pivot);

// --- mergeSorted: merge two sorted ranges -------------------------------------------------------
//
// Returns a new vector containing all elements of `a` and `b` (both assumed
// sorted ascending, duplicates allowed), merged into one sorted sequence.
//
// Example: mergeSorted({1,3,5,7}, {2,4,6}) == {1,2,3,4,5,6,7};
// mergeSorted({}, {1,2,3}) == {1,2,3};
// mergeSorted({1,2,2,5}, {2,3}) == {1,2,2,2,3,5}.
std::vector<int> mergeSorted(const std::vector<int>& a, const std::vector<int>& b);

// --- findDuplicate: Floyd's cycle detection on an implicit linked list ---------------------------
//
// `nums` has n+1 elements (n >= 1), each in [1, n], with exactly one value
// duplicated (it may appear more than twice; every other value appears
// exactly once). Returns the duplicated value, in O(n) time and O(1) extra
// space, by treating `nums` as a function i -> nums[i] and finding the cycle
// it forms (Floyd's Tortoise and Hare) -- the same algorithm used to detect
// a cycle in a linked list.
//
// Example: findDuplicate({1,3,4,2,2}) == 2; findDuplicate({1,2,3,4,4}) == 4;
// findDuplicate({3,1,3,3,2}) == 3 (duplicated 3 times).
int findDuplicate(const std::vector<int>& nums);
