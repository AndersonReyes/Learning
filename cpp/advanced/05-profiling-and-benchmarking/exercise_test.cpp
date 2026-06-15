#include "exercise.h"

#include <cmath>
#include <stdexcept>
#include <vector>

#include "../../testing.h"

// --- BenchmarkStats / summarize --------------------------------------------------------------------

TEST(SummarizeBasicStats) {
    BenchmarkStats s1 = summarize({10, 20, 30, 40, 50});
    CHECK_EQ(s1.count, static_cast<size_t>(5));
    CHECK(std::abs(s1.mean - 30.0) < 1e-9);
    CHECK(std::abs(s1.median - 30.0) < 1e-9);
    CHECK(std::abs(s1.stddev - std::sqrt(250.0)) < 1e-9);
    CHECK(std::abs(s1.min - 10.0) < 1e-9);
    CHECK(std::abs(s1.max - 50.0) < 1e-9);

    // Even count: median averages the two middle values.
    BenchmarkStats s2 = summarize({10, 20, 30, 40});
    CHECK_EQ(s2.count, static_cast<size_t>(4));
    CHECK(std::abs(s2.mean - 25.0) < 1e-9);
    CHECK(std::abs(s2.median - 25.0) < 1e-9);
    CHECK(std::abs(s2.stddev - std::sqrt(500.0 / 3.0)) < 1e-9);
    CHECK(std::abs(s2.min - 10.0) < 1e-9);
    CHECK(std::abs(s2.max - 40.0) < 1e-9);

    // Single sample: stddev defined as 0.
    BenchmarkStats s3 = summarize({42});
    CHECK_EQ(s3.count, static_cast<size_t>(1));
    CHECK(std::abs(s3.mean - 42.0) < 1e-9);
    CHECK(std::abs(s3.median - 42.0) < 1e-9);
    CHECK(std::abs(s3.stddev - 0.0) < 1e-9);
    CHECK(std::abs(s3.min - 42.0) < 1e-9);
    CHECK(std::abs(s3.max - 42.0) < 1e-9);

    // Unsorted input -- median/min/max must not depend on input order.
    BenchmarkStats s4 = summarize({50, 10, 30, 20, 40});
    CHECK(std::abs(s4.median - 30.0) < 1e-9);
    CHECK(std::abs(s4.min - 10.0) < 1e-9);
    CHECK(std::abs(s4.max - 50.0) < 1e-9);

    bool threw = false;
    try {
        summarize({});
    } catch (const std::invalid_argument&) {
        threw = true;
    }
    CHECK(threw);
}

// --- percentile -------------------------------------------------------------------------------------

TEST(PercentileLinearInterpolation) {
    std::vector<double> data = {10, 20, 30, 40, 50};
    CHECK(std::abs(percentile(data, 0) - 10.0) < 1e-9);
    CHECK(std::abs(percentile(data, 100) - 50.0) < 1e-9);
    CHECK(std::abs(percentile(data, 50) - 30.0) < 1e-9);
    CHECK(std::abs(percentile(data, 25) - 20.0) < 1e-9);
    CHECK(std::abs(percentile(data, 10) - 14.0) < 1e-9);  // rank 0.4 between 10 and 20
    CHECK(std::abs(percentile(data, 90) - 46.0) < 1e-9);  // rank 3.6 between 40 and 50

    // A single sample is every percentile.
    CHECK(std::abs(percentile({42}, 0) - 42.0) < 1e-9);
    CHECK(std::abs(percentile({42}, 100) - 42.0) < 1e-9);
    CHECK(std::abs(percentile({42}, 37) - 42.0) < 1e-9);

    // Unsorted input gives the same result as sorted.
    CHECK(std::abs(percentile({50, 10, 30, 20, 40}, 50) - 30.0) < 1e-9);

    bool threw = false;
    try {
        percentile({1, 2, 3}, -1);
    } catch (const std::invalid_argument&) {
        threw = true;
    }
    CHECK(threw);

    threw = false;
    try {
        percentile({1, 2, 3}, 101);
    } catch (const std::invalid_argument&) {
        threw = true;
    }
    CHECK(threw);

    threw = false;
    try {
        percentile({}, 50);
    } catch (const std::invalid_argument&) {
        threw = true;
    }
    CHECK(threw);
}

