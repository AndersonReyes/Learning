// Spec for exercise.h/exercise.cpp. No test framework yet (that's built in
// fundamentals/06) -- just assert(). Compile and run:
//
//   g++ -std=c++20 -Wall -Wextra -o /tmp/test exercise_test.cpp exercise.cpp && /tmp/test
//
// Every assert() must pass. If one fails, the program aborts and prints the
// failing file:line -- fix exercise.cpp and re-run.

#include "exercise.h"

#include <cassert>
#include <iostream>
#include <string>
#include <vector>

static void testFormatCurrency() {
    assert(formatCurrency(1234567.89) == "$1,234,567.89");
    assert(formatCurrency(0) == "$0.00");
    assert(formatCurrency(100) == "$100.00");
    assert(formatCurrency(999) == "$999.00");
    assert(formatCurrency(1000) == "$1,000.00");
    assert(formatCurrency(-1234.5) == "-$1,234.50");
    assert(formatCurrency(-1) == "-$1.00");
    assert(formatCurrency(1000000000) == "$1,000,000,000.00");
}

static void testToHexString() {
    assert(toHexString(255, 4) == "0x00FF");
    assert(toHexString(0, 4) == "0x0000");
    assert(toHexString(10, 1) == "0xA");
    assert(toHexString(4096, 2) == "0x1000");
    assert(toHexString(0xABCDEF, 6) == "0xABCDEF");
}

static void testParseInts() {
    assert(parseInts("12, -7 foo34;56") == (std::vector<int>{12, -7, 34, 56}));
    assert(parseInts("") == (std::vector<int>{}));
    assert(parseInts("no numbers here") == (std::vector<int>{}));
    assert(parseInts("   ") == (std::vector<int>{}));
    assert(parseInts("-1,-2,-3") == (std::vector<int>{-1, -2, -3}));
}

static void testFormatTable() {
    assert(formatTable({{"Alice", 90}, {"Bob", 85}, {"Charlie", 100}}) ==
           "Alice   |  90\nBob     |  85\nCharlie | 100");
    assert(formatTable({}) == "");
    assert(formatTable({{"x", -5}, {"yy", 10}}) == "x  | -5\nyy | 10");
}

static void testWordWrap() {
    assert(wordWrap("The quick brown fox jumps over the lazy dog", 10) ==
           "The quick\nbrown fox\njumps over\nthe lazy\ndog");
    assert(wordWrap("", 10) == "");
    assert(wordWrap("supercalifragilistic word", 5) == "supercalifragilistic\nword");
    assert(wordWrap("a b c", 1) == "a\nb\nc");
}

int main() {
    testFormatCurrency();
    testToHexString();
    testParseInts();
    testFormatTable();
    testWordWrap();
    std::cout << "All tests passed!\n";
    return 0;
}
