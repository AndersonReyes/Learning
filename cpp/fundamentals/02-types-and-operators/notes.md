# 2. Type System, Fundamental Types & Operators

## Fundamental types

| Category  | Types                                            |
|-----------|--------------------------------------------------|
| Boolean   | `bool`                                            |
| Character | `char`, `signed char`, `unsigned char`, `wchar_t` |
| Integer   | `short`, `int`, `long`, `long long` (+ `unsigned` variants) |
| Floating  | `float`, `double`, `long double`                  |
| Other     | `void`                                             |

- `char` has *unspecified* signedness — it may behave like `signed char` or
  `unsigned char` depending on the platform. `signed char`/`unsigned char` are
  distinct types from `char`, even when they have the same representation.
- Sizes are implementation-defined; the standard only guarantees minimums
  (`int` >= 16 bits, `long` >= 32 bits, `long long` >= 64 bits, and
  `sizeof(char) == 1`). On x86-64 Linux/g++: `char`=1, `short`=2, `int`=4,
  `long`=8, `long long`=8, `float`=4, `double`=8 bytes.
- `sizeof(T)` / `sizeof expr` gives the size in bytes as a `std::size_t`
  (unsigned). Use it instead of hardcoding sizes.

### Fixed-width types (`<cstdint>`)

When exact width matters (bit manipulation, binary formats, protocol
fields), use `<cstdint>`: `int8_t`/`uint8_t`, `int16_t`/`uint16_t`,
`int32_t`/`uint32_t`, `int64_t`/`uint64_t`. These are guaranteed-width
aliases for one of the fundamental types above.

## Integer promotion & usual arithmetic conversions

