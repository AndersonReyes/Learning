// Run with:
//   g++ -std=c++20 -Wall -Wextra -o /tmp/ex examples.cpp && /tmp/ex

#include <cmath>
#include <iomanip>
#include <iostream>
#include <limits>

int main() {
    // --- <limits> ------------------------------------------------------------
    std::cout << "int range: [" << std::numeric_limits<int>::min() << ", "
              << std::numeric_limits<int>::max() << "]\n";
    std::cout << "unsigned int max: " << std::numeric_limits<unsigned int>::max()
              << "\n";
    std::cout << "double epsilon: " << std::numeric_limits<double>::epsilon()
              << "\n";

    // --- Unsigned wraparound is well-defined ---------------------------------
    unsigned int zero = 0;
    std::cout << "\n0u - 1 = " << (zero - 1) << "  (wraps to UINT_MAX)\n";
    std::cout << "UINT_MAX + 1 = "
              << (std::numeric_limits<unsigned int>::max() + 1u)
              << "  (wraps to 0)\n";

    // --- Detecting unsigned overflow after the fact --------------------------
    unsigned int a = 3000000000u;
    unsigned int b = 2000000000u;
    unsigned int sum = a + b;  // wraps
    bool overflowed = sum < a;
    std::cout << "\n" << a << " + " << b << " = " << sum
              << " (overflowed: " << std::boolalpha << overflowed << ")\n";

    // --- 0.1 + 0.2 != 0.3 -----------------------------------------------------
    double x = 0.1 + 0.2;
    std::cout << std::setprecision(17);
    std::cout << "\n0.1 + 0.2 = " << x << "\n";
    std::cout << "(0.1 + 0.2) == 0.3: " << (x == 0.3) << "\n";
    std::cout << "abs((0.1+0.2) - 0.3) <= epsilon: "
              << (std::abs(x - 0.3) <= std::numeric_limits<double>::epsilon())
              << "\n";
    std::cout << std::setprecision(6);  // restore default precision

    // --- Signed zero, infinity, NaN -------------------------------------------
    double posZero = 0.0;
    double negZero = -0.0;
    std::cout << "\n+0.0 == -0.0: " << (posZero == negZero) << "\n";
    std::cout << "signbit(+0.0): " << std::signbit(posZero)
              << ", signbit(-0.0): " << std::signbit(negZero) << "\n";

    double inf = std::numeric_limits<double>::infinity();
    std::cout << "1.0 / 0.0 == +inf: " << (1.0 / 0.0 == inf) << "\n";

    double nan = std::numeric_limits<double>::quiet_NaN();
    std::cout << "NaN == NaN: " << (nan == nan) << "\n";
    std::cout << "NaN != NaN: " << (nan != nan) << "\n";
    std::cout << "isnan(0.0 / 0.0): " << std::isnan(0.0 / 0.0) << "\n";

    // --- A huge value swallows a small one ------------------------------------
    double huge = 1e16;
    double tiny = 1.0;
    std::cout << "\n1e16 + 1.0 == 1e16: " << (huge + tiny == huge) << "\n";

    // --- Floating-point addition is not associative ----------------------------
    double p = 1e20;
    double q = -1e20;
    double r = 1.0;
    std::cout << "\n(p + q) + r = " << ((p + q) + r) << "\n";
    std::cout << "p + (q + r) = " << (p + (q + r)) << "\n";

    return 0;
}
