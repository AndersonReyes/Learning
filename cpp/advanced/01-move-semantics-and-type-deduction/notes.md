# Advanced 01: Move Semantics, Value Categories & Type Deduction

## Value categories

Every C++ expression has a **value category**. The three primary ones:

- **lvalue** -- has identity (an address you could take with `&`). Variables,
  references, `*p`, `v[i]`, `obj.member`.
- **prvalue** ("pure rvalue") -- has no identity, is about to be
  materialized. Literals (`42`), `a + b`, a function returning by value,
  `std::string("x")`.
- **xvalue** ("expiring value") -- has identity but is marked as movable
  from. The result of `std::move(x)`, or a function returning `T&&`.

Two umbrella categories: **glvalue** = lvalue or xvalue (has identity);
**rvalue** = prvalue or xvalue (can be moved from). `std::move` doesn't move
anything -- it's a `static_cast<T&&>` that turns an lvalue into an xvalue,
i.e. it just **changes which overload is selected** (rvalue-ref overloads
become viable).

```cpp
int x = 5;
int& lref = x;          // x: lvalue
int&& rref = std::move(x); // std::move(x): xvalue; rref binds to it
int y = x + 1;           // x + 1: prvalue
```

## `std::move`, move constructors & move assignment

A **move constructor** (`T(T&& other)`) and **move assignment**
(`T& operator=(T&& other)`) let a class transfer ownership of a resource
(heap buffer, file handle, ...) instead of deep-copying it. After a move,
the source object must be left in a valid (but unspecified -- typically
"empty") state, since it's still going to be destroyed.

```cpp
struct Buffer {
    int* data;
    size_t n;

    Buffer(Buffer&& other) noexcept : data(other.data), n(other.n) {
        other.data = nullptr;  // source no longer owns the memory
        other.n = 0;
    }
};
```

`std::move(x)` is how you tell the compiler "I'm done with `x`, you may
steal from it" -- it makes `x` bind to `T&&` overloads instead of `const T&`
ones. Without it, `Buffer b = a;` always calls the **copy** constructor, even
if `a` is never used again.

## The Rule of 5 (and Rule of 0)

If a class manages a resource directly (raw `new`/`delete`, file handles,
...), it generally needs all **five** special member functions defined
together:

1. Destructor
2. Copy constructor
3. Copy assignment operator
4. Move constructor
5. Move assignment operator

Defining *any* of these (especially the destructor) suppresses the
compiler-generated move operations and may suppress copy operations too --
"all or nothing." The **Rule of 0** is the better default for everyday
classes: hold resources in members that already manage themselves
(`std::vector`, `std::unique_ptr`, `std::string`...) and let the compiler
generate all five for free. The Rule of 5 only applies when *you* are the
one writing a resource-owning class (like `IntBuffer` below).

### Self-assignment safety

Both assignment operators must handle `a = a;` (copy) and `a = std::move(a);`
(self-move) without corrupting `a`. The classic guard:

```cpp
T& operator=(T&& other) {
    if (this == &other) return *this;  // self-move: no-op
    delete[] data_;
    data_ = other.data_;
    ...
}
```

