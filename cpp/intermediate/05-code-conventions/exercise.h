#pragma once

#include <ostream>
#include <stdexcept>
#include <string>
#include <vector>

// Topic 15-16 (Intermediate 05): Project Organization & Code Conventions
//
// This header is itself written to the conventions discussed in notes.md:
// <system headers> first, alphabetical; PascalCase for types; camelCase for
// functions/parameters; an explicit underlying type + bitwise operators for
// a "flags" enum class; structs returned by value instead of output
// parameters; const-reference parameters for containers.
//
// All five exercises operate on Task, Priority, and Status below.
// Stub bodies throw std::logic_error("not implemented").

// --- Priority: a scoped enum, ordered by underlying value --------------------------------------
//
// enum class ("scoped enum"): Priority::Low does NOT implicitly convert to
// int, and Low/Medium/High don't pollute the surrounding namespace (unlike a
// plain `enum Priority { Low, Medium, High };`). Relational operators (<, >,
// etc.) between two values of the SAME enum type ARE built in, comparing the
// underlying int values: Priority::High > Priority::Medium > Priority::Low.
enum class Priority { Low, Medium, High };

// --- Status: a scoped enum used as a bitmask ("flags enum") ------------------------------------
//
// An explicit underlying type (`: unsigned`) is required for a flags enum --
// without it, the compiler picks a type just large enough for the largest
// enumerator, which may be too small once bits are combined. Scoped enums get
// NO operators by default, not even |/& -- operator| and operator& below are
// provided (as "library" code, not exercises) so Status values can be
// combined and tested.
enum class Status : unsigned {
    None = 0,
    Active = 1u << 0,
    Blocked = 1u << 1,
    Done = 1u << 2,
};

// Combine flags, e.g. (Active | Done) sets both bits.
constexpr Status operator|(Status a, Status b) {
    return static_cast<Status>(static_cast<unsigned>(a) | static_cast<unsigned>(b));
}

// Intersect flags -- used by hasStatus (exercise 1) to test for a subset.
constexpr Status operator&(Status a, Status b) {
    return static_cast<Status>(static_cast<unsigned>(a) & static_cast<unsigned>(b));
}

// --- Task: a small aggregate -- PascalCase type, camelCase members -----------------------------
struct Task {
    std::string name;
    Priority priority;
    Status status;
};

// --- hasStatus: bitmask subset test -------------------------------------------------------------
//
// True iff every bit set in `required` is also set in `flags`, i.e.
// (flags & required) == required. Note the vacuous-truth edge case:
// hasStatus(flags, Status::None) is true for ANY flags, since Status::None
// has no bits set.
//
// Example: hasStatus(Status::Active | Status::Done, Status::Active) == true;
// hasStatus(Status::Active, Status::Active | Status::Done) == false;
// hasStatus(Status::Active, Status::None) == true.
bool hasStatus(Status flags, Status required);

// --- sortByPriority: stable sort, descending -----------------------------------------------------
//
// Returns `tasks` sorted by priority, highest first. Tasks with equal
// priority keep their original relative order (a stable sort).
//
// Example: sortByPriority({{"A",Medium,_},{"B",High,_},{"C",Low,_},{"D",High,_}})
// has names {"B","D","A","C"} -- B before D, since B came first among the
// two High-priority tasks.
std::vector<Task> sortByPriority(std::vector<Task> tasks);

// --- countByPriority: return a struct instead of 3 output parameters -----------------------------
//
// Counts tasks at each Priority level. Returning one struct by value
// (rather than three `int&` output parameters) keeps the function signature
// self-documenting and lets the compiler apply NRVO. operator== and
// operator<< are provided (as "library" code) so PriorityCount works with
// CHECK_EQ.
struct PriorityCount {
    int low;
    int medium;
    int high;
};

inline bool operator==(const PriorityCount& a, const PriorityCount& b) {
    return a.low == b.low && a.medium == b.medium && a.high == b.high;
}

inline std::ostream& operator<<(std::ostream& os, const PriorityCount& c) {
    return os << "{low=" << c.low << ", medium=" << c.medium << ", high=" << c.high << "}";
}

PriorityCount countByPriority(const std::vector<Task>& tasks);

// --- namesWithStatus: filter by bitmask subset, preserving order ---------------------------------
//
// Names (in original order) of tasks whose status has every bit in
// `required` set (per hasStatus). required == Status::None matches every
// task (vacuous truth).
std::vector<std::string> namesWithStatus(const std::vector<Task>& tasks, Status required);

// --- highestPriorityActive: filter + find-max in one pass -----------------------------------------
//
// Among tasks with Status::Active set, returns the one with the highest
// priority; ties broken by first occurrence. Precondition: at least one task
// has Status::Active set.
Task highestPriorityActive(const std::vector<Task>& tasks);
