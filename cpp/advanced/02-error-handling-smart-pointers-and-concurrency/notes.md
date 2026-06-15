# Advanced 02: Error Handling, Smart Pointers & Concurrency

## Exception handling basics

`throw` raises an exception; `try`/`catch` handles it. Catch blocks are
tried **in order**, and a derived-type exception matches a base-type catch
(`catch (const std::exception&)` catches almost everything standard).

```cpp
try {
    riskyOperation();
} catch (const std::invalid_argument& e) {
    // most-derived/specific first
} catch (const std::exception& e) {
    // fallback: anything else derived from std::exception
}
```

- `<stdexcept>` provides a small hierarchy: `std::logic_error` (programmer
  errors -- `std::invalid_argument`, `std::out_of_range`, `std::domain_error`,
  ...) and `std::runtime_error` (runtime conditions --
  `std::overflow_error`, `std::range_error`, ...). Both derive from
  `std::exception` but **not from each other** -- catching `std::logic_error&`
  does NOT catch a `std::runtime_error`.
- Custom exceptions derive from `std::runtime_error`/`std::logic_error` (or
  `std::exception` directly) and pass a message to the base constructor.
  Add extra fields for structured error data:

```cpp
class InsufficientFundsError : public std::runtime_error {
public:
    InsufficientFundsError(double requested, double available)
        : std::runtime_error("insufficient funds: requested " +
                              std::to_string(requested) + ", available " +
                              std::to_string(available)),
          requested_(requested), available_(available) {}
    double requested() const { return requested_; }
    double available() const { return available_; }
private:
    double requested_, available_;
};
```

### Exception safety guarantees

- **No-throw guarantee**: the operation never throws (e.g. `swap`,
  destructors -- destructors are implicitly `noexcept` and throwing from one
  during stack unwinding calls `std::terminate`).
- **Strong guarantee**: if the operation throws, the program state is
  **unchanged** -- as if the call never happened. Validate inputs *before*
  mutating any state:

```cpp
void withdraw(double amount) {
    if (amount <= 0) throw std::invalid_argument("...");      // checked first
    if (amount > balance_) throw InsufficientFundsError(...);  // checked first
    balance_ -= amount;  // only mutated once nothing can throw
}
```

- **Basic guarantee**: if it throws, no resources leak and invariants hold,
  but the state may have changed. RAII (destructors run during unwinding)
  is what makes the basic guarantee the *default* in C++ -- no manual
  cleanup code needed in `catch` blocks.

## `std::unique_ptr`: exclusive ownership

`std::unique_ptr<T>` owns a heap object exclusively -- **move-only**
(copying is deleted), `delete`s the object in its destructor.
`std::make_unique<T>(args...)` is the standard way to create one
(exception-safe: no raw `new` that could leak if construction throws
elsewhere in the expression).

```cpp
std::unique_ptr<Shape> p = std::make_unique<Circle>(2.0);
std::vector<std::unique_ptr<Shape>> shapes;
shapes.push_back(std::move(p));     // ownership transferred into the vector
// p is now nullptr
```

- Polymorphism through `unique_ptr<Base>` works normally -- `p->area()`
  dispatches virtually to `Circle::area()`.
- To move ownership *out* of a container element, `std::move` it and leave
  `nullptr` behind: `auto extracted = std::move(shapes[i]);` (now
  `shapes[i] == nullptr`, but `shapes.size()` is unchanged -- the slot still
  exists, just empty).
- A `unique_ptr<Base>`'s destructor correctly calls the *derived* class's
  destructor only if `Base` has a **virtual destructor**
  (`virtual ~Shape() = default;`) -- otherwise deleting through a base
  pointer is undefined behavior (only `~Base` runs, derived members leak).

## `std::shared_ptr` and `std::weak_ptr`: shared ownership

`std::shared_ptr<T>` uses **atomic reference counting**: the managed object
is destroyed when the last `shared_ptr` to it is destroyed/reset.
`use_count()` returns the current count (debugging only -- don't branch
production logic on it due to races in multithreaded code).

```cpp
auto a = std::make_shared<std::string>("x");
auto b = a;             // use_count() == 2
a.reset();              // use_count() == 1 (b keeps the object alive)
b.reset();              // object destroyed
```

`std::weak_ptr<T>` is a **non-owning** reference to a `shared_ptr`-managed
object -- it doesn't keep the object alive and doesn't count toward
`use_count()`. To use it, call `.lock()`, which returns a `shared_ptr<T>`
(non-null if the object is still alive, `nullptr`/empty if it's been
destroyed) -- `.lock()` is itself how you safely "promote" a weak reference
back to a strong (owning) one. `.expired()` checks without promoting.