Without the guard, self-move-assignment would `delete[] data_` and then read
`other.data_` -- which is the same (now-dangling) pointer -- undefined
behavior. `-Wself-move` warns on `a = std::move(a)` at the call site, but the
*class* must still be correct if it happens (e.g. via an alias, or generic
code that doesn't know `a` and `b` are the same object).

## Forwarding references (`T&&` in a template)

`T&& x` in a **template** parameter (where `T` is deduced from the call) is
a **forwarding reference**, not a plain rvalue reference -- it can bind to
*both* lvalues and rvalues, because of **reference collapsing**:

| Argument          | `T` deduced as | `T&&` becomes |
|--------------------|-----------------|----------------|
| lvalue `int`       | `int&`          | `int& &&` -> `int&`  |
| rvalue / literal    | `int`           | `int&&`        |
| `const` lvalue      | `const int&`    | `const int& &&` -> `const int&` |

Reference collapsing rule: `& + && = &`, `&& + && = &&` (an lvalue ref
"wins"). This is exactly what `std::is_lvalue_reference_v<T>` detects --
`T` itself is a reference type only when the argument was an lvalue:

```cpp
template <typename T>
std::string kind(T&& x) {
    (void)x;
    return std::is_lvalue_reference_v<T> ? "lvalue" : "rvalue";
}

int a = 5;
kind(a);            // T = int&  -> "lvalue"
kind(5);            // T = int   -> "rvalue"
kind(std::move(a)); // T = int   -> "rvalue"
```

**Important**: a plain (non-template) `void f(T&& x)` is just an rvalue
reference -- it only binds to rvalues. Forwarding-reference behavior
requires `T` to be a template parameter deduced *from this very call*.
`template <typename T> void f(std::vector<T>&& x)` is NOT a forwarding
reference either -- `T&&` must be the *exact* deduced parameter type.

## `std::forward` and perfect forwarding

Inside a function taking `T&& x`, the name `x` is itself an **lvalue**
(every named variable is), even if it's bound to an rvalue. Passing `x` on
to another function therefore always copies unless you restore its original
value category with `std::forward<T>(x)` -- itself just a conditional
`std::move`, equivalent to:

```cpp
static_cast<T&&>(x)   // collapses to T& if T is a reference type (lvalue arg),
                       // or T&& if T is non-reference (rvalue arg)
```

```cpp
template <typename T>
void relay(T&& x) {
    target(std::forward<T>(x));  // "remembers" whether the original was an lvalue/rvalue
}
```

This is **perfect forwarding**: `relay` passes its argument through to
`target` exactly as it was received -- lvalue-ness and constness preserved,
no extra copies.

## `auto` deduction: decay by default

`auto` deduction follows the same rules as template argument deduction by
value: it **strips references and top-level `const`/`volatile`**, and
arrays/functions decay to pointers.

```cpp
const int& cref = someInt;
auto a = cref;       // int (NOT const int&) -- a copy, mutable
auto& b = cref;      // const int& -- reference preserved, const preserved
const auto& c = someInt; // const int& -- explicit
auto&& d = someInt;  // forwarding reference if in a template; int& here (not a template)
```

If you want a copy, use `auto`. If you want to alias the original
(no copy, see/mutate through it), use `auto&` or `const auto&`.

## `decltype` vs `decltype(auto)`

`decltype(expr)` yields the **exact declared type** of `expr` --
references, `const`, everything -- *without* the decay `auto` applies, and
**without evaluating** `expr`.

```cpp
int x = 0;
int& rx = x;
decltype(x)  a = x;   // int
decltype(rx) b = x;   // int& -- reference preserved!
decltype((x)) c = x;  // int& -- extra parens make it an lvalue *expression*,
                       // and decltype on an lvalue expression (not just a
                       // name) yields a reference type
```

`decltype(auto)` (as a return type or variable type) deduces using
`decltype`'s rules from the initializer/return expression, instead of
`auto`'s decaying rules -- useful for "return whatever this expression's
type really is, reference-ness included":

```cpp
template <typename Container>
decltype(auto) first(Container& c) { return c[0]; }
// if c[0] returns int&, first returns int& too (not a copy)
```

### Trailing return types: `decltype` fixed by the *signature*

`auto f(...) -> decltype(expr)` fixes the return type from `expr` **at
declaration time**, independent of the function body. This matters for stub
code: a function declared `auto f() -> decltype(c[0])` has a well-defined
return type (e.g. `int&`) even if its body is just `throw ...;` with no
`return` statement. Compare to a bare `auto f() { throw ...; }` (no
`return` anywhere) -- that deduces a return type of `void`, which can break
`static_assert`s that check the return type, or callers that try to use the
"value."

## Copy elision / RVO

Since C++17, `T x = makeT();` where `makeT()` returns a prvalue is
guaranteed to construct `x` **directly** from `makeT()`'s return expression
-- no temporary, no copy/move constructor call at all (mandatory copy
elision for prvalues). This is why returning big objects by value from
factory functions is fine and idiomatic; don't reach for `std::move` on a
local variable in a `return` statement (`return std::move(localVar);`) --
it can *defeat* this guarantee (NRVO, the non-mandatory cousin for named
locals, is no longer eligible once you wrap the name in `std::move`) and is
considered a pessimization, not an optimization.

## Further Reading

- [MCPP ch. 21 -- Move Semantics and Forwarding](https://federico-busato.github.io/Modern-CPP-Programming/htmls/21.Move_Semantics.html)
- [cppreference: Value categories](https://en.cppreference.com/w/cpp/language/value_category)
- [cppreference: `std::move`](https://en.cppreference.com/w/cpp/utility/move)
- [cppreference: `std::forward`](https://en.cppreference.com/w/cpp/utility/forward)
- [cppreference: Move constructors](https://en.cppreference.com/w/cpp/language/move_constructor)
- [cppreference: Move assignment operators](https://en.cppreference.com/w/cpp/language/move_assignment)
- [cppreference: Reference declaration (reference collapsing)](https://en.cppreference.com/w/cpp/language/reference#Reference_collapsing)
- [cppreference: `auto` specifier](https://en.cppreference.com/w/cpp/language/auto)
- [cppreference: `decltype`](https://en.cppreference.com/w/cpp/language/decltype)
- [cppreference: Copy elision](https://en.cppreference.com/w/cpp/language/copy_elision)
