#include "exercise.h"

// --- BenchmarkStats / summarize --------------------------------------------------------------------

BenchmarkStats summarize(const std::vector<double>& samples) {
    (void)samples;
    throw std::logic_error("not implemented");
}

// --- percentile -------------------------------------------------------------------------------------

double percentile(std::vector<double> samples, double p) {
    (void)samples;
    (void)p;
    throw std::logic_error("not implemented");
}

// --- trimmedMean -------------------------------------------------------------------------------------

double trimmedMean(std::vector<double> samples, double trimFraction) {
    (void)samples;
    (void)trimFraction;
    throw std::logic_error("not implemented");
}

// --- amdahlSpeedup / gustafsonSpeedup -----------------------------------------------------------------

double amdahlSpeedup(double parallelFraction, int numProcessors) {
    (void)parallelFraction;
    (void)numProcessors;
    throw std::logic_error("not implemented");
}

double gustafsonSpeedup(double parallelFraction, int numProcessors) {
    (void)parallelFraction;
    (void)numProcessors;
    throw std::logic_error("not implemented");
}

// --- isSignificantSpeedup -----------------------------------------------------------------------------

bool isSignificantSpeedup(const BenchmarkStats& before, const BenchmarkStats& after) {
    (void)before;
    (void)after;
    throw std::logic_error("not implemented");
}
