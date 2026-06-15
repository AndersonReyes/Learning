# Intermediate 03: Translation Units, Linkage & the ODR

## Translation units (TUs)

A **translation unit** is one `.cpp` file *after* preprocessing -- i.e.
with every `#include` textually expanded, macros substituted, etc. Each
`.cpp` file is compiled into its own **object file** (`.o`), independently;
the linker then combines all object files (plus libraries) into one
executable.

```
exercise.cpp      --(preprocess + compile)--> exercise.o      \
exercise_test.cpp --(preprocess + compile)--> exercise_test.o  >--(link)--> ./test
```

`g++ -std=c++20 -Wall -Wextra -o test exercise_test.cpp exercise.cpp` compiles
*both* `.cpp` files (each #including `exercise.h`, so each TU gets its own
copy of every declaration in the header) and links them together. This
topic's exercises are deliberately split so that **this build command
itself exercises every concept below**.

## Declarations vs. definitions

- A **declaration** introduces a name and its type, without necessarily
  describing the *entity* fully: `int nextId();` (a function), `extern int
  globalRequestCount;` (a variable).
- A **definition** is a declaration that also provides everything needed to
  *use* the entity: a function body, or (for a variable) storage + an
  initializer.
- A declaration can appear in many TUs (e.g. via a shared header). A
  definition with external linkage must appear in **exactly one** TU --
  this is (part of) the **One Definition Rule (ODR)**.

## Linkage

Every name with namespace or file scope has a **linkage**, controlling
whether the same name in different TUs refers to the same entity:

- **External linkage** (the default for non-`const`, non-`static`
  namespace-scope names): the name refers to the *same* entity across all
  TUs. This is how `exercise.cpp` and `exercise_test.cpp` can share
  `globalRequestCount`, `nextId`, `normalizeWhitespace`, `toRoman`.
- **Internal linkage**: the name is local to its TU -- a name with the same
  spelling in another TU refers to a *different*, unrelated entity (or
  doesn't exist at all, from that other TU's perspective). Two ways to get
  internal linkage:
  - `static` at namespace scope (the pre-C++11 way): `static bool
    isSpaceChar(char c) { ... }`.
  - An **unnamed (anonymous) namespace**: `namespace { ... }` -- everything
    inside has internal linkage. This is the modern, preferred style (also
    works for types, which `static` cannot give internal linkage to).
  - `const`/`constexpr` namespace-scope variables ALSO default to internal
    linkage (unlike non-`const` variables) -- `kRomanNumerals` doesn't need
    `static` or an anonymous namespace for this reason alone, though putting
    it in one *additionally* documents "this is a private implementation
    detail of this TU", which is good practice regardless.
- **No linkage**: local variables, and names of types declared inside
  functions.

### Why hide helpers with internal linkage?

`isSpaceChar` and `kRomanNumerals` in `exercise.cpp` are implementation
details of `normalizeWhitespace`/`toRoman` -- not part of this library's
public API. Giving them internal linkage:

- Prevents other TUs from accidentally depending on them.
- Lets a *different* TU define its own unrelated `isSpaceChar` (e.g. with
  different semantics) without a name collision at link time.
- Communicates intent: anything in an anonymous namespace (or marked
  `static`) is "private to this file".

## `extern`: declaring a variable defined elsewhere

```cpp
// exercise.h
extern int globalRequestCount;   // declaration: "exists somewhere, type int"

// exercise.cpp
int globalRequestCount = 0;      // the ONE definition (has an initializer)
```

Any TU that `#include`s `exercise.h` can refer to `globalRequestCount` --
all such references resolve to the single object defined in
`exercise.cpp`, because the variable has external linkage. Without `extern`,
`int globalRequestCount;` repeated (via the header) in every including TU
would either violate the ODR (multiple definitions) or, prior to C++17, rely
on the "common symbol" linker extension for uninitialized globals -- fragile
and non-portable. `extern` + a single defining TU is the correct, portable
pattern.

## Function-local `static`: storage duration, not linkage

```cpp
int nextId() {
    static int counter = 0;  // initialized ONCE, on first call
    return ++counter;
}
```

A function-local `static` variable:

- Is initialized the first time execution reaches its declaration (NOT at
  program startup, for non-constant initializers -- though `0` here could be
  initialized at compile time too).
- Persists between calls -- unlike an ordinary local variable, it is NOT
  destroyed and recreated each call.
- Has **no linkage** (it's still a local name, invisible outside the
  function) -- but its *storage* lasts for the whole program, like a
  namespace-scope variable's.
- Is shared by all calls from anywhere in the program (there's only one
  `nextId`, hence only one `counter`) -- but is NOT shared between different
  instantiations of a function *template* (each instantiation gets its own
  static).

## `inline`: permitting identical definitions across TUs

Defining an ordinary (non-template) function in a header, `#include`d by
multiple TUs, normally breaks the ODR: each TU compiles its own definition
of the same function, and the linker rejects the duplicate symbols.

`inline` changes the rule: it asserts "every definition of this
function/variable, in every TU, will be identical" -- and the linker keeps
only one copy (typically choosing whichever it sees first, or merging via
COMDAT sections). This is *exactly* what templates need too (which is why
function/class templates -- intermediate/01-02 -- can be defined in headers
without `inline`: template definitions are implicitly treated this way).

```cpp
// exercise.h -- defined (not just declared) here, included by 2 TUs
inline bool nearlyEqual(double a, double b) { return std::abs(a - b) < kEpsilon; }
inline constexpr double kEpsilon = 1e-9;
```

Without `inline` on `nearlyEqual`, linking `exercise.o` and
`exercise_test.o` together would fail with "multiple definition of
`nearlyEqual'". `kEpsilon` doesn't strictly need `inline` to avoid a link
error (it's `constexpr`, hence internal linkage, hence each TU's copy is its
own non-conflicting symbol) -- but `inline` (C++17 "inline variables") makes
it ONE shared object across TUs rather than per-TU copies, which matters if
its address is ever taken or it's passed by reference.

### Modern rule of thumb

> "Does this function/variable's *definition* live in a header, included by
> more than one TU?" If yes, and it's not a template (templates get this
> automatically) -- it needs `inline`.

## The One Definition Rule (ODR), summarized

- Every function/variable/class/template with external linkage must have
  **exactly one** definition across the whole program -- *except* `inline`
  entities and templates, which may be defined identically in multiple TUs
  (the duplicates are merged).
- `#pragma once` / include guards prevent a *single TU* from seeing the same
  header's declarations twice (a `#include`d-twice problem) -- a different,
  narrower problem than the ODR (a *cross-TU* problem). Both matter, for
  different reasons; you need both.

## Further Reading

- [MCPP ch. 13 -- Translation Units and Headers I](https://federico-busato.github.io/Modern-CPP-Programming/htmls/13.Translation_Units_I.html)
- [cppreference: Translation units](https://en.cppreference.com/w/cpp/language/translation_phases)
- [cppreference: Storage duration](https://en.cppreference.com/w/cpp/language/storage_duration)
- [cppreference: Language linkage / `extern`](https://en.cppreference.com/w/cpp/language/storage_duration#Linkage)
- [cppreference: Unnamed (anonymous) namespaces](https://en.cppreference.com/w/cpp/language/namespace#Unnamed_namespaces)
- [cppreference: `inline` specifier](https://en.cppreference.com/w/cpp/language/inline)
- [cppreference: Definitions and ODR](https://en.cppreference.com/w/cpp/language/definition)
