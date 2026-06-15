// Spec for exercise.h/exercise.cpp. No test framework yet (that's built in
// fundamentals/06) -- just assert(). Compile and run:
//
//   g++ -std=c++20 -Wall -Wextra -o /tmp/test exercise_test.cpp exercise.cpp && /tmp/test
//
// Every assert() must pass. If one fails, the program aborts and prints the
// failing file:line -- fix exercise.cpp and re-run.

#include "exercise.h"

#include <cassert>
#include <iostream>
#include <vector>

// --- Linked-list test helpers -----------------------------------------------

// Builds a singly-linked list from `values` (in order) using `new`, and
// returns its head (or nullptr if `values` is empty).
static ListNode* buildList(const std::vector<int>& values) {
    ListNode* head = nullptr;
    ListNode* tail = nullptr;
    for (int v : values) {
        ListNode* node = new ListNode{v, nullptr};
        if (tail == nullptr) {
            head = node;
        } else {
            tail->next = node;
        }
        tail = node;
    }
    return head;
}

// Reads a (non-cyclic) linked list into a vector.
static std::vector<int> toVector(ListNode* head) {
    std::vector<int> result;
    for (ListNode* cur = head; cur != nullptr; cur = cur->next) {
        result.push_back(cur->value);
    }
    return result;
}

// Frees every node in a (non-cyclic) linked list.
static void freeList(ListNode* head) {
    while (head != nullptr) {
        ListNode* next = head->next;
        delete head;
        head = next;
    }
}

static void testCopyReversed() {
    int arr[] = {1, 2, 3, 4, 5};
    int* rev = copyReversed(arr, 5);
    assert(rev != nullptr);
    assert(rev[0] == 5);
    assert(rev[1] == 4);
    assert(rev[2] == 3);
    assert(rev[3] == 2);
    assert(rev[4] == 1);
    assert(arr[0] == 1 && arr[4] == 5);  // original untouched
    delete[] rev;

    int single[] = {42};
    int* revSingle = copyReversed(single, 1);
    assert(revSingle != nullptr);
    assert(revSingle[0] == 42);
    delete[] revSingle;

    assert(copyReversed(arr, 0) == nullptr);
}

static void testRotateLeftInPlace() {
    int a[] = {1, 2, 3, 4, 5};
    rotateLeftInPlace(a, 5, 2);
    assert((std::vector<int>(a, a + 5) == std::vector<int>{3, 4, 5, 1, 2}));

    int b[] = {1, 2, 3, 4, 5};
    rotateLeftInPlace(b, 5, 0);
    assert((std::vector<int>(b, b + 5) == std::vector<int>{1, 2, 3, 4, 5}));

    int c[] = {1, 2, 3, 4, 5};
    rotateLeftInPlace(c, 5, 5);  // full rotation == no-op
    assert((std::vector<int>(c, c + 5) == std::vector<int>{1, 2, 3, 4, 5}));

    int d[] = {1, 2, 3, 4, 5};
    rotateLeftInPlace(d, 5, 7);  // 7 % 5 == 2, same as rotating by 2
    assert((std::vector<int>(d, d + 5) == std::vector<int>{3, 4, 5, 1, 2}));

    int e[] = {1, 2, 3, 4, 5};
    rotateLeftInPlace(e, 5, -1);  // rotate right by 1
    assert((std::vector<int>(e, e + 5) == std::vector<int>{5, 1, 2, 3, 4}));

    int single[] = {99};
    rotateLeftInPlace(single, 1, 3);  // size <= 1: no-op
    assert(single[0] == 99);

    rotateLeftInPlace(single, 0, 3);  // size == 0: no-op (and no div-by-zero)
    assert(single[0] == 99);
}

static void testMinMaxSum() {
    int lo = -1, hi = -1;
    long long sum = -1;

    minMaxSum({3, -1, 4, 1, 5}, lo, hi, sum);
    assert(lo == -1);
    assert(hi == 5);
    assert(sum == 12);

    minMaxSum({}, lo, hi, sum);
    assert(lo == 0);
    assert(hi == 0);
    assert(sum == 0);

    minMaxSum({7}, lo, hi, sum);
    assert(lo == 7);
    assert(hi == 7);
    assert(sum == 7);

    // Sum exceeds INT_MAX -- requires accumulating in `long long`.
    minMaxSum({1000000000, 1000000000, 1000000000, 1000000000}, lo, hi, sum);
    assert(lo == 1000000000);
    assert(hi == 1000000000);
    assert(sum == 4000000000LL);
}

static void testMergeSortedLists() {
    ListNode* a = buildList({1, 3, 5});
    ListNode* b = buildList({2, 4, 6});
    ListNode* merged = mergeSortedLists(a, b);
    assert((toVector(merged) == std::vector<int>{1, 2, 3, 4, 5, 6}));
    freeList(merged);

    ListNode* c = nullptr;
    ListNode* d = buildList({1, 2, 3});
    ListNode* merged2 = mergeSortedLists(c, d);
    assert((toVector(merged2) == std::vector<int>{1, 2, 3}));
    freeList(merged2);

    ListNode* e = buildList({});
    ListNode* f = buildList({});
    assert(mergeSortedLists(e, f) == nullptr);

    ListNode* g = buildList({1, 1, 1});
    ListNode* h = buildList({1, 1});
    ListNode* merged3 = mergeSortedLists(g, h);
    assert((toVector(merged3) == std::vector<int>{1, 1, 1, 1, 1}));
    freeList(merged3);

    ListNode* i = buildList({5});
    ListNode* j = buildList({1, 2, 3, 4, 6, 7});
    ListNode* merged4 = mergeSortedLists(i, j);
    assert((toVector(merged4) == std::vector<int>{1, 2, 3, 4, 5, 6, 7}));
    freeList(merged4);
}

static void testHasCycle() {
    assert(hasCycle(nullptr) == false);

    ListNode* acyclic = buildList({1, 2, 3, 4});
    assert(hasCycle(acyclic) == false);
    freeList(acyclic);

    ListNode* selfLoop = new ListNode{1, nullptr};
    selfLoop->next = selfLoop;  // 1 -> 1 (cycle of length 1)
    assert(hasCycle(selfLoop) == true);
    selfLoop->next = nullptr;  // break the cycle before freeing
    delete selfLoop;

    ListNode* cyclic = buildList({1, 2, 3, 4, 5});
    ListNode* third = cyclic->next->next;  // node with value 3
    ListNode* last = third->next->next;    // node with value 5
    last->next = third;                    // 5 -> back to 3
    assert(hasCycle(cyclic) == true);
    last->next = nullptr;  // break the cycle before freeing
    freeList(cyclic);
}

int main() {
    testCopyReversed();
    testRotateLeftInPlace();
    testMinMaxSum();
    testMergeSortedLists();
    testHasCycle();
    std::cout << "All tests passed!\n";
    return 0;
}
