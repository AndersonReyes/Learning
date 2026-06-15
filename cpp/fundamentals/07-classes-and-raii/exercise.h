#pragma once

// Topic 7: Classes, Constructors, Destructors & RAII
//
// Five classes covering: RAII (ScopedFlag), the Rule of Three plus a fluent
// interface (BoundedStack), static members for instance counting
// (InstanceCounter), const data members via initializer lists
// (ImmutablePoint), and a static factory function plus a computed accessor
// (Temperature).

// --- ScopedFlag: minimal RAII guard -------------------------------------------------
//
// Binds a reference to a bool on construction and sets it to true; sets it
// back to false on destruction. Non-copyable -- a guard shouldn't be
// duplicated (copying would leave two destructors fighting over one flag).
class ScopedFlag {
public:
    // Binds `flag` and sets it to true.
    explicit ScopedFlag(bool& flag);

    // Sets the bound flag back to false.
    ~ScopedFlag();

    ScopedFlag(const ScopedFlag&) = delete;
    ScopedFlag& operator=(const ScopedFlag&) = delete;

private:
    bool& flag_;
};

// --- BoundedStack: fixed-capacity stack of ints, Rule of Three ----------------------
//
// A stack backed by a fixed-size heap array, allocated once at construction.
// push()/pop() are no-ops once full/empty -- no exceptions, no UB. push()
// and pop() return *this by reference to support method chaining.
class BoundedStack {
public:
    // Allocates storage for up to `capacity` ints. size() starts at 0.
    explicit BoundedStack(int capacity);

    // Frees the underlying storage.
    ~BoundedStack();

    // Deep-copies `other`'s buffer into newly allocated, independent storage.
    BoundedStack(const BoundedStack& other);

    // Deep-copy assignment. Frees this object's old buffer only after the
    // new one is successfully allocated and copied. Self-assignment safe.
    BoundedStack& operator=(const BoundedStack& other);

    // Pushes `value` if not full; no-op if full. Returns *this for chaining.
    BoundedStack& push(int value);

    // Removes the top element if not empty; no-op if empty. Returns *this.
    BoundedStack& pop();

    // Returns the top element, or 0 if the stack is empty.
    int top() const;

    int size() const;
    int capacity() const;
    bool full() const;
    bool empty() const;

private:
    int* data_;
    int capacity_;
    int size_;
};

// --- InstanceCounter: static member tracks live instances ---------------------------
//
// liveCount() returns how many InstanceCounter objects currently exist.
// Construction (including via the copy constructor) increments it;
// destruction decrements it.
class InstanceCounter {
public:
    InstanceCounter();
    InstanceCounter(const InstanceCounter& other);
    ~InstanceCounter();

    // Returns the number of InstanceCounter objects currently alive.
    static int liveCount();

private:
    static int liveCount_;
};

// --- ImmutablePoint: const data members, no setters ----------------------------------
//
// A 2D point whose coordinates are fixed at construction. "Mutating"
// operations return a new ImmutablePoint rather than modifying *this.
class ImmutablePoint {
public:
    ImmutablePoint(double x, double y);

    double x() const;
    double y() const;

    // Returns a new point offset by (dx, dy); *this is unchanged.
    ImmutablePoint translated(double dx, double dy) const;

    // Euclidean distance from *this to `other`.
    double distanceTo(const ImmutablePoint& other) const;

private:
    const double x_;
    const double y_;
};

// --- Temperature: static factory + computed accessor ---------------------------------
//
// Stores a temperature internally in Celsius. fromFahrenheit() is a static
// factory that converts on construction. fahrenheit() computes the
// Fahrenheit value on demand rather than storing it.
class Temperature {
public:
    explicit Temperature(double celsius);

    // Converts `fahrenheit` to Celsius and constructs from that.
    static Temperature fromFahrenheit(double fahrenheit);

    double celsius() const;
    double fahrenheit() const;

    // Returns a new Temperature `deltaCelsius` warmer than *this.
    Temperature warmerBy(double deltaCelsius) const;

private:
    double celsius_;
};
