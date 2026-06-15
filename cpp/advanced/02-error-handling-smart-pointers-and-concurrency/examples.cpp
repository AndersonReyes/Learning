#include <atomic>
#include <future>
#include <iostream>
#include <memory>
#include <mutex>
#include <stdexcept>
#include <string>
#include <thread>
#include <vector>

// Topic 22 (Advanced 02): Error Handling, Smart Pointers & Concurrency
//
// Different illustrative examples than exercise.h's BankAccount/Shape
// hierarchy/WeakCache/parallelSum/firstSuccessfulResult -- same concepts,
// different classes/functions, so the exercises stay unspoiled.
//
// Build with -pthread (needed for <thread>/<future> on most platforms):
//   g++ -std=c++20 -Wall -Wextra -pthread -o /tmp/ex examples.cpp && /tmp/ex

// --- Custom exception hierarchy + strong exception guarantee --------------------------------------

class StackOverflowError : public std::runtime_error {
public:
    explicit StackOverflowError(size_t capacity)
        : std::runtime_error("stack overflow: capacity " + std::to_string(capacity)) {}
};

class StackUnderflowError : public std::runtime_error {
public:
    StackUnderflowError() : std::runtime_error("stack underflow: pop from empty stack") {}
};

// A fixed-capacity stack. push/pop validate FIRST, so a thrown exception
// never leaves `data_` modified (strong exception guarantee).
template <typename T>
class BoundedStack {
public:
    explicit BoundedStack(size_t capacity) : capacity_(capacity) {}

    void push(T value) {
        if (data_.size() >= capacity_) throw StackOverflowError(capacity_);
        data_.push_back(std::move(value));
    }

    T pop() {
        if (data_.empty()) throw StackUnderflowError();
        T value = std::move(data_.back());
        data_.pop_back();
        return value;
    }

    size_t size() const { return data_.size(); }

private:
    std::vector<T> data_;
    size_t capacity_;
};

// --- std::unique_ptr + polymorphism: Animal hierarchy -----------------------------------------

class Animal {
public:
    virtual ~Animal() = default;
    virtual std::string speak() const = 0;
};

class Dog : public Animal {
public:
    std::string speak() const override { return "Woof!"; }
};

class Cat : public Animal {
public:
    std::string speak() const override { return "Meow!"; }
};

// Takes ownership of `a` (caller must std::move it in) and returns its
// speak() text -- demonstrates a unique_ptr passed BY VALUE (move-only).
std::string announce(std::unique_ptr<Animal> a) { return a->speak(); }

// --- std::shared_ptr / std::weak_ptr lifetime --------------------------------------------------

struct Resource {
    std::string name;
    explicit Resource(std::string n) : name(std::move(n)) {
        std::cout << "  Resource(" << name << ") created\n";
    }
    ~Resource() { std::cout << "  Resource(" << name << ") destroyed\n"; }
};

// --- std::thread + atomic/mutex -------------------------------------------------------------------

void incrementMany(std::atomic<int>& counter, int times) {
    for (int i = 0; i < times; ++i) ++counter;
}

void appendMany(std::vector<int>& shared, std::mutex& m, int start, int count) {
    for (int i = 0; i < count; ++i) {
        std::lock_guard<std::mutex> lock(m);
        shared.push_back(start + i);
    }
}

// --- std::async / std::future + exception propagation ---------------------------------------------

int riskyComputation(int x) {
    if (x < 0) throw std::invalid_argument("x must be non-negative, got " + std::to_string(x));
    return x * x;
}

int main() {
    std::cout << "-- custom exception hierarchy + strong exception guarantee --\n";
    {
        BoundedStack<int> s(3);
        s.push(1);
        s.push(2);
        s.push(3);
        std::cout << "size after 3 pushes: " << s.size() << "\n";

        try {
            s.push(4);  // capacity exceeded
        } catch (const StackOverflowError& e) {
            std::cout << "push(4) failed: " << e.what() << " (size still " << s.size() << ")\n";
        }

        while (s.size() > 0) std::cout << "pop() -> " << s.pop() << "\n";

        try {
            s.pop();  // empty
        } catch (const std::runtime_error& e) {  // caught via base class
            std::cout << "pop() on empty failed: " << e.what() << "\n";
        }
    }

    std::cout << "\n-- unique_ptr + polymorphism --\n";
    {
        std::vector<std::unique_ptr<Animal>> animals;
        animals.push_back(std::make_unique<Dog>());
        animals.push_back(std::make_unique<Cat>());
        for (const auto& a : animals) std::cout << "  " << a->speak() << "\n";

        // Move ownership of the Dog out of the vector into announce().
        std::cout << "announce(move(animals[0])) -> " << announce(std::move(animals[0])) << "\n";
        std::cout << "animals[0] is now " << (animals[0] == nullptr ? "nullptr" : "non-null") << "\n";
    }

    std::cout << "\n-- shared_ptr / weak_ptr lifetime --\n";
    {
        auto a = std::make_shared<Resource>("config");
        std::cout << "use_count after creation: " << a.use_count() << "\n";

        std::weak_ptr<Resource> w = a;
        {
            auto b = a;  // shared_ptr copy: use_count == 2
            std::cout << "use_count with b alive: " << a.use_count() << "\n";
        }  // b destroyed
        std::cout << "use_count after b destroyed: " << a.use_count() << "\n";

        if (auto locked = w.lock()) {
            std::cout << "weak_ptr still valid: " << locked->name << "\n";
        }

        a.reset();  // last shared_ptr gone -> Resource destroyed
        std::cout << "w.expired() after a.reset(): " << std::boolalpha << w.expired() << "\n";
        std::cout << "w.lock() is " << (w.lock() == nullptr ? "nullptr" : "non-null") << "\n";
    }

    std::cout << "\n-- std::thread + std::atomic --\n";
    {
        std::atomic<int> counter{0};
        std::vector<std::thread> threads;
        for (int i = 0; i < 4; ++i) threads.emplace_back(incrementMany, std::ref(counter), 1000);
        for (auto& t : threads) t.join();
        std::cout << "counter after 4 threads x 1000 increments: " << counter.load() << "\n";
    }

    std::cout << "\n-- std::thread + std::mutex --\n";
    {
        std::vector<int> shared;
        std::mutex m;
        std::vector<std::thread> threads;
        threads.emplace_back(appendMany, std::ref(shared), std::ref(m), 0, 50);
        threads.emplace_back(appendMany, std::ref(shared), std::ref(m), 100, 50);
        for (auto& t : threads) t.join();
        std::cout << "shared.size() after 2 threads x 50 appends: " << shared.size() << "\n";
    }

    std::cout << "\n-- std::async / std::future + exception propagation --\n";
    {
        std::vector<std::future<int>> futures;
        for (int x : {3, -1, 5}) {
            futures.push_back(std::async(std::launch::async, riskyComputation, x));
        }
        for (auto& f : futures) {
            try {
                std::cout << "result: " << f.get() << "\n";
            } catch (const std::invalid_argument& e) {
                std::cout << "caught from future: " << e.what() << "\n";
            }
        }
    }
}
