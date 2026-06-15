# 6. Functions, Lambdas & the Preprocessor

## Functions, recap and extras

```cpp
int add(int a, int b);           // declaration (signature only)
int add(int a, int b) { return a + b; }  // definition

int greet(std::string name, std::string greeting = "Hello");  // default argument
greet("Ada");              // greeting defaults to "Hello"
greet("Ada", "Hi");
```

- A function's **signature** is its name + parameter types (not its return
  type) -- overload resolution picks among same-named functions by argument
  types. Two functions differing only in return type is a compile error.
- **Default arguments** are filled in from the *rightmost* parameter inward;
  once one parameter has a default, every parameter after it must too.
  Defaults are specified in the **declaration** (e.g. in a header), not
  repeated in the definition.
- **Recursion**: a function calling itself. Each call gets its own stack
  frame (its own copies of locals/parameters) -- this is just normal function
  call mechanics, nothing special. Deep recursion (thousands of frames) risks
  **stack overflow**; recursion with overlapping subproblems (e.g. naive
  Fibonacci) is exponential without **memoization** (caching results of
  subproblems, usually via a reference parameter or `static`/external cache).
- **Mutual recursion** (`a()` calls `b()` and `b()` calls `a()`) requires a
  **forward declaration** of one of them, since C++ reads top-to-bottom and a
  name must be declared before use.

## Function pointers

```cpp
int square(int x) { return x * x; }

int (*fn)(int) = square;   // fn points to square's code
int result = fn(5);         // calls square(5) -- "*fn" not required to call
```

- A function name used as a value **decays to a pointer to that function**
  (like array-to-pointer decay for arrays, topic 5).
- Reading `int (*fn)(int)`: `fn` is a pointer (`*fn`) to a function taking
  `int` and returning `int`. The parentheses around `*fn` are required --
  `int* fn(int)` would declare a function *returning* `int*`.
- `using IntFn = int (*)(int);` gives the type a readable name:
  `IntFn fn = square;`.
- A function-pointer parameter (`int (*fn)(int)`) lets a function take
  *behavior* as an argument -- the original "higher-order function" mechanism,
  predating lambdas and `std::function`.

## Lambdas

```cpp
auto add = [](int a, int b) { return a + b; };
add(2, 3);  // 5

int threshold = 10;
auto aboveThreshold = [threshold](int x) { return x > threshold; };  // capture by value
auto raiseThreshold = [&threshold](int x) { threshold = x; };         // capture by reference
```

- `[capture](params) { body }` -- an unnamed function object ("closure").
  `auto` is the natural type for a variable holding one (each lambda has a
  unique, compiler-generated type).
- **Capture modes**:
  - `[x]` -- capture `x` **by value** (a copy, made at lambda-creation time).
  - `[&x]` -- capture `x` **by reference** (sees later changes to `x`; danger
    of dangling if `x` outlives... or rather if the lambda outlives `x`).
  - `[=]` / `[&]` -- capture *everything* used in the body, by value /
    by reference respectively. Prefer explicit captures (`[x, &y]`) --
    `[=]`/`[&]` make it easy to accidentally capture more than intended.
  - `[]` -- no captures; the lambda can only use its parameters and
    globals/statics.
- **By-value captures are `const` inside the lambda body by default** --
  `[x](){ x++; }` is a compile error. Add `mutable` to allow mutating the
  *lambda's own copy* (the original variable outside is unaffected):
  ```cpp
  auto counter = [n = 0]() mutable { return n++; };
  counter();  // 0
  counter();  // 1 -- n persists across calls in *this* closure's state
  ```
- Each lambda **instantiation** has independent state: calling
  `makeCounter(10, 3)` twice produces two closures, each with its own
  captured `current`/`step` -- they don't share storage.
- **Returning a lambda that captures a local by reference is a dangling
  reference** (same rule as topic 5) -- if a closure must outlive the
  function that creates it, capture by value.

## `std::function`

```cpp
#include <functional>

std::function<int(int)> op = square;                 // holds a function pointer
op = [](int x) { return x * 2; };                     // ...or a lambda
op = [factor = 3](int x) { return x * factor; };      // ...or a lambda with captures
```

- `std::function<R(Args...)>` is a **type-erased wrapper** that can hold *any*
  callable (function pointer, lambda, with or without captures) matching the
  signature `R(Args...)`. Unlike a raw function pointer, it can hold a lambda
  with captured state.
- Use it for parameters/return types/containers that need to hold "a
  callable", when the concrete type doesn't matter to the caller -- e.g. a
  registry of named callbacks (`std::vector<std::pair<std::string,
  std::function<void()>>>`), or a function that builds and returns a closure
  (`std::function<long long()>`).
- Cost: a small amount of indirection/allocation overhead vs. a plain
  function pointer or a template parameter (templates, `intermediate/01`, let
  the compiler generate a specialized, inlined version per callable type --
  `std::function` trades that for a single, uniform runtime type).

## The preprocessor

The preprocessor runs *before* the compiler proper, doing pure text
substitution. `#pragma once` (topic 1) and `#include` are preprocessor
directives you've already used.

```cpp
#define MAX_RETRIES 3                  // object-like macro (a named constant)
#define SQUARE(x) ((x) * (x))           // function-like macro
#define STRINGIFY(x) #x                 // stringification: x -> "x"
#define CONCAT(a, b) a##b                // token-pasting: a, b -> ab
```

- **Object-like macros** are simple find-and-replace constants. Prefer
  `constexpr` (topic 2-3) -- it's type-checked and scoped, a macro is neither.
