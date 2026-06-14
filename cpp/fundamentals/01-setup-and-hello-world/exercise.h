#pragma once

#include <cstddef>
#include <string>
#include <utility>
#include <vector>

// Formats amount as US-style currency: a leading '$' (or "-$" for negative
// values), the integer part with comma thousands separators every 3 digits,
// and exactly two digits after the decimal point. Examples:
//   formatCurrency(1234567.89) -> "$1,234,567.89"
//   formatCurrency(-1234.5)    -> "-$1,234.50"
//   formatCurrency(0)          -> "$0.00"
std::string formatCurrency(double amount);

// Formats value as an uppercase hexadecimal string prefixed with "0x",
// zero-padded to at least `width` hex digits. If the natural representation
// is wider than `width`, it is shown in full (no truncation). Examples:
//   toHexString(255, 4)  -> "0x00FF"
//   toHexString(4096, 2) -> "0x1000"
//   toHexString(0, 4)    -> "0x0000"
std::string toHexString(unsigned int value, int width);

// Extracts every base-10 integer (optionally signed) found in `input`,
// treating any run of non-numeric characters as a separator, in the order
// they appear. Examples:
//   parseInts("12, -7 foo34;56") -> {12, -7, 34, 56}
//   parseInts("no numbers here") -> {}
//   parseInts("")                -> {}
std::vector<int> parseInts(const std::string& input);

// Renders `rows` as a two-column table: column 1 (names) left-justified,
// column 2 (values) right-justified, separated by " | ". Each column is
// sized to its widest entry across all rows. Rows are joined by '\n' with no
// trailing newline. Returns "" if `rows` is empty. Example, for
// {{"Alice", 90}, {"Bob", 85}, {"Charlie", 100}}:
//   "Alice   |  90\nBob     |  85\nCharlie | 100"
std::string formatTable(const std::vector<std::pair<std::string, int>>& rows);

// Wraps `text` to lines of at most `width` characters, breaking only between
// words (any run of whitespace is treated as a single separator and is not
// preserved). A single word longer than `width` is placed alone on its own
// (overflowing) line. Lines are joined by '\n'. Returns "" if `text` contains
// no words. Example, wordWrap("The quick brown fox", 10):
//   "The quick\nbrown fox"
std::string wordWrap(const std::string& text, std::size_t width);
