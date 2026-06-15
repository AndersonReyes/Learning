#pragma once

#include <atomic>
#include <functional>
#include <future>
#include <memory>
#include <mutex>
#include <numbers>
#include <stdexcept>
#include <string>
#include <thread>
#include <unordered_map>
#include <vector>

// Topic 22 (Advanced 02): Error Handling, Smart Pointers & Concurrency
//
// Five exercises: custom exception hierarchies + exception safety,
// std::unique_ptr + polymorphism, std::shared_ptr/std::weak_ptr caching,
// std::thread + synchronization, and std::async/std::future + cross-thread
// exception propagation. Class member functions are declared here and
// defined in exercise.cpp. Stub bodies throw std::logic_error("not
// implemented").

// --- InsufficientFundsError: shared test infrastructure (not one of the 5 exercises) -----------
//
// A custom exception type deriving from std::runtime_error, carrying the
// requested and available amounts as members (in addition to the inherited
// what() message). BankAccount::withdraw (exercise 1) throws this when
// `amount > balance()`. Fully implemented here -- it's infrastructure the
// exercise USES, not something to implement.
class InsufficientFundsError : public std::runtime_error {
public:
    InsufficientFundsError(double requested, double available)
        : std::runtime_error("insufficient funds: requested " + std::to_string(requested) +
                              ", available " + std::to_string(available)),
          requested_(requested),
          available_(available) {}

    double requested() const { return requested_; }
    double available() const { return available_; }

private:
    double requested_;
    double available_;
};

// --- BankAccount: custom exceptions + exception safety ------------------------------------------
//
// A simple account with a balance, deposit, and withdraw. Both deposit and
// withdraw validate `amount`, and withdraw additionally checks funds:
//
//   BankAccount(initialBalance) -- throws std::invalid_argument if
//                                   initialBalance < 0.
//   balance() const             -- current balance.
//   deposit(amount)             -- throws std::invalid_argument if
//                                   amount <= 0 (no effect on balance).
//                                   Otherwise balance() += amount.
//   withdraw(amount)            -- throws std::invalid_argument if
//                                   amount <= 0 (no effect on balance).
//                                   Throws InsufficientFundsError(amount,
//                                   balance()) if amount > balance() (no
//                                   effect on balance() -- strong exception
//                                   guarantee: a failed withdraw leaves the
//                                   account completely unchanged). Otherwise
//                                   balance() -= amount.
//
// Example: BankAccount acc(100.0);
// acc.deposit(50.0);   acc.balance() == 150.0
// acc.withdraw(30.0);  acc.balance() == 120.0
// acc.withdraw(1000.0) throws InsufficientFundsError with
//   .requested() == 1000.0 and .available() == 120.0; acc.balance() is
//   still 120.0 afterward.
// acc.deposit(-5.0) and acc.withdraw(0.0) throw std::invalid_argument;
//   balance() unchanged by either.
class BankAccount {
public:
    explicit BankAccount(double initialBalance);
    double balance() const;
    void deposit(double amount);
    void withdraw(double amount);

private:
    double balance_ = 0.0;
};

// --- Shape / Circle / Rectangle / totalArea / extractLargest: unique_ptr + polymorphism ----------
//
// A small shape hierarchy owned via std::unique_ptr<Shape>.
//
//   Circle(radius)             -- area() == pi * radius^2
//   Rectangle(width, height)   -- area() == width * height
//
//   totalArea(shapes) -- sum of area() over every non-null element of
//                         `shapes` (nullptr elements, e.g. left behind by
//                         extractLargest, contribute 0). 0 for an empty
//                         vector.
//
//   extractLargest(shapes) -- finds the element with the largest area()
//                              (ties broken by lowest index), moves it OUT
//                              of `shapes` (that slot becomes nullptr, the
//                              vector's size is unchanged) and returns it.
//                              Returns nullptr, and leaves `shapes`
//                              unchanged, if `shapes` is empty or contains
//                              only nullptrs.
//
// Example: shapes = {Circle(1), Rectangle(3,4), Rectangle(2,6)}
// totalArea(shapes) == pi*1^2 + 12 + 12 == pi + 24
// auto largest = extractLargest(shapes);  // Rectangle(3,4) (index 1, first
//                                          // of the tied area-12 shapes)
// shapes[1] == nullptr; shapes.size() == 3
// totalArea(shapes) == pi + 12  (the remaining Circle(1) + Rectangle(2,6))
class Shape {
public:
    virtual ~Shape() = default;
    virtual double area() const = 0;
};

