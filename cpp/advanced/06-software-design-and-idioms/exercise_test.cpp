#include "exercise.h"

#include <memory>
#include <stdexcept>
#include <string>
#include <type_traits>
#include <utility>
#include <vector>

#include "../../testing.h"

// --- MaxStack: PIMPL idiom + Rule of Five --------------------------------------------------------

TEST(MaxStackPushPopAndMax) {
    // PIMPL: MaxStack's only member is a single pointer to its Impl.
    CHECK_EQ(sizeof(MaxStack), sizeof(void*));

    MaxStack s;
    CHECK(s.empty());
    CHECK_EQ(s.size(), static_cast<size_t>(0));

    bool threw = false;
    try {
        s.pop();
    } catch (const std::out_of_range&) {
        threw = true;
    }
    CHECK(threw);

    threw = false;
    try {
        s.top();
    } catch (const std::out_of_range&) {
        threw = true;
    }
    CHECK(threw);

    threw = false;
    try {
        s.max();
    } catch (const std::out_of_range&) {
        threw = true;
    }
    CHECK(threw);

    for (int v : {3, 1, 4, 1, 5}) s.push(v);
    CHECK_EQ(s.size(), static_cast<size_t>(5));
    CHECK_EQ(s.top(), 5);
    CHECK_EQ(s.max(), 5);

    s.pop();  // remove 5
    CHECK_EQ(s.top(), 1);
    CHECK_EQ(s.max(), 4);

    s.pop();  // remove 1
    CHECK_EQ(s.top(), 4);
    CHECK_EQ(s.max(), 4);

    s.pop();  // remove 4
    CHECK_EQ(s.top(), 1);
    CHECK_EQ(s.max(), 3);
    CHECK_EQ(s.size(), static_cast<size_t>(2));
}

TEST(MaxStackCopyIsDeepAndMoveLeavesValidEmptyState) {
    MaxStack a;
    for (int v : {10, 20, 5}) a.push(v);

    // Copy is independent of the original.
    MaxStack b = a;
    b.push(100);
    CHECK_EQ(b.max(), 100);
    CHECK_EQ(a.max(), 20);
    CHECK_EQ(a.size(), static_cast<size_t>(3));
    CHECK_EQ(b.size(), static_cast<size_t>(4));

    // Move transfers state; source becomes a valid, usable empty stack.
    MaxStack c = std::move(a);
    CHECK_EQ(c.max(), 20);
    CHECK_EQ(c.size(), static_cast<size_t>(3));
    CHECK(a.empty());
    CHECK_EQ(a.size(), static_cast<size_t>(0));

    a.push(7);
    CHECK_EQ(a.top(), 7);
    CHECK_EQ(a.max(), 7);

    // Copy assignment, then move assignment, including self-assignment.
    MaxStack d;
    d.push(1);
    d = b;
    CHECK_EQ(d.max(), 100);
    d.push(200);
    CHECK_EQ(b.max(), 100);  // d's later mutation doesn't affect b

    d = d;  // self copy-assignment must not corrupt d
    CHECK_EQ(d.max(), 200);
    CHECK_EQ(d.size(), static_cast<size_t>(5));

    MaxStack e;
    e = std::move(d);
    CHECK_EQ(e.max(), 200);
    CHECK(d.empty());
}

// --- IdRegistry: Meyer's Singleton --------------------------------------------------------------

static_assert(!std::is_copy_constructible_v<IdRegistry>, "IdRegistry must not be copyable");
static_assert(!std::is_copy_assignable_v<IdRegistry>, "IdRegistry must not be copy-assignable");

TEST(SingletonLazyInitMemoizedIdsAndSingleConstruction) {
    IdRegistry& r1 = IdRegistry::instance();
    IdRegistry& r2 = IdRegistry::instance();
    CHECK(&r1 == &r2);

    CHECK_EQ(r1.issueId("alice"), 0);
    CHECK_EQ(r1.issueId("bob"), 1);
    CHECK_EQ(r1.issueId("alice"), 0);  // memoized: same name -> same id
    CHECK_EQ(r1.issueId("carol"), 2);
    CHECK_EQ(r2.registeredCount(), static_cast<size_t>(3));

    // instance() was called several times above, but the constructor ran once.
    CHECK_EQ(IdRegistry::constructionCount(), 1);
}

// --- Validator hierarchy: Non-Virtual Interface (Template Method) -------------------------------

