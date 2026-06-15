#include "exercise.h"

#include <stdexcept>

// Constructors, regular methods, static methods, and the copy-assignment
// operator throw std::logic_error("not implemented") so TEST_MAIN() reports
// a clean [FAIL] per test. Destructors are no-op -- a throwing destructor
// would call std::terminate() (destructors are implicitly noexcept).

// --- ScopedFlag ----------------------------------------------------------------------

ScopedFlag::ScopedFlag(bool& flag) : flag_(flag) {
    throw std::logic_error("not implemented");
}

ScopedFlag::~ScopedFlag() {}

// --- BoundedStack --------------------------------------------------------------------

BoundedStack::BoundedStack(int capacity)
    : data_(nullptr), capacity_(capacity), size_(0) {
    throw std::logic_error("not implemented");
}

BoundedStack::~BoundedStack() {}

BoundedStack::BoundedStack(const BoundedStack& other)
    : data_(nullptr), capacity_(other.capacity_), size_(0) {
    throw std::logic_error("not implemented");
}

BoundedStack& BoundedStack::operator=(const BoundedStack& other) {
    (void)other;
    throw std::logic_error("not implemented");
}

BoundedStack& BoundedStack::push(int value) {
    (void)value;
    throw std::logic_error("not implemented");
}

BoundedStack& BoundedStack::pop() {
    throw std::logic_error("not implemented");
}

int BoundedStack::top() const {
    throw std::logic_error("not implemented");
}

int BoundedStack::size() const {
    throw std::logic_error("not implemented");
}

int BoundedStack::capacity() const {
    throw std::logic_error("not implemented");
}

bool BoundedStack::full() const {
    throw std::logic_error("not implemented");
}

bool BoundedStack::empty() const {
    throw std::logic_error("not implemented");
}

// --- InstanceCounter -------------------------------------------------------------------

int InstanceCounter::liveCount_ = 0;

InstanceCounter::InstanceCounter() {
    throw std::logic_error("not implemented");
}

InstanceCounter::InstanceCounter(const InstanceCounter& other) {
    (void)other;
    throw std::logic_error("not implemented");
}

InstanceCounter::~InstanceCounter() {}

int InstanceCounter::liveCount() {
    throw std::logic_error("not implemented");
}

// --- ImmutablePoint -----------------------------------------------------------------

ImmutablePoint::ImmutablePoint(double x, double y) : x_(x), y_(y) {
    throw std::logic_error("not implemented");
}

double ImmutablePoint::x() const {
    throw std::logic_error("not implemented");
}

double ImmutablePoint::y() const {
    throw std::logic_error("not implemented");
}

ImmutablePoint ImmutablePoint::translated(double dx, double dy) const {
    (void)dx;
    (void)dy;
    throw std::logic_error("not implemented");
}

double ImmutablePoint::distanceTo(const ImmutablePoint& other) const {
    (void)other;
    throw std::logic_error("not implemented");
}

// --- Temperature ----------------------------------------------------------------------

Temperature::Temperature(double celsius) : celsius_(celsius) {
    throw std::logic_error("not implemented");
}

Temperature Temperature::fromFahrenheit(double fahrenheit) {
    (void)fahrenheit;
    throw std::logic_error("not implemented");
}

double Temperature::celsius() const {
    throw std::logic_error("not implemented");
}

double Temperature::fahrenheit() const {
    throw std::logic_error("not implemented");
}

Temperature Temperature::warmerBy(double deltaCelsius) const {
    (void)deltaCelsius;
    throw std::logic_error("not implemented");
}
