#include "exercise.h"

#include <cmath>
#include <cstdint>
#include <stdexcept>
#include <vector>

#include "../../testing.h"

// --- DirectMappedCache -----------------------------------------------------------------------

TEST(DirectMappedCacheHitsAndMisses) {
    // 2 lines x 4 bytes (matches the worked example in exercise.h).
    DirectMappedCache c(2, 4);
    CHECK(c.access(0) == false);   // miss; block 0 -> line 0, tag 0
    CHECK(c.access(4) == false);   // miss; block 1 -> line 1, tag 0
    CHECK(c.access(8) == false);   // miss; block 2 -> line 0, tag 1 != 0 -> evict
    CHECK(c.access(0) == false);   // miss; line 0 holds tag 1, want tag 0 -> evict
    CHECK(c.access(0) == true);    // hit; line 0 now holds tag 0
    CHECK_EQ(c.hits(), static_cast<size_t>(1));
    CHECK_EQ(c.misses(), static_cast<size_t>(4));

    // 3 lines x 5 bytes -- non-power-of-two sizes.
    DirectMappedCache c2(3, 5);
    CHECK(c2.access(0) == false);   // block 0 -> line 0, tag 0
    CHECK(c2.access(5) == false);   // block 1 -> line 1, tag 0
    CHECK(c2.access(10) == false);  // block 2 -> line 2, tag 0
    CHECK(c2.access(15) == false);  // block 3 -> line 0, tag 1 != 0 -> evict
    CHECK(c2.access(0) == false);   // block 0 -> line 0, tag 0; line 0 has tag 1 -> evict
    CHECK(c2.access(0) == true);    // line 0 now holds tag 0 -> hit
    CHECK(c2.access(7) == true);    // block 1 -> line 1, tag 0; line 1 still holds tag 0 -> hit
    CHECK(c2.access(12) == true);   // block 2 -> line 2, tag 0; line 2 still holds tag 0 -> hit
    CHECK_EQ(c2.hits(), static_cast<size_t>(3));
    CHECK_EQ(c2.misses(), static_cast<size_t>(5));
}

// --- Arena -------------------------------------------------------------------------------------

TEST(ArenaAllocatesAlignedAndResets) {
    Arena a(24);
    CHECK_EQ(a.capacity(), static_cast<size_t>(24));
    CHECK_EQ(a.bytesUsed(), static_cast<size_t>(0));

    void* p1 = a.allocate(8, 8);  // offset 0 -> 8
    CHECK_EQ(a.bytesUsed(), static_cast<size_t>(8));

    void* p2 = a.allocate(3, 1);  // offset 8 -> 11
    CHECK_EQ(a.bytesUsed(), static_cast<size_t>(11));
    CHECK_EQ(reinterpret_cast<uintptr_t>(p2) - reinterpret_cast<uintptr_t>(p1),
             static_cast<uintptr_t>(8));

    void* p3 = a.allocate(8, 8);  // pad 11 -> 16, then -> 24
    CHECK_EQ(a.bytesUsed(), static_cast<size_t>(24));
    CHECK_EQ(reinterpret_cast<uintptr_t>(p3) - reinterpret_cast<uintptr_t>(p1),
             static_cast<uintptr_t>(16));

    bool threw = false;
    try {
        a.allocate(1, 1);  // 24 + 1 > 24 -> bad_alloc
    } catch (const std::bad_alloc&) {
        threw = true;
    }
    CHECK(threw);
    CHECK_EQ(a.bytesUsed(), static_cast<size_t>(24));  // unchanged by the failed allocation

    a.reset();
    CHECK_EQ(a.bytesUsed(), static_cast<size_t>(0));
    CHECK_EQ(a.capacity(), static_cast<size_t>(24));  // capacity unaffected by reset

    void* p1Again = a.allocate(8, 8);  // bump pointer restarted from the same buffer
    CHECK(p1Again == p1);
}

// --- toSoA / totalKineticEnergy -----------------------------------------------------------------

