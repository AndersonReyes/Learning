# Intermediate 01: Function Templates & Compile-Time Utilities

## Function templates

A template is a blueprint; the compiler generates ("instantiates") a real
function for each set of template arguments actually used.

```cpp
template <typename T>
T maxOf(const T& a, const T& b) {
    return (a < b) ? b : a;
}

maxOf(3, 7);        // instantiates maxOf<int>
maxOf(3.0, 7.5);     // instantiates maxOf<double>
maxOf(std::string("a"), std::string("b"));  // maxOf<std::string>
```

- `typename` and `class` are interchangeable in this position (`template
  <class T>` is identical to `template <typename T>`).
- Each instantiation is a fully separate function -- compiled, type-checked,
  and optimized independently. A bug only triggered by `T = std::string`
  won't show up when only `maxOf<int>` is used.
- **Templates must be fully defined wherever they're used.** The compiler
  needs the full body at the point of instantiation, so template
  definitions normally live in headers, not `.cpp` files (more on this in
  `intermediate/03`, Translation Units & the ODR).

## Template argument deduction

The compiler infers `T` from the function arguments -- no need to write
`maxOf<int>(3, 7)` explicitly (though you can, to force a type or disambiguate).

```cpp
template <typename T>
T identity(T x) { return x; }

identity(42);     // T = int
identity(42.0);   // T = double
identity<long>(42); // T explicitly long, even though 42 is int
```

**Independent type parameters**: each template parameter is deduced
separately from its corresponding argument(s). `T` and `U` need not match:

```cpp
template <typename T, typename U>
auto add(T a, U b) { return a + b; }

add(2, 3.5);  // T=int, U=double, returns 5.5
```

### Multiple deductions must agree

If the same template parameter `T` is used for multiple arguments (like
`clampValue(const T& value, const T& low, const T& high)` below), every
argument must deduce to the *same* `T` -- no implicit conversions during
deduction. `clampValue(5, 0.0, 10)` is a compile error: `T` would be `int`
from the first argument but `double` from the second. Pass matching types,
or cast explicitly.

## Trailing return types: `decltype` + `std::declval`

`decltype(expr)` yields the type of `expr` without evaluating it. Combined
with `std::declval<T>()` (from `<utility>`), which conjures a "fake" `T&&`
value purely for type computation (never actually called), you can express
"the type of `a + b`" without needing real values of `a` and `b`:

```cpp
template <typename T, typename U>
auto addValues(T a, U b) -> decltype(std::declval<T>() + std::declval<U>()) {
    return a + b;
}
```

This is more precise than bare `auto` (which deduces the return type *from
the function body*) when the body's apparent return type might not match
what you want documented in the signature -- and it's required if the body
has no `return` statement at all reachable for some instantiation.

### Gotcha: `auto` + a body with no `return`

A function declared `auto foo(...)` with NO `return` statements anywhere
in its body deduces a return type of `void`. If you stub out a function
with `auto` and `throw std::logic_error("not implemented")` as its only
statement, the deduced return type is `void` -- and any caller that uses
the "return value" fails to compile, even though the throw never executes.
A trailing `decltype(...)` return type sidesteps this: the declared type is
fixed by the signature, independent of the body.

## Non-type template parameters + `constexpr`

Templates can take non-type parameters: compile-time constants (integers,
enums, pointers, references -- not arbitrary types).

```cpp
template <int N>
constexpr long long power(long long base) {
    if constexpr (N == 0) {
        return 1;
    } else {
        return base * power<N - 1>(base);
    }
}

power<3>(2);   // 8 -- a distinct instantiation per N
power<10>(2);  // a *different* function, power<10>
```

Each `N` produces a separate instantiation -- `power<3>` and `power<10>`
are different functions, and the recursion `power<N-1>` bottoms out at
`power<0>` at compile time (no runtime recursion overhead once inlined).

`constexpr` on a function means: **if** called with arguments that are
themselves compile-time constants, the call **can** be evaluated at compile
time (e.g. in a `static_assert`, as an array bound, or to initialize a
`constexpr` variable). It does NOT mean the function is *always* evaluated
at compile time -- called with a runtime `long long`, it just runs normally.

```cpp
static_assert(power<3>(2) == 8);   // evaluated at compile time
long long x = readFromStdin();
power<3>(x);                        // evaluated at runtime, same function
```

### Gotcha: `constexpr` + a stub body that always throws

