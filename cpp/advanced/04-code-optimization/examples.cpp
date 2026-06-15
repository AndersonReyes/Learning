#include <array>
#include <chrono>
#include <cmath>
#include <iostream>
#include <unordered_map>
#include <vector>

// Topic 24 (Advanced 04): Code-Level Optimization Techniques
//
// Different illustrative examples than exercise.h's branchlessAbs/Min/Max,
// countSetBits/reverseBits, computeStats, StepCounter, and sortNetwork4 --
// same concepts, different code, so the exercises stay unspoiled.

using Clock = std::chrono::steady_clock;

// --- Branchless sign and clamp ---------------------------------------------------------------

int branchlessSign(int x) {
    // (x > 0) and (x < 0) are each 0 or 1 -- no control flow depends on x's value.
    return (x > 0) - (x < 0);
}

int bMin(int a, int b) { return b ^ ((a ^ b) & -static_cast<int>(a < b)); }
int bMax(int a, int b) { return a ^ ((a ^ b) & -static_cast<int>(a < b)); }

int branchlessClamp(int x, int lo, int hi) { return bMax(lo, bMin(x, hi)); }

// --- Bit tricks: power-of-two checks -----------------------------------------------------------

bool isPowerOfTwo(unsigned int x) { return x != 0 && (x & (x - 1)) == 0; }

// Rounds x up to the next power of two (classic "fill from the top bit down" trick).
unsigned int nextPowerOfTwo(unsigned int x) {
    if (x == 0) return 1;
    --x;
    x |= x >> 1;
    x |= x >> 2;
    x |= x >> 4;
    x |= x >> 8;
    x |= x >> 16;
    return x + 1;
}

// --- Loop fusion: fused vs separate passes -----------------------------------------------------

struct Summary {
    double sum;
    int positiveCount;
    double maxAbs;
};

Summary fusedSummary(const std::vector<double>& v) {
    Summary s{0.0, 0, 0.0};
    for (double x : v) {
        s.sum += x;
        if (x > 0) ++s.positiveCount;
        double a = x < 0 ? -x : x;
        if (a > s.maxAbs) s.maxAbs = a;
    }
    return s;
}

Summary separateSummary(const std::vector<double>& v) {
    double sum = 0.0;
    for (double x : v) sum += x;

    int positiveCount = 0;
    for (double x : v) {
        if (x > 0) ++positiveCount;
    }

    double maxAbs = 0.0;
    for (double x : v) {
        double a = x < 0 ? -x : x;
        if (a > maxAbs) maxAbs = a;
    }

    return {sum, positiveCount, maxAbs};
}

// --- Memoization: naive vs cached Fibonacci call counts -------------------------------------------

long long naiveCalls = 0;

long long fibNaive(int n) {
    ++naiveCalls;
    if (n < 2) return n;
    return fibNaive(n - 1) + fibNaive(n - 2);
}

long long memoCalls = 0;
std::unordered_map<int, long long> fibCache;

long long fibMemo(int n) {
    ++memoCalls;
    if (n < 2) return n;
    if (auto it = fibCache.find(n); it != fibCache.end()) return it->second;
    long long result = fibMemo(n - 1) + fibMemo(n - 2);
    fibCache[n] = result;
    return result;
}

// --- Sorting network for 3 elements ---------------------------------------------------------------

std::array<int, 3> sortNetwork3(std::array<int, 3> a) {
    auto cswap = [](int& x, int& y) {
        int lo = bMin(x, y);
        int hi = bMax(x, y);
        x = lo;
        y = hi;
    };
    cswap(a[0], a[1]);
    cswap(a[1], a[2]);
    cswap(a[0], a[1]);
    return a;
}

int main() {
    std::cout << "-- branchless sign and clamp --\n";
    {
        for (int x : {-5, 0, 7}) std::cout << "  branchlessSign(" << x << ") = " << branchlessSign(x) << "\n";
        for (int x : {-10, 5, 100}) {
            std::cout << "  branchlessClamp(" << x << ", 0, 10) = " << branchlessClamp(x, 0, 10) << "\n";
        }
    }

    std::cout << "\n-- bit tricks: power-of-two checks --\n";
    {
        for (unsigned int x : {0u, 1u, 2u, 3u, 16u, 17u, 1023u}) {
            std::cout << "  isPowerOfTwo(" << x << ") = " << std::boolalpha << isPowerOfTwo(x)
                      << ", nextPowerOfTwo(" << x << ") = " << nextPowerOfTwo(x) << "\n";
        }
    }

    std::cout << "\n-- loop fusion: fused vs separate passes --\n";
    {
        constexpr size_t N = 5'000'000;
        std::vector<double> v(N);
        for (size_t i = 0; i < N; ++i) v[i] = std::sin(static_cast<double>(i)) * 100.0;

        auto t0 = Clock::now();
        Summary fused = fusedSummary(v);
        auto t1 = Clock::now();
        Summary separate = separateSummary(v);
        auto t2 = Clock::now();

        auto fusedMicros = std::chrono::duration_cast<std::chrono::microseconds>(t1 - t0).count();
        auto separateMicros = std::chrono::duration_cast<std::chrono::microseconds>(t2 - t1).count();

        std::cout << "  fused:    sum=" << fused.sum << " positives=" << fused.positiveCount
                  << " maxAbs=" << fused.maxAbs << " (" << fusedMicros << " us)\n";
        std::cout << "  separate: sum=" << separate.sum << " positives=" << separate.positiveCount
                  << " maxAbs=" << separate.maxAbs << " (" << separateMicros << " us)\n";
    }

    std::cout << "\n-- memoization: naive vs cached Fibonacci call counts --\n";
    {
        constexpr int N = 25;
        long long naiveResult = fibNaive(N);
        std::cout << "  fibNaive(" << N << ") = " << naiveResult << " in " << naiveCalls << " calls\n";

        long long memoResult = fibMemo(N);
        std::cout << "  fibMemo(" << N << ")  = " << memoResult << " in " << memoCalls << " calls\n";
    }

    std::cout << "\n-- sorting network for 3 elements --\n";
    {
        for (auto triple : {std::array<int, 3>{3, 2, 1}, std::array<int, 3>{1, 3, 2},
                             std::array<int, 3>{2, 1, 3}, std::array<int, 3>{5, 5, -1}}) {
            auto sorted = sortNetwork3(triple);
            std::cout << "  sortNetwork3({" << triple[0] << ", " << triple[1] << ", " << triple[2]
                      << "}) = {" << sorted[0] << ", " << sorted[1] << ", " << sorted[2] << "}\n";
        }
    }
}
