# Intermediate 02: Class Templates, CTAD, SFINAE & Concepts

## Class templates

Like function templates, a class template is a blueprint -- the compiler
generates a distinct class for each set of template arguments used.

```cpp
template <typename T>
class Box {
public:
    explicit Box(T value) : value_(value) {}
    const T& get() const { return value_; }

private:
    T value_;
};

Box<int> a(42);
Box<std::string> b("hi");
```

- `Box<int>` and `Box<std::string>` are two *distinct* types, each compiled
  and instantiated independently -- a bug in `Box<std::string>::get()` won't
  surface unless that instantiation is used.
- Member functions of class templates are themselves implicitly templates --
  `Box<T>::get()` is a different function for each `T`.
- Like function templates, the full definition (member bodies included) must
  be visible wherever the class is instantiated -- so class templates live in
  headers too.

## Class Template Argument Deduction (CTAD), C++17+

Before C++17, instantiating a class template required spelling out every
template argument: `std::pair<int, double> p(1, 2.5)`. CTAD lets the
compiler deduce them from constructor arguments, like function template
deduction:

```cpp
std::pair p(1, 2.5);   // deduces std::pair<int, double>
Box b(42);             // deduces Box<int>, via Box's own constructor
```

### Implicit deduction guides

For a simple class template whose constructor parameters directly use the
class's template parameters (like `Box(T value)` above), the compiler
synthesizes an *implicit deduction guide* from each constructor automatically
-- no extra code needed.

### Explicit deduction guides

Sometimes `T` does **not** appear directly in the constructor's parameter
types, so no implicit guide can deduce it. Example: a class storing
`std::vector<T>`, constructed from an iterator *pair* (the constructor is
templated on `Iter`, not `T`):

```cpp
template <typename T>
class Sequence {
public:
    template <typename Iter>
    Sequence(Iter first, Iter last) : data_(first, last) {}
    // ...
private:
    std::vector<T> data_;
};
```

`Sequence(v.begin(), v.end())` can't deduce `T` from `Iter` alone -- `T` is
only used inside the class body. An **explicit deduction guide** (a
function-template-like declaration at namespace scope, using `->` to name
the resulting specialization) tells the compiler how to compute `T`:

```cpp
template <typename Iter>
Sequence(Iter, Iter) -> Sequence<typename std::iterator_traits<Iter>::value_type>;
```

Now `Sequence seq(v.begin(), v.end())` for `std::vector<int> v` deduces
`Iter = std::vector<int>::iterator`, then
`std::iterator_traits<Iter>::value_type = int`, giving `Sequence<int>`.
Deduction guides are declarations only -- they have no body, and participate
in overload resolution among themselves (implicit + explicit) to pick the
best match for the constructor call.

## SFINAE: Substitution Failure Is Not An Error

When the compiler substitutes deduced/specified template arguments into a
function template's signature (return type, parameter types), if that
substitution produces an **invalid type** (not a *body* error -- a
signature-level error), the compiler doesn't emit a hard error. It just
removes that candidate from overload resolution, as if it were never
written. This is SFINAE.

`std::enable_if_t<Cond, T>` (from `<type_traits>`) is the classic SFINAE
tool:

- If `Cond` is `true`, `std::enable_if_t<Cond, T>` is just `T`.
- If `Cond` is `false`, `std::enable_if<Cond, T>` has **no member `type`** --
  so `std::enable_if_t<false, T>` is ill-formed. Used as a return type, that
  makes the *whole function signature* ill-formed -- which SFINAE then
  quietly drops from the candidate set.

```cpp
template <typename T>
std::enable_if_t<std::is_arithmetic_v<T>, T> doubleValue(T x) {
    return x * 2;
}

template <typename T>
std::enable_if_t<!std::is_arithmetic_v<T>, T> doubleValue(T x) {
    return x + x;
}

doubleValue(5);              // only the first overload's signature is valid -> 10
doubleValue(std::string("ab")); // only the second is valid -> "abab"
```

