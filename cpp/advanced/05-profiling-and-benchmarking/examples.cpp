#include <algorithm>
#include <chrono>
#include <cmath>
#include <iomanip>
#include <iostream>
#include <vector>

// Topic 25 (Advanced 05): Compiler Optimization, Profiling & Benchmarking
//
// Different illustrative examples than exercise.h's summarize/percentile/
// trimmedMean/amdahlSpeedup/gustafsonSpeedup/isSignificantSpeedup -- same
// concepts (collecting samples, summarizing, percentiles, predicted
// speedup, significance), different code and a different percentile
// method, so the exercises stay unspoiled.
//
// Build with -O2 for the timing comparison to be meaningful (see
// advanced/03's examples.cpp for why -O0 can invert small timing
// differences).

using Clock = std::chrono::steady_clock;

// --- Section 1: mini-benchmark harness ----------------------------------------------------------
//
// Two ways to build a vector of N squares: pushBackBuild (no reserve --
// repeated reallocation/copying as the vector grows) vs reservedBuild
// (reserve up front -- no reallocation).

constexpr int kElements = 200'000;
constexpr int kWarmup = 5;
constexpr int kSamples = 30;

long long pushBackBuild(volatile long long& sink) {
    std::vector<int> v;
    for (int i = 0; i < kElements; ++i) v.push_back(i * i);
    long long total = 0;
    for (int x : v) total += x;
    sink = total;  // prevent dead-code elimination of the loop above
    return total;
}

long long reservedBuild(volatile long long& sink) {
    std::vector<int> v;
    v.reserve(kElements);
    for (int i = 0; i < kElements; ++i) v.push_back(i * i);
    long long total = 0;
    for (int x : v) total += x;
    sink = total;
    return total;
}

// Runs `f` `warmup` times (discarded), then `samples` times, returning each
// timed iteration's duration in microseconds.
template <typename F>
std::vector<double> collectSamples(F&& f, int warmup, int samples) {
    volatile long long sink = 0;

    for (int i = 0; i < warmup; ++i) f(sink);

    std::vector<double> durations;
    durations.reserve(static_cast<size_t>(samples));
    for (int i = 0; i < samples; ++i) {
        auto t0 = Clock::now();
        f(sink);
        auto t1 = Clock::now();
        durations.push_back(std::chrono::duration<double, std::micro>(t1 - t0).count());
    }
    return durations;
}

// --- Section 2: inline summary stats (mean / min / max / stddev) --------------------------------

struct Summary {
    double mean;
    double min;
    double max;
    double stddev;
};

Summary summarizeInline(const std::vector<double>& samples) {
    double sum = 0.0, mn = samples[0], mx = samples[0];
    for (double x : samples) {
        sum += x;
        mn = std::min(mn, x);
        mx = std::max(mx, x);
    }
    double mean = sum / static_cast<double>(samples.size());

    double sumSqDev = 0.0;
    for (double x : samples) sumSqDev += (x - mean) * (x - mean);
    double stddev = samples.size() > 1 ? std::sqrt(sumSqDev / static_cast<double>(samples.size() - 1)) : 0.0;

    return {mean, mn, mx, stddev};
}

// --- Section 3: percentiles via the nearest-rank method ------------------------------------------
//
// Nearest-rank ("NIST method 1"): no interpolation -- sort, then pick the
// sample at 1-indexed rank ceil(p/100 * n), clamped to [1, n]. Differs from
// exercise.h's percentile, which uses linear interpolation between ranks.

double nearestRankPercentile(std::vector<double> samples, double p) {
    std::sort(samples.begin(), samples.end());
    long n = static_cast<long>(samples.size());
    long rank = static_cast<long>(std::ceil(p / 100.0 * static_cast<double>(n)));
    rank = std::clamp<long>(rank, 1, n);
    return samples[static_cast<size_t>(rank - 1)];
}

// --- Section 4: Amdahl's law -- predicted speedup table -------------------------------------------

double amdahlPredicted(double parallelFraction, int numProcessors) {
    return 1.0 / ((1.0 - parallelFraction) + parallelFraction / static_cast<double>(numProcessors));
}

// --- Section 5: significance via 95% confidence-interval overlap ----------------------------------
//
// Returns true if the two samples' 95% CIs (mean +/- 1.96 * stddev/sqrt(n))
// don't overlap -- i.e. the difference between them is unlikely to be noise.

bool ciNonOverlapping(const Summary& a, const Summary& b, size_t n) {
    constexpr double z95 = 1.96;
    double seA = a.stddev / std::sqrt(static_cast<double>(n));
    double seB = b.stddev / std::sqrt(static_cast<double>(n));
    double aLower = a.mean - z95 * seA, aUpper = a.mean + z95 * seA;
    double bLower = b.mean - z95 * seB, bUpper = b.mean + z95 * seB;
    return aUpper < bLower || bUpper < aLower;
}

int main() {
    std::cout << std::fixed << std::setprecision(3);

    std::cout << "-- mini-benchmark harness: push_back vs reserve (" << kSamples << " samples, "
              << kWarmup << " warm-up) --\n";
    std::vector<double> pushSamples = collectSamples(pushBackBuild, kWarmup, kSamples);
    std::vector<double> reserveSamples = collectSamples(reservedBuild, kWarmup, kSamples);
    std::cout << "  collected " << pushSamples.size() << " / " << reserveSamples.size() << " samples (us)\n";

    std::cout << "\n-- summary stats --\n";
    Summary pushSummary = summarizeInline(pushSamples);
    Summary reserveSummary = summarizeInline(reserveSamples);
    std::cout << "  push_back: mean=" << pushSummary.mean << "us min=" << pushSummary.min << "us max="
              << pushSummary.max << "us stddev=" << pushSummary.stddev << "us\n";
    std::cout << "  reserve:   mean=" << reserveSummary.mean << "us min=" << reserveSummary.min << "us max="
              << reserveSummary.max << "us stddev=" << reserveSummary.stddev << "us\n";

    std::cout << "\n-- percentiles (nearest-rank method) --\n";
    for (double p : {50.0, 90.0, 99.0}) {
        std::cout << "  p" << p << ": push_back=" << nearestRankPercentile(pushSamples, p)
                  << "us  reserve=" << nearestRankPercentile(reserveSamples, p) << "us\n";
    }

    std::cout << "\n-- Amdahl's law: predicted speedup for parallel fraction p, n processors --\n";
    for (double p : {0.5, 0.75, 0.9, 0.95, 0.99}) {
        std::cout << "  p=" << p << ": ";
        for (int n : {2, 4, 8, 16, 64}) {
            std::cout << "n=" << n << " -> " << amdahlPredicted(p, n) << "x  ";
        }
        std::cout << "\n";
    }

    std::cout << "\n-- significance via 95% CI overlap --\n";
    bool significant = ciNonOverlapping(pushSummary, reserveSummary, pushSamples.size());
    std::cout << "  push_back vs reserve difference is statistically significant: " << std::boolalpha
              << significant << "\n";
}
