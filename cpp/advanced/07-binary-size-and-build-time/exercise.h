#pragma once

#include <cstddef>
#include <set>
#include <stdexcept>
#include <string>
#include <unordered_map>
#include <vector>

// Topic 28-29 (Advanced 07): Binary Size & Build Time
//
// `nm`/`size`/`objdump` and a build's timing log are graphs and numbers in
// disguise. Five exercises build a small "build-graph & binary-size analysis
// toolkit" -- the reasoning behind reading those tools' output: #include-graph
// reachability (what does a file drag in, and what breaks if it changes),
// critical-path scheduling (the theoretical fastest build with unlimited
// parallelism), greedy load-balancing across compile workers, and
// COMDAT/vague-linkage symbol deduplication with ODR-violation detection.
// Free functions/struct are declared here and defined in exercise.cpp. Stub
// bodies throw std::logic_error("not implemented").

// --- transitiveIncludeCount: forward #include-graph reachability ----------------------------------
//
// `includes` maps each file to the list of files it directly #includes.
// Returns the number of DISTINCT files transitively reachable from `file`
// by following #include edges -- i.e. everything the compiler must parse
// when compiling `file`, NOT counting `file` itself. Diamond dependencies
// (a header reachable via two different paths) are counted once; cycles
// (mutually -- normally illegal, but #pragma once / include guards make the
// SECOND #include a no-op) must not cause infinite recursion.
//
// Throws std::invalid_argument if `file` is not a key of `includes`.
//
// Example:
//   includes = {
//     {"main.cpp", {"a.h", "b.h"}},
//     {"a.h",      {"c.h"}},
//     {"b.h",      {"c.h", "d.h"}},
//     {"c.h",      {}},
//     {"d.h",      {}},
//   }
//   transitiveIncludeCount(includes, "main.cpp") == 4  // a.h, b.h, c.h, d.h
//   transitiveIncludeCount(includes, "c.h")      == 0  // no includes of its own
//   transitiveIncludeCount(includes, "a.h")      == 1  // c.h
size_t transitiveIncludeCount(const std::unordered_map<std::string, std::vector<std::string>>& includes,
                               const std::string& file);

// --- filesToRebuild: reverse #include-graph reachability (incremental builds) ---------------------
//
// `includedBy` maps each file to the list of files that directly #include
// it (the reverse of an #include graph). Returns the set of every file that
// must be RECOMPILED if `changedFile` changes: `changedFile` itself, plus
// every file that transitively includes it (directly, or via another header
// that includes it).
//
// Throws std::invalid_argument if `changedFile` is not a key of `includedBy`.
//
// Example:
//   includedBy = {
//     {"c.h",        {"a.h", "b.h"}},
//     {"a.h",        {"main.cpp", "x.cpp"}},
//     {"b.h",        {"main.cpp", "other.cpp"}},
//     {"d.h",        {"other.cpp"}},
//     {"main.cpp",   {}},
//     {"x.cpp",      {}},
//     {"other.cpp",  {}},
//   }
//   filesToRebuild(includedBy, "c.h") ==
//       {"c.h", "a.h", "b.h", "main.cpp", "x.cpp", "other.cpp"}     // 6 files
//   filesToRebuild(includedBy, "d.h") == {"d.h", "other.cpp"}       // 2 files
//   filesToRebuild(includedBy, "x.cpp") == {"x.cpp"}                // 1 file (a leaf .cpp)
std::set<std::string> filesToRebuild(const std::unordered_map<std::string, std::vector<std::string>>& includedBy,
                                      const std::string& changedFile);

// --- criticalPathBuildTime: DAG critical path / build makespan ------------------------------------
//
// `deps` maps each build target to the list of targets it depends on
// (must be compiled/linked before it). `compileTime` gives each target's
// own compile duration in seconds. Returns the minimum possible wall-clock
// time to build EVERY target, assuming unlimited parallel workers -- i.e.
// the length of the longest path through the dependency DAG ("Amdahl's law
// for builds": more workers can't beat the critical path).
//
// Define finish(t) = compileTime.at(t) + max(finish(d) for d in deps[t]),
// or finish(t) = compileTime.at(t) if deps[t] is empty. The result is
// max(finish(t) for all t appearing in `deps`).
//
// Throws std::invalid_argument if:
//   - any target name appearing in `deps` (as a key, or as one of another
//     target's dependencies) is missing from `compileTime`, or
//   - the dependency graph contains a cycle.
//
// Example (diamond):
//   deps = {
//     {"main", {"a", "b"}},
//     {"a",    {"base"}},
//     {"b",    {"base"}},
//     {"base", {}},
//   }
//   compileTime = {{"main", 1.5}, {"a", 2.0}, {"b", 1.5}, {"base", 3.0}}
//   finish(base) = 3.0
//   finish(a)    = 2.0 + 3.0 = 5.0
//   finish(b)    = 1.5 + 3.0 = 4.5
//   finish(main) = 1.5 + max(5.0, 4.5) = 6.5
//   criticalPathBuildTime(deps, compileTime) == 6.5
//
// Example (chain): deps = {{"c", {"b"}}, {"b", {"a"}}, {"a", {}}},
//   compileTime = {{"a", 2.0}, {"b", 1.0}, {"c", 3.0}}
//   finish(a) = 2.0, finish(b) = 3.0, finish(c) = 6.0
//   criticalPathBuildTime(deps, compileTime) == 6.0
//
// Example (cycle): deps = {{"x", {"y"}}, {"y", {"x"}}} -> throws std::invalid_argument.
double criticalPathBuildTime(const std::unordered_map<std::string, std::vector<std::string>>& deps,
                              const std::unordered_map<std::string, double>& compileTime);