- **Function-like macros** substitute their arguments textually, with **no
  type checking** and **no scoping** -- `SQUARE` works on any type with `*`,
  for better or worse. **Always parenthesize** every parameter and the whole
  expansion (`((x) * (x))`, not `x * x`) -- otherwise `SQUARE(a + b)` expands
  to `a + b * a + b`, not `(a + b) * (a + b)`.
- **Macro arguments are substituted as-is, possibly multiple times** -- a
  macro argument with side effects is evaluated once per appearance in the
  expansion: `SQUARE(i++)` expands to `((i++) * (i++))` -- two increments of
  the same object with no sequencing between them, which is **undefined
  behavior**. A real function evaluates its argument exactly once, sequenced
  normally. This is the main reason to prefer `constexpr`/inline functions
  over function-like macros wherever possible.
- `#x` (**stringification**) turns the literal argument text into a string
  literal -- `STRINGIFY(1+2)` -> `"1+2"` (the *text*, not `"3"`).
- `a##b` (**token-pasting**) glues two tokens into one identifier at
  preprocessing time -- used to generate unique names per macro invocation
  (see `cpp/testing.h` below).
- **`do { ... } while (false)`**: wrapping a multi-statement macro body in
  this makes the whole macro act like one statement, so it works correctly
  after `if (cond)` without braces, requires a trailing `;` like a real
  statement, and can use `break`-like early exits inside if needed.
- Macros that remain genuinely useful in modern C++: conditional compilation
  (`#ifdef`/`#ifndef`, platform-specific code), `__FILE__`/`__LINE__`/`__func__`
  for diagnostics, and exactly this kind of small test-framework glue.

## Building `cpp/testing.h`

This topic's `runTests` exercise is the *core logic* of a test runner: given
a list of named thunks (`std::function<void()>`), run each one, catch any
`std::exception` it throws, and record pass/fail + message -- without letting
one failure stop the rest. `cpp/testing.h` (at the repo's `cpp/` root, shared
by every topic from `fundamentals/07` onward) wraps that same idea in three
macros:

```cpp
#define TEST(name)                                              \
    void name();                                                \
    static ::testing::Registrar registrar_##name(#name, name);  \
    void name()
```

- `TEST(Foo)` expands to: forward-declare `void Foo();`, declare a `static
  Registrar` (constructed *before* `main()` runs, appending `{"Foo", Foo}` --
  a name plus a function pointer that converts to `std::function<void()>` --
  to a global registry), then open `void Foo() { ... }` so the `{ ... }`
  written after `TEST(Foo)` becomes that function's body.
- `registrar_##name` uses token-pasting so each `TEST(...)` in a file declares
  a uniquely-named (and thus non-conflicting) `Registrar`.
- `#name` (stringification) turns `Foo` into `"Foo"` for the printed test name.

```cpp
#define CHECK(cond)                                                  \
    do {                                                             \
        if (!(cond)) {                                                \
            throw ::testing::CheckFailure(                            \
                std::string(__FILE__) + ":" + std::to_string(__LINE__) + \
                ": CHECK failed: " #cond);                            \
        }                                                              \
    } while (false)
```

- `CHECK(x == 3)` throws a `CheckFailure` (a `std::exception`) carrying the
  failed condition's source text (`#cond` -> `"x == 3"`) and its
  `__FILE__:__LINE__`, if the condition is false. The `do/while(false)`
  wrapper makes `CHECK(...)` a single statement.

```cpp
#define TEST_MAIN()                                            \
    int main() {                                               \
        int failed = 0;                                        \
        for (const auto& test : ::testing::registry()) {       \
            try {                                               \
                test.body();                                    \
                std::cout << "[PASS] " << test.name << "\n";    \
            } catch (const std::exception& e) {                 \
                std::cout << "[FAIL] " << test.name << ": "     \
                          << e.what() << "\n";                  \
                ++failed;                                        \
            }                                                    \
        }                                                        \
        /* ... print summary, return failed == 0 ? 0 : 1 */     \
    }
```

- `TEST_MAIN()` expands to a full `main()` -- this is the `runTests` exercise,
  generalized: iterate the registry, run each thunk, catch `std::exception`
  (including a `CheckFailure` from a failed `CHECK`, *or* a stub's
  `throw std::logic_error("not implemented")`), print `[PASS]`/`[FAIL]`, and
  exit non-zero if anything failed.
- `registry()` returns a reference to a function-local `static
  std::vector<Test>` -- guaranteed initialized on first use, so it works
  correctly no matter which translation unit's `static Registrar` runs first
  (sidesteps the "static initialization order fiasco" that plain global
  variables across files can suffer from).

From `fundamentals/07` onward, `exercise_test.cpp` looks like:

```cpp
#include "exercise.h"
#include "../../testing.h"

TEST(SomeBehavior) {
    CHECK(someFunction(1) == 2);
}

TEST_MAIN()
```

and stub bodies in `exercise.cpp` switch from sentinel return values to
`throw std::logic_error("not implemented");` -- `TEST_MAIN()` catches that and
reports it as a clean `[FAIL] ...: not implemented`, rather than aborting the
whole binary the way an unmet `assert` does.

## Further Reading (Modern C++ Programming)

- [Chapter 8 — Basic Concepts VI](https://federico-busato.github.io/Modern-CPP-Programming/htmls/08.Basic_Concepts_VI.html)
  (functions, lambdas, the preprocessor)
- [Function declarations](https://en.cppreference.com/w/cpp/language/functions)
- [Default arguments](https://en.cppreference.com/w/cpp/language/default_arguments)
- [Lambda expressions](https://en.cppreference.com/w/cpp/language/lambda)
- [`std::function`](https://en.cppreference.com/w/cpp/utility/functional/function)
- [The preprocessor](https://en.cppreference.com/w/cpp/preprocessor)
- [`#`/`##` operators](https://en.cppreference.com/w/cpp/preprocessor/replace)
