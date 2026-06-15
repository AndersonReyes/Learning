# Intermediate 07: Standard Library Utilities

Five small, widely-used standard-library facilities: `std::optional`,
`std::from_chars`/`std::to_chars` (`<charconv>`), `std::variant`/`std::visit`,
`<random>`, `std::filesystem::path`, and `std::string_view`.

## `std::optional<T>`

Represents "a `T`, or nothing" -- without sentinel values (`-1`, `""`,
nullptr) or output parameters.

```cpp
std::optional<int> find(const std::vector<int>& v, int target) {
    for (size_t i = 0; i < v.size(); ++i)
        if (v[i] == target) return i;
    return std::nullopt;  // or {}
}

std::optional<int> idx = find(v, 5);
if (idx) { /* idx.has_value() */ }
int i = idx.value();        // throws std::bad_optional_access if empty
int j = *idx;                // UB if empty -- no check
int k = idx.value_or(-1);   // 0 if empty... well, -1 here
```

- `has_value()` / `operator bool()` -- same thing, bool form is idiomatic.
- `operator*` / `operator->` -- unchecked access, like a pointer. UB if empty.
- `value()` -- checked access, throws on empty.
- `value_or(default)` -- get-or-default, no branching needed.
- Comparable directly to `T` and to `std::nullopt`:
  `opt == 5`, `opt == std::nullopt`, `opt.has_value()`.
- `std::optional<int>{}` and `std::nullopt` both construct an empty optional.

Gotcha: `std::optional<T*>` is redundant -- a pointer already has a null
state. `std::optional` shines for value types (`int`, `struct`s) that have
no natural "missing" sentinel.

## `<charconv>`: `std::from_chars` / `std::to_chars`

Locale-independent, allocation-free, **never-throwing** numeric
conversions. Report success/failure via a result struct, not exceptions or
errno.

```cpp
int value;
auto [ptr, ec] = std::from_chars(s.data(), s.data() + s.size(), value);
// ec == std::errc{}        -> success
// ec == std::errc::invalid_argument  -> no conversion (e.g. empty, "+7")
// ec == std::errc::result_out_of_range -> parsed but overflows `value`'s type
// ptr -> one past the last character consumed
```

Key rules that trip people up:

- **Only an optional leading `-`** is accepted -- no `+`, no leading/trailing
  whitespace, no `0x` prefix (unless you pass `base = 16`).
- **Partial parses succeed**: `from_chars("123abc", ...)` parses `123` and
  returns `ec == std::errc{}` with `ptr` pointing at `'a'`. To require the
  *whole* string be consumed, check `ptr == s.data() + s.size()`.
- **Overflow is detected**: `from_chars("99999999999999999999", ...)` (for
  an `int` out param) returns `ec == std::errc::result_out_of_range`. The
  value of `value` itself is left unmodified on this error.
- `std::to_chars(buf, buf+N, value)` is the inverse: writes the shortest
  round-trippable text representation, no allocation, returns
  `{ptr, ec}` where `ptr` is one-past-the-last-character-written.

Compare to `std::stoi`/`std::atoi`: those throw/UB on bad input, skip
leading whitespace, and allow a leading `+`/`-` -- different (looser) rules.
`from_chars` is the right tool when you need to validate "is this string
*exactly* a number".

```cpp
std::optional<int> parseInt(std::string_view s) {
    if (s.empty()) return std::nullopt;
    int value = 0;
    auto [ptr, ec] = std::from_chars(s.data(), s.data() + s.size(), value);
    if (ec != std::errc{} || ptr != s.data() + s.size()) return std::nullopt;
    return value;
}
```

(The `s.empty()` check isn't strictly required -- `from_chars` on an empty
range already returns `invalid_argument` -- but it documents intent.)

## `std::variant<Ts...>` + `std::visit`

A type-safe union: holds exactly one of `Ts...` at a time, knows which one,
and never holds an invalid/uninitialized state (unlike a C `union`).

