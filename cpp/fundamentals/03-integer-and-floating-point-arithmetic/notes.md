# 3. Integer & Floating-Point Arithmetic

## Integer overflow, recap and detection

- **Signed** integer overflow (`INT_MAX + 1`, `INT_MIN - 1`, `INT_MIN / -1`,
  `INT_MIN * -1`, ...) is **undefined behavior** — not a wraparound, not a
  trap, *anything* (including the compiler optimizing away code that "can't"
  overflow). Never write code whose correctness depends on signed overflow.
- **Unsigned** integer arithmetic is **always well-defined**: it wraps modulo
  2^N, where N is the type's bit width. `UINT_MAX + 1 == 0`,
  `0u - 1 == UINT_MAX`.
- This asymmetry is *useful*: unsigned wraparound lets you **detect**
  overflow after the fact (check whether the result is smaller than an
  operand), while signed overflow must be **prevented** beforehand (check
  bounds *before* the operation, using `<limits>`).

### `<limits>`

`std::numeric_limits<T>` gives portable access to a type's range:

```cpp
#include <limits>

std::numeric_limits<int>::max();    // largest int (e.g. 2147483647)
std::numeric_limits<int>::min();    // smallest int (e.g. -2147483648)
std::numeric_limits<unsigned>::max(); // e.g. 4294967295
std::numeric_limits<double>::epsilon(); // smallest x where 1.0 + x != 1.0
std::numeric_limits<double>::infinity();
std::numeric_limits<double>::quiet_NaN();
```

Prefer this over hardcoded `INT_MAX`/`INT_MIN` (from `<climits>`) — it works
for any type, including ones you template later.

### Preventing signed overflow before it happens

To check whether `a + b` would overflow `int`, you cannot compute `a + b`
first (that's the UB you're trying to avoid). Instead, rearrange the
inequality so every intermediate value stays in range:

```cpp
bool willAddOverflow(int a, int b) {
    if (b > 0 && a > std::numeric_limits<int>::max() - b) return true; // a+b > INT_MAX
    if (b < 0 && a < std::numeric_limits<int>::min() - b) return true; // a+b < INT_MIN
    return false;
}
```

`INT_MAX - b` and `INT_MIN - b` are always in range here because `b`'s sign
is already known, so the subtraction can't itself overflow.

### Detecting unsigned overflow after the fact

Because unsigned wraparound is well-defined, you can compute first and check
after:

```cpp
unsigned int sum = a + b;       // wraps if it overflows — well-defined
bool overflowed = sum < a;      // if it wrapped, the result is smaller
                                 // than either original operand
```

## Floating-point representation (IEEE 754 `double`)

A `double` is 64 bits: 1 sign bit, 11 exponent bits, 52 mantissa
(significand) bits — giving roughly **15–17 significant decimal digits** of
precision. Most decimal fractions (`0.1`, `0.2`, `0.3`, ...) have **no exact
binary representation**, so `0.1 + 0.2 == 0.3` is `false` — both sides are
*approximations* that don't happen to match bit-for-bit
(`0.1 + 0.2` rounds to `0.30000000000000004`).

### Special values

- **Signed zero**: `+0.0` and `-0.0` are distinct bit patterns, but
  `+0.0 == -0.0` is `true`. Use `std::signbit(x)` to tell them apart (e.g.
  the sign of a result that underflowed to zero).
- **Infinity**: `1.0 / 0.0 == +inf`, `-1.0 / 0.0 == -inf` — floating-point
  division by zero is **well-defined** (unlike integer division by zero,
  which is UB). Test with `std::isinf(x)`.
- **NaN** ("not a number"): `0.0 / 0.0` and other invalid operations produce
  NaN. **`NaN != NaN` is `true`** — NaN compares unequal to *everything*,
  including itself. This means `x == x` is a (rarely-used) way to test "is
  `x` NaN?", but `std::isnan(x)` is clearer. A function that returns NaN on
  invalid input "poisons" any later computation that uses the result, since
  every arithmetic op on NaN produces NaN.

