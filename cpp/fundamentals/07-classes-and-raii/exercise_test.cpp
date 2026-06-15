#include "exercise.h"
#include "../../testing.h"

TEST(ScopedFlagSetsAndRestoresFlag) {
    bool flag = false;
    CHECK(flag == false);
    {
        ScopedFlag guard(flag);
        CHECK(flag == true);
    }
    CHECK(flag == false);
}

TEST(BoundedStackBasicPushPopTop) {
    BoundedStack s(3);
    CHECK(s.capacity() == 3);
    CHECK(s.empty());
    CHECK(s.size() == 0);
    CHECK(s.top() == 0);  // empty -> 0, not UB

    s.push(10).push(20).push(30);  // method chaining
    CHECK(s.size() == 3);
    CHECK(s.full());
    CHECK(s.top() == 30);

    // pushing while full is a no-op
    s.push(40);
    CHECK(s.size() == 3);
    CHECK(s.top() == 30);

    s.pop();
    CHECK(s.size() == 2);
    CHECK(!s.full());
    CHECK(s.top() == 20);

    s.pop().pop();
    CHECK(s.empty());
    CHECK(s.top() == 0);

    // popping while empty is a no-op
    s.pop();
    CHECK(s.empty());
    CHECK(s.size() == 0);
}

TEST(BoundedStackCopyConstructorIsDeepCopy) {
    BoundedStack original(3);
    original.push(1).push(2);

    BoundedStack copy(original);
    copy.push(3);  // mutate the copy

    CHECK(copy.size() == 3);
    CHECK(copy.top() == 3);
    CHECK(original.size() == 2);  // original unaffected
    CHECK(original.top() == 2);
}

TEST(BoundedStackCopyAssignmentIsDeepCopyAndSelfSafe) {
    BoundedStack a(2);
    a.push(1).push(2);

    BoundedStack b(5);
    b.push(100);

    b = a;  // replaces b's buffer with a deep copy of a's
    CHECK(b.capacity() == 2);
    CHECK(b.size() == 2);
    CHECK(b.top() == 2);

    b.pop();
    CHECK(b.top() == 1);
    CHECK(a.top() == 2);  // a unaffected by mutating b

    // self-assignment must not corrupt or double-free
    a = a;
    CHECK(a.size() == 2);
    CHECK(a.top() == 2);
}

TEST(InstanceCounterTracksLiveInstances) {
    CHECK(InstanceCounter::liveCount() == 0);
    {
        InstanceCounter a;
        CHECK(InstanceCounter::liveCount() == 1);
        {
            InstanceCounter b(a);  // copy also counts
            CHECK(InstanceCounter::liveCount() == 2);
        }
        CHECK(InstanceCounter::liveCount() == 1);
    }
    CHECK(InstanceCounter::liveCount() == 0);
}

TEST(ImmutablePointAccessorsAndTranslation) {
    ImmutablePoint p(3.0, 4.0);
    CHECK(p.x() == 3.0);
    CHECK(p.y() == 4.0);

    ImmutablePoint q = p.translated(1.0, -2.0);
    CHECK(q.x() == 4.0);
    CHECK(q.y() == 2.0);

    // original unaffected
    CHECK(p.x() == 3.0);
    CHECK(p.y() == 4.0);
}

TEST(ImmutablePointDistance) {
    ImmutablePoint origin(0.0, 0.0);
    ImmutablePoint p(3.0, 4.0);
    CHECK(origin.distanceTo(p) == 5.0);
    CHECK(p.distanceTo(origin) == 5.0);
    CHECK(p.distanceTo(p) == 0.0);
}

TEST(TemperatureCelsiusAndFahrenheit) {
    Temperature freezing(0.0);
    CHECK(freezing.celsius() == 0.0);
    CHECK(freezing.fahrenheit() == 32.0);

    Temperature boiling(100.0);
    CHECK(boiling.fahrenheit() == 212.0);
}

TEST(TemperatureFromFahrenheitFactory) {
    Temperature t = Temperature::fromFahrenheit(32.0);
    CHECK(t.celsius() == 0.0);

    Temperature t2 = Temperature::fromFahrenheit(212.0);
    CHECK(t2.celsius() == 100.0);
}

TEST(TemperatureWarmerByReturnsNewInstance) {
    Temperature t(20.0);
    Temperature warmer = t.warmerBy(5.0);
    CHECK(warmer.celsius() == 25.0);
    CHECK(t.celsius() == 20.0);  // original unaffected
}

TEST_MAIN()
