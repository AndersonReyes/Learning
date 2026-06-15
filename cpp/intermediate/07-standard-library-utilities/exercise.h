#pragma once

#include <charconv>
#include <filesystem>
#include <optional>
#include <random>
#include <stdexcept>
#include <string>
#include <string_view>
#include <variant>
#include <vector>

// Topic 19 (Intermediate 07): Standard Library Utilities
//
// Five exercises, each centered on one standard-library utility:
// <optional> + <charconv>, <variant>, <random>, <filesystem>, and
// <string_view>. Stub bodies throw std::logic_error("not implemented").

// --- parseInt: std::optional + std::from_chars (<charconv>) ------------------------------------
//
// Parses `s` as a base-10 int, returning std::nullopt unless `s` is EXACTLY
// a valid int literal: no leading/trailing whitespace, no leading '+'
// (std::from_chars accepts only an optional '-'), the ENTIRE string must be
// consumed, and the value must fit in int. std::from_chars is
// locale-independent and never throws -- it reports success/failure via a
// result struct {ptr, ec}.
//
// Example: parseInt("123") == 123; parseInt("-45") == -45;
// parseInt("+7") == std::nullopt; parseInt("") == std::nullopt;
// parseInt("123abc") == std::nullopt (not fully consumed);
// parseInt("2147483647") == 2147483647 (INT_MAX);
// parseInt("2147483648") == std::nullopt (overflows int);
// parseInt("  123") == std::nullopt (leading whitespace).
std::optional<int> parseInt(std::string_view s);

// --- jsonValueToString: std::variant + std::visit -----------------------------------------------

using JsonValue = std::variant<std::monostate, bool, int, double, std::string>;

// Renders `value` as JSON text: std::monostate -> "null", bool -> "true"/
// "false" (NOT "1"/"0"), int/double -> their numeric text (doubles via
// default stream formatting, so 3.5 -> "3.5", 100.0 -> "100" -- not
// "3.500000"), std::string -> wrapped in double quotes (no escaping needed
// for this exercise's inputs).
//
// Example: jsonValueToString(std::monostate{}) == "null";
// jsonValueToString(true) == "true"; jsonValueToString(42) == "42";
// jsonValueToString(3.5) == "3.5";
// jsonValueToString(std::string("hi")) == "\"hi\"".
std::string jsonValueToString(const JsonValue& value);

// --- fisherYatesShuffle: <random>, calling the engine directly -----------------------------------
//
// Shuffles `items` in place (the Fisher-Yates/Durstenfeld algorithm: for i
// from items.size()-1 down to 1, swap items[i] with items[j] for a random
// j in [0, i]) and returns the shuffled vector.
//
// Deliberately calls `rng()` directly (`rng() % (i + 1)`) rather than
// std::uniform_int_distribution: a std::mt19937's sequence of rng() values
// for a given seed is fixed by the standard's algorithm parameters, so this
// function's output for a given seed is exactly reproducible across
// implementations. std::uniform_int_distribution's mapping from engine
// output to its range is implementation-defined -- NOT reproducible across
// standard library implementations, even with the same engine and seed.
//
// Example: with `std::mt19937 rng(42)`,
// fisherYatesShuffle({1,2,3,4,5,6,7,8,9,10}, rng) ==
// {2,4,10,8,7,1,9,5,6,3} (see exercise_test.cpp).
std::vector<int> fisherYatesShuffle(std::vector<int> items, std::mt19937& rng);

// --- normalizedPath: std::filesystem::path::lexically_normal --------------------------------------
//
// Returns the lexically-normal form of `path` (resolving "." and ".."
// components and redundant separators PURELY TEXTUALLY -- no filesystem
// access, symlinks not considered), as a '/'-separated string
// (generic_string()).
//
// Example: normalizedPath("a/./b/../c") == "a/c";
// normalizedPath("a/b/../../c") == "c";
// normalizedPath("./a/b/") == "a/b/";
// normalizedPath("/a/../../b") == "/b" (".." above root is discarded);
// normalizedPath("a/b/c/../..") == "a/".
std::string normalizedPath(const std::string& path);

// --- splitOnWhitespace: std::string_view, non-owning tokens ---------------------------------------
//
// Splits `s` on runs of whitespace (per std::isspace), returning non-owning
// views into `s` -- leading/trailing/repeated whitespace produces no empty
// tokens. The returned string_views alias `s` and are only valid as long as
// `s` is alive -- unlike normalizeWhitespace (intermediate/03), which
// returns an owning std::string.
//
// Example: splitOnWhitespace("  hello   world  ") == {"hello", "world"};
// splitOnWhitespace("") == {}; splitOnWhitespace("   ") == {};
// splitOnWhitespace("a") == {"a"}.
std::vector<std::string_view> splitOnWhitespace(std::string_view s);
