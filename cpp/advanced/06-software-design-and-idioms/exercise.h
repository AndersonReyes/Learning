#pragma once

#include <cstddef>
#include <functional>
#include <memory>
#include <stdexcept>
#include <string>
#include <unordered_map>
#include <vector>

// Topic 26-27 (Advanced 06): Software Design Principles, Idioms & Patterns
//
// Chapter 26 covers design principles -- encapsulation/information hiding,
// low coupling/high cohesion, value vs. reference semantics. Chapter 27
// covers the C++ idioms that implement them: Rule of Five (recap from
// advanced/01, reused here inside PIMPL), Singleton, PIMPL, CRTP, and
// "template virtual functions" (the Non-Virtual Interface / Template
// Method pattern). Five exercises, one idiom/pattern each. Free
// functions/classes are declared here and defined in exercise.cpp. Stub
// bodies throw std::logic_error("not implemented").

// --- MaxStack: PIMPL idiom + Rule of Five with an incomplete type ----------------------------------
//
// A stack of ints that also reports its current maximum in O(1) via
// `max()`. The implementation (a vector of (value, runningMax) pairs) is
// hidden behind a forward-declared `Impl` struct -- the PIMPL ("Pointer to
// IMPLementation") idiom: callers never see `Impl`'s definition, so changing
// it doesn't require recompiling callers, and `sizeof(MaxStack)` is just one
// pointer.
//
// Because `impl_` is a `std::unique_ptr<Impl>` to an INCOMPLETE type (from
// the header's point of view), the compiler cannot generate the destructor,
// copy constructor/assignment, or move constructor/assignment inline --
// `~unique_ptr<Impl>()` needs `Impl`'s definition to call `delete`. All five
// special member functions must therefore be declared here and DEFINED in
// exercise.cpp (where `Impl` is complete):
//   - destructor: defined as `= default` (now legal -- Impl is complete)
//   - copy ctor/assign: deep-copy `*other.impl_` (default-copyable Impl)
//   - move ctor/assign: take ownership of `other.impl_`, and leave `other`
//     holding a fresh EMPTY Impl (not nullptr) so a moved-from MaxStack
//     remains a valid, usable empty stack
//
// (Real code would mark the move constructor/assignment `noexcept` --
// omitted here only so the stub bodies can `throw` like every other
// exercise in this track.)
//
// push(value): O(1) amortized. pop()/top()/max() on an empty stack throw
// std::out_of_range.
//
// Example: push 3,1,4,1,5 -> max() == 5, top() == 5.
// pop() -> top() == 1, max() == 4.
// pop() -> top() == 4, max() == 4.
// pop() -> top() == 1, max() == 3, size() == 2.
class MaxStack {
public:
    MaxStack();
    ~MaxStack();
    MaxStack(const MaxStack& other);
    MaxStack(MaxStack&& other);
    MaxStack& operator=(const MaxStack& other);
    MaxStack& operator=(MaxStack&& other);

    void push(int value);
    void pop();          // throws std::out_of_range if empty
    int top() const;     // throws std::out_of_range if empty
    int max() const;     // throws std::out_of_range if empty
    size_t size() const;
    bool empty() const;

private:
    struct Impl;
    std::unique_ptr<Impl> impl_;
};

// --- IdRegistry: Meyer's Singleton -----------------------------------------------------------------
//
// A process-wide registry that assigns each distinct string name a unique
// sequential integer id (0, 1, 2, ... in first-seen order), memoizing
// repeated lookups.
//
// `instance()` returns a reference to the single shared IdRegistry, created
// lazily on first call via a function-local `static` (Meyer's Singleton --
// thread-safe initialization since C++11, no separate init function or
// global constructor ordering issues). The constructor is private; copying
// and assignment are deleted.
//
// `constructionCount()` returns how many times the constructor has run --
// it must be exactly 1 even if `instance()` is called many times, proving
// the singleton is constructed lazily and only once.
//
// Example: instance().issueId("alice") == 0; instance().issueId("bob") == 1;
// instance().issueId("alice") == 0 (memoized); instance().registeredCount()
// == 2; constructionCount() == 1 no matter how many times instance() was
// called.
class IdRegistry {
public:
    static IdRegistry& instance();

