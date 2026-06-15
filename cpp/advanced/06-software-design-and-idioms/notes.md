# Advanced 06: Software Design Principles, Idioms & Patterns

advanced/01-05 covered language mechanics and performance. This topic is
about **structuring code** -- the principles (MCPP ch. 26) that make code
maintainable, and the C++ idioms (MCPP ch. 27) that implement them. The
exercises (`MaxStack`, `IdRegistry`, `Validator`/`EmailValidator`/
`PositiveIntegerValidator`, `EventBus`, `Comparable<Derived>`/`Version`)
each embody one idiom/pattern below.

## Design principles (ch. 26)

- **Encapsulation / information hiding** -- expose *what* a type does, hide
  *how*. Callers depend on an interface, not on representation details that
  might change. PIMPL (below) takes this to its logical extreme: the
  representation isn't even visible in the header.
- **Separation of concerns / low coupling, high cohesion** -- each
  class/module does one thing, and modules interact through narrow
  interfaces rather than reaching into each other's internals. The Observer
  pattern (below) is a textbook example: publishers and subscribers don't
  reference each other's types at all.
- **SOLID** (brief): **S**ingle Responsibility (one reason to change per
  class), **O**pen/Closed (extend behavior via new derived classes, without
  modifying existing code -- the NVI/Template-Method pattern below is a
  direct application), **L**iskov Substitution (a derived class must be
  usable anywhere its base is expected -- don't override a virtual to
  *narrow* its preconditions or *weaken* its postconditions), **I**nterface
  Segregation (small, focused interfaces over one giant one), **D**ependency
  Inversion (depend on abstractions, not concrete types -- e.g. `EventBus`
  handlers are `std::function`, not concrete classes).
- **Value vs. reference semantics** -- a value type's copy is fully
  independent (`std::vector`, `std::string`, plain structs); a reference-like
  type's copies share state (`std::shared_ptr`, a view/span, an iterator).
  Pick deliberately: surprising shared mutation through "copies" is a classic
  bug source. `std::unique_ptr` is reference-like in *access* but value-like
  in *ownership* (exactly one owner) -- this is why PIMPL's `MaxStack` needs a
  hand-written deep-copying copy constructor: the default member-wise copy of
  a `unique_ptr<Impl>` member wouldn't compile (copy ctor deleted), and even
  if it did, copying the pointer would alias the same `Impl`.
- **Owning objects vs. views** -- a `std::vector<int>` *owns* its storage; a
  `std::span<int>`/`std::string_view` *views* someone else's storage without
  owning it (cf. intermediate/07). A view is only valid as long as the
  owner is alive -- returning a view into a local variable is a dangling
  reference, the same bug as returning `&localVar`.

## Idioms (ch. 27)

### Rule of Zero / Three / Five (recap from advanced/01)

- **Rule of Zero**: prefer types that need NO custom destructor/copy/move --
  compose them from members that already manage their own resources
  (`std::vector`, `std::unique_ptr`, `std::string`, ...). The compiler-
  generated special members are then correct automatically.
- **Rule of Three** (pre-C++11): if you write a custom destructor, copy
  constructor, or copy-assignment operator, you almost certainly need all
  three (a custom destructor usually means the default copy would be wrong --
  e.g. double-`delete`).
- **Rule of Five** (C++11+): adds move constructor and move-assignment to the
  Rule of Three -- a type managing a resource should usually define all five
  (or `= delete`/`= default` them explicitly) so copies are deep and moves
  are cheap and leave the source in a valid state.

### PIMPL (Pointer to IMPLementation)

Hide a class's data members behind `std::unique_ptr<Impl>` to an
incomplete type, forward-declared in the header:

```cpp
// header
class Widget {
public:
    Widget();
    ~Widget();                       // must be declared, defined in .cpp
    Widget(const Widget&);           // deep-copies *other.impl_
    Widget(Widget&&);                // (or noexcept, in real code)
    Widget& operator=(const Widget&);
    Widget& operator=(Widget&&);
    void doThing();
private:
    struct Impl;                     // incomplete here
    std::unique_ptr<Impl> impl_;
};
```

```cpp
// .cpp
struct Widget::Impl { /* real members */ void doThingImpl(); };
Widget::Widget() : impl_(std::make_unique<Impl>()) {}
Widget::~Widget() = default;          // OK now -- Impl is complete
void Widget::doThing() { impl_->doThingImpl(); }
```

