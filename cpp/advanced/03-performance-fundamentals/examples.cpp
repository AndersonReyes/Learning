#include <chrono>
#include <cstddef>
#include <cstdint>
#include <iostream>
#include <memory>
#include <vector>

// Topic 23 (Advanced 03): Performance Fundamentals: Architecture & Memory Hierarchy
//
// Different illustrative examples than exercise.h's DirectMappedCache /
// Arena / ParticleAoS-ParticlesSoA / transposeBlocked / multiplyBlocked --
// same concepts, different code, so the exercises stay unspoiled. Several
// sections time themselves with <chrono> -- actual numbers depend on the
// machine AND on optimization level. At -O0, loop/bookkeeping overhead
// dominates and can hide or even invert the expected gap; build with -O2 to
// see cache-friendly access patterns actually win:
//   g++ -std=c++20 -Wall -Wextra -O2 -o /tmp/ex examples.cpp && /tmp/ex

using Clock = std::chrono::steady_clock;

// --- Locality: row-major vs column-major traversal ----------------------------------------------

long long sumRowMajor(const std::vector<std::vector<int>>& m) {
    long long total = 0;
    for (size_t i = 0; i < m.size(); ++i) {
        for (size_t j = 0; j < m[i].size(); ++j) total += m[i][j];
    }
    return total;
}

long long sumColumnMajor(const std::vector<std::vector<int>>& m) {
    long long total = 0;
    size_t rows = m.size(), cols = m[0].size();
    for (size_t j = 0; j < cols; ++j) {
        for (size_t i = 0; i < rows; ++i) total += m[i][j];
    }
    return total;
}

// --- Alignment and padding -----------------------------------------------------------------------

// Fields ordered worst-to-best by size -> the compiler pads `a` and `c` out
// to `double`'s 8-byte alignment.
struct BadLayout {
    char a;
    double b;
    char c;
};

// Largest-alignment member first -> `a` and `c` pack into b's trailing
// padding instead of each needing their own.
struct GoodLayout {
    double b;
    char a;
    char c;
};

// --- AoS vs SoA: summing one field -----------------------------------------------------------------

struct PointAoS {
    double x, y, z;
};

struct PointsSoA {
    std::vector<double> x, y, z;
};

double sumXAoS(const std::vector<PointAoS>& points) {
    double total = 0.0;
    for (const auto& p : points) total += p.x;
    return total;
}

double sumXSoA(const PointsSoA& points) {
    double total = 0.0;
    for (double x : points.x) total += x;
    return total;
}

// --- Cache blocking: naive vs blocked matrix transpose --------------------------------------------

std::vector<std::vector<double>> transposeNaive(const std::vector<std::vector<double>>& m) {
    size_t rows = m.size(), cols = m[0].size();
    std::vector<std::vector<double>> result(cols, std::vector<double>(rows));
    for (size_t i = 0; i < rows; ++i) {
        for (size_t j = 0; j < cols; ++j) result[j][i] = m[i][j];
    }
    return result;
}

std::vector<std::vector<double>> transposeTiled(const std::vector<std::vector<double>>& m,
                                                  size_t blockSize) {
    size_t rows = m.size(), cols = m[0].size();
    std::vector<std::vector<double>> result(cols, std::vector<double>(rows));
    for (size_t ii = 0; ii < rows; ii += blockSize) {
        for (size_t jj = 0; jj < cols; jj += blockSize) {
            size_t iMax = std::min(ii + blockSize, rows);
            size_t jMax = std::min(jj + blockSize, cols);
            for (size_t i = ii; i < iMax; ++i) {
                for (size_t j = jj; j < jMax; ++j) result[j][i] = m[i][j];
            }
        }
    }
    return result;
}