    IdRegistry(const IdRegistry&) = delete;
    IdRegistry& operator=(const IdRegistry&) = delete;

    // Returns the id for `name`, assigning a new sequential id (starting at
    // 0) the first time `name` is seen, and returning the same id on every
    // later call with the same `name`.
    int issueId(const std::string& name);

    // Number of distinct names issued so far.
    size_t registeredCount() const;

    // Number of times the IdRegistry constructor has run (always 1).
    static int constructionCount();

private:
    IdRegistry();

    std::unordered_map<std::string, int> ids_;
    int nextId_ = 0;
    static int constructionCount_;
};

// --- Validator hierarchy: Non-Virtual Interface (Template Method) ----------------------------------
//
// "Template virtual functions" (NVI): the PUBLIC interface (`validate`) is
// a single NON-virtual function implementing a fixed algorithm skeleton --
// run `checkNotEmpty`, then (if that passed) `checkContent` -- while the
// individual STEPS are PRIVATE/PROTECTED virtuals that derived classes
// override. Callers can never skip or reorder the steps (validate() is not
// virtual), but derived classes can customize each step.
//
// `checkNotEmpty` has a default implementation (reject only the empty
// string) that derived classes may override to add stricter rules.
// `checkContent` is pure virtual -- every validator must define its own
// content check.
//
// ValidationResult{valid, reason}: reason is "ok" when valid is true,
// otherwise a short description of what failed.
//
// EmailValidator::checkContent: input must contain exactly one '@', with at
// least one character before and after it, and the part after '@' must
// contain a '.'.
//   validate("") -> {false, "input is empty"}            (base checkNotEmpty)
//   validate("foo") -> {false, "missing or misplaced '@'"}
//   validate("@b.com") -> {false, "missing or misplaced '@'"}
//   validate("a@") -> {false, "missing or misplaced '@'"}
//   validate("a@b@c.com") -> {false, "multiple '@'"}
//   validate("a@bcom") -> {false, "domain missing '.'"}
//   validate("a@b.com") -> {true, "ok"}
//
// PositiveIntegerValidator overrides checkNotEmpty to ALSO reject
// whitespace-only strings, and checkContent to require all-digit input with
// no leading zero (so "0" and any zero-padded value are rejected as content
// errors, not emptiness errors):
//   validate("") -> {false, "input is empty"}            (base checkNotEmpty)
//   validate("   ") -> {false, "input is whitespace only"} (overridden checkNotEmpty)
//   validate("0") -> {false, "value is zero or has a leading zero"}
//   validate("007") -> {false, "value is zero or has a leading zero"}
//   validate("-5") -> {false, "contains non-digit characters"}
//   validate("42") -> {true, "ok"}
struct ValidationResult {
    bool valid;
    std::string reason;
};

class Validator {
public:
    virtual ~Validator() = default;

    // NVI: the fixed validation algorithm. Calls checkNotEmpty(input); if
    // that fails, returns its result. Otherwise calls and returns
    // checkContent(input).
    ValidationResult validate(const std::string& input) const;

protected:
    // Default: {false, "input is empty"} if input.empty(), else {true, "ok"}.
    virtual ValidationResult checkNotEmpty(const std::string& input) const;

    virtual ValidationResult checkContent(const std::string& input) const = 0;
};

class EmailValidator : public Validator {
protected:
    ValidationResult checkContent(const std::string& input) const override;
};

class PositiveIntegerValidator : public Validator {
protected:
    ValidationResult checkNotEmpty(const std::string& input) const override;
    ValidationResult checkContent(const std::string& input) const override;
};