**Why all five special members must be hand-written**: a
`unique_ptr<Impl>` member's destructor calls `delete` on `Impl*`, which
needs `Impl`'s definition (size, destructor) -- not visible in the header.
If the compiler tried to implicitly generate `~Widget()` (or the implicitly
move-generated members, which also need the destructor) in the header, it
would error on an incomplete type. Declaring `~Widget()` (even as
`= default`) and defining it in the `.cpp` defers generation to where `Impl`
is complete. Copy ctor/assign must be hand-written regardless (`unique_ptr`
isn't copyable) -- deep-copy `*other.impl_` via `Impl`'s own (implicit)
copy constructor.

**Benefits**: changing `Impl`'s members doesn't change `Widget`'s size or
require recompiling `Widget`'s callers (only `widget.cpp` rebuilds) -- a
stable ABI and faster incremental builds. **Cost**: one extra heap
allocation and pointer indirection per object.

### Singleton (Meyer's Singleton)

Exactly one instance, lazily created on first use:

```cpp
class Logger {
public:
    static Logger& instance() {
        static Logger inst;     // constructed on first call, thread-safe since C++11
        return inst;
    }
    Logger(const Logger&) = delete;
    Logger& operator=(const Logger&) = delete;
private:
    Logger() = default;
};
```

A function-local `static` is initialized exactly once, the first time
control passes through that declaration -- and C++11 guarantees this
initialization is thread-safe (concurrent first calls block until
construction finishes) without any manual locking. Delete copy/move so no
second instance can be created by accident. **Caveats**: singletons are
global mutable state (testing/parallelism hazards), and destruction order
of multiple singletons at program exit is unspecified relative to each
other -- avoid singletons that depend on each other in their destructors.

### CRTP (Curiously Recurring Template Pattern)

A base class template **parameterized on its own derived class**, used for
compile-time ("static") polymorphism -- no virtual functions, no vtable:

```cpp
template <typename Derived>
class Comparable {
public:
    bool operator<(const Derived& other) const {
        return static_cast<const Derived&>(*this).compareTo(other) < 0;
    }
    // ... <=, >, >=, ==, != similarly, all via compareTo
};

class Version : public Comparable<Version> {
public:
    int compareTo(const Version& other) const { /* ... */ }
};
```

`Comparable<Derived>::operator<` calls `static_cast<const Derived&>(*this)`
to access `Derived::compareTo` -- resolved entirely at compile time (the
exact `compareTo` to call is known from the template argument `Derived`,
not looked up via a vtable at runtime). Six operators come "for free" from
one method, with zero runtime overhead and zero extra bytes in `Derived`
(an empty CRTP base contributes nothing to `sizeof(Derived)` under the empty
base optimization). Contrast with a virtual-function approach, which would
add a vtable pointer to every instance and resolve the call at runtime.
**Gotcha**: `Derived` must be *complete* by the time `Comparable<Derived>`'s
member functions are *instantiated* (not when the class is defined) --
inheriting `class Version : public Comparable<Version>` is fine because
template member functions aren't instantiated until called, by which point
`Version` is complete.

### "Template virtual functions" -- Non-Virtual Interface (NVI) / Template Method

Make the PUBLIC entry point non-virtual, and have it call PRIVATE/PROTECTED
virtual "hook" functions for the customizable steps:

```cpp
class Validator {
public:
    ValidationResult validate(const std::string& input) const {  // NOT virtual
        ValidationResult r = checkNotEmpty(input);
        return r.valid ? checkContent(input) : r;
    }
protected:
    virtual ValidationResult checkNotEmpty(const std::string& input) const;  // has a default
    virtual ValidationResult checkContent(const std::string& input) const = 0;  // must override
};
```

The base class controls the *algorithm's shape* (the sequence and overall
contract of `validate`); derived classes customize individual *steps*
(`checkContent`, optionally `checkNotEmpty`) without being able to change
the overall sequence or skip steps -- callers can't either, since
`validate()` itself isn't virtual. This is the Open/Closed Principle in
practice: add a new validator by writing a new derived class (open for
extension), without touching `Validator::validate` (closed for
modification).

## Further Reading

- [MCPP ch. 26 -- Software Design I: Basic Concepts](https://federico-busato.github.io/Modern-CPP-Programming/htmls/26.Software_Design_I.html)
- [MCPP ch. 27 -- Software Design II: Design Patterns and Idioms](https://federico-busato.github.io/Modern-CPP-Programming/htmls/27.Software_Design_II.html)
- [cppreference: Rule of three/five/zero](https://en.cppreference.com/w/cpp/language/rule_of_three)
- [cppreference: `std::unique_ptr`](https://en.cppreference.com/w/cpp/memory/unique_ptr)
- [cppreference: `static` local variables (initialization)](https://en.cppreference.com/w/cpp/language/storage_duration#Static_local_variables)
- [Wikipedia: Curiously recurring template pattern](https://en.wikipedia.org/wiki/Curiously_recurring_template_pattern)
- [Wikipedia: Non-virtual interface pattern](https://en.wikipedia.org/wiki/Non-virtual_interface_pattern)
- [Wikipedia: SOLID](https://en.wikipedia.org/wiki/SOLID)
