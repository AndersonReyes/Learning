#pragma once

#include <vector>

// Allocates a new array of `size` ints with `new int[size]` and fills it
// with the elements of `arr` in reverse order. The caller is responsible for
// freeing the result with `delete[]`. Returns `nullptr` (and allocates
// nothing) if `size <= 0`. Does not modify `arr`. Examples:
//   copyReversed({1, 2, 3}, 3) -> heap array {3, 2, 1}
//   copyReversed(arr, 0)       -> nullptr
int* copyReversed(const int* arr, int size);

// Rotates `arr` (length `size`) left by `k` positions in place, using O(1)
// extra space. Rotating left by 1 moves every element one slot toward index
// 0, with the front element wrapping around to the end:
//   [1,2,3,4,5] rotated left by 2 -> [3,4,5,1,2]
// `k` may be negative (rotates right) or have |k| >= size; the effective
// shift is `k` modulo `size`. No-op if `size <= 1`.
void rotateLeftInPlace(int* arr, int size, int k);

// Computes the minimum, maximum, and sum of `values` in a single pass,
// writing the results through the output reference parameters. If `values`
// is empty, `outMin` and `outMax` are left at 0 and `outSum` is left at 0.
// Example: values = {3, -1, 4, 1, 5} ->
//   outMin = -1, outMax = 5, outSum = 12
void minMaxSum(const std::vector<int>& values, int& outMin, int& outMax,
               long long& outSum);

// A node in a singly-linked list.
struct ListNode {
    int value;
    ListNode* next = nullptr;
};

// Merges two singly-linked lists, each already sorted in non-decreasing
// order, into one sorted list by relinking the existing nodes -- no new
// nodes are allocated. Returns a pointer to the head of the merged list.
// Either `a` or `b` (or both) may be `nullptr`. Example:
//   a = 1 -> 3 -> 5
//   b = 2 -> 4 -> 6
//   mergeSortedLists(a, b) -> 1 -> 2 -> 3 -> 4 -> 5 -> 6 (same nodes)
ListNode* mergeSortedLists(ListNode* a, ListNode* b);

// Returns true if the linked list starting at `head` contains a cycle (some
// node's `next` pointer, followed repeatedly, leads back to a node already
// visited), using Floyd's cycle-detection algorithm (O(1) extra space).
// Returns false for an acyclic list, including `head == nullptr`.
bool hasCycle(ListNode* head);
