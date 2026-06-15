// Run with:
//   g++ -std=c++20 -Wall -Wextra -o /tmp/ex examples.cpp && /tmp/ex

#include <iostream>
#include <vector>

// A node in a singly-linked list, used in the "linked structures" demo below.
struct ListNode {
    int value;
    ListNode* next = nullptr;
};

int main() {
    // --- Pointers: address-of, dereference, nullptr -----------------------------
    int x = 42;
    int* p = &x;
    std::cout << "x = " << x << ", *p = " << *p << "\n";
    *p = 7;
    std::cout << "after *p = 7, x = " << x << "\n";

    int* nothing = nullptr;
    std::cout << "nothing == nullptr: " << std::boolalpha
              << (nothing == nullptr) << "\n";

    // --- Pointer to pointer ------------------------------------------------------
    int** pp = &p;
    std::cout << "\n**pp = " << **pp << " (via pointer to pointer)\n";

    // --- Pointer arithmetic and array decay --------------------------------------
    int arr[5] = {10, 20, 30, 40, 50};
    int* begin = arr;          // array decays to pointer to arr[0]
    int* end = arr + 5;        // one-past-the-end (valid, not dereferenced)
    std::cout << "\narr via pointer walk:";
    for (int* it = begin; it != end; ++it) {
        std::cout << " " << *it;
    }
    std::cout << "\n";
    std::cout << "end - begin = " << (end - begin) << " (element count, not bytes)\n";

    // --- References ---------------------------------------------------------------
    int y = 100;
    int& ref = y;
    ref = 200;  // mutates y through the reference
    std::cout << "\ny = " << y << " (mutated via reference)\n";

    // --- const and pointers ---------------------------------------------------------
    int a = 1, b = 2;
    const int* p1 = &a;  // pointer to const: can't do *p1 = ..., can repoint
    p1 = &b;
    std::cout << "\n*p1 (now points at b) = " << *p1 << "\n";

    int c = 3;
    int* const p2 = &c;  // const pointer: can do *p2 = ..., can't repoint
    *p2 = 99;
    std::cout << "*p2 (mutated through const pointer) = " << *p2 << "\n";

    // --- Dynamic memory: new/delete and new[]/delete[] -------------------------------
    int* heapInt = new int(123);
    std::cout << "\n*heapInt = " << *heapInt << "\n";
    delete heapInt;
    heapInt = nullptr;  // defensive: delete nullptr is a no-op

    int* heapArr = new int[5];
    for (int i = 0; i < 5; ++i) heapArr[i] = i * i;
    std::cout << "heapArr:";
    for (int i = 0; i < 5; ++i) std::cout << " " << heapArr[i];
    std::cout << "\n";
    delete[] heapArr;

    // --- Linked structures: building, traversing, and relinking a list ------------
    // Build 1 -> 2 -> 3 using `new`.
    ListNode* head = new ListNode{1, nullptr};
    head->next = new ListNode{2, nullptr};
    head->next->next = new ListNode{3, nullptr};

    std::cout << "\nlinked list:";
    for (ListNode* cur = head; cur != nullptr; cur = cur->next) {
        std::cout << " " << cur->value;
    }
    std::cout << "\n";

    // Floyd's cycle detection on this acyclic list.
    ListNode* slow = head;
    ListNode* fast = head;
    bool cycle = false;
    while (fast != nullptr && fast->next != nullptr) {
        slow = slow->next;
        fast = fast->next->next;
        if (slow == fast) {
            cycle = true;
            break;
        }
    }
    std::cout << "has cycle: " << cycle << "\n";

    // Free every node.
    while (head != nullptr) {
        ListNode* next = head->next;
        delete head;
        head = next;
    }

    return 0;
}