class Circle : public Shape {
public:
    explicit Circle(double radius);
    double area() const override;

private:
    double radius_;
};

class Rectangle : public Shape {
public:
    Rectangle(double width, double height);
    double area() const override;

private:
    double width_;
    double height_;
};

double totalArea(const std::vector<std::unique_ptr<Shape>>& shapes);

std::unique_ptr<Shape> extractLargest(std::vector<std::unique_ptr<Shape>>& shapes);

// --- WeakCache: shared_ptr/weak_ptr lifetime-aware caching ----------------------------------------
//
// Caches std::string values by key, but holds only std::weak_ptr to them --
// so a cached value is freed as soon as nothing outside the cache still
// holds a shared_ptr to it (no manual eviction needed).
//
//   get(key) -- if the cache has a LIVE entry for `key` (a weak_ptr that
//                hasn't expired), returns a shared_ptr to that same string.
//                Otherwise (no entry, or an expired weak_ptr) constructs a
//                new shared_ptr<std::string> holding a copy of `key`, stores
//                a weak_ptr to it (replacing any expired entry), and returns
//                it.
//   size() const -- number of currently-LIVE entries (does not count
//                    expired weak_ptrs, and does not erase them either --
//                    a pure count).
//
// Example: WeakCache cache;
// auto a = cache.get("x");           // new entry; *a == "x"
// auto b = cache.get("x");           // same object: a.get() == b.get()
// cache.size() == 1
// a.reset(); b.reset();              // last owners gone -> entry expires
// cache.size() == 0
// auto c = cache.get("x");           // expired -> brand-new object
// cache.size() == 1
class WeakCache {
public:
    std::shared_ptr<std::string> get(const std::string& key);
    size_t size() const;

private:
    std::unordered_map<std::string, std::weak_ptr<std::string>> cache_;
};

// --- parallelSum: std::thread + synchronization -----------------------------------------------
//
// Computes the sum of `v` using `numThreads` worker threads (numThreads ==
// 0 is treated as 1). `v` is split into `numThreads` contiguous chunks as
// evenly as possible (earlier chunks may have one extra element); each
// chunk's partial sum is computed on its own std::thread, and the partial
// sums are combined into the final total using thread-safe synchronization
// (e.g. std::mutex or std::atomic) -- not a data race. If `numThreads >
// v.size()`, some threads handle empty chunks (contribute 0). Returns 0 for
// an empty `v`, regardless of numThreads.
//
// Example: parallelSum({1,2,3,4,5}, 2) == 15  (chunks {1,2,3} and {4,5}, or
// any other even split -- the total is the same either way)
// parallelSum({}, 4) == 0
// parallelSum({42}, 8) == 42  (7 of the 8 threads handle an empty chunk)
long long parallelSum(const std::vector<int>& v, unsigned numThreads);

// --- firstSuccessfulResult: std::async/std::future + exception propagation -----------------------
//
// Runs every callable in `tasks` via std::async(std::launch::async, ...),
// producing one std::future<int> per task (all tasks are started). Then,
// in order, calls future::get() on each: returns the result of the FIRST
// task whose future::get() does not throw. If every task throws, rethrows
// whatever exception the LAST task's future::get() threw (std::future
// propagates an exception thrown inside the async task out of get()).
// Throws std::invalid_argument("no tasks") if `tasks` is empty (without
// launching anything).
//
// Example: firstSuccessfulResult({[]{ throw std::runtime_error("x"); return 0; },
//                                  []{ return 42; }}) == 42
// firstSuccessfulResult({[]{ return 1; }, []{ return 2; }}) == 1
// firstSuccessfulResult({[]{ throw std::runtime_error("a"); return 0; },
//                         []{ throw std::logic_error("b"); return 0; }})
//   rethrows std::logic_error("b")
// firstSuccessfulResult({}) throws std::invalid_argument
int firstSuccessfulResult(std::vector<std::function<int()>> tasks);
