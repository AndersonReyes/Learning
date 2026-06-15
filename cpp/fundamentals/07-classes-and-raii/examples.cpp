// Run with:
//   g++ -std=c++20 -Wall -Wextra -o /tmp/ex examples.cpp && /tmp/ex

#include <iostream>
#include <string>

// --- RAII: a guard that logs when it's constructed/destroyed -----------------------
class ScopeLogger {
public:
    explicit ScopeLogger(std::string name) : name_(std::move(name)) {
        std::cout << "  enter " << name_ << "\n";
    }
    ~ScopeLogger() {
        std::cout << "  exit  " << name_ << "\n";
    }

private:
    std::string name_;
};

// --- explicit constructors -----------------------------------------------------------
class Meters {
public:
    explicit Meters(double value) : value_(value) {}
    double value() const { return value_; }

private:
    double value_;
};

double describeLength(Meters m) { return m.value(); }

// --- Rule of Three: IntArray, a small owning heap-array type ------------------------
class IntArray {
public:
    explicit IntArray(int size) : data_(new int[size]), size_(size) {
        for (int i = 0; i < size_; ++i) data_[i] = 0;
    }

    ~IntArray() { delete[] data_; }

    IntArray(const IntArray& other) : data_(new int[other.size_]), size_(other.size_) {
        for (int i = 0; i < size_; ++i) data_[i] = other.data_[i];
    }

    IntArray& operator=(const IntArray& other) {
        if (this == &other) return *this;
        int* newData = new int[other.size_];
        for (int i = 0; i < other.size_; ++i) newData[i] = other.data_[i];
        delete[] data_;
        data_ = newData;
        size_ = other.size_;
        return *this;
    }

    int& operator[](int i) { return data_[i]; }
    int operator[](int i) const { return data_[i]; }
    int size() const { return size_; }

private:
    int* data_;
    int size_;
};

// --- this and method chaining ---------------------------------------------------------
class StringBuilder {
public:
    StringBuilder& add(const std::string& s) {
        value_ += s;
        return *this;
    }
    const std::string& str() const { return value_; }

private:
    std::string value_;
};

// --- static members: shared counter + static factory ------------------------------------
class Widget {
public:
    explicit Widget(std::string label) : id_(nextId_++), label_(std::move(label)) {}

    static Widget makeUnlabeled() { return Widget("(unlabeled)"); }

    int id() const { return id_; }
    const std::string& label() const { return label_; }
    static int totalCreated() { return nextId_; }

private:
    int id_;
    std::string label_;
    static int nextId_;
};
int Widget::nextId_ = 0;

// --- const data members / immutable-style objects ----------------------------------------
class Fraction {
public:
    Fraction(int numerator, int denominator) : numerator_(numerator), denominator_(denominator) {}

    int numerator() const { return numerator_; }
    int denominator() const { return denominator_; }

    Fraction plus(const Fraction& other) const {
        return Fraction(numerator_ * other.denominator_ + other.numerator_ * denominator_,
                         denominator_ * other.denominator_);
    }

private:
    const int numerator_;
    const int denominator_;
};

int main() {
    // --- RAII and destruction order ---
    std::cout << "RAII scope:\n";
    {
        ScopeLogger outer("outer");
        ScopeLogger inner("inner");
        std::cout << "  ...inside scope...\n";
    }  // inner destroyed first, then outer -- reverse of construction order
    std::cout << "  (back outside scope)\n";

    // --- explicit ---
    Meters m(5.0);
    std::cout << "\ndescribeLength(m) = " << describeLength(m) << "\n";
    // Meters m2 = 5.0;       // ERROR: explicit blocks implicit double -> Meters
    // describeLength(5.0);    // ERROR: same reason

    // --- Rule of Three ---
    std::cout << "\nIntArray:\n";
    IntArray a(3);
    a[0] = 1;
    a[1] = 2;
    a[2] = 3;

    IntArray b(a);  // deep copy via copy constructor
    b[0] = 99;
    std::cout << "  a[0] = " << a[0] << " (unchanged)\n";
    std::cout << "  b[0] = " << b[0] << " (independent copy)\n";

    IntArray c(1);
    c = a;  // deep copy via copy assignment
    c[1] = 42;
    std::cout << "  a[1] = " << a[1] << ", c[1] = " << c[1] << " (independent)\n";

    c = c;  // self-assignment -- must not corrupt c
    std::cout << "  c after self-assignment: c[0]=" << c[0] << " c[1]=" << c[1] << " c[2]="
              << c[2] << "\n";

    // --- this and method chaining ---
    StringBuilder sb;
    sb.add("Hello").add(", ").add("world").add("!");
    std::cout << "\nStringBuilder: " << sb.str() << "\n";

    // --- static members ---
    Widget w1("first");
    Widget w2("second");
    Widget w3 = Widget::makeUnlabeled();
    std::cout << "\nWidget ids: " << w1.id() << ", " << w2.id() << ", " << w3.id() << "\n";
    std::cout << "Widget::totalCreated() = " << Widget::totalCreated() << "\n";
    std::cout << "w3.label() = " << w3.label() << "\n";

    // --- const data members ---
    Fraction half(1, 2);
    Fraction third(1, 3);
    Fraction sum = half.plus(third);
    std::cout << "\n1/2 + 1/3 = " << sum.numerator() << "/" << sum.denominator() << "\n";

    return 0;
}