TEST(AoSToSoAAndKineticEnergy) {
    std::vector<ParticleAoS> particles = {
        {0, 0, 0, 1, 0, 0, 2},
        {1, 1, 1, 0, 3, 4, 1},
    };

    ParticlesSoA soa = toSoA(particles);
    CHECK(soa.x == (std::vector<double>{0, 1}));
    CHECK(soa.y == (std::vector<double>{0, 1}));
    CHECK(soa.z == (std::vector<double>{0, 1}));
    CHECK(soa.vx == (std::vector<double>{1, 0}));
    CHECK(soa.vy == (std::vector<double>{0, 3}));
    CHECK(soa.vz == (std::vector<double>{0, 4}));
    CHECK(soa.mass == (std::vector<double>{2, 1}));

    // 0.5*2*(1^2 + 0^2 + 0^2) + 0.5*1*(0^2 + 3^2 + 4^2) == 1.0 + 12.5 == 13.5
    CHECK(std::abs(totalKineticEnergy(soa) - 13.5) < 1e-9);

    ParticlesSoA empty;
    CHECK(std::abs(totalKineticEnergy(empty) - 0.0) < 1e-9);
}

// --- transposeBlocked ----------------------------------------------------------------------------

TEST(TransposeBlockedVariousBlockSizes) {
    std::vector<std::vector<int>> m1 = {{1, 2}, {3, 4}, {5, 6}};
    std::vector<std::vector<int>> expected1 = {{1, 3, 5}, {2, 4, 6}};

    std::vector<std::vector<int>> m2 = {{1, 2, 3, 4}, {5, 6, 7, 8}};
    std::vector<std::vector<int>> expected2 = {{1, 5}, {2, 6}, {3, 7}, {4, 8}};

    for (size_t blockSize : std::vector<size_t>{0, 1, 2, 3, 5, 100}) {
        CHECK(transposeBlocked(m1, blockSize) == expected1);
        CHECK(transposeBlocked(m2, blockSize) == expected2);
    }

    // Empty matrix -> empty result.
    CHECK(transposeBlocked({}, 2) == (std::vector<std::vector<int>>{}));

    // Ragged matrix -> throws.
    std::vector<std::vector<int>> ragged = {{1, 2}, {3}};
    bool threw = false;
    try {
        transposeBlocked(ragged, 1);
    } catch (const std::invalid_argument&) {
        threw = true;
    }
    CHECK(threw);
}

// --- multiplyBlocked ------------------------------------------------------------------------------

TEST(MultiplyBlockedVariousBlockSizes) {
    std::vector<std::vector<double>> A = {{1, 2, 3}, {4, 5, 6}};   // 2x3
    std::vector<std::vector<double>> B = {{7, 8}, {9, 10}, {11, 12}};  // 3x2
    std::vector<std::vector<double>> expected = {{58, 64}, {139, 154}};

    for (size_t blockSize : std::vector<size_t>{0, 1, 2, 3, 5, 100}) {
        auto result = multiplyBlocked(A, B, blockSize);
        CHECK_EQ(result.size(), expected.size());
        for (size_t i = 0; i < expected.size(); ++i) {
            CHECK_EQ(result[i].size(), expected[i].size());
            for (size_t j = 0; j < expected[i].size(); ++j) {
                CHECK(std::abs(result[i][j] - expected[i][j]) < 1e-9);
            }
        }
    }

    // Dimension mismatch: A is 2x3, but this B is 2x2 (rows != A's columns).
    std::vector<std::vector<double>> bMismatch = {{1, 2}, {3, 4}};
    bool threw = false;
    try {
        multiplyBlocked(A, bMismatch, 1);
    } catch (const std::invalid_argument&) {
        threw = true;
    }
    CHECK(threw);

    // Ragged A -> throws.
    std::vector<std::vector<double>> ragged = {{1, 2}, {3}};
    threw = false;
    try {
        multiplyBlocked(ragged, B, 1);
    } catch (const std::invalid_argument&) {
        threw = true;
    }
    CHECK(threw);
}

TEST_MAIN()