```cpp
using JsonValue = std::variant<std::monostate, bool, int, double, std::string>;

JsonValue v = 42;                 // holds int
v = std::string("hi");            // now holds std::string
v = std::monostate{};             // "no value" -- variant's default state
```

- `std::monostate` is an empty tag type, useful as the "default/none"
  alternative for a variant whose other types aren't default-constructible
  or where you want an explicit "nothing here" state (the JSON `null`).
- `std::holds_alternative<T>(v)` -- is the active type `T`?
- `std::get<T>(v)` -- access as `T`, throws `std::bad_variant_access` if
  inactive. `std::get_if<T>(&v)` -- pointer form, `nullptr` if inactive.
- **`std::visit`** -- the idiomatic way to handle *every* alternative,
  exhaustively, at compile time. Pass a callable (often a generic lambda
  with `if constexpr`) that can be invoked with each alternative type:

```cpp
std::string jsonValueToString(const JsonValue& value) {
    return std::visit(
        [](const auto& v) -> std::string {
            using T = std::decay_t<decltype(v)>;
            if constexpr (std::is_same_v<T, std::monostate>) {
                return "null";
            } else if constexpr (std::is_same_v<T, bool>) {
                return v ? "true" : "false";
            } else if constexpr (std::is_same_v<T, std::string>) {
                return "\"" + v + "\"";
            } else {  // int or double
                std::ostringstream oss;
                oss << v;
                return oss.str();
            }
        },
        value);
}
```

