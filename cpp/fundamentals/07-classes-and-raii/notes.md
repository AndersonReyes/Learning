# 7. Classes, Constructors, Destructors & RAII

## Classes: the basics

```cpp
class Point {
public:                      // accessible from anywhere
    Point(double x, double y) : x_(x), y_(y) {}  // constructor, member-init list
    double x() const { return x_; }               // const member function
    double y() const { return y_; }

private:                      // accessible only inside Point's own members
    double x_;
    double y_;
};
```

- `class` defaults members to `private`; `struct` (topic 4) defaults to
  `public` -- otherwise identical. Convention: `class` for types with
  invariants/encapsulation, `struct` for plain data aggregates.
- **Member-initializer list** (`: x_(x), y_(y)`) initializes members
  *before* the constructor body runs -- the only way to initialize `const`
  members, reference members, or members with no default constructor.
  Members are initialized in **declaration order**, not initializer-list
  order (mismatched order is a `-Wreorder` warning).
- **`const` member functions** (`double x() const`) promise not to modify
  `*this` -- required to call on a `const Point&`. Modifying a non-`mutable`
  member inside one is a compile error.

## Constructors

```cpp
class Wrapper {
public:
    explicit Wrapper(int value) : value_(value) {}  // explicit: blocks implicit conversion
    Wrapper() : value_(0) {}                          // default constructor
private:
    int value_;
};

Wrapper w1(5);       // OK -- direct initialization
Wrapper w2 = 5;       // ERROR -- explicit blocks implicit int -> Wrapper conversion
void take(Wrapper);
take(5);              // ERROR for the same reason
```

- A constructor callable with no arguments (no parameters, or all defaulted)
  is the **default constructor** -- used for `T t;` and as a
  container/array element type with no initializer.
- **`explicit`** on a converting constructor (one callable with a single
  argument) blocks the compiler from using it for *implicit* conversions --
  `Wrapper w = 5;` and passing `5` where a `Wrapper` is expected both become
  errors, while `Wrapper w(5);` and `Wrapper w{5};` still work. Default to
  `explicit` unless implicit conversion is genuinely wanted.
- If a class declares **any** constructor, the compiler-generated default
  constructor disappears -- `Wrapper w;` is an error if only `Wrapper(int)`
  is declared.
- A class with `const` or reference members, or members with no default
  constructor, **cannot have a default constructor** unless every such
  member is given a value in the initializer list.

## Destructors & RAII

```cpp
class FileHandle {
public:
    explicit FileHandle(const char* path) : fp_(std::fopen(path, "r")) {}
    ~FileHandle() { if (fp_) std::fclose(fp_); }   // runs automatically at scope exit
private:
    FILE* fp_;
};
```

- `~ClassName()` runs automatically when an object goes out of scope (or is
  `delete`d, for heap objects). Multiple objects in a scope are destroyed in
  **reverse order of construction**; a class's members are destroyed (in
  reverse declaration order) *after* the destructor body finishes.
- **RAII** ("Resource Acquisition Is Initialization"): tie a resource's
  lifetime to an object's lifetime -- acquire in the constructor, release in
  the destructor. The resource is released on *every* exit path (normal
  return, early `return`, exception unwinding) with no manual `cleanup()`
  call to remember and no leak on early exit.
- **Destructors must not throw.** They're implicitly `noexcept`; an
  exception escaping a destructor calls `std::terminate()` and aborts the
  program immediately -- there is no way to "fail" a cleanup.

## The Rule of Three

If a class manages a resource (e.g. a heap array via `new`/`delete`) and you
write **any one** of these three, you almost always need **all three**:

```cpp
class Buffer {
public:
    explicit Buffer(int size) : data_(new int[size]), size_(size) {}

    ~Buffer() { delete[] data_; }                              // 1. destructor

    Buffer(const Buffer& other)                                 // 2. copy constructor
        : data_(new int[other.size_]), size_(other.size_) {
        for (int i = 0; i < size_; ++i) data_[i] = other.data_[i];
    }

    Buffer& operator=(const Buffer& other) {                    // 3. copy assignment
        if (this == &other) return *this;        // self-assignment guard
        int* newData = new int[other.size_];
        for (int i = 0; i < other.size_; ++i) newData[i] = other.data_[i];
        delete[] data_;            // only after the copy succeeds
        data_ = newData;
        size_ = other.size_;
        return *this;
    }

private:
    int* data_;
    int size_;
};
```

- Without a user-defined copy constructor, the compiler generates one that
  copies each member **as-is** -- for a raw pointer, that's a shallow copy:
  two `Buffer`s pointing at the *same* heap array. Destroying either one
  frees it; the other now holds a **dangling pointer** -- using it is UB,
  and destroying it too is a **double-free**.
