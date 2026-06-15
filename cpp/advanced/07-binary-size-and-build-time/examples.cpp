#include <algorithm>
#include <functional>
#include <iostream>
#include <set>
#include <string>
#include <unordered_map>
#include <vector>

// Topic 28-29 (Advanced 07): Binary Size & Build Time
//
// Different illustrative examples than exercise.h's transitiveIncludeCount/
// filesToRebuild/criticalPathBuildTime/groupLoadsGreedy/linkedBinarySize --
// same algorithms, different data, so the exercises stay unspoiled.

// --- Forward #include reachability: how much a TU's compile must parse ----------------------------

size_t countTransitiveIncludes(const std::unordered_map<std::string, std::vector<std::string>>& includes,
                                const std::string& file) {
    std::set<std::string> visited;
    std::vector<std::string> stack(includes.at(file).begin(), includes.at(file).end());
    while (!stack.empty()) {
        std::string cur = stack.back();
        stack.pop_back();
        if (cur == file || visited.count(cur)) continue;
        visited.insert(cur);
        auto it = includes.find(cur);
        if (it != includes.end()) {
            for (const auto& dep : it->second) stack.push_back(dep);
        }
    }
    return visited.size();
}

// --- Reverse #include reachability: incremental rebuild set ---------------------------------------

std::set<std::string> rebuildSet(const std::unordered_map<std::string, std::vector<std::string>>& includedBy,
                                  const std::string& changedFile) {
    std::set<std::string> result;
    std::vector<std::string> stack{changedFile};
    while (!stack.empty()) {
        std::string cur = stack.back();
        stack.pop_back();
        if (result.count(cur)) continue;
        result.insert(cur);
        auto it = includedBy.find(cur);
        if (it != includedBy.end()) {
            for (const auto& dependent : it->second) stack.push_back(dependent);
        }
    }
    return result;
}

// --- Critical path: fastest possible full build with unlimited workers -----------------------------

double criticalPath(const std::unordered_map<std::string, std::vector<std::string>>& deps,
                     const std::unordered_map<std::string, double>& compileTime) {
    std::unordered_map<std::string, double> finish;
    std::function<double(const std::string&)> computeFinish = [&](const std::string& target) -> double {
        auto it = finish.find(target);
        if (it != finish.end()) return it->second;
        double maxDepFinish = 0.0;
        auto depsIt = deps.find(target);
        if (depsIt != deps.end()) {
            for (const auto& dep : depsIt->second) maxDepFinish = std::max(maxDepFinish, computeFinish(dep));
        }
        double result = compileTime.at(target) + maxDepFinish;
        finish[target] = result;
        return result;
    };
    double overall = 0.0;
    for (const auto& [target, _] : deps) overall = std::max(overall, computeFinish(target));
    return overall;
}

// --- LPT greedy scheduling: balancing compile jobs across workers ----------------------------------

std::vector<double> lptSchedule(std::vector<double> times, int numWorkers) {
    std::sort(times.begin(), times.end(), std::greater<double>());
    std::vector<double> loads(static_cast<size_t>(numWorkers), 0.0);
    for (double t : times) {
        size_t minIdx = 0;
        for (size_t i = 1; i < loads.size(); ++i) {
            if (loads[i] < loads[minIdx]) minIdx = i;
        }
        loads[minIdx] += t;
    }
    return loads;
}

// --- COMDAT/vague-linkage symbol deduplication for binary size -------------------------------------

struct Symbol {
    std::string translationUnit;
    std::string name;
    size_t sizeBytes;
    bool isTemplateInstantiation;
};

size_t binarySize(const std::vector<Symbol>& symbols) {
    size_t total = 0;
    std::unordered_map<std::string, size_t> templateSizes;
    for (const auto& sym : symbols) {
        if (!sym.isTemplateInstantiation) {
            total += sym.sizeBytes;
            continue;
        }
        templateSizes[sym.name] = sym.sizeBytes;
    }
    for (const auto& [name, size] : templateSizes) {
        (void)name;
        total += size;
    }
    return total;
}

