#include "exercise.h"

#include <stdexcept>

// Constructors, member functions, and free-function operators throw
// std::logic_error("not implemented") so TEST_MAIN() reports a clean [FAIL]
// per test. Shape has no entries here -- its destructor is `= default` and
// its area()/perimeter() are pure virtual (`= 0`), both fully specified in
// exercise.h.

// --- Circle ----------------------------------------------------------------------------

Circle::Circle(double radius) : radius_(radius) {
    throw std::logic_error("not implemented");
}

double Circle::area() const {
    throw std::logic_error("not implemented");
}

double Circle::perimeter() const {
    throw std::logic_error("not implemented");
}

double Circle::radius() const {
    throw std::logic_error("not implemented");
}

// --- Rectangle -------------------------------------------------------------------------

Rectangle::Rectangle(double width, double height) : width_(width), height_(height) {
    throw std::logic_error("not implemented");
}

double Rectangle::area() const {
    throw std::logic_error("not implemented");
}

double Rectangle::perimeter() const {
    throw std::logic_error("not implemented");
}

double Rectangle::width() const {
    throw std::logic_error("not implemented");
}

double Rectangle::height() const {
    throw std::logic_error("not implemented");
}

// --- totalArea -------------------------------------------------------------------------

double totalArea(const std::vector<Shape*>& shapes) {
    (void)shapes;
    throw std::logic_error("not implemented");
}

// --- Vector2D --------------------------------------------------------------------------

Vector2D::Vector2D(double x, double y) : x_(x), y_(y) {
    throw std::logic_error("not implemented");
}

double Vector2D::x() const {
    throw std::logic_error("not implemented");
}

double Vector2D::y() const {
    throw std::logic_error("not implemented");
}

Vector2D Vector2D::operator+(const Vector2D& other) const {
    (void)other;
    throw std::logic_error("not implemented");
}

Vector2D Vector2D::operator-(const Vector2D& other) const {
    (void)other;
    throw std::logic_error("not implemented");
}

Vector2D Vector2D::operator-() const {
    throw std::logic_error("not implemented");
}

Vector2D Vector2D::operator*(double scalar) const {
    (void)scalar;
    throw std::logic_error("not implemented");
}

bool Vector2D::operator==(const Vector2D& other) const {
    (void)other;
    throw std::logic_error("not implemented");
}

double Vector2D::dot(const Vector2D& other) const {
    (void)other;
    throw std::logic_error("not implemented");
}

double Vector2D::length() const {
    throw std::logic_error("not implemented");
}

Vector2D operator*(double scalar, const Vector2D& v) {
    (void)scalar;
    (void)v;
    throw std::logic_error("not implemented");
}

std::ostream& operator<<(std::ostream& os, const Vector2D& v) {
    (void)os;
    (void)v;
    throw std::logic_error("not implemented");
}

// --- Matrix2x2 -------------------------------------------------------------------------

Matrix2x2::Matrix2x2(double a, double b, double c, double d) : a_(a), b_(b), c_(c), d_(d) {
    throw std::logic_error("not implemented");
}

double Matrix2x2::a() const {
    throw std::logic_error("not implemented");
}

double Matrix2x2::b() const {
    throw std::logic_error("not implemented");
}

double Matrix2x2::c() const {
    throw std::logic_error("not implemented");
}

double Matrix2x2::d() const {
    throw std::logic_error("not implemented");
}

Matrix2x2 Matrix2x2::operator+(const Matrix2x2& other) const {
    (void)other;
    throw std::logic_error("not implemented");
}

Matrix2x2 Matrix2x2::operator*(const Matrix2x2& other) const {
    (void)other;
    throw std::logic_error("not implemented");
}

Vector2D Matrix2x2::operator*(const Vector2D& v) const {
    (void)v;
    throw std::logic_error("not implemented");
}

bool Matrix2x2::operator==(const Matrix2x2& other) const {
    (void)other;
    throw std::logic_error("not implemented");
}

double Matrix2x2::determinant() const {
    throw std::logic_error("not implemented");
}

std::ostream& operator<<(std::ostream& os, const Matrix2x2& m) {
    (void)os;
    (void)m;
    throw std::logic_error("not implemented");
}