- **Copy constructor**: builds a new object as an independent copy (here, a
  fresh `new[]` plus an element-wise copy).
- **Copy assignment** (`operator=`): `*this` already exists and owns a
  resource -- it must (1) guard against self-assignment (`a = a;`, or two
  references to the same object), (2) acquire the new resource, (3)
  **only then** release the old one, (4) copy over the new state. Allocating
  before `delete[]`-ing means a failed allocation leaves the original object
  intact.
- Without the self-assignment guard, `delete[] data_` would free the buffer
  *before* `other.data_` (== `data_`, the same pointer for `a = a`) is read
  -- use-after-free.
- **`= delete`** on the copy constructor/assignment operator makes a type
  **non-copyable** -- for RAII guards that must never be duplicated
  (`ScopedFlag(const ScopedFlag&) = delete;`). Attempting to copy then is a
  compile error, not a runtime bug.

## `this` and method chaining

```cpp
class Builder {
public:
    Builder& setName(std::string name) { name_ = std::move(name); return *this; }
    Builder& setAge(int age) { age_ = age; return *this; }
private:
    std::string name_;
    int age_ = 0;
};

Builder b;
b.setName("Ada").setAge(36);   // method chaining via the returned *this
```

- `this` is a pointer to the current object (`ClassName* this`, or
  `const ClassName* this` inside a `const` member function) -- implicitly
  available in every non-static member function.
- Returning `*this` **by reference** from a mutating method enables **method
  chaining** / a **fluent interface**: each call returns the same object, so
  another call can follow immediately.

## Static members

```cpp
class Counter {
public:
    Counter() { ++count_; }
    ~Counter() { --count_; }
    static int count() { return count_; }
private:
    static int count_;            // declaration only
};
int Counter::count_ = 0;            // definition -- exactly one, in a .cpp file
```

- A **static data member** is shared by *all* instances -- one copy total,
  not one per object. Declared inside the class, **defined** (and
  initialized) once at namespace scope (`Type ClassName::member_ = init;`).
- A **static member function** (`static int count()`) has no `this` -- it
  can't access non-static members, and is called as `ClassName::count()`
  without an object.
- **Static factory functions** -- named alternative constructors:
  ```cpp
  class Temperature {
  public:
      explicit Temperature(double celsius) : celsius_(celsius) {}
      static Temperature fromFahrenheit(double f) {
          return Temperature((f - 32.0) * 5.0 / 9.0);
      }
  private:
      double celsius_;
  };
  Temperature t = Temperature::fromFahrenheit(212.0);  // 100.0 C
  ```
  Useful when a type has multiple meaningful ways to construct it that don't
  differ enough in parameter *types* to overload, or when construction
  involves a conversion/computation best given a descriptive name.
- A copy constructor that's `= default` does **not** update static members
  -- if a static member tracks "number of live instances including copies",
  the copy constructor must be **user-defined** and increment it explicitly
  (the compiler-generated one only copies non-static members).

## Const data members and immutability

```cpp
class ImmutablePoint {
public:
    ImmutablePoint(double x, double y) : x_(x), y_(y) {}
    double x() const { return x_; }
    ImmutablePoint translated(double dx, double dy) const {
        return ImmutablePoint(x_ + dx, y_ + dy);   // new object, *this unchanged
    }
private:
    const double x_;
    const double y_;
};
```

- `const` data members must be set in the constructor's initializer list and
  never change afterward -- there is no default constructor (nothing to
  initialize them to), but the **copy constructor** still works (copying a
  `const` member into another object's `const` member of the same type is
  fine).
- A common immutable-object pattern: "mutating" operations return a *new*
  instance with the updated state rather than modifying `*this` -- combine
  with `const` member functions, since the operation only needs to *read*
  `*this` to compute the new object.

## Further Reading (Modern C++ Programming)

- [Chapter 9 — Object-Oriented Programming I](https://federico-busato.github.io/Modern-CPP-Programming/htmls/09.Object_Oriented_I.html)
- [Classes](https://en.cppreference.com/w/cpp/language/classes)
- [Constructors and member initializer lists](https://en.cppreference.com/w/cpp/language/constructor)
- [Destructors](https://en.cppreference.com/w/cpp/language/destructor)
- [`this` pointer](https://en.cppreference.com/w/cpp/language/this)
- [Static members](https://en.cppreference.com/w/cpp/language/static)
- [Deleted functions (`= delete`)](https://en.cppreference.com/w/cpp/language/function#Deleted_functions)
- [`explicit` specifier](https://en.cppreference.com/w/cpp/language/explicit)
