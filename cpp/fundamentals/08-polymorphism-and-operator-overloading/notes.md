# 8. Polymorphism & Operator Overloading

## Inheritance

```cpp
class Animal {
public:
    explicit Animal(std::string name) : name_(std::move(name)) {}
protected:                 // accessible to Animal AND classes derived from it
    std::string name_;
};

class Dog : public Animal {
public:
    explicit Dog(std::string name) : Animal(std::move(name)) {}  // calls base ctor
};
```

- `class Derived : public Base` -- `Derived` inherits `Base`'s members.
  `public` inheritance means "is-a": a `Dog` *is an* `Animal`. (`private`/
  `protected` inheritance exist but are rare -- not covered here.)
- A derived class's constructor **must** initialize its base class -- via the
  initializer list (`Dog(...) : Animal(...) {}`). If omitted, the base's
  default constructor is called implicitly (error if the base has none).
- **`protected`**: like `private`, but also accessible to derived classes
  (vs. `private`, which is not). Use sparingly -- it's still an exposed
  implementation detail to every derived class, current and future.

## Virtual functions & dynamic dispatch

```cpp
class Shape {
public:
    virtual double area() const { return 0.0; }   // virtual -- can be overridden
};

class Circle : public Shape {
public:
    double area() const override { return 3.14159 * r_ * r_; }  // overrides Shape's
private:
    double r_ = 1.0;
};

Shape* s = new Circle();
s->area();   // calls Circle::area(), NOT Shape::area() -- decided at RUNTIME
```

