#include <cmath>
#include <string>
#include <vector>

#include "exercise.h"
#include "shapes.h"
#include "../../testing.h"

TEST(PerimeterOfRightTriangleAndSquare) {
    CHECK_EQ(perimeterOf({{0, 0}, {4, 0}, {0, 3}}), 12.0);
    CHECK_EQ(perimeterOf({{0, 0}, {2, 0}, {2, 2}, {0, 2}}), 8.0);
}

TEST(CentroidOfTriangleAndQuad) {
    CHECK_EQ(centroidOf({{0, 0}, {4, 0}, {2, 3}}), (Point{2.0, 1.0}));
    CHECK_EQ(centroidOf({{1, 1}, {2, 2}, {3, 3}, {4, 4}}), (Point{2.5, 2.5}));
}

TEST(IsConvexDistinguishesSquareFromDart) {
    CHECK(isConvex({{0, 0}, {2, 0}, {2, 2}, {0, 2}}));         // square
    CHECK(isConvex({{0, 0}, {4, 0}, {0, 3}}));                  // triangle
    CHECK(!isConvex({{0, 0}, {4, 0}, {4, 4}, {2, 2}, {0, 4}})); // dart/arrow
}

TEST(FarthestFromFindsMaxDistance) {
    Point origin{0, 0};
    std::vector<Point> points = {{1, 0}, {0, 5}, {6, 8}};
    CHECK_EQ(farthestFrom(origin, points), (Point{6.0, 8.0}));
}

TEST(ClassifyTriangleBySideLength) {
    CHECK_EQ(classifyTriangle({0, 0}, {4, 0}, {2, 3}), std::string("isosceles"));
    CHECK_EQ(classifyTriangle({0, 0}, {4, 0}, {1, 2}), std::string("scalene"));

    double s3 = std::sqrt(3.0);
    CHECK_EQ(classifyTriangle({0, 0}, {2, 0}, {1, s3}), std::string("equilateral"));
}

TEST_MAIN()
