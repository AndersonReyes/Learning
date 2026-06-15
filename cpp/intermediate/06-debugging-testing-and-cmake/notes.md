# Intermediate 06: Debugging, Sanitizers, Testing & CMake

## CMake: this topic's `CMakeLists.txt`

Every topic so far built with a single `g++` invocation. **CMake** generates
the actual build files (Makefiles, Ninja, etc.) from a portable
`CMakeLists.txt`, so the same project builds with whatever toolchain is
installed:

```cmake
cmake_minimum_required(VERSION 3.20)
project(intermediate_06_debugging_testing_and_cmake CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_compile_options(-Wall -Wextra)

add_executable(exercise_test exercise_test.cpp exercise.cpp)
add_executable(examples examples.cpp)

enable_testing()
add_test(NAME exercise_test COMMAND exercise_test)
```

- **`cmake -S . -B build`**: *configure* -- reads `CMakeLists.txt` (source
  dir `.`), writes generated build files into `build/` (an out-of-source
  build -- `build/` can be `rm -rf`'d without touching source).
- **`cmake --build build`**: *build* -- invokes the underlying build tool
  (make/ninja) to compile `add_executable` targets.
- **`add_executable(name srcs...)`**: one target (here, `exercise_test` and
  `examples`, mirroring the two binaries every topic's plain `g++` commands
  already produced).
- **`enable_testing()` + `add_test(...)`**: registers a CTest test that runs
  the `exercise_test` binary; `ctest --test-dir build` runs it and reports
  pass/fail (CTest considers a test "passed" iff the program exits 0 --
  exactly `TEST_MAIN()`'s convention).
- The target name **`test`** is reserved by CTest once `enable_testing()` is
  called -- hence `exercise_test`, not `test`, as the binary name here.

### Build types & options

```sh
cmake -S . -B build && cmake --build build
./build/exercise_test
ctest --test-dir build
```

This topic also defines an `ENABLE_SANITIZERS` option (default `OFF`):

```cmake
option(ENABLE_SANITIZERS "Build with AddressSanitizer + UndefinedBehaviorSanitizer" OFF)
if(ENABLE_SANITIZERS)
    add_compile_options(-fsanitize=address,undefined -g)
    add_link_options(-fsanitize=address,undefined)
endif()
```

Sanitizer flags must be baked into EVERY compiled object AND the final link
step, so they need their own build directory:

```sh
cmake -S . -B build-asan -DENABLE_SANITIZERS=ON && cmake --build build-asan
./build-asan/exercise_test
```

`-DCMAKE_BUILD_TYPE=Release` (a separate, CMake-builtin concept) adds
`-O2 -DNDEBUG` -- optimized, with `assert()` compiled out (see
`examples.cpp`). `Debug` (this topic's default) adds `-g` (debug symbols, for
gdb) with no optimization.

## gdb: stepping through a running program

Compile with `-g` (debug symbols -- CMake's `Debug` build type does this
automatically) so gdb can map machine code back to source lines/variable
names. Core commands, in a typical session:

```
$ gdb ./build/exercise_test
(gdb) break rotateLeft        # set a breakpoint at the start of rotateLeft
(gdb) run                     # start the program; stops at the breakpoint
(gdb) print v                 # inspect a variable (prints the vector's contents)
(gdb) print k
(gdb) next                    # execute the next line (step OVER function calls)
(gdb) step                    # execute the next line (step INTO function calls)
(gdb) watch k                 # stop whenever k's value changes
(gdb) backtrace               # print the call stack (which function called which)
(gdb) continue                # resume until the next breakpoint/watchpoint/exit
```

- **`break FILE:LINE`** sets a breakpoint at a specific line, not just a
  function name -- useful for breaking inside a loop body.
- **`print EXPR`** evaluates any C++ expression in the current scope --
  `print v[2]`, `print v.size()`, even `print v[lo] == v[hi]`.
- A **watchpoint** (`watch`) is invaluable for "this variable changes to a
  wrong value somewhere, but I don't know where" -- gdb stops at the exact
  line that changes it.
- `backtrace` after a crash (e.g. a `SIGSEGV` from dereferencing a bad
  pointer) shows exactly which call chain led there.

## Sanitizers: compiler-inserted runtime checks

Sanitizers instrument the binary at compile time to detect specific bug
classes at the moment they happen (not just "sometime later, maybe, if you're
unlucky" -- which is what raw undefined behavior usually does).

### AddressSanitizer (`-fsanitize=address`, "ASan")

Catches out-of-bounds memory access and memory-lifetime bugs:

- Heap buffer overflow (reading/writing past an allocation)
- Stack buffer overflow (e.g. an array index past the end of a local array)
- Use-after-free / use-after-scope
- Double-free

Example (NOT in this topic's compiled code -- `exercise.cpp`'s reference
implementation has none of these bugs):

```cpp
int* arr = new int[5];
arr[5] = 42;  // one past the end -- undefined behavior
```

```
==12345==ERROR: AddressSanitizer: heap-buffer-overflow on address 0x...
WRITE of size 4 at 0x... thread T0
    #0 0x... in main examples.cpp:12
```

### UndefinedBehaviorSanitizer (`-fsanitize=undefined`, "UBSan")

Catches a wide range of other undefined behavior:

- Signed integer overflow (`INT_MAX + 1`)
- Division/modulo by zero
- Null-pointer dereference, misaligned access
- Invalid enum values, out-of-range `static_cast`

Example:

```cpp
int x = INT_MAX;
x = x + 1;  // signed overflow -- UB, not "wraps to INT_MIN"
```

```
examples.cpp:20:7: runtime error: signed integer overflow:
2147483647 + 1 cannot be represented in type 'int'
```

### Why this topic's exercises are sanitizer-relevant

Every exercise indexes a `std::vector<int>` with `operator[]`, which does
**not** bounds-check. The documented edge cases -- empty-vector `k %
v.size()` in `rotateLeft`, empty-vector `v.size() - 1` in `isPalindrome` --
are exactly the kind of off-by-one that's SILENT without tooling (might read
garbage, might "happen to work", might corrupt memory) but becomes an
immediate, precise error report under ASan/UBSan. Building this topic's
`exercise_test` with `-DENABLE_SANITIZERS=ON` and seeing it run clean is part
of this topic's verification.

### Other sanitizers (not used in this build)

- **ThreadSanitizer** (`-fsanitize=thread`, "TSan"): data races between
  threads -- relevant once `advanced/02` introduces concurrency.
- **MemorySanitizer** (`-fsanitize=memory`, "MSan"): use of uninitialized
  memory (Clang only).
- **LeakSanitizer**: detects memory leaks; bundled with ASan by default on
  Linux.

ASan and UBSan can be combined (`-fsanitize=address,undefined`, as this
topic's CMake option does); TSan cannot be combined with ASan (different,
incompatible instrumentation).

## Testing frameworks: the landscape vs. `cpp/testing.h`

This track's `cpp/testing.h` (built in `fundamentals/06`) is a minimal
`TEST`/`CHECK`/`TEST_MAIN` framework -- enough to express "named test, several
assertions, pass/fail summary". Production C++ projects typically use a
fuller framework:

- **GoogleTest**: `TEST(Suite, Name) { EXPECT_EQ(a, b); }`, rich assertion
  macros (`EXPECT_*` continues on failure, `ASSERT_*` aborts the test),
  fixtures (`SetUp`/`TearDown`), parameterized/typed tests, death tests
  (`EXPECT_DEATH` -- assert a statement crashes/aborts).
- **Catch2**: header-only (like `cpp/testing.h`), `TEST_CASE("name") {
  REQUIRE(a == b); }`, BDD-style `SECTION`s, property-based testing
  extensions.
- **CTest**: not an assertion library -- a test *runner/aggregator* (used
  above), typically running binaries built with one of the above (or
  anything that exits 0/nonzero).

What `cpp/testing.h` is missing relative to these: `EXPECT_*` (continue after
a failed check within one test -- ours throws on the first failed `CHECK`,
stopping that test), fixtures/setup-teardown, and test filtering by
name/pattern. The underlying ideas -- registration via static initialization,
macros capturing `__FILE__`/`__LINE__`, exceptions (or `longjmp`) to abort a
single test without aborting the run -- are the same.

## Further Reading

- [MCPP ch. 17 -- Debugging](https://federico-busato.github.io/Modern-CPP-Programming/htmls/17.Debugging.html)
- [MCPP ch. 18 -- Ecosystem (Build Systems, Testing, Sanitizers)](https://federico-busato.github.io/Modern-CPP-Programming/htmls/18.Ecosystem.html)
- [CMake documentation](https://cmake.org/cmake/help/latest/)
- [Clang/GCC AddressSanitizer documentation](https://clang.llvm.org/docs/AddressSanitizer.html)
- [Clang/GCC UndefinedBehaviorSanitizer documentation](https://clang.llvm.org/docs/UndefinedBehaviorSanitizer.html)
- [cppreference: `assert`](https://en.cppreference.com/w/cpp/error/assert)
- [cppreference: `static_assert`](https://en.cppreference.com/w/cpp/language/static_assert)
- [GDB documentation](https://sourceware.org/gdb/documentation/)