- Without `virtual`, calling `s->area()` through a `Shape*` always calls
  `Shape::area()`, regardless of the pointed-to object's actual type
  ("static"/compile-time dispatch, resolved by the pointer's declared type).
- With `virtual`, the call is **dispatched dynamically**: at runtime, the
  *actual* object's class's override runs. Mechanism (conceptually): a class
  with virtual functions has a hidden per-object pointer to a **vtable** (a
  table of function pointers, one per virtual function, filled in per class);
  a virtual call indirects through it.
- **`override`** (on the derived function) isn't required, but always write
  it: it's a compile error if the signature doesn't *actually* override a
  base virtual function (typo'd name, mismatched `const`/parameters) --
  without it, such a mismatch silently declares an unrelated new function
  instead of overriding, and dynamic dispatch silently doesn't happen.

## Pure virtual functions & abstract classes

```cpp
class Shape {
public:
    virtual ~Shape() = default;
    virtual double area() const = 0;        // pure virtual -- no body
    virtual double perimeter() const = 0;
};

// Shape s;        // ERROR -- Shape is abstract (has pure virtual functions)
Shape* s = new Circle(1.0);  // OK -- Circle overrides both, so it's concrete
```

- `= 0` instead of a body marks a virtual function **pure virtual** -- the
  base class provides no implementation, only an interface.
- A class with **any** pure virtual function (inherited or its own) is
  **abstract**: it cannot be instantiated directly (`Shape s;` is a compile
  error), only through a fully-overriding derived class.
- A derived class becomes **concrete** (instantiable) once it overrides
  *every* pure virtual function it inherits; otherwise it's abstract too.

## Virtual destructors

```cpp
class Base {
public:
    virtual ~Base() = default;   // virtual -- required for safe polymorphic delete
};
class Derived : public Base {
    std::string data_;            // has its own resources to clean up
};

Base* b = new Derived();
delete b;   // virtual ~Base() dispatches to ~Derived() first, then ~Base()
```

- If a class is ever used **polymorphically** (deleted through a base
  pointer, as above), its destructor **must** be `virtual`. Without it,
  `delete b` calls only `~Base()` -- `~Derived()` (and the destructors of
  `Derived`'s own members) never run. This is undefined behavior, and in
  practice leaks/skips cleanup of everything `Derived` added.
- `virtual ~Base() = default;` is enough if `Base` itself owns nothing --
  marking it `virtual` is what matters; derived destructors are then called
  correctly via the vtable, each cleaning up its own members as normal.
- Rule of thumb: a class with **any** virtual function should almost always
  have a virtual destructor too.

## Object slicing

```cpp
class Base {
public:
    virtual std::string describe() const { return "Base"; }
};
class Derived : public Base {
public:
    std::string describe() const override { return "Derived"; }
    int extra = 42;
};

Derived d;
Base byValue = d;     // SLICED -- copies only the Base part of d
Base& byRef = d;       // NOT sliced -- refers to the whole Derived object

byValue.describe();    // "Base"    -- byValue's dynamic type is Base
byRef.describe();       // "Derived" -- byRef's dynamic type is still Derived
```

- Assigning/copying a `Derived` object into a `Base`-by-value variable
  (or passing/returning `Base` by value) **slices off** everything
  `Derived` added -- the result is a genuine `Base` object, and any virtual
  calls on it use `Base`'s overrides, not `Derived`'s.
- Polymorphism requires **references or pointers** to the base type --
  never by-value base-type variables/parameters/returns -- so the object's
  actual (derived) type and vtable are preserved.

## Operator overloading: basics

```cpp
class Vector2D {
public:
    Vector2D(double x, double y) : x_(x), y_(y) {}
    Vector2D operator+(const Vector2D& other) const {     // member operator
        return Vector2D(x_ + other.x_, y_ + other.y_);
    }
private:
    double x_, y_;
};

Vector2D operator*(double scalar, const Vector2D& v) {     // free-function operator
    return Vector2D(v.x() * scalar, v.y() * scalar);
}
```

- `a + b` for objects calls `operator+` -- either a **member** of `a`'s
  class (`a.operator+(b)`, called as `a + b`) or a **free function** taking
  both operands (`operator+(a, b)`).
- Use a **member** when the left operand is (or can be) `*this` -- `a + b`
  where `a` is your class. Use a **free function** when the left operand
  *isn't* your class -- `3.0 * v` (left operand `double`, can't add a member
  to `double`), or `os << v` (left operand `std::ostream&` -- `operator<<`
  for a user type is *always* a free function for this reason).
- A free-function operator that needs `private` members either takes only
  values reachable via **public accessors** (as above), or is declared
  `friend` inside the class (not needed here, since `x()`/`y()` are public).

## Conventions & rules

- **Arithmetic operators** (`+`, `-`, binary and unary, `*`) return a **new
  object by value**, leaving both operands unchanged -- `a + b` doesn't
  modify `a` or `b`. Compare **compound assignment** (`+=`), which *does*
  mutate `*this` and returns `*this` by reference (not used in this topic's
  exercises, but the common pairing: `a + b` calls `operator+`, often
  implemented in terms of `a += b` on a copy).
- **Comparison operators** (`==`, `<`, ...) return `bool`. In C++20, defining
  `operator==(const T&) const` is enough for both `a == b` and `a != b` --
  the compiler rewrites `a != b` as `!(a == b)` automatically (a "rewritten
  candidate"); you don't need to write `operator!=` separately.
- **`operator<<`/`operator>>`** for `std::ostream`/`std::istream` are *always*
  free functions returning the stream by reference (`std::ostream&`), so
  `os << a << b` chains left-to-right.
- You **cannot**: invent new operators, change an operator's arity (number of
  operands) or precedence/associativity, or overload an operator when *all*
  operands are built-in types (`operator+(int, int)` is fixed by the
  language).
- Overloaded operators should behave the way programmers *expect* from the
  built-in operators of the same symbol ("principle of least astonishment")
  -- e.g. don't make `operator+` mutate an operand, or `operator==` do
  something other than equality.

## Further Reading (Modern C++ Programming)

- [Chapter 10 — Object-Oriented Programming II](https://federico-busato.github.io/Modern-CPP-Programming/htmls/10.Object_Oriented_II.html)
- [Derived classes](https://en.cppreference.com/w/cpp/language/derived_class)
- [`virtual` function specifier](https://en.cppreference.com/w/cpp/language/virtual)
- [Abstract classes](https://en.cppreference.com/w/cpp/language/abstract_class)
- [`override` specifier](https://en.cppreference.com/w/cpp/language/override)
- [Operator overloading](https://en.cppreference.com/w/cpp/language/operators)
- [`operator<<` / stream insertion](https://en.cppreference.com/w/cpp/language/operators#Stream_extraction_and_insertion)
