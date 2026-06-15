# Intermediate 05: Project Organization & Code Conventions

No new core-language feature this topic -- instead, the conventions that make
C++ code maintainable across a team/project, illustrated by `exercise.h`
itself (a small `Task`/`Priority`/`Status` API written to follow them).

## Project organization

A C++ project's directory layout signals intent:

```
include/   -- public headers (what other projects/TUs #include)
src/       -- .cpp implementation files (not exported)
test/      -- test sources
CMakeLists.txt
```

This repo's own layout follows the same idea at a smaller scale: each topic
folder is self-contained (`notes.md`, `examples.cpp`, `exercise.h`/`.cpp`,
`exercise_test.cpp`), and `intermediate/04`'s `lib/` subdirectory separated a
small reusable library from the code that consumes it. `intermediate/06`
introduces CMake, which is where larger `include/`/`src/`/`test/` splits
typically get formalized.

## Header conventions

- **Include guards**: `#pragma once` (this track's choice throughout) or
  `#ifndef FOO_H / #define FOO_H / #endif`. `#pragma once` is simpler and
  supported everywhere relevant; the `#ifndef` form is more portable to
  exotic toolchains but verbose and error-prone (mismatched macro names).
- **Include order** (in a `.cpp` file): the file's OWN header first (catches
  a header that's missing its own `#include`s), then C++ standard library
  headers (alphabetical), then third-party, then other project headers.
  `exercise.cpp` in this repo always starts with `#include "exercise.h"`.
- **Self-sufficiency**: every header should compile on its own (`#include`
  everything it uses; don't rely on includers to provide dependencies first).

## Preprocessing

Prefer `constexpr`/`inline` functions and `enum class` over `#define` macros
wherever possible (covered in `fundamentals/06`):

- Macros have no type checking, no scope, and can collide across files.
- `constexpr` functions/variables are type-checked, scoped, and debuggable.
- When a macro IS necessary (e.g. `cpp/testing.h`'s `TEST`/`CHECK`, which need
  `__FILE__`/`__LINE__`/token-pasting that no language feature replaces),
  keep it minimal, `UPPER_CASE`-named (visually flags "this is a macro"), and
  documented.

## Variable conventions

- One declaration per line: `int x = 0; int y = 0;` not `int x, y;` (the
  latter is especially dangerous with pointers: `int* a, b;` declares `a` as
  `int*` but `b` as plain `int`).
- Initialize at declaration: `int total = 0;`, not a separate assignment
  later -- avoids a window where the variable holds an indeterminate value.
- Prefer `const`/`constexpr` for anything that doesn't change -- communicates
  intent and lets the compiler catch accidental mutation.
- Prefer brace-initialization (`int x{0};` / `Point p{1.0, 2.0};`) -- it
  rejects narrowing conversions (`int x{3.7};` is an error, `int x = 3.7;`
  silently truncates).

## Enumerator styling

- Prefer scoped `enum class` over plain `enum` (covered by `Priority` and
  `Status` in `exercise.h`): no implicit conversion to `int`, no namespace
  pollution (`Priority::Low`, not bare `Low`).
- PascalCase enumerator names (`Active`, `Blocked`, `Done`), matching
  PascalCase type names.
- For a **flags enum** meant to be OR'd together (`Status`), give it an
  explicit unsigned underlying type (`enum class Status : unsigned { ... }`)
  and define `operator|`/`operator&` explicitly -- scoped enums get NO
  bitwise operators by default, unlike plain `enum`/`int`.
- Built-in relational operators (`<`, `>`, `==`, ...) work directly between
  two values of the *same* enum type (scoped or not), comparing the
  underlying values -- `Priority::High > Priority::Low` is valid without
  defining anything. This is what `sortByPriority`'s comparator relies on.

## Arithmetic type considerations

- Prefer fixed-width types (`int32_t`, `uint64_t`, from `<cstdint>`) when a
  size guarantee matters (file formats, network protocols, serialization) --
  plain `int`/`long` sizes vary by platform (recall `fundamentals/03`).
- Avoid mixing signed and unsigned in comparisons/arithmetic -- a classic
  source of bugs (`fundamentals/03`'s `unsigned` underflow). `Status`'s
  underlying type is `unsigned` because it's a bitmask, never compared with a
  signed value.
- `size_t` for sizes/indices (what `std::vector::size()` returns); avoid
  `int` for loop indices over container sizes unless you've ruled out
  underflow from `size() - 1` on an empty container.

## Function & struct design

- Small, single-purpose functions -- each of `exercise.h`'s five functions
  does ONE thing (`hasStatus` tests a bitmask, `sortByPriority` sorts, etc.).
- Pass containers/large types by `const&` (`const std::vector<Task>&`); pass
  small types (an `int`, an `enum class`, a `Point`-sized struct) by value.
- **Return structs instead of output parameters.** Compare:

  ```cpp
  // Output-parameter style -- unclear at the call site what's
  // "in" vs "out", and the function signature doesn't document
  // what's being computed.
  void countByPriority(const std::vector<Task>& tasks, int& low, int& medium, int& high);

  // Returning a struct: self-documenting, and the compiler can
  // apply NRVO (Named Return Value Optimization) to avoid a copy.
  PriorityCount countByPriority(const std::vector<Task>& tasks);
  ```

- `[[nodiscard]]` on functions whose return value must not be silently
  dropped (a query/computation with no side effect -- ignoring its result is
  almost always a bug). Demonstrated in `examples.cpp`.

## `auto` and type deduction

- Use `auto` when the type is obvious from the right-hand side or is verbose
  boilerplate: `auto it = container.begin();` (the alternative,
  `std::vector<Task>::iterator it = ...`, adds nothing).
- Avoid `auto` when it HIDES a meaningful type: `auto result = compute();`
  forces a reader to look up `compute`'s signature to know what `result` is --
  an explicit `double result = compute();` documents intent at the use site.
- "Almost Always Auto" (AAA) is one school of style (use `auto` everywhere,
  rely on good naming); this track favors explicit return types in function
  *signatures* (so headers are self-documenting without reading the
  implementation) and `auto` for local variables/iterators/lambdas where the
  type is verbose or unimportant.

## Control flow structure

- **Guard clauses / early returns** over deeply nested `if`/`else`:

  ```cpp
  // Nested
  std::string gradeLabel(int score) {
      if (score >= 0 && score <= 100) {
          if (score >= 90) { return "A"; }
          else { ... }
      } else { return "invalid"; }
  }

  // Guard clauses -- flat, reads top-to-bottom
  std::string gradeLabel(int score) {
      if (score < 0 || score > 100) return "invalid";
      if (score >= 90) return "A";
      ...
  }
  ```

- `highestPriorityActive` uses a `continue` guard clause (`if (!hasStatus(...))
  continue;`) to skip non-Active tasks, keeping the loop body's main logic
  unindented.
- Avoid `goto` entirely (the one legitimate modern use -- breaking out of
  nested loops -- is usually better served by extracting a function and
  `return`ing, or `break`ing a flag).
- Prefer range-based `for` (`for (const auto& t : tasks)`) over index loops
  unless the index itself is needed.

## Namespace organization

- Wrap project code in a project-specific namespace to avoid collisions with
  other libraries' names -- `examples.cpp`'s `conventions::` namespace.
- **Never** write `using namespace std;` (or any `using namespace`) at
  namespace scope in a HEADER -- it pollutes the namespace of every TU that
  includes that header, however indirectly. `using namespace` (or a local
  `using std::string;`) is fine inside a function body or `.cpp` file, where
  its effect is contained.

## Modern idioms

- `nullptr` (typed, `std::nullptr_t`) instead of `NULL` (typically just `0`)
  or `0` for pointers -- avoids ambiguity in overload resolution between
  pointer and integer overloads.
- Structured bindings (`auto [a, b] = pair;`) to unpack pairs/tuples/aggregate
  structs (any struct with public members, like `PriorityCount`) without
  naming each field access individually.
- Range-based `for`, `auto`, and `enum class` (all covered above/earlier) are
  themselves "modern idioms" relative to C-style code.

## Naming schemes

This repo's convention (consistent with `exercise.h`):

| Kind | Style | Examples |
|---|---|---|
| Types (`class`/`struct`/`enum class`) | PascalCase | `Task`, `Priority`, `PriorityCount` |
| Enumerators | PascalCase | `Priority::High`, `Status::Active` |
| Functions, variables, parameters | camelCase | `hasStatus`, `sortByPriority`, `tasks` |
| Namespace-scope `constexpr` constants | `k` + PascalCase | `kEpsilon`, `kRomanNumerals`, `kHalf` |
| Private member variables | trailing underscore | `name_` (seen in `examples.cpp` across topics) |
| Macros | `UPPER_CASE` | `TEST`, `CHECK`, `CHECK_EQ` |

## Readability & formatting

- Consistent indentation (4 spaces throughout this repo), consistent brace
  placement, consistent spacing around operators.
- In a real project, a `.clang-format` file at the repo root + `clang-format
  -i` (or an editor/CI integration) enforces this automatically -- removing
  formatting from code review discussions entirely.

## Documentation

- File-level comment at the top of a header explaining its purpose -- every
  `exercise.h` in this repo starts with a "Topic N" comment block.
- Per-declaration doc comments covering: what it does, parameters,
  return value, and an example -- this repo's `//` comments above each
  function follow exactly this shape. Larger projects often formalize this
  with Doxygen (`/** @brief ... @param ... @return ... */` or `///` triple-
  slash), which can generate browsable HTML docs from the same comments --
  the *content* (what/params/returns/example) is the same either way.

## Further Reading

- [MCPP ch. 15 -- Code Conventions I](https://federico-busato.github.io/Modern-CPP-Programming/htmls/15.Code_Convention_I.html)
- [MCPP ch. 16 -- Code Conventions II](https://federico-busato.github.io/Modern-CPP-Programming/htmls/16.Code_Convention_II.html)
- [cppreference: Enumeration declaration](https://en.cppreference.com/w/cpp/language/enum)
- [cppreference: Structured binding declaration](https://en.cppreference.com/w/cpp/language/structured_binding)
- [cppreference: `nodiscard`](https://en.cppreference.com/w/cpp/language/attributes/nodiscard)
- [cppreference: `constexpr` specifier](https://en.cppreference.com/w/cpp/language/constexpr)
- [Google C++ Style Guide](https://google.github.io/styleguide/cppguide.html) (a widely-used, fully-worked example of these conventions)