`if constexpr` is required here: without it, every branch of the lambda
body would need to compile for *every* `T` (e.g. `v ? "true" : "false"`
doesn't compile for `std::string`). `if constexpr` discards the
non-matching branches at compile time, per-instantiation.

Gotcha: `bool` and numeric types convert to each other implicitly --
`JsonValue v = true;` picks the `bool` alternative (exact match wins over
conversions), but be careful with ambiguous literal types in variant
construction.

## `<random>`: engines vs. distributions

A *random number engine* (e.g. `std::mt19937`) produces a deterministic
sequence of unsigned integers from a seed -- pure algorithm, fully
specified by the standard. A *distribution* (e.g.
`std::uniform_int_distribution`) maps engine output onto a desired range
and shape.

```cpp
std::mt19937 rng(42);          // seeded Mersenne Twister
unsigned int x = rng();        // raw 32-bit output, advances state

std::uniform_int_distribution<int> dist(0, 9);
int y = dist(rng);              // a number in [0, 9], ALSO advances state
```

**Portability gotcha**: `std::mt19937`'s *raw output sequence* for a given
seed is fully specified by the standard (it's a literal algorithm with
fixed parameters) -- `rng()` produces the *same* sequence of values on
every conforming implementation. But `std::uniform_int_distribution`'s
algorithm for mapping that raw sequence onto `[a, b]` is
**implementation-defined** -- the *same* seed and range can produce
*different* sequences on libstdc++ vs. libc++ vs. MSVC's STL.

So: if you need byte-for-byte reproducible "random" output across
toolchains (tests, save files, replays), either call the engine directly
(`rng() % n`, accepting the resulting modulo bias for small `n`) or use a
fixed, hand-rolled mapping -- not `std::uniform_int_distribution`.

Fisher-Yates (Durstenfeld) shuffle, calling the engine directly:

```cpp
for (size_t i = items.size(); i > 1; --i) {
    size_t j = rng() % i;       // j in [0, i-1]... actually [0, i)
    std::swap(items[i - 1], items[j]);
}
```

Each iteration places a uniformly-random remaining element at position
`i - 1`, then shrinks the "remaining" range by one. Single-element and
empty vectors are no-ops (the loop body never runs).

## `std::filesystem::path` and `lexically_normal`

`<filesystem>` (C++17) provides `std::filesystem::path` for manipulating
paths *as text* -- constructing, decomposing, and joining paths without
necessarily touching the filesystem. Functions that *do* touch disk
(`exists`, `is_directory`, `file_size`, directory iteration, ...) are a
separate, larger part of the library not used in this exercise.

```cpp
std::filesystem::path p = "a/./b/../c";
p.lexically_normal();          // -> "a/c" (path, platform separators)
p.generic_string();            // -> "a/c" using '/' regardless of platform
```

`lexically_normal()` performs **purely textual** normalization (no symlink
resolution, no filesystem access -- the path need not exist):

- Removes `.` components.
- Resolves `name/..` pairs by removing both.
- A leading `..` (or one that would go above the root) is **discarded**,
  not turned into `../..` -- e.g. `/a/../../b` -> `/b`.
- A trailing separator is preserved as a trailing `.` is implied --
  `./a/b/` normalizes to `a/b/` (the trailing slash survives).
- An entirely-resolved-away path becomes `"."`.

```cpp
std::string normalizedPath(const std::string& path) {
    return std::filesystem::path(path).lexically_normal().generic_string();
}
```

`generic_string()` always uses `/` as the separator (vs. `string()`, which
uses the native separator -- `\` on Windows) -- important for producing
portable, testable output.

## `std::string_view`

A non-owning "view" into character data: a pointer + length, no allocation,
trivially copyable. Used for read-only access -- function parameters that
only need to *read* string data shouldn't force a `std::string` copy or
construction from a C-string.

```cpp
void greet(std::string_view name) {   // works with std::string, "literal", or substrings
    std::cout << "Hello, " << name << "\n";
}

std::string s = "hello world";
std::string_view sub = std::string_view(s).substr(0, 5);  // "hello", no copy
```

**Lifetime gotcha**: a `string_view` does not own its data. If the
underlying buffer (a `std::string`, array, etc.) is destroyed or
reallocated (e.g. the `std::string` it points into is modified or goes out
of scope), the `string_view` dangles -- using it is UB. Never return a
`string_view` into a local `std::string`, and be wary of returning views
into a container that the caller might subsequently mutate.

```cpp
std::vector<std::string_view> splitOnWhitespace(std::string_view s) {
    std::vector<std::string_view> tokens;
    size_t i = 0;
    while (i < s.size()) {
        while (i < s.size() && std::isspace(static_cast<unsigned char>(s[i]))) ++i;
        size_t start = i;
        while (i < s.size() && !std::isspace(static_cast<unsigned char>(s[i]))) ++i;
        if (i > start) tokens.push_back(s.substr(start, i - start));
    }
    return tokens;
}
```

Each returned `string_view` aliases `s` -- valid only as long as the
original string data `s` refers to remains alive. `std::isspace` takes an
`int`; cast `char` to `unsigned char` first to avoid UB on negative `char`
values (chars >= 0x80 on platforms where `char` is signed).

## Further Reading

- [Modern C++ Programming, ch. 19: Standard Library Utilities](https://federico-busato.github.io/Modern-CPP-Programming/htmls/19.Utilities.html)
- [cppreference: std::optional](https://en.cppreference.com/w/cpp/utility/optional)
- [cppreference: std::from_chars](https://en.cppreference.com/w/cpp/utility/from_chars)
- [cppreference: std::to_chars](https://en.cppreference.com/w/cpp/utility/to_chars)
- [cppreference: std::variant](https://en.cppreference.com/w/cpp/utility/variant)
- [cppreference: std::visit](https://en.cppreference.com/w/cpp/utility/variant/visit)
- [cppreference: <random>](https://en.cppreference.com/w/cpp/numeric/random)
- [cppreference: std::mt19937](https://en.cppreference.com/w/cpp/numeric/random/mersenne_twister_engine)
- [cppreference: std::filesystem::path](https://en.cppreference.com/w/cpp/filesystem/path)
- [cppreference: lexically_normal](https://en.cppreference.com/w/cpp/filesystem/path/lexically_normal)
- [cppreference: std::string_view](https://en.cppreference.com/w/cpp/string/basic_string_view)