// --- EventBus: Observer / publish-subscribe ----------------------------------------------------
//
// Decouples event producers from consumers (low coupling / separation of
// concerns): producers call `publish`, consumers register independently via
// `subscribe`, and neither side knows about the other.
//
// subscribe(type, handler): registers `handler` to be called for every
// future `publish`ed Event whose `type` field equals `type`. Returns a
// unique, non-negative subscription id (ids are assigned sequentially:
// 0, 1, 2, ... across ALL subscriptions regardless of event type).
//
// unsubscribe(subscriptionId): removes that subscription. A nonexistent id
// (already removed, or never issued) is a no-op.
//
// publish(event) const: calls every currently-subscribed handler whose
// registered type equals event.type, in the order they were subscribed,
// passing `event` to each. Returns the number of handlers invoked (0 if
// none match).
//
// Example: subscribe two handlers to "click" (ids 0, 1) and one to "hover"
// (id 2). publish({"click", 1}) calls both click handlers and returns 2.
// publish({"hover", 2}) calls the hover handler and returns 1.
// publish({"drag", 3}) calls nothing and returns 0.
// After unsubscribe(1), publish({"click", 4}) calls only handler 0 and
// returns 1.
struct Event {
    std::string type;
    int payload;
};

class EventBus {
public:
    using Handler = std::function<void(const Event&)>;

    int subscribe(const std::string& type, Handler handler);
    void unsubscribe(int subscriptionId);
    int publish(const Event& event) const;

private:
    struct Subscription {
        int id;
        std::string type;
        Handler handler;
    };
    std::vector<Subscription> subscriptions_;
    int nextId_ = 0;
};

// --- Comparable<Derived> / Version: CRTP static polymorphism ---------------------------------------
//
// CRTP (Curiously Recurring Template Pattern): `Comparable<Derived>` is a
// mixin base class TEMPLATED ON ITS OWN DERIVED CLASS. It provides
// `<, <=, >, >=, ==, !=` for `Derived` purely in terms of ONE method,
// `Derived::compareTo`, returning <0/0/>0 like `strcmp` -- the same idea as
// Java's `Comparable<T>`, but resolved at compile time via
// `static_cast<const Derived&>(*this)` (NO virtual dispatch, NO vtable
// pointer added to Derived).
//
// `Comparable<Derived>` itself is complete, given code (templates must live
// in headers) -- the exercise is `Version`, which derives from
// `Comparable<Version>` and must implement `compareTo` so the inherited
// operators produce correct LEXICOGRAPHIC ordering on (major, minor, patch).
//
// compareTo returns a negative number if *this < other, zero if equal,
// and a positive number if *this > other -- comparing major first, then
// minor, then patch (like comparing version strings "1.2.3" vs "1.3.0").
//
// Example: Version(1,2,3).compareTo(Version(1,2,3)) == 0.
// Version(1,2,3) < Version(1,3,0) -- true (minor differs).
// Version(2,0,0) > Version(1,9,9) -- true (major differs).
// Version(1,2,3) < Version(1,2,4) -- true (patch differs).
// Version(1,0,0) >= Version(1,0,0) -- true. Version(1,0,0) != Version(1,0,1) -- true.
template <typename Derived>
class Comparable {
public:
    bool operator<(const Derived& other) const { return self().compareTo(other) < 0; }
    bool operator<=(const Derived& other) const { return self().compareTo(other) <= 0; }
    bool operator>(const Derived& other) const { return self().compareTo(other) > 0; }
    bool operator>=(const Derived& other) const { return self().compareTo(other) >= 0; }
    bool operator==(const Derived& other) const { return self().compareTo(other) == 0; }
    bool operator!=(const Derived& other) const { return self().compareTo(other) != 0; }

private:
    const Derived& self() const { return static_cast<const Derived&>(*this); }
};

class Version : public Comparable<Version> {
public:
    Version(int major, int minor, int patch);

    int compareTo(const Version& other) const;

    int major() const;
    int minor() const;
    int patch() const;

private:
    int major_;
    int minor_;
    int patch_;
};
