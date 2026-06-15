#include "exercise.h"

// Each stub below returns a placeholder value so the program compiles and
// runs, but every assert() in exercise_test.cpp fails until you replace the
// body with a real implementation.

bool willAddOverflow(int a, int b) {
    (void)a;
    (void)b;
    return false;
}

std::optional<int> safeDivide(int a, int b) {
    (void)a;
    (void)b;
    return std::nullopt;
}

unsigned int saturatingAdd(unsigned int a, unsigned int b) {
    (void)a;
    (void)b;
    return 0;
}

std::string classifyFloat(double x) {
    (void)x;
    return "";
}

double kahanSum(const std::vector<double>& values) {
    (void)values;
    return 0.0;
}