A `constexpr` function whose body unconditionally `throw`s can still
compile cleanly -- the rule ("a `constexpr` function must have at least one
set of arguments for which it could produce a constant expression") is
*ill-formed, no diagnostic required* (IFNDR): compilers generally don't
reject it at the definition. It only becomes a hard error if you actually
try to use such a call in a constant-expression context (e.g.
`static_assert(power<3>(2) == 8)` with a throwing body -- THAT fails to
compile, because the call can't produce a constant). As long as a stub is
only exercised via ordinary runtime calls (`CHECK_EQ`, not `static_assert`),
a throwing `constexpr` stub is fine.

## Variadic templates + fold expressions

`Args...` is a *template parameter pack*; `args...` is the corresponding
*function parameter pack*. `sizeof...(args)` gives the pack size (can be 0).

A **fold expression** applies a binary operator across all elements of a
pack. `(... op pack)` is a left fold, `(pack op ...)` is a right fold; both
forms can include an initial value:

```cpp
template <typename... Args>
long long sumAll(Args... args) {
    return (static_cast<long long>(args) + ... + 0LL);
}
```

`(static_cast<long long>(args) + ... + 0LL)` is a right fold with init
value `0LL`. For `sumAll(1, 2, 3)`, it expands to
`1 + (2 + (3 + 0LL))`. For `sumAll()` (empty pack), it's just `0LL` -- the
init value lets an empty pack still produce a sensible result (a fold over
an empty pack with NO init value, like `(... + args)`, is a compile error
for zero arguments).

## `if constexpr` + `<type_traits>`

`if constexpr (cond)` evaluates `cond` at compile time and **discards** the
untaken branch entirely -- it doesn't even need to type-check for the
current instantiation's `T`.

```cpp
template <typename T>
std::string typeCategory(const T& value) {
    if constexpr (std::is_integral_v<T>) {
        return (value % 2 == 0) ? "integral:even" : "integral:odd";
    } else if constexpr (std::is_floating_point_v<T>) {
        return (value == static_cast<T>(static_cast<long long>(value)))
                   ? "floating-point:whole" : "floating-point:fractional";
    } else {
        return "other";
    }
}
```

For `T = std::string`, `value % 2` would be a compile error if it were a
plain `if` -- `operator%` isn't defined for `std::string`. With
`if constexpr`, that branch is pruned for `T = std::string` before the
compiler ever tries to type-check `value % 2` against `std::string`.

Useful traits from `<type_traits>` (all `_v` variants are
`constexpr bool`, C++17+):

- `std::is_integral_v<T>` -- true for `int`, `long`, `char`, `bool`, etc.
- `std::is_floating_point_v<T>` -- true for `float`, `double`, `long double`.
- `std::is_same_v<T, U>` -- true if `T` and `U` are the exact same type.
- `std::is_pointer_v<T>`, `std::is_class_v<T>`, `std::is_arithmetic_v<T>`, etc.

## Gotcha: name collisions with `std::` via ADL

Argument-Dependent Lookup (ADL) means an unqualified call `foo(x)` also
searches the namespace(s) of `x`'s type. If you write a function template
named `clamp` and call `clamp(someStdString, ...)`, ADL searches namespace
`std` too (because `std::string` lives there) -- and `<algorithm>`'s
`std::clamp` (C++17) becomes a candidate alongside your `::clamp`. Both are
equally viable via template deduction, so the call is **ambiguous: a hard
compile error**, not a quiet pick of "your" overload. This bit the exercise
below -- `clamp` was renamed to `clampValue` to avoid colliding with
`std::clamp` (pulled in transitively via `<functional>` -> `<algorithm>`
from `testing.h`). Builtin types (`int`, `double`) aren't affected --
ADL doesn't search `std` for them, only for types actually declared there.

**Takeaway**: avoid naming your own generic utilities after names that
already exist in `std` (`clamp`, `min`, `max`, `swap`, `move`, ...),
especially if you'll call them unqualified with standard-library argument
types.

## `cpp/testing.h`: `CHECK_EQ` and `checkEq`

This topic extends `cpp/testing.h` (built in `fundamentals/06`) with
`CHECK_EQ(a, b)`, which on failure prints *both operands' values*, not just
their source text.

```cpp
template <typename A, typename B>
void checkEq(const A& a, const B& b, const char* aExpr, const char* bExpr,
              const char* file, int line) {
    if (!(a == b)) {
        std::ostringstream oss;
        oss << file << ":" << line << ": CHECK_EQ failed: " << aExpr << " == " << bExpr
            << " (left = " << a << ", right = " << b << ")";
        throw CheckFailure(oss.str());
    }
}

#define CHECK_EQ(a, b) ::testing::checkEq((a), (b), #a, #b, __FILE__, __LINE__)
```

- `checkEq` is a function **template** -- it works for any `A`/`B` pair
  that support `==` and `operator<<(std::ostream&, ...)`, deduced
  independently per call site (e.g. `CHECK_EQ(addValues(2, 3.5), 5.5)`
  deduces `A = double, B = double`; `CHECK_EQ(power<0>(100), 1LL)` deduces
  `A = long long, B = long long`).
- It's a **function**, not a macro, so `a` and `b` are each evaluated
  exactly once -- important if either expression has side effects.
- The `#a`/`#b` stringification still happens in the macro (`CHECK_EQ`),
  since only the preprocessor can turn an expression into its literal
  source text; the *values* are captured by the function template via
  `operator<<`.
- Compare to plain `CHECK(a == b)`: that only reports the source text
  `a == b` on failure. `CHECK_EQ(a, b)` additionally prints the actual
  runtime values of `a` and `b`, which is far more useful for debugging
  ("left = 7, right = 8" vs. just "CHECK failed: a == b").

## Further Reading

- [MCPP ch. 11 -- Templates and Meta-programming I](https://federico-busato.github.io/Modern-CPP-Programming/htmls/11.Templates_I.html)
- [cppreference: Function templates](https://en.cppreference.com/w/cpp/language/function_template)
- [cppreference: Template argument deduction](https://en.cppreference.com/w/cpp/language/template_argument_deduction)
- [cppreference: `decltype`](https://en.cppreference.com/w/cpp/language/decltype)
- [cppreference: `std::declval`](https://en.cppreference.com/w/cpp/utility/declval)
- [cppreference: `constexpr` specifier](https://en.cppreference.com/w/cpp/language/constexpr)
- [cppreference: Non-type template parameters](https://en.cppreference.com/w/cpp/language/template_parameters)
- [cppreference: Fold expressions](https://en.cppreference.com/w/cpp/language/fold)
- [cppreference: `if constexpr`](https://en.cppreference.com/w/cpp/language/if#Constexpr_if)
- [cppreference: `<type_traits>`](https://en.cppreference.com/w/cpp/header/type_traits)
- [cppreference: `std::clamp`](https://en.cppreference.com/w/cpp/algorithm/clamp)
