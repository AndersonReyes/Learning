#include "exercise.h"

#include <algorithm>
#include <functional>

// --- transitiveIncludeCount ----------------------------------------------------------------------

size_t transitiveIncludeCount(const std::unordered_map<std::string, std::vector<std::string>>& includes,
                               const std::string& file) {
    (void)includes;
    (void)file;
    throw std::logic_error("not implemented");
}

// --- filesToRebuild --------------------------------------------------------------------------------

std::set<std::string> filesToRebuild(const std::unordered_map<std::string, std::vector<std::string>>& includedBy,
                                      const std::string& changedFile) {
    (void)includedBy;
    (void)changedFile;
    throw std::logic_error("not implemented");
}

// --- criticalPathBuildTime --------------------------------------------------------------------------

double criticalPathBuildTime(const std::unordered_map<std::string, std::vector<std::string>>& deps,
                              const std::unordered_map<std::string, double>& compileTime) {
    (void)deps;
    (void)compileTime;
    throw std::logic_error("not implemented");
}

// --- groupLoadsGreedy --------------------------------------------------------------------------------

std::vector<double> groupLoadsGreedy(std::vector<double> fileTimes, int numGroups) {
    (void)fileTimes;
    (void)numGroups;
    throw std::logic_error("not implemented");
}

// --- linkedBinarySize --------------------------------------------------------------------------------

size_t linkedBinarySize(const std::vector<SymbolInfo>& symbols) {
    (void)symbols;
    throw std::logic_error("not implemented");
}
