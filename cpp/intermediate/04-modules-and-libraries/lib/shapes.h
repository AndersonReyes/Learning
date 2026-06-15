#pragma once

#include <ostream>

// Topic 14 (Intermediate 04): Multi-File Projects, #include, Modules & Libraries
//
// This is a small internal "library" -- fully implemented (not exercises),
// living in its own lib/ subdirectory to demonstrate a multi-file project
// layout. exercise.h/.cpp build on top of it. See notes.md for the build
// command (it now spans three .cpp files and uses -I to find this header).

struct Point {
    double x;
    double y;
};

bool operator==(const Point& a, const Point& b);
std::ostream& operator<<(std::ostream& os, const Point& p);

// Euclidean distance between a and b.
double distance(const Point& a, const Point& b);
