#include "exercise.h"

#include <utility>
#include <vector>

// --- nextId -----------------------------------------------------------------------------------

int nextId() { throw std::logic_error("not implemented"); }

// --- normalizeWhitespace -------------------------------------------------------------------------

namespace {
// Internal linkage: visible only within this translation unit. [[maybe_unused]]
// avoids a -Wunused-function warning while normalizeWhitespace's stub body
// doesn't call it yet.
[[maybe_unused]] bool isSpaceChar(char c) {
    return c == ' ' || c == '\t' || c == '\n' || c == '\r' || c == '\f' || c == '\v';
}
}  // namespace

std::string normalizeWhitespace(const std::string& text) {
    (void)text;
    throw std::logic_error("not implemented");
}

// --- extern global + accessors ---------------------------------------------------------------------

int globalRequestCount = 0;  // the ONE definition required by the ODR

void recordRequest() { throw std::logic_error("not implemented"); }

int getRequestCount() { throw std::logic_error("not implemented"); }

void resetRequestCount() { throw std::logic_error("not implemented"); }

// --- toRoman --------------------------------------------------------------------------------------

namespace {
// Internal linkage: visible only within this translation unit.
const std::vector<std::pair<int, std::string>> kRomanNumerals = {
    {1000, "M"}, {900, "CM"}, {500, "D"}, {400, "CD"}, {100, "C"}, {90, "XC"},
    {50, "L"},   {40, "XL"},  {10, "X"},  {9, "IX"},   {5, "V"},   {4, "IV"},
    {1, "I"},
};
}  // namespace

std::string toRoman(int value) {
    (void)value;
    throw std::logic_error("not implemented");
}
