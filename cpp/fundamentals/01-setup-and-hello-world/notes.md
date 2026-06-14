# 1. Setup, Compilation & Hello World

## From source to executable

A C++ source file goes through 4 stages before it runs:

1. **Preprocessing** — textual substitution: `#include`, `#define`,
   conditional `#if`/`#ifdef`. Output is pure C++ (no directives left).
2. **Compilation** — preprocessed source → assembly for the target CPU.
   This is where syntax/type errors are caught.
3. **Assembly** — assembly → machine code in an **object file** (`.o`),
   with unresolved references to symbols defined elsewhere (other files,
   libraries).
4. **Linking** — combines object files + libraries into an executable,
   resolving cross-file symbol references. "Undefined reference" errors come
   from here, not compilation.

```sh
g++ -std=c++20 -Wall -Wextra -o prog main.cpp other.cpp
```

- `-std=c++20` — language standard. Without it you get the compiler's
  default, which varies by version — always pin it.
- `-Wall -Wextra` — enable extra warnings. Treat warnings as bugs; they
  catch real mistakes (uninitialized variables, signed/unsigned comparisons,
  unused values).
- `-g` — include debug info (for `gdb`/`lldb`).
- `-O0`/`-O2`/`-O3` — optimization level. `-O0` (default) compiles fastest
  and matches source most closely for debugging.
- `-o prog` — output executable name. Without it, the default is `a.out`.

You can also compile and link separately:

```sh
g++ -std=c++20 -Wall -Wextra -c main.cpp   # -> main.o (compile only)
g++ -std=c++20 -Wall -Wextra -c other.cpp  # -> other.o
g++ main.o other.o -o prog                 # link
```

This is why a topic's `exercise.cpp` and `exercise_test.cpp` are **both**
passed to `g++` on one command line — each becomes an object file and they're
linked together into one test binary.

## Anatomy of a minimal program

```cpp
#include <iostream>

int main() {
    std::cout << "Hello, world!\n";
    return 0;
}
```

- `#include <iostream>` pulls in declarations for `std::cout`/`std::cin`/etc.
  (a *header*, not a library — the actual implementation is linked in
  automatically for standard library facilities).
- `main` is the entry point. Its return value becomes the process's **exit
  code**: `0` means success, nonzero means failure (checked by shells,
  `assert`-based test runners, CI, etc.). `return 0;` at the end of `main` is
  optional — falling off the end of `main` implicitly returns `0` — but
  writing it is good practice for clarity, and required for any other exit
  code (`return 1;`).
- `std::` is the standard library namespace. Avoid `using namespace std;` in
  real code — it pulls every standard name into scope and can silently
  collide with your own identifiers. Fine to write once in a tiny throwaway
  script; avoid it in headers and larger files.

## `<<` and `>>`, `std::cout`/`std::cin`/`std::cerr`

- `std::cout << x` — stream `x` to standard output. Chainable:
  `std::cout << "a=" << a << ", b=" << b << "\n";`
- `std::cin >> x` — read into `x` from standard input. `>>` **skips leading
  whitespace** by default (`std::skipws`), then reads until the next
  whitespace or until `x`'s type stops matching (e.g. an `int` read stops at
  the first non-digit).
- `std::cerr` — standard error, for diagnostics. Unlike `std::cout`, it's
  unbuffered (writes appear immediately), which matters when a program
  crashes — buffered `std::cout` output can be lost, `std::cerr` output
  isn't.
- `'\n'` vs `std::endl` — both write a newline, but `std::endl` *also flushes
  the output buffer*. Flushing is relatively slow; in a tight loop, `'\n'` is
  noticeably faster. Use `'\n'` unless you specifically need to flush (e.g.
  right before a program might crash, or before reading input that depends on
  a prompt being visible).

## Stream manipulators (`<iomanip>`)

Manipulators change how subsequent `<<`/`>>` operations format data. Most are
**sticky** — they apply to every following operation until changed — except
`std::setw`, which applies to **only the next field**.

```cpp
#include <iomanip>

std::cout << std::hex << 255 << "\n";              // ff
std::cout << std::uppercase << std::hex << 255;    // FF (uppercase is sticky)
std::cout << std::setw(6) << std::setfill('0') << 42 << "\n"; // 000042
std::cout << std::dec << 42 << "\n";               // back to decimal: 42

std::cout << std::fixed << std::setprecision(2) << 3.14159 << "\n"; // 3.14
```

- `std::hex`/`std::oct`/`std::dec` — integer base (sticky, default `dec`).
- `std::uppercase` — use uppercase digits for hex (`A-F` not `a-f`) (sticky).
- `std::setw(n)` — pad the **next** field to width `n` (default pads with
  spaces, right-justified for numbers). Not sticky — must be set before
  *every* field you want padded.
- `std::setfill(c)` — change the padding character used by `setw` (sticky).
- `std::left`/`std::right` — justify padded fields (sticky).
- `std::fixed` + `std::setprecision(n)` — fixed-point notation with exactly
  `n` digits after the decimal point (both sticky). Without `std::fixed`,
  `setprecision` controls *significant digits*, not decimal places — a
  common source of surprising output.

## String streams (`<sstream>`)

`std::ostringstream`/`std::istringstream` let you use `<<`/`>>` to build or
parse strings in memory — useful for formatting (build piece by piece, then
`.str()`) or parsing (split a line into typed fields).

```cpp
#include <sstream>

std::ostringstream out;
out << "x=" << 42 << ", y=" << 3.5;
std::string s = out.str(); // "x=42, y=3.5"

std::istringstream in("10 20 hello");
int a, b;
std::string word;
in >> a >> b >> word; // a=10, b=20, word="hello"
```

## Stream state and the extraction-failure gotcha

Every stream tracks state flags: `goodbit` (no error), `eofbit` (hit end of
input), `failbit` (an operation failed, e.g. `>>` couldn't parse the
requested type), `badbit` (serious I/O error). `if (stream)` / `while
(stream)` check `!fail()` — true unless `failbit`/`badbit` is set (`eofbit`
alone doesn't make a stream "false").

**Gotcha**: if `>>` into an `int` fails because the next character isn't a
digit/sign, **that character is *not* consumed** — `failbit` is set, but the
stream's read position doesn't move past it. To recover and continue reading,
you must:

```cpp
if (!(stream >> value)) {
    stream.clear();   // clear failbit/eofbit so the stream is usable again
    stream.ignore(1); // manually skip the character that caused the failure
}
```

Without `clear()`, every subsequent operation on the stream is a no-op (it
short-circuits because `failbit` is set). Without `ignore()`, you'll loop
forever retrying the same character. This pattern — "try to extract a token,
and if it fails, clear + skip one character" — is exactly how you scan a
string for embedded numbers while skipping non-numeric text.

## Further Reading (Modern C++ Programming)

- [Chapter 2 — Preparation](https://federico-busato.github.io/Modern-CPP-Programming/htmls/02.Preparation.html)
  (compiling, hello world / iostream)
- [`std::basic_ostream`](https://en.cppreference.com/w/cpp/io/basic_ostream),
  [`std::basic_istream`](https://en.cppreference.com/w/cpp/io/basic_istream)
- [`<iomanip>` manipulators](https://en.cppreference.com/w/cpp/io/manip)
- [`std::basic_istringstream`](https://en.cppreference.com/w/cpp/io/basic_istringstream),
  [`std::basic_ostringstream`](https://en.cppreference.com/w/cpp/io/basic_ostringstream)
- [`std::basic_ios::rdstate` / stream state](https://en.cppreference.com/w/cpp/io/basic_ios/rdstate)