- **Integer promotion**: operands smaller than `int` (`bool`, `char`,
  `short`, and unsigned versions of these) are converted to `int` (or
  `unsigned int` if `int` can't represent all their values) before being used
  in arithmetic, bitwise, or shift expressions.
  - Gotcha: `unsigned char a = 0x80, b = 1; auto c = a << b;` — `a` and `b`
    are promoted to `int` first, so `c` is `int` (value `0x100`), **not**
    `unsigned char` (which would wrap to `0`). If you need an 8-bit result,
    mask and cast back explicitly: `static_cast<unsigned char>(c & 0xFF)`.
- **Usual arithmetic conversions**: when a binary operator's operands have
  different types, both convert to a common type before the operation.
  - Mixing a **signed** and **unsigned** integer of the same rank converts
    the signed operand to unsigned. `-1 < 5u` is **false**, because `-1`
    becomes `UINT_MAX`. This is exactly what `-Wsign-compare` warns about —
    take the warning seriously, or cast explicitly to make the comparison's
    intent unambiguous.

## Conversions

- **Implicit conversions** happen automatically: widening (`int` ->
  `double`) is lossless; narrowing (`double` -> `int`, `int` -> `char`) can
  lose information and may warn (especially inside `{}` brace-initialization,
  where narrowing is an error).
- **`static_cast<T>(expr)`** — the general-purpose explicit conversion for
  related types (numeric conversions, base/derived pointers, `void*`, enum
  <-> integer). Prefer it over C-style casts `(T)expr`, which silently allow
  far more (including dangerous reinterpretation).
- **Conversion to an unsigned type** is always well-defined: the result is
  the source value reduced modulo 2^N, where N is the destination's bit
  width. `static_cast<unsigned char>(-1) == 255`,
  `static_cast<unsigned char>(300) == 44` (300 - 256).
- **Conversion to a signed type** from a value outside its range is
  well-defined as of C++20 (it wraps the same way, via 2's complement) —
  before C++20 it was implementation-defined (but every real compiler did
  this anyway).
- **Signed integer *arithmetic* overflow** (e.g. `INT_MAX + 1`) is undefined
  behavior — this is different from a narrowing *conversion*, which is
  always well-defined. Never rely on signed overflow wrapping.

## Operators

### Arithmetic: `+ - * / % `

- `/` on integers truncates toward zero (guaranteed since C++11): `7 / 2 ==
  3`, `-7 / 2 == -3`.
- `%` (remainder) satisfies `(a / b) * b + a % b == a`, so its result has the
  same sign as the dividend: `-7 % 2 == -1`, `7 % -2 == 1`.

### Relational & equality: `< <= > >= == !=`

Beware signed/unsigned mixing (see above) — both operands convert to a
common type first, which can flip the "obvious" answer.

### Logical: `&& || !`

- Short-circuit evaluation: in `a && b`, `b` is only evaluated if `a` is
  true; in `a || b`, `b` is only evaluated if `a` is false. The unevaluated
  side's side effects (function calls, increments) do **not** happen.
- This enables guard patterns: `ptr != nullptr && ptr->value > 0` is safe —
  `ptr->value` is never read through a null pointer.

### Bitwise: `& | ^ ~ << >>`

- `&` AND, `|` OR, `^` XOR, `~` NOT (one's complement — flips every bit).
- `<<` shifts bits left, filling with `0` from the right. For non-negative
  values that don't overflow the type, `x << n == x * 2^n`.
- `>>` on an **unsigned** type is a *logical* shift: fills with `0` from the
  left.
- `>>` on a **signed** type is, as of C++20, guaranteed to be an *arithmetic*
  shift: the sign bit is replicated into the vacated high bits. So `-1 >> 1
  == -1` (all bits stay set). This guarantee is what makes bit-trick
  identities like `(a & b) + ((a ^ b) >> 1)` (average without overflow) work
  correctly for negative operands.
- Shifting by a negative amount, or by >= the operand type's bit width, is
  undefined behavior — always normalize the shift amount (e.g. `((n % width)
  + width) % width`) before shifting.

### Assignment & compound assignment

`= += -= *= /= %= &= |= ^= <<= >>=`. `a op= b` is shorthand for `a = a op
b`, evaluating `a` only once — matters when `a` is an expression with side
effects (e.g. `arr[i++] += 1`).

### Increment/decrement: `++x` vs `x++`

- Prefix `++x` increments in place and evaluates to the **new** value.
- Postfix `x++` evaluates to the **old** value, requiring a temporary copy —
  prefer prefix for non-fundamental types (iterators, etc.) where the copy
  isn't free; for `int` it makes no practical difference.
- Never write an expression that both reads and modifies the same scalar
  more than once without an intervening sequence point (e.g. `x++ + x++`) —
  the order of side effects is unsequenced and the result is undefined.

### Ternary: `cond ? a : b`

Right-associative, so ternaries chain naturally into a classification
ladder:

```cpp
std::string grade = score >= 90 ? "A"
                  : score >= 80 ? "B"
                  : score >= 70 ? "C"
                  :               "F";
```

### Comma: `,`

Evaluates the left operand, discards its value, then evaluates and returns
the right operand. Rarely needed outside a `for` loop's increment clause
(`for (int i = 0, j = n; i < j; ++i, --j)`).

## Operator precedence gotchas

- **Bitwise `& | ^` bind *looser* than relational/equality operators.**
  `if (flags & MASK == 0)` parses as `if (flags & (MASK == 0))` — almost
  never what you want. Always parenthesize: `if ((flags & MASK) == 0)`.
- **`<<`/`>>` bind tighter than `< > <= >=` but looser than `+ -`.**
  `std::cout << a + b` is fine (prints `a + b`), but `std::cout << a < b`
  parses as `(std::cout << a) < b` — a comparison of a stream against `b`,
  not what it looks like.
- When in doubt, parenthesize. cppreference's [operator precedence
  table](https://en.cppreference.com/w/cpp/language/operator_precedence) is
  the authoritative reference.

## Further Reading (Modern C++ Programming)

- [Chapter 3 — Basic Concepts I](https://federico-busato.github.io/Modern-CPP-Programming/htmls/03.Basic_Concepts_I.html)
  (fundamental types, conversions, operators)
- [Fundamental types](https://en.cppreference.com/w/cpp/language/types)
- [Implicit conversions](https://en.cppreference.com/w/cpp/language/implicit_conversion)
- [`static_cast`](https://en.cppreference.com/w/cpp/language/static_cast)
- [Arithmetic operators](https://en.cppreference.com/w/cpp/language/operator_arithmetic)
- [Bitwise operators](https://en.cppreference.com/w/cpp/language/operator_arithmetic#Bitwise_logic_operators)
- [Operator precedence](https://en.cppreference.com/w/cpp/language/operator_precedence)
- [`<cstdint>`](https://en.cppreference.com/w/cpp/header/cstdint)
