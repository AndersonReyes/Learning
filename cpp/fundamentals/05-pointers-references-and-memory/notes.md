# 5. Pointers, References, Memory & `const`

## Pointers

```cpp
int x = 42;
int* p = &x;   // p holds the address of x
*p = 7;        // dereference: x is now 7
```

- `&x` — address-of operator, yields `int*`.
- `*p` — dereference operator, yields `int&` (an lvalue referring to `x`).
- `int* p = nullptr;` — the null pointer literal (C++11). Prefer `nullptr`
  over `NULL`/`0` — it has its own type (`std::nullptr_t`) and can't be
  confused with an integer in overload resolution.
- **Pointer to pointer**: `int** pp = &p;` — `*pp` is `p` (an `int*`), `**pp`
  is `x`. Used for "output pointer" parameters and arrays of pointers.

### Pointer arithmetic

```cpp
int arr[5] = {10, 20, 30, 40, 50};
int* p = arr;       // array decays to a pointer to its first element
int* q = p + 2;      // q points to arr[2] -- advances by 2*sizeof(int) bytes
int diff = q - p;    // 2 (difference in elements, not bytes)
```

- `p + n` advances by `n * sizeof(*p)` bytes — the compiler scales by the
  pointed-to type's size.
- Valid range for a pointer derived from an array of size `N` is
  `[arr, arr + N]` inclusive of the **one-past-the-end** pointer `arr + N`
  (used as a sentinel, e.g. `end()` iterators) — but `arr + N` may not be
  *dereferenced*. Any pointer arithmetic landing outside `[arr, arr+N]`, or
  dereferencing outside `[arr, arr+N)`, is undefined behavior — even if you
  never read the result, just *computing* an out-of-range pointer is UB.
- **Array-to-pointer decay**: when an array is passed to a function, it
  decays to a pointer to its first element — the function has no way to
  recover the array's length via `sizeof`. Always pass the size/length
  explicitly alongside a raw-pointer parameter:
  ```cpp
  void process(int* arr, int size);   // size is mandatory -- arr carries no length
  ```

## References

```cpp
int x = 42;
int& r = x;   // r is an alias for x -- not a separate object
r = 7;        // x is now 7
```

- A reference **must be initialized** at declaration and **cannot be
  rebound** afterward — `r = y;` assigns `y`'s *value* into `x` (via `r`), it
  does not make `r` refer to `y`.
- No "null reference" — every reference refers to a real object (assuming no
  UB was involved in creating it). If you need an optional/nullable
  "reference", use a pointer (or `std::optional<T>`, covered later).
- **Reference vs pointer**:

  | | pointer | reference |
  |---|---|---|
  | can be null | yes (`nullptr`) | no |
  | can be rebound | yes (`p = &other`) | no |
  | needs `*`/`&` to access | yes | no — looks like the object itself |
  | can form arrays of it | yes (`int* arr[5]`) | no |

### Reference parameters

```cpp
void increment(int& x) { ++x; }   // mutates the caller's variable, no copy

void printAll(const std::vector<int>& v) {  // const&: no copy, read-only
    for (int x : v) std::cout << x << " ";
}
```

- `T&` parameter: pass-by-reference, callee can read **and mutate** the
  caller's object, no copy.
- `const T&` parameter: pass-by-reference, read-only, no copy — the default
  choice for "input" parameters of non-trivial types (avoids copying
  `std::string`/`std::vector`/etc.). For small types (`int`, `double`,
  `bool`), pass by value instead — a reference is the same size as a pointer,
  so referencing a small type buys nothing.
- **Output parameters**: a function can "return" multiple values via `T&`
  out-parameters:
  ```cpp
  void minMax(const std::vector<int>& v, int& outMin, int& outMax);
  // caller: int lo, hi; minMax(v, lo, hi);
  ```

### Dangling references (and pointers)

