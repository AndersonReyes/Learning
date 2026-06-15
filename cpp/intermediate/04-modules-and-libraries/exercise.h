#pragma once

#include <stdexcept>
#include <string>
#include <vector>

#include "shapes.h"

// Topic 14 (Intermediate 04): Multi-File Projects, #include, Modules & Libraries
//
// This topic's project spans THREE .cpp files (a first for this track):
// exercise_test.cpp, exercise.cpp, AND lib/shapes.cpp (a small internal
// "library" of Point/distance, fully implemented -- not part of the
// exercises). Build with:
//
//   g++ -std=c++20 -Wall -Wextra -Ilib -o test exercise_test.cpp exercise.cpp lib/shapes.cpp && ./test
//
// `-Ilib` lets `#include "shapes.h"` (used by this header and by
// exercise_test.cpp, neither of which lives in lib/) resolve to
// lib/shapes.h. lib/shapes.cpp's own `#include "shapes.h"` resolves via the
// "same directory as the including file" rule, needing no -I.
//
// The five stubs below all build on shapes.h's Point/distance.
// Stub bodies throw std::logic_error("not implemented").

// --- perimeterOf -----------------------------------------------------------------------------
//
// Sum of distances between consecutive vertices, including the closing edge
// (last vertex back to the first). Precondition: vertices.size() >= 2.
//
// Example: perimeterOf({(0,0), (4,0), (0,3)}) == 12 (a 3-4-5 right triangle).
double perimeterOf(const std::vector<Point>& vertices);

// --- centroidOf -------------------------------------------------------------------------------
//
// The arithmetic mean of all vertex coordinates (the "vertex centroid",
// distinct from the area centroid). Precondition: !vertices.empty().
//
// Example: centroidOf({(0,0), (4,0), (2,3)}) == Point{2, 1}.
Point centroidOf(const std::vector<Point>& vertices);

// --- isConvex ---------------------------------------------------------------------------------
//
// True iff the polygon (>= 3 vertices, in order) is convex: the
// cross product of consecutive edge vectors has the same sign (or zero) at
// every vertex.
//
// Example: isConvex({(0,0),(2,0),(2,2),(0,2)}) == true (a square);
// isConvex({(0,0),(4,0),(4,4),(2,2),(0,4)}) == false (a dart/arrow shape).
bool isConvex(const std::vector<Point>& vertices);

// --- farthestFrom -----------------------------------------------------------------------------
//
// The point in `points` with the greatest distance from `origin`.
// Precondition: !points.empty().
//
// Example: farthestFrom({0,0}, {(1,0),(0,5),(6,8)}) == Point{6, 8}.
Point farthestFrom(const Point& origin, const std::vector<Point>& points);

// --- classifyTriangle -------------------------------------------------------------------------
//
// Classifies the triangle with vertices a, b, c by side length:
// "equilateral" (all three sides equal), "isosceles" (exactly two equal),
// or "scalene" (all different). Side lengths come from sqrt (shapes.h's
// distance), so compare them with a small epsilon, not exact ==.
//
// Example: classifyTriangle({0,0}, {4,0}, {2,3}) == "isosceles".
std::string classifyTriangle(const Point& a, const Point& b, const Point& c);
