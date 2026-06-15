# Intermediate 04: Multi-File Projects, `#include`, Modules & Libraries

## `#include`: quote vs. angle forms, and search paths

```cpp
#include <vector>     // angle form: searches "system" include paths first
#include "shapes.h"   // quote form: searches the including file's directory first
```

- **Angle form (`<...>`)**: searches implementation-defined system directories
  (standard library headers, `/usr/include`, paths from `-isystem`). By
  convention, used for standard-library and third-party headers.
- **Quote form (`"..."`)**: first searches the directory containing the
  **current file** (not the current working directory!), then falls back to
  the same search as the angle form. Used for "my project's own headers".
- **`-I<dir>`**: adds `<dir>` to the quote-form (and angle-form) search path.
  This topic's own build command uses it:

  ```
  g++ -std=c++20 -Wall -Wextra -Ilib -o test exercise_test.cpp exercise.cpp lib/shapes.cpp
  ```

  `exercise.h` and `exercise_test.cpp` (both in the topic root) write
  `#include "shapes.h"`. Neither file's own directory contains `shapes.h` --
  it lives in `lib/`. `-Ilib` adds `lib/` to the search path, so the quote
  form finds it there. `lib/shapes.cpp`'s own `#include "shapes.h"` resolves
  via the *first* rule (same directory as the including file, `lib/`) and
  needs no `-I` at all.

### Project layout for this topic

```
intermediate/04-modules-and-libraries/
  exercise.h         #include "shapes.h"  -- resolved via -Ilib
  exercise.cpp
  exercise_test.cpp  #include "shapes.h"  -- resolved via -Ilib
  lib/
    shapes.h         Point, operator==, operator<<, distance
    shapes.cpp       #include "shapes.h" -- resolved via same-directory rule
```

`lib/` is a small, fully-implemented "library" of geometric primitives that
`exercise.h`/`.cpp`'s 5 functions build on. Three `.cpp` files are compiled
(each into its own `.o`) and linked into one `test` binary -- this is what a
"multi-file project" means in practice: more translation units, tied together
by shared headers and the linker, not by anything magic.

## Forward declarations & incomplete types

A **forward declaration** (`class Engine;`) introduces a name as an
**incomplete type**: the compiler knows it exists but not its size or
members. Some things work fine with only a forward declaration; others need
the **complete type** (the full `class`/`struct` definition):

| Works with incomplete type | Needs complete type |
|---|---|
| `Engine*`, `Engine&` (declaring pointer/reference types) | `Engine e;` (defining an object -- needs `sizeof`) |
| Function declarations using `Engine*`/`Engine&` parameters | `e.member` / `e->member` (needs to know the members) |
| `std::unique_ptr<Engine>` declarations (not destruction) | `sizeof(Engine)`, inheritance from `Engine` |

`examples.cpp` demonstrates the pattern: `class Engine;` forward-declares the
type, `void describeEngine(const Engine* engine);` is declared using only
`Engine*`, and `Engine`'s full definition appears later -- by the time
`describeEngine`'s *body* is defined (which calls `engine->name()`), `Engine`
is complete.

**Why bother?** In a real multi-file project, a header that only needs
`Engine*` in function signatures doesn't need to `#include "engine.h"` at
all -- just `class Engine;`. This cuts compile-time coupling: changing
`Engine`'s private members doesn't force recompilation of every file that
merely passes `Engine*` around, only files that `#include "engine.h"` for the
complete type.

## `extern "C"`: calling C functions from C++

C++ **mangles** function names -- encoding parameter types into the symbol
name -- so overloaded functions (`add(int, int)` vs `add(double, double)`)
get distinct linker symbols. C has no overloading and doesn't mangle names.

To declare a function with C linkage (so the linker looks for the unmangled
name -- e.g. when linking against a library compiled from C source):

```cpp
extern "C" {
int c_library_add(int a, int b);
}
```

`examples.cpp` declares `c_library_add` this way (and, just so the example
links and runs standalone, also *defines* it with `extern "C"`). In a real
project, the declaration would come from a C library's header (often itself
wrapped in `#ifdef __cplusplus extern "C" { ... } #endif` so the same header
works for both C and C++ callers), and the definition would live in a
precompiled `.a`/`.so`.

## C++20 Modules (conceptual only -- not built/tested here)

C++20 introduces **modules** as an alternative to headers. Illustrative
syntax (this is NOT compiled as part of this topic):

```cpp
// shapes.cppm -- a module interface unit
export module shapes;

export struct Point {
    double x, y;
};

export double distance(const Point& a, const Point& b);
```