**Never** return a reference or pointer to a local variable — the local's
storage is gone the moment the function returns, so the caller is left with
a dangling reference/pointer. Using it afterward is UB (often appears to
"work" until it doesn't):

```cpp
int& bad() {
    int local = 42;
    return local;   // DANGLING -- local's storage no longer exists
}                     // compilers warn about this (-Wreturn-local-addr)

const std::string& worse() {
    return std::string("temp");  // DANGLING -- temporary destroyed at ';'
}
```

Returning a reference/pointer is only safe when it refers to something that
outlives the function call: a parameter passed by reference, a member of an
object the caller owns, a `static`/global, or heap memory the caller is
responsible for freeing.

## `const` and pointers

Read pointer-`const` declarations **right to left** from the variable name:

```cpp
const int* p1;        // p1 points to a `const int` -- can't modify *p1,
                       // but p1 itself can be reassigned to point elsewhere
int* const p2 = &x;    // p2 is a const pointer to `int` -- can modify *p2,
                       // but p2 itself can't be reassigned (must init here)
const int* const p3 = &x;  // both: can't modify *p3, can't reassign p3
```

- `const int*` and `int const*` are equivalent — the `const` binds to
  whatever is on its *left*, except when it's leftmost (then it binds right).
- A `const T*` (or `const T&`) can bind to a non-const object (read-only
  *view*), but a non-const `T*`/`T&` cannot bind to a `const T` (would allow
  mutation through it).
- Use `const T*`/`const T&` parameters liberally — it documents "this
  function does not modify what you passed in" and lets callers pass
  `const` objects or temporaries.

## Dynamic memory: `new` / `delete`

```cpp
int* p = new int(42);     // allocate one int on the heap, initialized to 42
delete p;                  // free it
p = nullptr;               // defensive: avoid an accidental use-after-free

int* arr = new int[10];   // allocate an array of 10 ints
delete[] arr;              // free an array -- MUST use delete[], not delete
```

- Every `new` must be matched by **exactly one** `delete` (and every `new[]`
  by exactly one `delete[]`); mismatching `delete`/`delete[]` is UB.
- **Memory leak**: forgetting `delete` — the memory is never reclaimed until
  the program exits (or, for long-running programs, never).
- **Dangling pointer / use-after-free**: using a pointer after its memory has
  been `delete`d — UB, often crashes or corrupts unrelated memory.
- **Double free**: calling `delete` twice on the same pointer — UB. Setting
  the pointer to `nullptr` after `delete` makes a second `delete nullptr`
  a harmless no-op (the language guarantees `delete nullptr` does nothing).
- **Ownership**: every heap allocation has exactly one "owner" responsible for
  freeing it. Tracking ownership by hand (as in this topic) is error-prone —
  `advanced/02` introduces smart pointers (`unique_ptr`/`shared_ptr`), which
  automate this via RAII (`fundamentals/07`).

## Linked structures: pointers as "links"

A node-based structure (e.g. a singly-linked list) is built from a `struct`
whose member is a pointer to another instance of itself:

```cpp
struct ListNode {
    int value;
    ListNode* next = nullptr;
};
```

- Traversal: `for (ListNode* cur = head; cur != nullptr; cur = cur->next)`.
- `cur->next` is shorthand for `(*cur).next` — `->` dereferences and accesses
  a member in one step.
- **Floyd's cycle detection** ("tortoise and hare"): two pointers traverse the
  list at different speeds (`slow` advances one node, `fast` advances two);
  if there's a cycle, `fast` will eventually equal `slow` (they meet inside
  the loop); if `fast` (or `fast->next`) reaches `nullptr`, there's no cycle.
- Merging/reversing linked lists by **relinking** existing nodes (rewiring
  `next` pointers) needs no `new`/`delete` — only traversal and pointer
  reassignment. A **dummy head** node (a throwaway placeholder before the
  real first node) simplifies merge/build logic by removing the "is this the
  first node?" special case.

## Further Reading (Modern C++ Programming)

- [Chapter 7 — Basic Concepts V](https://federico-busato.github.io/Modern-CPP-Programming/htmls/07.Basic_Concepts_V.html)
  (pointers, references, dynamic memory)
- [Pointer declaration](https://en.cppreference.com/w/cpp/language/pointer)
- [Reference declaration](https://en.cppreference.com/w/cpp/language/reference)
- [`const`/`volatile` type qualifiers](https://en.cppreference.com/w/cpp/language/cv)
- [`new` expression](https://en.cppreference.com/w/cpp/language/new)
- [`delete` expression](https://en.cppreference.com/w/cpp/language/delete)
- [`nullptr`](https://en.cppreference.com/w/cpp/language/nullptr)
