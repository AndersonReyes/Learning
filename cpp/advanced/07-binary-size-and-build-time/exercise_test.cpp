#include "exercise.h"

#include <set>
#include <stdexcept>
#include <string>
#include <unordered_map>
#include <vector>

#include "../../testing.h"

// --- transitiveIncludeCount: forward #include-graph reachability ----------------------------------

TEST(TransitiveIncludeCountForwardReachability) {
    std::unordered_map<std::string, std::vector<std::string>> includes = {
        {"main.cpp", {"a.h", "b.h"}},
        {"a.h", {"c.h"}},
        {"b.h", {"c.h", "d.h"}},
        {"c.h", {}},
        {"d.h", {}},
    };

    CHECK_EQ(transitiveIncludeCount(includes, "main.cpp"), static_cast<size_t>(4));
    CHECK_EQ(transitiveIncludeCount(includes, "c.h"), static_cast<size_t>(0));
    CHECK_EQ(transitiveIncludeCount(includes, "a.h"), static_cast<size_t>(1));

    bool threw = false;
    try {
        transitiveIncludeCount(includes, "missing.h");
    } catch (const std::invalid_argument&) {
        threw = true;
    }
    CHECK(threw);
}

TEST(TransitiveIncludeCountHandlesCyclesWithoutInfiniteLoop) {
    // Mutual #include via guards: x.h includes y.h and vice versa. Neither
    // file should ever count itself, even when cyclically reachable.
    std::unordered_map<std::string, std::vector<std::string>> includes = {
        {"x.h", {"y.h"}},
        {"y.h", {"x.h"}},
    };

    CHECK_EQ(transitiveIncludeCount(includes, "x.h"), static_cast<size_t>(1));
    CHECK_EQ(transitiveIncludeCount(includes, "y.h"), static_cast<size_t>(1));
}

// --- filesToRebuild: reverse #include-graph reachability (incremental builds) ---------------------

TEST(FilesToRebuildReverseReachability) {
    std::unordered_map<std::string, std::vector<std::string>> includedBy = {
        {"c.h", {"a.h", "b.h"}},
        {"a.h", {"main.cpp", "x.cpp"}},
        {"b.h", {"main.cpp", "other.cpp"}},
        {"d.h", {"other.cpp"}},
        {"main.cpp", {}},
        {"x.cpp", {}},
        {"other.cpp", {}},
    };

    CHECK(filesToRebuild(includedBy, "c.h") ==
          (std::set<std::string>{"c.h", "a.h", "b.h", "main.cpp", "x.cpp", "other.cpp"}));
    CHECK(filesToRebuild(includedBy, "d.h") == (std::set<std::string>{"d.h", "other.cpp"}));
    CHECK(filesToRebuild(includedBy, "x.cpp") == (std::set<std::string>{"x.cpp"}));

    bool threw = false;
    try {
        filesToRebuild(includedBy, "missing.h");
    } catch (const std::invalid_argument&) {
        threw = true;
    }
    CHECK(threw);
}

// --- criticalPathBuildTime: DAG critical path / build makespan ------------------------------------

TEST(CriticalPathBuildTimeDiamondChainAndCycle) {
    std::unordered_map<std::string, std::vector<std::string>> diamond = {
        {"main", {"a", "b"}},
        {"a", {"base"}},
        {"b", {"base"}},
        {"base", {}},
    };
    std::unordered_map<std::string, double> diamondTimes = {
        {"main", 1.5},
        {"a", 2.0},
        {"b", 1.5},
        {"base", 3.0},
    };
    CHECK_EQ(criticalPathBuildTime(diamond, diamondTimes), 6.5);

    std::unordered_map<std::string, std::vector<std::string>> chain = {
        {"c", {"b"}},
        {"b", {"a"}},
        {"a", {}},
    };
    std::unordered_map<std::string, double> chainTimes = {{"a", 2.0}, {"b", 1.0}, {"c", 3.0}};
    CHECK_EQ(criticalPathBuildTime(chain, chainTimes), 6.0);

    std::unordered_map<std::string, std::vector<std::string>> cycle = {
        {"x", {"y"}},
        {"y", {"x"}},
    };
    std::unordered_map<std::string, double> cycleTimes = {{"x", 1.0}, {"y", 1.0}};
    bool threw = false;
    try {
        criticalPathBuildTime(cycle, cycleTimes);
    } catch (const std::invalid_argument&) {
        threw = true;
    }
    CHECK(threw);

    // "b" is a dependency but has no entry in compileTime.
    std::unordered_map<std::string, std::vector<std::string>> missingTime = {{"a", {"b"}}, {"b", {}}};
    std::unordered_map<std::string, double> missingTimes = {{"a", 1.0}};
    threw = false;
    try {
        criticalPathBuildTime(missingTime, missingTimes);
    } catch (const std::invalid_argument&) {
        threw = true;
    }
    CHECK(threw);
}

// --- groupLoadsGreedy: LPT scheduling across parallel compile workers ------------------------------

TEST(GroupLoadsGreedyLptScheduling) {
    CHECK(groupLoadsGreedy({10, 8, 6, 4, 4}, 2) == (std::vector<double>{18, 14}));
    CHECK(groupLoadsGreedy({5, 5, 5}, 3) == (std::vector<double>{5, 5, 5}));
    CHECK(groupLoadsGreedy({3, 2, 1}, 1) == (std::vector<double>{6}));
    CHECK(groupLoadsGreedy({5}, 3) == (std::vector<double>{5, 0, 0}));

    bool threw = false;
    try {
        groupLoadsGreedy({1.0, 2.0}, 0);
    } catch (const std::invalid_argument&) {
        threw = true;
    }
    CHECK(threw);

    threw = false;
    try {
        groupLoadsGreedy({}, 2);
    } catch (const std::invalid_argument&) {
        threw = true;
    }
    CHECK(threw);

    threw = false;
    try {
        groupLoadsGreedy({1.0, -2.0}, 2);
    } catch (const std::invalid_argument&) {
        threw = true;
    }
    CHECK(threw);
}

// --- linkedBinarySize: COMDAT/vague-linkage symbol deduplication & ODR check -----------------------

TEST(LinkedBinarySizeDedupesTemplatesAndDetectsOdrViolations) {
    std::vector<SymbolInfo> symbols = {
        {"a.cpp", "main", 200, false},
        {"a.cpp", "g_counter", 8, false},
        {"b.cpp", "helper()", 142, false},
        {"a.cpp", "vector<int>::push_back", 100, true},
        {"b.cpp", "vector<int>::push_back", 100, true},
        {"c.cpp", "set<string>::insert", 50, true},
    };
    CHECK_EQ(linkedBinarySize(symbols), static_cast<size_t>(500));

    CHECK_EQ(linkedBinarySize({}), static_cast<size_t>(0));

    std::vector<SymbolInfo> odrViolation = {
        {"a.cpp", "vector<int>::push_back", 100, true},
        {"b.cpp", "vector<int>::push_back", 120, true},
    };
    bool threw = false;
    try {
        linkedBinarySize(odrViolation);
    } catch (const std::invalid_argument&) {
        threw = true;
    }
    CHECK(threw);
}

TEST_MAIN()