```cpp
std::weak_ptr<std::string> w = a;
if (auto sp = w.lock()) {
    // sp is a valid shared_ptr -- object still alive
} else {
    // expired -- object was destroyed
}
```

**Common uses**: breaking reference cycles (e.g. a tree node's `parent`
pointer as `weak_ptr` so parent<->child `shared_ptr`s don't form a cycle
that leaks), and caches that shouldn't *force* an entry to stay alive --
store `weak_ptr`s so cache entries are freed once nothing else references
them, and `.lock()` returns `nullptr` for evicted entries.

## `std::thread` + synchronization

`std::thread` runs a callable on a new OS thread; `join()` blocks until it
finishes (every thread must be `join()`ed or `detach()`ed before its
`std::thread` object is destroyed, or the program terminates).

```cpp
std::vector<std::thread> threads;
for (int i = 0; i < n; ++i) {
    threads.emplace_back([i]() { /* work */ });
}
for (auto& t : threads) t.join();
```

**Data races**: if multiple threads read/write the same memory without
synchronization, and at least one is a write, that's a data race --
undefined behavior. Two common fixes:

- **`std::mutex` + `std::lock_guard`**: only one thread at a time holds the
  lock. `lock_guard`'s destructor unlocks automatically (RAII), even if an
  exception is thrown.

```cpp
std::mutex m;
long long total = 0;
// in each thread:
{
    std::lock_guard<std::mutex> lock(m);
    total += partialSum;
}
```

- **`std::atomic<T>`**: lock-free atomic read-modify-write for simple types
  (`int`, `long long`, pointers, ...). `total += partialSum` on a
  `std::atomic<long long>` is a single atomic operation -- often simpler and
  faster than a mutex for "accumulate a number" patterns.

```cpp
std::atomic<long long> total{0};
// in each thread:
total += partialSum;   // atomic; no separate lock needed
```

Splitting work into **chunks** (one per thread) and combining results is
the classic fork-join pattern. Capturing loop variables by reference in a
lambda passed to `std::thread`'s constructor is fine as long as the
captured variables outlive the thread (true here: they're locals of the
enclosing function, and we `join()` before returning).

## `std::async` / `std::future`: tasks with results (and exceptions)

`std::async(std::launch::async, callable)` runs `callable` (possibly on a
new thread) and returns a `std::future<R>` -- a handle to its eventual
result. `future.get()` blocks until the result is ready and returns it --
**or, if the callable threw, `get()` rethrows that same exception** in the
calling thread. This is how exceptions cross thread boundaries safely.

```cpp
std::future<int> f = std::async(std::launch::async, []() -> int {
    if (badCondition) throw std::runtime_error("failed");
    return 42;
});

try {
    int result = f.get();   // blocks; rethrows if the task threw
} catch (const std::runtime_error& e) {
    // handle the task's exception here, on the calling thread
}
```

- Always pass `std::launch::async` explicitly -- the default launch policy
  (`std::launch::async | std::launch::deferred`) lets the implementation
  defer execution until `.get()`/`.wait()` is called, which can surprise you
  if you expected concurrent execution.
- `get()` can only be called **once** per future (it consumes the result).
- Launching several `std::async` tasks up front (each returns its own
  future), then calling `.get()` on each *in order*, lets you process
  results deterministically by position even though the tasks may finish in
  any order.

## Further Reading

- [MCPP ch. 22 -- Custom Exceptions, Smart Pointers, and Concurrency](https://federico-busato.github.io/Modern-CPP-Programming/htmls/22.Exception_Handling_Smart_Pointers_Multithreading.html)
- [cppreference: Exceptions](https://en.cppreference.com/w/cpp/language/exceptions)
- [cppreference: `<stdexcept>`](https://en.cppreference.com/w/cpp/header/stdexcept)
- [cppreference: `std::unique_ptr`](https://en.cppreference.com/w/cpp/memory/unique_ptr)
- [cppreference: `std::shared_ptr`](https://en.cppreference.com/w/cpp/memory/shared_ptr)
- [cppreference: `std::weak_ptr`](https://en.cppreference.com/w/cpp/memory/weak_ptr)
- [cppreference: `std::thread`](https://en.cppreference.com/w/cpp/thread/thread)
- [cppreference: `std::mutex` / `std::lock_guard`](https://en.cppreference.com/w/cpp/thread/lock_guard)
- [cppreference: `std::atomic`](https://en.cppreference.com/w/cpp/atomic/atomic)
- [cppreference: `std::async`](https://en.cppreference.com/w/cpp/thread/async)
- [cppreference: `std::future`](https://en.cppreference.com/w/cpp/thread/future)