TEST(EmailValidatorChecksAtAndDomain) {
    EmailValidator v;

    ValidationResult empty = v.validate("");
    CHECK(!empty.valid);
    CHECK_EQ(empty.reason, std::string("input is empty"));

    CHECK_EQ(v.validate("foo").reason, std::string("missing or misplaced '@'"));
    CHECK_EQ(v.validate("@b.com").reason, std::string("missing or misplaced '@'"));
    CHECK_EQ(v.validate("a@").reason, std::string("missing or misplaced '@'"));
    CHECK_EQ(v.validate("a@b@c.com").reason, std::string("multiple '@'"));
    CHECK_EQ(v.validate("a@bcom").reason, std::string("domain missing '.'"));

    ValidationResult ok = v.validate("a@b.com");
    CHECK(ok.valid);
    CHECK_EQ(ok.reason, std::string("ok"));
}

TEST(PositiveIntegerValidatorOverridesBothHooks) {
    PositiveIntegerValidator v;

    CHECK_EQ(v.validate("").reason, std::string("input is empty"));            // base checkNotEmpty
    CHECK_EQ(v.validate("   ").reason, std::string("input is whitespace only"));  // overridden hook
    CHECK_EQ(v.validate("0").reason, std::string("value is zero or has a leading zero"));
    CHECK_EQ(v.validate("007").reason, std::string("value is zero or has a leading zero"));
    CHECK_EQ(v.validate("-5").reason, std::string("contains non-digit characters"));

    ValidationResult ok = v.validate("42");
    CHECK(ok.valid);
    CHECK_EQ(ok.reason, std::string("ok"));
}

TEST(ValidatorPolymorphicDispatchThroughBasePointer) {
    std::vector<std::unique_ptr<Validator>> validators;
    validators.push_back(std::make_unique<EmailValidator>());
    validators.push_back(std::make_unique<PositiveIntegerValidator>());

    CHECK(validators[0]->validate("a@b.com").valid);
    CHECK(!validators[0]->validate("nope").valid);
    CHECK(validators[1]->validate("123").valid);
    CHECK(!validators[1]->validate("0").valid);
}

// --- EventBus: Observer / publish-subscribe -------------------------------------------------------

TEST(EventBusSubscribePublishUnsubscribe) {
    EventBus bus;

    int clickCountA = 0, clickCountB = 0, hoverCount = 0;
    int lastClickPayloadA = -1;

    int idA = bus.subscribe("click", [&](const Event& e) {
        ++clickCountA;
        lastClickPayloadA = e.payload;
    });
    int idB = bus.subscribe("click", [&](const Event&) { ++clickCountB; });
    int idC = bus.subscribe("hover", [&](const Event&) { ++hoverCount; });

    CHECK_EQ(idA, 0);
    CHECK_EQ(idB, 1);
    CHECK_EQ(idC, 2);

    CHECK_EQ(bus.publish({"click", 1}), 2);
    CHECK_EQ(clickCountA, 1);
    CHECK_EQ(clickCountB, 1);
    CHECK_EQ(hoverCount, 0);
    CHECK_EQ(lastClickPayloadA, 1);

    CHECK_EQ(bus.publish({"hover", 2}), 1);
    CHECK_EQ(hoverCount, 1);

    CHECK_EQ(bus.publish({"drag", 3}), 0);

    bus.unsubscribe(idB);
    CHECK_EQ(bus.publish({"click", 4}), 1);
    CHECK_EQ(clickCountA, 2);
    CHECK_EQ(clickCountB, 1);  // unsubscribed handler not called again
    CHECK_EQ(lastClickPayloadA, 4);

    bus.unsubscribe(999);  // nonexistent id is a no-op
    CHECK_EQ(bus.publish({"click", 5}), 1);
    CHECK_EQ(clickCountA, 3);
}

// --- Comparable<Derived> / Version: CRTP -----------------------------------------------------------

TEST(VersionCrtpComparisons) {
    Version v1(1, 2, 3);
    Version v1again(1, 2, 3);
    Version v2(1, 3, 0);
    Version v3(2, 0, 0);
    Version v4(1, 2, 4);

    CHECK(v1.compareTo(v1again) == 0);
    CHECK(v1 == v1again);
    CHECK(!(v1 != v1again));

    CHECK(v1.compareTo(v2) < 0);
    CHECK(v1 < v2);
    CHECK(v2 > v1);

    CHECK(v1.compareTo(v3) < 0);  // major differs
    CHECK(v3 > v1);

    CHECK(v1.compareTo(v4) < 0);  // patch differs
    CHECK(v1 < v4);

    CHECK(v1 <= v1again);
    CHECK(v1 >= v1again);
    CHECK(v1 != v2);

    CHECK_EQ(v1.major(), 1);
    CHECK_EQ(v1.minor(), 2);
    CHECK_EQ(v1.patch(), 3);
}

TEST_MAIN()
