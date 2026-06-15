# Topic 17-18 (Intermediate 06): Debugging, Sanitizers, Testing & CMake
#
# The first CMakeLists.txt in this track. Usage:
#
#   cmake -S . -B build && cmake --build build
#   ./build/exercise_test
#   ./build/examples
#
# Run the test suite under AddressSanitizer + UndefinedBehaviorSanitizer with
# a separate build directory (sanitizer flags must be baked in at compile
# time, so they need their own build):
#
#   cmake -S . -B build-asan -DENABLE_SANITIZERS=ON && cmake --build build-asan
#   ./build-asan/exercise_test
#
# CTest integration (registers exercise_test as a test CMake can run/report
# on -- note the target can't be named "test", that name is reserved once
# enable_testing() is on):
#
#   ctest --test-dir build

cmake_minimum_required(VERSION 3.20)
project(intermediate_06_debugging_testing_and_cmake CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_compile_options(-Wall -Wextra)

# Off by default: a normal build stays fast and matches the plain g++
# invocation used by every other topic. Turn this on (in its own build
# directory) to compile with AddressSanitizer + UndefinedBehaviorSanitizer,
# which instrument the binary to abort with a diagnostic the moment they
# detect out-of-bounds access, use-after-free, signed overflow, etc. -- see
# notes.md for what each sanitizer catches.
option(ENABLE_SANITIZERS "Build with AddressSanitizer + UndefinedBehaviorSanitizer" OFF)
if(ENABLE_SANITIZERS)
    add_compile_options(-fsanitize=address,undefined -g)
    add_link_options(-fsanitize=address,undefined)
endif()

add_executable(exercise_test exercise_test.cpp exercise.cpp)
add_executable(examples examples.cpp)

enable_testing()
add_test(NAME exercise_test COMMAND exercise_test)