int main() {
    std::cout << "-- Forward #include reachability: countTransitiveIncludes --\n";
    {
        std::unordered_map<std::string, std::vector<std::string>> includes = {
            {"app.cpp", {"logger.h", "network.h"}},
            {"logger.h", {"config.h"}},
            {"network.h", {"config.h", "buffer.h"}},
            {"config.h", {}},
            {"buffer.h", {}},
        };
        std::cout << "  app.cpp transitively includes " << countTransitiveIncludes(includes, "app.cpp")
                  << " files (logger.h, network.h, config.h, buffer.h)\n";
        std::cout << "  network.h transitively includes " << countTransitiveIncludes(includes, "network.h")
                  << " files (config.h, buffer.h)\n";
    }

    std::cout << "\n-- Reverse #include reachability: rebuildSet --\n";
    {
        std::unordered_map<std::string, std::vector<std::string>> includedBy = {
            {"config.h", {"logger.h", "network.h"}},
            {"logger.h", {"app.cpp"}},
            {"network.h", {"app.cpp", "server.cpp"}},
            {"app.cpp", {}},
            {"server.cpp", {}},
        };
        std::set<std::string> rebuild = rebuildSet(includedBy, "config.h");
        std::cout << "  changing config.h forces rebuilding " << rebuild.size() << " files: ";
        bool first = true;
        for (const auto& f : rebuild) {
            if (!first) std::cout << ", ";
            std::cout << f;
            first = false;
        }
        std::cout << "\n";
    }

    std::cout << "\n-- Critical path: criticalPath (a 4-step build pipeline) --\n";
    {
        std::unordered_map<std::string, std::vector<std::string>> deps = {
            {"link", {"compile_main", "compile_util"}},
            {"compile_main", {"generate_headers"}},
            {"compile_util", {"generate_headers"}},
            {"generate_headers", {}},
        };
        std::unordered_map<std::string, double> compileTime = {
            {"link", 1.0},
            {"compile_main", 4.0},
            {"compile_util", 2.0},
            {"generate_headers", 1.0},
        };
        std::cout << "  total sequential time = " << (1.0 + 4.0 + 2.0 + 1.0) << "s\n";
        std::cout << "  critical path (unlimited workers) = " << criticalPath(deps, compileTime) << "s\n";
    }

    std::cout << "\n-- LPT greedy scheduling: lptSchedule --\n";
    {
        std::vector<double> times = {7, 5, 4, 3, 2};
        std::vector<double> loads = lptSchedule(times, 3);
        std::cout << "  5 files {7,5,4,3,2}s across 3 workers -> loads = {";
        for (size_t i = 0; i < loads.size(); ++i) {
            if (i > 0) std::cout << ", ";
            std::cout << loads[i];
        }
        std::cout << "} (slowest worker finishes at " << *std::max_element(loads.begin(), loads.end()) << "s)\n";
    }

    std::cout << "\n-- COMDAT symbol deduplication: binarySize --\n";
    {
        std::vector<Symbol> symbols = {
            {"render.cpp", "renderFrame", 300, false},
            {"render.cpp", "g_frameCount", 8, false},
            {"input.cpp", "pollInput", 120, false},
            {"render.cpp", "vector<Vertex>::push_back", 90, true},
            {"input.cpp", "vector<Vertex>::push_back", 90, true},
            {"physics.cpp", "vector<Vertex>::push_back", 90, true},
            {"physics.cpp", "unique_ptr<Body>::~unique_ptr", 40, true},
        };
        size_t rawSum = 0;
        for (const auto& s : symbols) rawSum += s.sizeBytes;
        std::cout << "  raw sum of all emitted symbols = " << rawSum << " bytes\n";
        std::cout << "  after COMDAT folding (template instantiations deduped) = " << binarySize(symbols)
                  << " bytes\n";
    }
}