int main() {
    std::cout << "-- locality: row-major vs column-major traversal --\n";
    {
        constexpr size_t N = 1024;
        std::vector<std::vector<int>> m(N, std::vector<int>(N));
        for (size_t i = 0; i < N; ++i) {
            for (size_t j = 0; j < N; ++j) m[i][j] = static_cast<int>((i * 31 + j) % 100);
        }

        auto t0 = Clock::now();
        long long rowSum = sumRowMajor(m);
        auto t1 = Clock::now();
        long long colSum = sumColumnMajor(m);
        auto t2 = Clock::now();

        auto rowMicros = std::chrono::duration_cast<std::chrono::microseconds>(t1 - t0).count();
        auto colMicros = std::chrono::duration_cast<std::chrono::microseconds>(t2 - t1).count();

        std::cout << "  row-major sum:    " << rowSum << " (" << rowMicros << " us)\n";
        std::cout << "  column-major sum: " << colSum << " (" << colMicros << " us)\n";
        std::cout << "  same total: " << std::boolalpha << (rowSum == colSum) << "\n";
        std::cout << "  row-major reads each cache line once; column-major jumps "
                  << N * sizeof(int) << " bytes between consecutive accesses.\n";
    }

    std::cout << "\n-- alignment and padding --\n";
    {
        std::cout << "  sizeof(BadLayout)  = " << sizeof(BadLayout) << " (alignof "
                  << alignof(BadLayout) << ")\n";
        std::cout << "    offsetof a=" << offsetof(BadLayout, a) << " b=" << offsetof(BadLayout, b)
                  << " c=" << offsetof(BadLayout, c) << "\n";
        std::cout << "  sizeof(GoodLayout) = " << sizeof(GoodLayout) << " (alignof "
                  << alignof(GoodLayout) << ")\n";
        std::cout << "    offsetof b=" << offsetof(GoodLayout, b) << " a=" << offsetof(GoodLayout, a)
                  << " c=" << offsetof(GoodLayout, c) << "\n";

        std::cout << "  rounding offsets up to an alignment, via (off + align - 1) & ~(align - 1):\n";
        struct { size_t offset, alignment; } cases[] = {{0, 8}, {1, 8}, {8, 8}, {11, 8}, {17, 16}};
        for (auto [offset, alignment] : cases) {
            size_t rounded = (offset + alignment - 1) & ~(alignment - 1);
            std::cout << "    roundUp(" << offset << ", " << alignment << ") = " << rounded << "\n";
        }
    }

    std::cout << "\n-- AoS vs SoA: summing one field --\n";
    {
        constexpr size_t N = 2'000'000;
        std::vector<PointAoS> aos(N);
        PointsSoA soa;
        soa.x.resize(N);
        soa.y.resize(N);
        soa.z.resize(N);
        for (size_t i = 0; i < N; ++i) {
            double v = static_cast<double>(i % 1000);
            aos[i] = {v, v, v};
            soa.x[i] = v;
            soa.y[i] = v;
            soa.z[i] = v;
        }

        auto t0 = Clock::now();
        double aosSum = sumXAoS(aos);
        auto t1 = Clock::now();
        double soaSum = sumXSoA(soa);
        auto t2 = Clock::now();

        auto aosMicros = std::chrono::duration_cast<std::chrono::microseconds>(t1 - t0).count();
        auto soaMicros = std::chrono::duration_cast<std::chrono::microseconds>(t2 - t1).count();

        std::cout << "  AoS sum of x: " << aosSum << " (" << aosMicros << " us)\n";
        std::cout << "  SoA sum of x: " << soaSum << " (" << soaMicros << " us)\n";
        std::cout << "  AoS touches 24 bytes (x,y,z) per element to read 8; "
                  << "SoA touches only the 8 it needs.\n";
    }

    std::cout << "\n-- cache blocking: naive vs tiled matrix transpose --\n";
    {
        constexpr size_t N = 512;
        std::vector<std::vector<double>> m(N, std::vector<double>(N));
        for (size_t i = 0; i < N; ++i) {
            for (size_t j = 0; j < N; ++j) m[i][j] = static_cast<double>(i) * N + static_cast<double>(j);
        }

        auto t0 = Clock::now();
        auto naive = transposeNaive(m);
        auto t1 = Clock::now();
        auto tiled = transposeTiled(m, 32);
        auto t2 = Clock::now();

        auto naiveMicros = std::chrono::duration_cast<std::chrono::microseconds>(t1 - t0).count();
        auto tiledMicros = std::chrono::duration_cast<std::chrono::microseconds>(t2 - t1).count();

        std::cout << "  naive transpose: " << naiveMicros << " us\n";
        std::cout << "  tiled transpose (blockSize=32): " << tiledMicros << " us\n";
        std::cout << "  same result: " << std::boolalpha << (naive == tiled) << "\n";
    }

    std::cout << "\n-- bump allocation via std::align over a raw buffer --\n";
    {
        std::vector<unsigned char> buffer(64);
        void* ptr = buffer.data();
        size_t space = buffer.size();

        // Place an int (4-byte aligned) at the start.
        void* intSlot = std::align(alignof(int), sizeof(int), ptr, space);
        std::cout << "  int slot offset:    " << (static_cast<unsigned char*>(intSlot) - buffer.data())
                  << " (space left: " << space << ")\n";
        ptr = static_cast<unsigned char*>(intSlot) + sizeof(int);
        space -= sizeof(int);

        // Now place a double (8-byte aligned) -- std::align pads to the boundary.
        void* doubleSlot = std::align(alignof(double), sizeof(double), ptr, space);
        std::cout << "  double slot offset: "
                  << (static_cast<unsigned char*>(doubleSlot) - buffer.data())
                  << " (space left: " << space << ")\n";
    }
}
