#include <sstream>

#include "exercise.h"
#include "../../testing.h"

TEST(CirclePolymorphicAreaAndPerimeter) {
    Circle c(2.0);
    CHECK(c.radius() == 2.0);
    CHECK(c.area() == kPi * 2.0 * 2.0);
    CHECK(c.perimeter() == 2.0 * kPi * 2.0);

    // accessed through a Shape& -- area()/perimeter() dispatch dynamically
    // to Circle's overrides, not Shape's (Shape has none -- pure virtual).
    Shape& s = c;
    CHECK(s.area() == c.area());
    CHECK(s.perimeter() == c.perimeter());
}

TEST(RectangleAreaAndPerimeter) {
    Rectangle r(3.0, 4.0);
    CHECK(r.width() == 3.0);
    CHECK(r.height() == 4.0);
    CHECK(r.area() == 12.0);
    CHECK(r.perimeter() == 14.0);
}

TEST(TotalAreaSumsPolymorphically) {
    Circle c(1.0);
    Rectangle r(2.0, 3.0);

    std::vector<Shape*> shapes = {&c, &r};
    CHECK(totalArea(shapes) == c.area() + r.area());

    std::vector<Shape*> empty;
    CHECK(totalArea(empty) == 0.0);

    Rectangle r2(1.0, 1.0);
    std::vector<Shape*> three = {&c, &r, &r2};
    CHECK(totalArea(three) == c.area() + r.area() + r2.area());
}

TEST(Vector2DArithmeticOperators) {
    Vector2D a(1.0, 2.0);
    Vector2D b(3.0, 4.0);

    CHECK(a + b == Vector2D(4.0, 6.0));
    CHECK(b - a == Vector2D(2.0, 2.0));
    CHECK(-a == Vector2D(-1.0, -2.0));
    CHECK(a * 3.0 == Vector2D(3.0, 6.0));
    CHECK(3.0 * a == a * 3.0);  // free-function operator*, scalar on the left

    CHECK(a.dot(b) == 11.0);  // 1*3 + 2*4
    CHECK(!(a == b));
    CHECK(a == Vector2D(1.0, 2.0));

    Vector2D threeFour(3.0, 4.0);
    CHECK(threeFour.length() == 5.0);
}

TEST(Vector2DStreamInsertion) {
    Vector2D v(1.5, -2.5);
    std::ostringstream oss;
    oss << v;
    CHECK(oss.str() == "(1.5, -2.5)");
}

TEST(Matrix2x2ArithmeticAndDeterminant) {
    Matrix2x2 m1(1.0, 2.0, 3.0, 4.0);
    Matrix2x2 m2(5.0, 6.0, 7.0, 8.0);

    CHECK(m1.a() == 1.0);
    CHECK(m1.b() == 2.0);
    CHECK(m1.c() == 3.0);
    CHECK(m1.d() == 4.0);
    CHECK(m1.determinant() == -2.0);  // 1*4 - 2*3

    CHECK(m1 + m2 == Matrix2x2(6.0, 8.0, 10.0, 12.0));

    // [[1,2],[3,4]] * [[5,6],[7,8]] = [[19,22],[43,50]]
    CHECK(m1 * m2 == Matrix2x2(19.0, 22.0, 43.0, 50.0));
}

TEST(Matrix2x2VectorMultiplication) {
    Matrix2x2 identity(1.0, 0.0, 0.0, 1.0);
    Vector2D v(3.0, 4.0);
    CHECK(identity * v == v);

    Matrix2x2 scaleByTwo(2.0, 0.0, 0.0, 2.0);
    CHECK(scaleByTwo * v == Vector2D(6.0, 8.0));

    // 90-degree rotation: (x, y) -> (-y, x)
    Matrix2x2 rot(0.0, -1.0, 1.0, 0.0);
    CHECK(rot * Vector2D(1.0, 0.0) == Vector2D(0.0, 1.0));
}

TEST(Matrix2x2StreamInsertion) {
    Matrix2x2 m(1.0, 2.0, 3.0, 4.0);
    std::ostringstream oss;
    oss << m;
    CHECK(oss.str() == "[[1, 2], [3, 4]]");
}

TEST_MAIN()