For `T = int`: the first overload's return type is `enable_if_t<true, int> =
int` (valid); the second's is `enable_if_t<false, int>` (invalid, *removed*
from the candidate set -- not a compile error). Exactly one overload survives
for any given `T`.

### `if constexpr` vs. SFINAE

`if constexpr` (intermediate/01) picks a branch *within one function body* at
compile time. SFINAE/`enable_if` picks *which overload/specialization* even
exists in the first place. They solve overlapping problems; `if constexpr`
is usually simpler when one function body can express all cases. SFINAE (and
Concepts, below) are needed when you want genuinely separate
overloads/specializations -- e.g., different return types, or enabling a
*member function* only for certain `T`.

## Concepts (C++20)

Concepts are a more readable, better-diagnosed replacement for many SFINAE
use cases. A `concept` names a compile-time predicate over a type:

```cpp
template <typename T>
concept Comparable = requires(const T& a, const T& b) {
    a < b;
};
```

`requires(params) { expr1; expr2; ... }` is a *requires-expression*: it's
`true` if every listed expression would be valid (well-formed) for the given
parameter types, `false` otherwise. Here, `Comparable<T>` is `true` iff
`a < b` compiles for `const T&` operands.

Use a concept to constrain a template parameter directly:

```cpp
template <Comparable T>
T findMax(const std::vector<T>& values) {
    T best = values[0];
    for (size_t i = 1; i < values.size(); ++i) {
        if (best < values[i]) best = values[i];
    }
    return best;
}
```

`template <Comparable T>` is shorthand for
`template <typename T> requires Comparable<T>`. If `findMax` is called with
a `T` that doesn't satisfy `Comparable` (e.g. a type with no `operator<`),
the error message names the unsatisfied concept directly -- far clearer than
a wall of SFINAE substitution-failure noise.

### Concepts vs. `if constexpr`/SFINAE here

- `if constexpr` (topic 11's `typeCategory`): one function, branches pruned
  per instantiation.
- SFINAE/`enable_if` (`doubleValue` above): multiple overloads, the invalid
  one's *signature* is silently dropped.
- Concepts (`findMax`): a *constraint* on what `T` may be at all --
  `findMax<SomeNonComparableType>` is rejected outright, with a clear
  "constraints not satisfied" diagnostic, before ever trying to compile the
  body.

## Member function templates

A class template's member can itself be a template, with its *own*
independent template parameter(s):

```cpp
template <typename T>
class Optional {
public:
    template <typename Func>
    auto map(Func f) const -> Optional<std::invoke_result_t<Func, T>> {
        using R = std::invoke_result_t<Func, T>;
        if (hasValue_) return Optional<R>(f(value_));
        return Optional<R>();
    }
    // ...
};
```

`map`'s `Func` is deduced per call from the lambda/callable passed in --
independent of `T`, the class's own template parameter. `std::invoke_result_t
<Func, T>` (from `<type_traits>`, C++17) computes the return type of calling
`f` with a `T` argument -- so `optionalInt.map([](int x){ return
std::to_string(x); })` returns `Optional<std::string>`: `map` can change the
contained type, going from `Optional<T>` to `Optional<R>` for any `R`.

## Further Reading

- [MCPP ch. 12 -- Templates and Meta-programming II](https://federico-busato.github.io/Modern-CPP-Programming/htmls/12.Templates_II.html)
- [cppreference: Class templates](https://en.cppreference.com/w/cpp/language/class_template)
- [cppreference: Class template argument deduction (CTAD)](https://en.cppreference.com/w/cpp/language/class_template_argument_deduction)
- [cppreference: `std::enable_if`](https://en.cppreference.com/w/cpp/types/enable_if)
- [cppreference: SFINAE](https://en.cppreference.com/w/cpp/language/sfinae)
- [cppreference: Constraints and concepts](https://en.cppreference.com/w/cpp/language/constraints)
- [cppreference: `requires` expressions](https://en.cppreference.com/w/cpp/language/requires)
- [cppreference: `std::invoke_result`](https://en.cppreference.com/w/cpp/types/result_of)
- [cppreference: `std::iterator_traits`](https://en.cppreference.com/w/cpp/iterator/iterator_traits)