```cpp
// main.cpp -- a module consumer
import shapes;

int main() {
    Point p{0, 0};
    // ...
}
```

How modules differ from `#include`-based headers:

- **No repeated textual inclusion**: `#include` is a textual copy-paste,
  re-parsed by every TU that includes the header (even with `#pragma once`,
  the *parsing* still happens per-TU, just not duplicate definitions).
  `import` loads a pre-compiled module interface -- parsed once.
- **Explicit exports**: only names marked `export` are visible to importers.
  Headers expose *everything* in the header to every includer.
- **No macro leakage**: macros `#define`d in a module are NOT visible to
  importers (unlike `#include`, where macros from the included file pollute
  the includer's preprocessor state for the rest of the TU).
- **Order-independence**: unlike headers (where include order can matter for
  macros/declarations), module import order doesn't affect meaning.

### Why this topic doesn't build with modules

As of this curriculum's timeframe, GCC's C++20 module support is still
version-dependent and requires extra build steps (precompiling module
interface units in dependency order, non-portable flags). Header + `-I` +
multi-`.cpp`-file builds -- what this topic actually uses -- remain the
portable, universally-supported baseline. Modules are worth knowing
*conceptually* (you'll see `import std;` and `export module` in modern
codebases/tutorials), but this track sticks to headers throughout.

## Libraries: static (`.a`) vs. shared (`.so`)

This topic's build compiles `lib/shapes.cpp` as source, every time, alongside
`exercise.cpp`/`exercise_test.cpp`. A real "library" is usually pre-built once
and then linked, in one of two forms:

### Static libraries (`.a`)

```sh
g++ -std=c++20 -Wall -Wextra -Ilib -c lib/shapes.cpp -o shapes.o
ar rcs libshapes.a shapes.o

g++ -std=c++20 -Wall -Wextra -Ilib -o test exercise_test.cpp exercise.cpp -L. -lshapes
```

- `ar rcs` archives one or more `.o` files into `libshapes.a`.
- `-L.` adds `.` to the **library search path**; `-lshapes` links
  `libshapes.a` (the `lib`/`.a` are added automatically).
- The library's code is **copied into** the final executable at link time --
  no runtime dependency on the `.a` file.

### Shared libraries (`.so`)

```sh
g++ -std=c++20 -Wall -Wextra -Ilib -fPIC -c lib/shapes.cpp -o shapes.o
g++ -shared -o libshapes.so shapes.o

g++ -std=c++20 -Wall -Wextra -Ilib -o test exercise_test.cpp exercise.cpp -L. -lshapes
LD_LIBRARY_PATH=. ./test     # or install libshapes.so somewhere ldconfig knows
```

- `-fPIC` (Position-Independent Code) is required for code going into a
  shared library.
- `-shared` produces `libshapes.so`.
- The executable records a dependency on `libshapes.so` but does NOT copy its
  code -- it's loaded at **runtime** by the dynamic linker. `ldd ./test` lists
  these dependencies; `LD_LIBRARY_PATH` (or installing into a standard
  location) tells the dynamic linker where to find `libshapes.so`.
- Multiple programs can share one copy of `libshapes.so` in memory, and
  updating the library doesn't require relinking executables -- **as long as
  its ABI (Application Binary Interface) doesn't change**: function
  signatures, struct layouts, vtable layouts must stay compatible, or
  every program linked against the old `.so` may crash or misbehave at
  runtime with the new one.

### When to use which

| | Static (`.a`) | Shared (`.so`) |
|---|---|---|
| Executable size | Larger (code copied in) | Smaller |
| Runtime dependency | None | Must locate `.so` at load time |
| Updating the library | Requires relinking | Drop-in replacement (if ABI-compatible) |
| Multiple processes | Each gets its own copy | Share one copy in memory |

This topic's source-based build (recompiling `lib/shapes.cpp` every time) is
the simplest option for a small, fast-changing internal library -- no archive
or `.so` to manage, no ABI concerns, since everything is rebuilt together.

## Further Reading

- [MCPP ch. 14 -- Translation Units and Headers II](https://federico-busato.github.io/Modern-CPP-Programming/htmls/14.Translation_Units_II.html)
- [cppreference: `#include`](https://en.cppreference.com/w/cpp/preprocessor/include)
- [cppreference: Incomplete type](https://en.cppreference.com/w/cpp/language/type#Incomplete_type)
- [cppreference: Language linkage (`extern "C"`)](https://en.cppreference.com/w/cpp/language/language_linkage)
- [cppreference: Modules](https://en.cppreference.com/w/cpp/language/modules)