// --- trimmedMean -------------------------------------------------------------------------------------

TEST(TrimmedMeanRemovesOutliers) {
    std::vector<double> data = {1, 2, 3, 4, 5, 100};
    CHECK(std::abs(trimmedMean(data, 0.0) - (115.0 / 6.0)) < 1e-9);  // no trimming
    CHECK(std::abs(trimmedMean(data, 0.2) - 3.5) < 1e-9);            // drop {1, 100} -> {2,3,4,5}

    // Unsorted input gives the same result.
    std::vector<double> unsorted = {100, 1, 4, 2, 5, 3};
    CHECK(std::abs(trimmedMean(unsorted, 0.2) - 3.5) < 1e-9);

    bool threw = false;
    try {
        trimmedMean({1, 2, 3, 4}, 0.5);  // 2*floor(4*0.5) == 4 >= 4 -> nothing left
    } catch (const std::invalid_argument&) {
        threw = true;
    }
    CHECK(threw);

    threw = false;
    try {
        trimmedMean({1, 2, 3}, -0.1);
    } catch (const std::invalid_argument&) {
        threw = true;
    }
    CHECK(threw);

    threw = false;
    try {
        trimmedMean({1, 2, 3}, 1.0);
    } catch (const std::invalid_argument&) {
        threw = true;
    }
    CHECK(threw);

    threw = false;
    try {
        trimmedMean({}, 0.0);
    } catch (const std::invalid_argument&) {
        threw = true;
    }
    CHECK(threw);
}

// --- amdahlSpeedup / gustafsonSpeedup -----------------------------------------------------------------

TEST(SpeedupLaws) {
    CHECK(std::abs(amdahlSpeedup(0.0, 4) - 1.0) < 1e-9);
    CHECK(std::abs(amdahlSpeedup(1.0, 4) - 4.0) < 1e-9);
    CHECK(std::abs(amdahlSpeedup(0.5, 4) - 1.6) < 1e-9);
    CHECK(std::abs(amdahlSpeedup(0.9, 10) - (1.0 / 0.19)) < 1e-9);
    CHECK(std::abs(amdahlSpeedup(0.75, 1) - 1.0) < 1e-9);  // 1 processor -> no speedup, any p

    CHECK(std::abs(gustafsonSpeedup(0.0, 4) - 1.0) < 1e-9);
    CHECK(std::abs(gustafsonSpeedup(1.0, 4) - 4.0) < 1e-9);
    CHECK(std::abs(gustafsonSpeedup(0.5, 4) - 2.5) < 1e-9);
    CHECK(std::abs(gustafsonSpeedup(0.9, 10) - 9.1) < 1e-9);

    bool threw = false;
    try {
        amdahlSpeedup(1.5, 4);
    } catch (const std::invalid_argument&) {
        threw = true;
    }
    CHECK(threw);

    threw = false;
    try {
        amdahlSpeedup(0.5, 0);
    } catch (const std::invalid_argument&) {
        threw = true;
    }
    CHECK(threw);

    threw = false;
    try {
        gustafsonSpeedup(-0.1, 4);
    } catch (const std::invalid_argument&) {
        threw = true;
    }
    CHECK(threw);
}

// --- isSignificantSpeedup -----------------------------------------------------------------------------

TEST(SignificantSpeedupDetection) {
    // Tight distributions, clearly non-overlapping 95% CIs -> significant.
    BenchmarkStats before1{100, 100.0, 100.0, 10.0, 70.0, 130.0};
    BenchmarkStats after1{100, 90.0, 90.0, 10.0, 60.0, 120.0};
    CHECK(isSignificantSpeedup(before1, after1) == true);

    // Small sample, wide stddev -> CIs overlap despite a lower mean.
    BenchmarkStats before2{4, 100.0, 100.0, 20.0, 50.0, 150.0};
    BenchmarkStats after2{4, 95.0, 95.0, 20.0, 50.0, 150.0};
    CHECK(isSignificantSpeedup(before2, after2) == false);

    // after is actually slower -> false regardless of stddev.
    BenchmarkStats before3{100, 50.0, 50.0, 1.0, 45.0, 55.0};
    BenchmarkStats after3{100, 55.0, 55.0, 1.0, 45.0, 65.0};
    CHECK(isSignificantSpeedup(before3, after3) == false);
}

TEST_MAIN()
