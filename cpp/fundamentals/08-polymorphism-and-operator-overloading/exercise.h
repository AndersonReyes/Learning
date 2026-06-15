#pragma once

#include <ostream>
#include <vector>

// Topic 8: Polymorphism & Operator Overloading
//
// Shape/Circle/Rectangle/totalArea demonstrate inheritance, virtual
// functions, abstract base classes, and dynamic dispatch through base
// pointers. Vector2D and Matrix2x2 demonstrate operator overloading: member
// operators, free-function operators (for cross-type operations and stream
// insertion, where the left-hand operand isn't the class itself).

inline constexpr double kPi = 3.141592653589793;

// --- Shape: abstract base class -----------------------------------------------------
//
// Fully defined here -- a pure virtual function (`= 0`) has no body, and
// `= default` needs no definition either, so Shape itself has nothing in
// exercise.cpp.
class Shape {
public:
    virtual ~Shape() = default;

    // Area of the shape.
    virtual double area() const = 0;

    // Perimeter (circumference) of the shape.
    virtual double perimeter() const = 0;
};

// --- Circle ---------------------------------------------------------------------------
class Circle : public Shape {
public:
    explicit Circle(double radius);

    double area() const override;
    double perimeter() const override;
    double radius() const;

private:
    double radius_;
};

// --- Rectangle ------------------------------------------------------------------------
class Rectangle : public Shape {
public:
    Rectangle(double width, double height);

    double area() const override;
    double perimeter() const override;
    double width() const;
    double height() const;

private:
    double width_;
    double height_;
};

// --- totalArea: dynamic dispatch through a vector of base pointers ---------------------
//
// Returns the sum of shape->area() for every shape in `shapes`, calling each
// shape's overridden area() through Shape's virtual function table. Returns
// 0.0 for an empty vector.
double totalArea(const std::vector<Shape*>& shapes);

// --- Vector2D: operator overloading -----------------------------------------------------
class Vector2D {
public:
    Vector2D(double x, double y);

    double x() const;
    double y() const;

    Vector2D operator+(const Vector2D& other) const;
    Vector2D operator-(const Vector2D& other) const;
    Vector2D operator-() const;               // unary negation
    Vector2D operator*(double scalar) const;  // vector * scalar

    bool operator==(const Vector2D& other) const;

    double dot(const Vector2D& other) const;
    double length() const;

private:
    double x_;
    double y_;
};

// scalar * vector -- a free function, since the left operand (double) isn't
// a Vector2D and so can't be the implicit `this` of a member operator.
Vector2D operator*(double scalar, const Vector2D& v);

// Prints "(x, y)".
std::ostream& operator<<(std::ostream& os, const Vector2D& v);

// --- Matrix2x2: operator overloading across types -----------------------------------------
//
// Row-major 2x2 matrix [[a, b], [c, d]].
class Matrix2x2 {
public:
    Matrix2x2(double a, double b, double c, double d);

    double a() const;
    double b() const;
    double c() const;
    double d() const;

    Matrix2x2 operator+(const Matrix2x2& other) const;
    Matrix2x2 operator*(const Matrix2x2& other) const;  // matrix * matrix
    Vector2D operator*(const Vector2D& v) const;         // matrix * vector

    bool operator==(const Matrix2x2& other) const;

    double determinant() const;

private:
    double a_;
    double b_;
    double c_;
    double d_;
};

// Prints "[[a, b], [c, d]]".
std::ostream& operator<<(std::ostream& os, const Matrix2x2& m);