// --- groupLoadsGreedy: LPT scheduling across parallel compile workers ------------------------------
//
// Distributes `fileTimes` (each translation unit's compile duration) across
// `numGroups` parallel compile workers using LPT (Longest Processing Time
// first): sort the times in DESCENDING order, then assign each one in turn
// to whichever group currently has the SMALLEST total load (ties broken by
// lowest group index). Returns each group's final total load, in group-index
// order.
//
// Throws std::invalid_argument if numGroups < 1, fileTimes is empty, or any
// element of fileTimes is negative.
//
// Example: groupLoadsGreedy({10, 8, 6, 4, 4}, 2)
//   sorted descending: 10, 8, 6, 4, 4 ; loads start at [0, 0]
//   10 -> group 0 (tie, lowest index)        -> [10, 0]
//    8 -> group 1 (load 0 < 10)              -> [10, 8]
//    6 -> group 1 (load 8 < 10)              -> [10, 14]
//    4 -> group 0 (load 10 < 14)             -> [14, 14]
//    4 -> group 0 (tie, lowest index)        -> [18, 14]
//   == {18, 14}
//
// groupLoadsGreedy({5, 5, 5}, 3) == {5, 5, 5}
// groupLoadsGreedy({3, 2, 1}, 1) == {6}
// groupLoadsGreedy({5}, 3)       == {5, 0, 0}
std::vector<double> groupLoadsGreedy(std::vector<double> fileTimes, int numGroups);

// --- linkedBinarySize: COMDAT/vague-linkage symbol deduplication & ODR check -----------------------
//
// One entry per symbol emitted into a translation unit's object file.
struct SymbolInfo {
    std::string translationUnit;
    std::string symbolName;
    size_t sizeBytes;
    bool isTemplateInstantiation;
};

// Computes the total size in bytes of a linked binary's symbols.
//
// Non-template symbols (isTemplateInstantiation == false) are summed
// UNCONDITIONALLY -- each is a distinct definition (e.g. a free function or
// global in one TU), and the linker keeps every one of them.
//
// Template instantiations (isTemplateInstantiation == true) use "vague
// linkage": if the SAME symbolName is instantiated in multiple translation
// units, the linker folds them into ONE copy (COMDAT folding). Deduplicate
// these by symbolName and count each distinct symbolName's size ONCE.
//
// Throws std::invalid_argument if two entries share a symbolName with
// isTemplateInstantiation == true but report DIFFERENT sizeBytes -- this
// signals an ODR violation (the "same" template instantiation compiled
// inconsistently across translation units, e.g. due to mismatched flags or
// macro definitions).
//
// Returns 0 for an empty `symbols`.
//
// Example:
//   symbols = {
//     {"a.cpp", "main",                  200, false},
//     {"a.cpp", "g_counter",               8, false},
//     {"b.cpp", "helper()",               142, false},
//     {"a.cpp", "vector<int>::push_back", 100, true},
//     {"b.cpp", "vector<int>::push_back", 100, true},   // same instantiation: counted once
//     {"c.cpp", "set<string>::insert",     50, true},   // distinct instantiation
//   }
//   non-template sum: 200 + 8 + 142 = 350
//   template sum (deduped): 100 + 50 = 150
//   linkedBinarySize(symbols) == 500
//
// ODR violation example:
//   symbols = {
//     {"a.cpp", "vector<int>::push_back", 100, true},
//     {"b.cpp", "vector<int>::push_back", 120, true},   // same name, different size -> throws
//   }
//   linkedBinarySize(symbols) -> throws std::invalid_argument
size_t linkedBinarySize(const std::vector<SymbolInfo>& symbols);
