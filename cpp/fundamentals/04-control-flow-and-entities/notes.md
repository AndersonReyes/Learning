# 4. Control Flow, Enums, Structs & Namespaces

## `if`/`else`

- The "dangling else" gotcha: `else` binds to the **nearest unmatched
  `if`**, which can be surprising with nested `if`s and no braces:

  ```cpp
  if (a)
      if (b)
          doX();
  else
      doY();  // binds to "if (b)", NOT "if (a)" -- despite the indentation!
  ```

  Always use braces for any `if`/`else` with a nested `if`, to make the
  binding visually unambiguous.

## `switch`

- Case labels must be compile-time constants (literals, `enum` values,
  `constexpr`). `default` is optional but recommended — without it, no case
  matching falls through silently.
- **No implicit `break`** — execution falls through to the next case unless
  you `break`, `return`, or `[[fallthrough]];`. Intentional fallthrough
  should use `[[fallthrough]];` as the last statement in a case — it
  documents intent and silences `-Wimplicit-fallthrough`.

  ```cpp
  switch (month) {
      case 1: case 3: case 5: case 7: case 8: case 10: case 12:
          return 31;          // grouped cases: no statements between
                               // labels, so no fallthrough warning
      case 4: case 6: case 9: case 11:
          return 30;
      default:
          return 0;
  }
  ```

- Switching on an `enum`/`enum class` without a `default`: if you handle
  every enumerator, `-Wswitch` warns if a new enumerator is added later and
  not handled here — a useful "did you forget a case?" check. Adding
  `default` silences this check entirely, so prefer exhaustive `case`s over
  `default` when switching on an enum you control.

## Loops

- `for (init; cond; step)`, `while (cond)`, `do { ... } while (cond)` (body
  runs at least once), and range-based `for (auto& x : container)`.
- Range-based `for` — **by-value `auto x` copies each element** (fine for
  small types like `int`, wasteful for `std::string`/`std::vector`); **`auto&
  x` binds by reference** (can mutate the container); **`const auto& x`**
  avoids both the copy and accidental mutation — default to this unless you
  need to modify elements or the type is trivially small.
- `break` exits the innermost loop/switch; `continue` skips to the next
  iteration's condition check. Both only affect the *innermost* enclosing
  loop — there's no "break two levels" without a flag variable or `goto`
  (rare, but exists for exactly this).

## Enums

### Unscoped `enum`

```cpp
enum Color { Red, Green, Blue };  // Red=0, Green=1, Blue=2 by default
int c = Red;                       // implicitly converts to int
```

- Enumerator names (`Red`, `Green`, `Blue`) leak into the **enclosing
  scope** — two unscoped enums in the same scope can't both have a
  `Red`. Implicit conversion to `int` means you can accidentally compare or
  assign across unrelated enums/ints with no error.

### Scoped `enum class` (prefer this)

```cpp
enum class Color { Red, Green, Blue };
Color c = Color::Red;              // must qualify with Color::
int i = static_cast<int>(c);       // no implicit conversion -- must cast
```

- Enumerator names are scoped to `Color::` — no leakage, no collisions.
- No implicit conversion to/from `int` — converting either direction needs
  `static_cast`. This catches accidental mixing of unrelated enums at
  compile time.
- You can set an explicit underlying type and/or explicit values:

  ```cpp
  enum class HttpStatus : int { Ok = 200, NotFound = 404, ServerError = 500 };
  ```

## Structs

```cpp
struct Point {
    int x = 0;   // default member initializer
    int y = 0;
};

Point p{1, 2};   // aggregate initialization, p.x=1, p.y=2
Point origin;    // x=0, y=0 (default member initializers apply)
```

- A `struct` with only public data members (no user-declared constructors,
  private/protected members, virtual functions, or base classes) is an
  **aggregate** — it supports `{}`-list initialization member-by-member, in
  declaration order. (Classes and constructors are covered in
  `fundamentals/07`.)
- `struct` vs `class`: the **only** difference is the default member/base
  access — `struct` defaults to `public`, `class` defaults to `private`.
  Convention: `struct` for plain data bundles, `class` once you add
  invariants/behavior.
- As of C++20, you can ask the compiler to generate comparison operators:

  ```cpp
  struct Point {
      int x, y;
      bool operator==(const Point&) const = default;  // compares x and y
  };
  ```

  `= default` here compares every member in declaration order — far less
  error-prone than hand-writing field-by-field comparisons (hand-written
  operator overloads are covered in `fundamentals/08`).

## Namespaces

```cpp
namespace geometry {
    struct Point { int x, y; };
    double distance(Point a, Point b);
}

geometry::Point p{1, 2};       // fully qualified
using geometry::Point;          // import one name
using namespace geometry;        // import everything (avoid in headers --
                                  // pollutes every includer's scope)
```

- Group related declarations and avoid name collisions across libraries
  (two libraries can each have a `Point` without conflict, as long as
  callers don't `using namespace` both).
- **Nested namespace definitions** (C++17): `namespace a::b::c { ... }` is
  shorthand for `namespace a { namespace b { namespace c { ... } } }`.
- **Namespace alias**: `namespace fs = std::filesystem;` — shorten a long or
  deeply-nested namespace name at the point of use.
- **Unnamed (anonymous) namespace**: `namespace { ... }` gives its contents
  **internal linkage** — each translation unit that defines one gets its own
  private copy, invisible to other `.cpp` files. Useful for helper
  functions/types that are implementation details of one `.cpp` file. (Linkage
  and translation units are covered in depth in `intermediate/03`.)

## Further Reading (Modern C++ Programming)

- [Chapter 6 — Control Flow and Entities](https://federico-busato.github.io/Modern-CPP-Programming/htmls/06.ControlFlow.html)
- [`if`/`else`](https://en.cppreference.com/w/cpp/language/if)
- [`switch`](https://en.cppreference.com/w/cpp/language/switch)
- [`enum`/`enum class`](https://en.cppreference.com/w/cpp/language/enum)
- [Classes (aggregates, structs)](https://en.cppreference.com/w/cpp/language/classes)
- [Namespaces](https://en.cppreference.com/w/cpp/language/namespace)
- [`[[fallthrough]]`](https://en.cppreference.com/w/cpp/language/attributes/fallthrough)
