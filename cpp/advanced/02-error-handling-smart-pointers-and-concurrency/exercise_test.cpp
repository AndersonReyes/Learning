#include <cmath>
#include <numbers>
#include <numeric>
#include <stdexcept>
#include <string>
#include <vector>

#include "exercise.h"
#include "../../testing.h"

TEST(BankAccountThrowsAndStaysConsistent) {
    BankAccount acc(100.0);
    CHECK(acc.balance() == 100.0);

    acc.deposit(50.0);
    CHECK(acc.balance() == 150.0);

    acc.withdraw(30.0);
    CHECK(acc.balance() == 120.0);

    // Invalid deposit: balance unchanged.
    bool threw = false;
    try {
        acc.deposit(-5.0);
    } catch (const std::invalid_argument&) {
        threw = true;
    }
    CHECK(threw);
    CHECK(acc.balance() == 120.0);

    // Invalid withdraw amount: balance unchanged.
    threw = false;
    try {
        acc.withdraw(0.0);
    } catch (const std::invalid_argument&) {
        threw = true;
    }
    CHECK(threw);
    CHECK(acc.balance() == 120.0);

    // Insufficient funds: strong exception guarantee (balance unchanged).
    threw = false;
    try {
        acc.withdraw(1000.0);
    } catch (const InsufficientFundsError& e) {
        threw = true;
        CHECK(e.requested() == 1000.0);
        CHECK(e.available() == 120.0);
    }
    CHECK(threw);
    CHECK(acc.balance() == 120.0);

    // InsufficientFundsError is catchable polymorphically as std::runtime_error.
    BankAccount acc2(10.0);
    try {
        acc2.withdraw(20.0);
        CHECK(false);  // must not reach here
    } catch (const std::runtime_error& e) {
        CHECK(std::string(e.what()).find("insufficient") != std::string::npos);
    }

    // Negative initial balance rejected.
    threw = false;
    try {
        BankAccount bad(-1.0);
        (void)bad;
    } catch (const std::invalid_argument&) {
        threw = true;
    }
    CHECK(threw);
}

TEST(ShapeHierarchyUniquePtrOwnership) {
    std::vector<std::unique_ptr<Shape>> shapes;
    shapes.push_back(std::make_unique<Circle>(1.0));          // area = pi
    shapes.push_back(std::make_unique<Rectangle>(3.0, 4.0));  // area = 12
    shapes.push_back(std::make_unique<Rectangle>(2.0, 6.0));  // area = 12 (tie)

    double total = totalArea(shapes);
    CHECK(std::abs(total - (std::numbers::pi + 24.0)) < 1e-9);

    auto largest = extractLargest(shapes);
    CHECK(largest != nullptr);
    CHECK(std::abs(largest->area() - 12.0) < 1e-9);

    // Tie broken by lowest index -> index 1 (Rectangle(3,4)) extracted.
    CHECK(shapes[1] == nullptr);
    CHECK(shapes.size() == 3);

    double remaining = totalArea(shapes);
    CHECK(std::abs(remaining - (std::numbers::pi + 12.0)) < 1e-9);

    // Empty vector.
    std::vector<std::unique_ptr<Shape>> empty;
    CHECK(totalArea(empty) == 0.0);
    CHECK(extractLargest(empty) == nullptr);

    // Vector of only nullptrs.
    std::vector<std::unique_ptr<Shape>> onlyNull;
    onlyNull.push_back(nullptr);
    onlyNull.push_back(nullptr);
    CHECK(totalArea(onlyNull) == 0.0);
    CHECK(extractLargest(onlyNull) == nullptr);
}

TEST(WeakCacheTracksLiveEntries) {
    WeakCache cache;
    CHECK(cache.size() == 0);

    auto a = cache.get("x");
    CHECK(*a == "x");
    CHECK(cache.size() == 1);

    auto b = cache.get("x");
    CHECK(a.get() == b.get());
    CHECK(a.use_count() == 2);
    CHECK(cache.size() == 1);

    auto c = cache.get("y");
    CHECK(*c == "y");
    CHECK(cache.size() == 2);

    a.reset();
    b.reset();
    CHECK(cache.size() == 1);  // only "y" still live

    auto d = cache.get("x");  // expired -> brand-new object
    CHECK(*d == "x");
    CHECK(d.use_count() == 1);
    CHECK(cache.size() == 2);
}

TEST(ParallelSumMatchesSerialSum) {
    std::vector<int> empty;
    CHECK(parallelSum(empty, 4) == 0);
    CHECK(parallelSum(empty, 0) == 0);

    std::vector<int> v1 = {42};
    CHECK(parallelSum(v1, 8) == 42);  // 7 of 8 threads handle an empty chunk

    std::vector<int> v2 = {1, 2, 3, 4, 5};
    long long expected2 = std::accumulate(v2.begin(), v2.end(), 0LL);
    CHECK(parallelSum(v2, 1) == expected2);
    CHECK(parallelSum(v2, 2) == expected2);
    CHECK(parallelSum(v2, 0) == expected2);  // 0 treated as 1

    std::vector<int> v3;
    for (int i = 1; i <= 100; ++i) v3.push_back(i);
    long long expected3 = std::accumulate(v3.begin(), v3.end(), 0LL);
    CHECK(parallelSum(v3, 4) == expected3);
    CHECK(parallelSum(v3, 7) == expected3);
    CHECK(parallelSum(v3, 1000) == expected3);  // far more threads than elements
}

TEST(FirstSuccessfulResultSelectsAndPropagates) {
    // No tasks -> std::invalid_argument, nothing launched.
    bool threw = false;
    try {
        firstSuccessfulResult({});
    } catch (const std::invalid_argument&) {
        threw = true;
    }
    CHECK(threw);

    // First task throws, second succeeds -> returns the second's result.
    int r1 = firstSuccessfulResult({
        []() -> int { throw std::runtime_error("boom"); },
        []() { return 42; },
    });
    CHECK(r1 == 42);

    // Both succeed -> returns the FIRST one's result.
    int r2 = firstSuccessfulResult({
        []() { return 1; },
        []() { return 2; },
    });
    CHECK(r2 == 1);

    // All throw -> rethrows the LAST task's exception.
    bool caughtLogic = false;
    try {
        firstSuccessfulResult({
            []() -> int { throw std::runtime_error("a"); },
            []() -> int { throw std::logic_error("b"); },
        });
    } catch (const std::logic_error& e) {
        caughtLogic = true;
        CHECK_EQ(std::string(e.what()), std::string("b"));
    }
    CHECK(caughtLogic);
}

TEST_MAIN()
