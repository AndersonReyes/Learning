#pragma once

#include <cstddef>
#include <stdexcept>
#include <vector>

// Topic 25 (Advanced 05): Compiler Optimization, Profiling & Benchmarking
//
// Profilers and benchmark harnesses produce raw timing SAMPLES; the real
// skill is interpreting them. Five exercises build a small "benchmark
// statistics toolkit" -- the math behind reading `perf`/Google Benchmark
// output: summary statistics, percentiles (tail latency), trimmed means
// (outlier-robust central tendency), Amdahl's/Gustafson's laws (predicting
// parallel speedup), and significance testing (did an optimization actually
// help, or is the difference noise?). Free functions/struct are declared
// here and defined in exercise.cpp. Stub bodies throw
// std::logic_error("not implemented").

// --- BenchmarkStats / summarize: summary statistics for a sample set ------------------------------
//
// summarize(samples) computes summary statistics over a non-empty vector of
// timing samples (e.g. seconds per iteration):
//   - count:  samples.size()
//   - mean:   arithmetic mean
//   - median: middle value of the SORTED samples (average of the two
//             middle values if count is even)
//   - stddev: SAMPLE standard deviation (divide the sum of squared
//             deviations by count - 1, i.e. Bessel's correction) -- 0.0 if
//             count == 1 (no variability can be measured from one sample)
//   - min / max: smallest / largest sample
//
// Throws std::invalid_argument if `samples` is empty.
//
// Example: summarize({10, 20, 30, 40, 50}) == {count: 5, mean: 30,
// median: 30, stddev: ~15.811 (sqrt(1000/4)), min: 10, max: 50}.
// summarize({10, 20, 30, 40}) == {count: 4, mean: 25, median: 25
// (avg of 20 and 30), stddev: ~12.910 (sqrt(500/3)), min: 10, max: 40}.
// summarize({42}) == {count: 1, mean: 42, median: 42, stddev: 0, min: 42,
// max: 42}.
struct BenchmarkStats {
    size_t count;
    double mean;
    double median;
    double stddev;
    double min;
    double max;
};

BenchmarkStats summarize(const std::vector<double>& samples);

// --- percentile: tail-latency analysis -------------------------------------------------------------
//
// Returns the `p`-th percentile (0 <= p <= 100) of `samples` using LINEAR
// INTERPOLATION between closest ranks (the method used by NumPy's default
// `percentile` and Excel's `PERCENTILE.INC`):
//
//   sort samples ascending (n == samples.size())
//   rank = (p / 100) * (n - 1)              // 0-indexed, may be fractional
//   lower = floor(rank), upper = ceil(rank)
//   result = samples[lower] + (rank - lower) * (samples[upper] - samples[lower])
//
// Throws std::invalid_argument if `samples` is empty or `p` is outside
// [0, 100].
//
// Example: percentile({10, 20, 30, 40, 50}, 0) == 10;
// percentile({10, 20, 30, 40, 50}, 100) == 50;
// percentile({10, 20, 30, 40, 50}, 50) == 30 (the median);
// percentile({10, 20, 30, 40, 50}, 10) == 14 (rank 0.4 between 10 and 20);
// percentile({10, 20, 30, 40, 50}, 90) == 46 (rank 3.6 between 40 and 50).
// percentile({42}, 37) == 42 (a single sample is every percentile).
double percentile(std::vector<double> samples, double p);

// --- trimmedMean: outlier-robust central tendency -----------------------------------------------
//
// Sorts `samples`, removes floor(samples.size() * trimFraction) values from
// EACH end (the most extreme low and high outliers), then returns the mean
// of the remaining values -- a standard technique for summarizing
// benchmark runs where occasional OS scheduling hiccups produce extreme
// outliers that would distort a plain mean.
//
// Throws std::invalid_argument if trimFraction is not in [0, 1), or if
// trimming would remove every sample (2 * floor(n * trimFraction) >= n,
// which also covers the empty-input case where n == 0).
//
// Example: trimmedMean({1, 2, 3, 4, 5, 100}, 0.0) == 115.0/6 (~19.1667, no
// trimming); trimmedMean({1, 2, 3, 4, 5, 100}, 0.2) == 3.5 (removes the 1
// lowest and 1 highest -- {2,3,4,5} -- since floor(6*0.2) == 1).
double trimmedMean(std::vector<double> samples, double trimFraction);

// --- amdahlSpeedup / gustafsonSpeedup: predicting parallel speedup --------------------------------
//
// Both take `parallelFraction` (the fraction 0 <= p <= 1 of the work that
// CAN be parallelized; the rest, 1-p, is inherently sequential) and
// `numProcessors` (an integer >= 1).
//
// amdahlSpeedup: Amdahl's law -- speedup of a FIXED-SIZE problem when its
// parallel portion is spread over `numProcessors`:
//   speedup = 1 / ((1 - p) + p / numProcessors)
// As numProcessors -> infinity, speedup -> 1 / (1 - p) (bounded by the
// sequential fraction, no matter how many processors).
//
// gustafsonSpeedup: Gustafson's law -- speedup when the PROBLEM SIZE scales
// with the number of processors (the parallel portion grows to fill the
// available processors, sequential portion stays fixed):
//   speedup = (1 - p) + p * numProcessors
//
// Both throw std::invalid_argument if parallelFraction is outside [0, 1]
// or numProcessors < 1.
//
// Example: amdahlSpeedup(0.0, 4) == 1 (nothing parallelizable -> no
// speedup); amdahlSpeedup(1.0, 4) == 4 (fully parallelizable -> linear
// speedup); amdahlSpeedup(0.5, 4) == 1.6; amdahlSpeedup(p, 1) == 1 for any
// p (one processor -> no speedup).
// gustafsonSpeedup(0.0, 4) == 1; gustafsonSpeedup(1.0, 4) == 4;
// gustafsonSpeedup(0.5, 4) == 2.5; gustafsonSpeedup(0.9, 10) == 9.1.
double amdahlSpeedup(double parallelFraction, int numProcessors);
double gustafsonSpeedup(double parallelFraction, int numProcessors);

// --- isSignificantSpeedup: did an optimization actually help? ------------------------------------
//
// Given `before`/`after` BenchmarkStats (e.g. from summarize() on two sets
// of timing samples for the same benchmark before/after a code change),
// returns true iff `after` is a STATISTICALLY SIGNIFICANT improvement: its
// mean is lower AND its ~95% confidence interval doesn't overlap `before`'s.
//
// Using the standard error SE = stddev / sqrt(count) and a 95% CI of
// mean +/- 1.96 * SE:
//   isSignificantSpeedup(before, after) ==
//       after.mean < before.mean &&
//       (after.mean + 1.96 * SE_after) < (before.mean - 1.96 * SE_before)
//
// This guards against declaring victory on noise: if the CIs overlap, the
// observed difference could plausibly be due to run-to-run variance alone.
//
// Example: before = {count: 100, mean: 100, stddev: 10, ...},
// after = {count: 100, mean: 90, stddev: 10, ...} -> true (CIs
// [98.04,101.96] and [88.04,91.96] don't overlap).
// before = {count: 4, mean: 100, stddev: 20, ...},
// after = {count: 4, mean: 95, stddev: 20, ...} -> false (CIs
// [80.4,119.6] and [75.4,114.6] overlap heavily, despite after.mean being
// lower).
bool isSignificantSpeedup(const BenchmarkStats& before, const BenchmarkStats& after);