```cpp
#include <cmath>

std::isnan(x);    // true if x is NaN
std::isinf(x);    // true if x is +inf or -inf
std::signbit(x);  // true if the sign bit is set (negative, or -0.0, or -NaN)
```

## Comparing floating-point values

Never compare computed floating-point results with `==` — accumulated
rounding error makes "the same" mathematical value land on slightly
different bit patterns depending on the order of operations. Compare against
a tolerance ("epsilon") instead:

```cpp
bool approxEqual(double a, double b, double epsilon) {
    return std::abs(a - b) <= epsilon;
}
```

- An **absolute** epsilon (as above) works when the values are roughly known
  magnitudes. For values that range over many orders of magnitude, a
  **relative** epsilon (`std::abs(a - b) <= epsilon * std::max(std::abs(a),
  std::abs(b))`) is more appropriate — but watch out near zero, where
  relative epsilon breaks down (division by ~0).
- NaN must be special-cased: `std::abs(NaN - x) <= epsilon` is always
  `false` (any comparison with NaN is `false`), so `approxEqual(NaN, NaN,
  eps)` correctly returns `false` even with this formula — which matches
  IEEE 754 semantics (NaN equals nothing), but is worth calling out
  explicitly since it's easy to assume `false` is a "bug."

## Floating-point arithmetic is not associative

`(a + b) + c` can differ from `a + (b + c)` — each `+` rounds to the nearest
representable `double`, and rounding twice in a different order can produce
different results. This means:

- The order you sum a list of numbers in **affects the result's accuracy**.
- Summing a huge value and a tiny value can make the tiny value
  **disappear entirely**: if `a` is `1e16` and `b` is `1.0`, `a + b == a` in
  `double`, because the gap between adjacent representable doubles near
  `1e16` is larger than `1`.

### Kahan summation

A compensated-summation algorithm that tracks the rounding error from each
addition and feeds it back into the next one, dramatically reducing
accumulated error for long sums of similar-magnitude values:

```cpp
double kahanSum(const std::vector<double>& values) {
    double sum = 0.0;
    double compensation = 0.0;       // running correction
    for (double value : values) {
        double y = value - compensation;
        double t = sum + y;
        compensation = (t - sum) - y; // recover the part of y that got
                                       // dropped when computing t
        sum = t;
    }
    return sum;
}
```

It doesn't help every case (e.g. one huge value swamping everything else —
that needs sorting or pairwise summation), but for sums of many
similar-magnitude terms it's a large, cheap accuracy win.

## Converting between integers and floating-point

- `int` -> `double`: exact for any value up to 2^53 (a `double`'s mantissa
  has 52 bits + 1 implicit bit). All 32-bit `int` values convert exactly.
- `double` -> integer (`static_cast<int>(x)`, etc.): **truncates toward
  zero** (`3.9 -> 3`, `-3.9 -> -3`). If `x` is outside the destination type's
  range (including `NaN`/`inf`), the conversion is **undefined behavior** —
  unlike integer-to-unsigned narrowing (topic 2), which is always
  well-defined modulo 2^N. Always range-check before converting an unknown
  `double` to an integer type.

## Further Reading (Modern C++ Programming)

- [Chapter 4 — Numbers](https://federico-busato.github.io/Modern-CPP-Programming/htmls/04.Numbers.html)
- [Chapter 5 — Floating-Point Numbers](https://federico-busato.github.io/Modern-CPP-Programming/htmls/05.FloatingPoint.html)
- [`std::numeric_limits`](https://en.cppreference.com/w/cpp/types/numeric_limits)
- [`<cmath>` classification functions](https://en.cppreference.com/w/cpp/numeric/math#Classification)
- [Floating-point arithmetic on cppreference](https://en.cppreference.com/w/cpp/language/floating_literal)
- [What Every Computer Scientist Should Know About Floating-Point Arithmetic](https://docs.oracle.com/cd/E19957-01/806-3568/ncg_goldberg.html)
