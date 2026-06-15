// Run with:
//   g++ -std=c++20 -Wall -Wextra -o /tmp/ex examples.cpp && /tmp/ex

#include <iostream>
#include <string>
#include <vector>

// --- Inheritance, virtual functions, abstract base class, virtual destructor ---------
class Animal {
public:
    explicit Animal(std::string name) : name_(std::move(name)) {}
    virtual ~Animal() { std::cout << "    ~Animal(" << name_ << ")\n"; }

    virtual std::string speak() const = 0;  // pure virtual -- Animal is abstract

    const std::string& name() const { return name_; }

protected:
    std::string name_;
};

class Dog : public Animal {
public:
    explicit Dog(std::string name) : Animal(std::move(name)) {}
    ~Dog() override { std::cout << "    ~Dog(" << name_ << ")\n"; }
    std::string speak() const override { return name_ + " says Woof!"; }
};

class Cat : public Animal {
public:
    explicit Cat(std::string name) : Animal(std::move(name)) {}
    ~Cat() override { std::cout << "    ~Cat(" << name_ << ")\n"; }
    std::string speak() const override { return name_ + " says Meow!"; }
};

// --- Object slicing: needs a concrete (non-abstract) base ----------------------------
class Base {
public:
    virtual std::string describe() const { return "Base"; }
};

class Derived : public Base {
public:
    std::string describe() const override { return "Derived"; }
    int extra = 42;
};

// --- Operator overloading: a small Complex number type --------------------------------
class Complex {
public:
    Complex(double re, double im) : re_(re), im_(im) {}

    double re() const { return re_; }
    double im() const { return im_; }

    Complex operator+(const Complex& other) const {
        return Complex(re_ + other.re_, im_ + other.im_);
    }
    Complex operator*(const Complex& other) const {
        return Complex(re_ * other.re_ - im_ * other.im_, re_ * other.im_ + im_ * other.re_);
    }
    Complex operator-() const { return Complex(-re_, -im_); }

    Complex& operator+=(const Complex& other) {  // compound assignment: mutates *this
        re_ += other.re_;
        im_ += other.im_;
        return *this;
    }

    bool operator==(const Complex& other) const {
        return re_ == other.re_ && im_ == other.im_;
    }

private:
    double re_;
    double im_;
};

std::ostream& operator<<(std::ostream& os, const Complex& c) {
    os << c.re();
    if (c.im() >= 0) os << "+";
    return os << c.im() << "i";
}

int main() {
    // --- dynamic dispatch through base pointers ---
    std::cout << "Dynamic dispatch:\n";
    std::vector<Animal*> animals = {new Dog("Rex"), new Cat("Whiskers")};
    for (Animal* a : animals) {
        std::cout << "  " << a->speak() << "\n";
    }

    // Animal a("nobody");  // ERROR: Animal is abstract (pure virtual speak())

    // --- virtual destructor: deleting through a base pointer runs the derived dtor ---
    std::cout << "\nDeleting through Animal* (virtual dtor runs Dog's/Cat's first):\n";
    for (Animal* a : animals) {
        delete a;
    }

    // --- object slicing ---
    std::cout << "\nSlicing:\n";
    Derived d;
    Base byValue = d;  // copies only the Base part -- slicing
    Base& byRef = d;    // refers to the whole Derived object -- no slicing

    std::cout << "  d.describe()       = " << d.describe() << "\n";
    std::cout << "  byValue.describe() = " << byValue.describe() << " (sliced to Base)\n";
    std::cout << "  byRef.describe()   = " << byRef.describe() << " (still Derived)\n";

    // --- operator overloading ---
    std::cout << "\nComplex numbers:\n";
    Complex a(1.0, 2.0);
    Complex b(3.0, -1.0);

    std::cout << "  a         = " << a << "\n";
    std::cout << "  b         = " << b << "\n";
    std::cout << "  a + b     = " << (a + b) << "\n";
    std::cout << "  a * b     = " << (a * b) << "\n";
    std::cout << "  -a        = " << (-a) << "\n";
    std::cout << "  a == a    = " << std::boolalpha << (a == a) << "\n";
    std::cout << "  a == b    = " << (a == b) << "\n";

    Complex c = a;
    c += b;  // compound assignment -- mutates c in place
    std::cout << "  c += b -> c = " << c << "\n";

    return 0;
}
