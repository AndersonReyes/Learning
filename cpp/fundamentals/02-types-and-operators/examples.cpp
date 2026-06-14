// Run with:
//   g++ -std=c++20 -Wall -Wextra -o /tmp/ex examples.cpp && /tmp/ex

#include <bitset>
#include <cstdint>
#include <iostream>
#include <string>

int main() {
    // --- Fundamental types & sizeof -----------------------------------------
    std::cout << "sizeof(bool)=" << sizeof(bool)
              << " sizeof(char)=" << sizeof(char)
              << " sizeof(short)=" << sizeof(short)
              << " sizeof(int)=" << sizeof(int)
              << " sizeof(long)=" << sizeof(long)
              << " sizeof(long long)=" << sizeof(long long) << "\n";
    std::cout << "sizeof(float)=" << sizeof(float)
              << " sizeof(double)=" << sizeof(double) << "\n";

    // <cstdint> fixed-width types: guaranteed bit widths, useful when the
    // exact size matters (binary formats, bit manipulation).
    std::uint8_t byte = 200;
    std::int32_t word = -1;
    std::cout << "sizeof(uint8_t)=" << sizeof(byte)
              << " sizeof(int32_t)=" << sizeof(word) << "\n";

    // --- Integer promotion --------------------------------------------------
    // unsigned char operands are promoted to int before '+' is applied, so
    // this addition does NOT wrap at 256 -- the result is a full int.
    unsigned char a = 200;
    unsigned char b = 100;
    auto sum = a + b;  // promoted to int: sum == 300, not 44
    std::cout << "\n200 + 100 as unsigned char operands (promoted to int): "
              << sum << "\n";
    // To get an 8-bit wraparound result, cast back explicitly:
    unsigned char wrapped = static_cast<unsigned char>(a + b);
    std::cout << "static_cast<unsigned char>(200 + 100): "
              << static_cast<int>(wrapped) << "\n";

    // --- Signed/unsigned comparison gotcha -----------------------------------
    int negative = -1;
    unsigned int positive = 5u;
    // -Wextra warns here (-Wsign-compare): 'negative' converts to unsigned
    // before the comparison, becoming UINT_MAX, so this is FALSE.
    bool result = static_cast<unsigned int>(negative) < positive;
    std::cout << "\nstatic_cast<unsigned int>(-1) < 5u: " << std::boolalpha
              << result << "\n";
    std::cout << "static_cast<unsigned int>(-1) == "
              << static_cast<unsigned int>(negative) << "\n";

    // --- static_cast narrowing ------------------------------------------------
    double pi = 3.99;
    int truncated = static_cast<int>(pi);  // truncates toward zero, not rounds
    std::cout << "\nstatic_cast<int>(3.99) = " << truncated << "\n";
    std::cout << "static_cast<unsigned char>(-1) = "
              << static_cast<int>(static_cast<unsigned char>(-1)) << "\n";

    // --- Bitwise operators ----------------------------------------------------
    unsigned int flags = 0b0000;
    constexpr unsigned int READ = 0b001;
    constexpr unsigned int WRITE = 0b010;
    constexpr unsigned int EXEC = 0b100;

    flags |= READ | WRITE;          // set READ and WRITE
    std::cout << "\nflags after setting READ|WRITE: 0b"
              << std::bitset<3>(flags) << "\n";

    bool canWrite = (flags & WRITE) != 0;  // must parenthesize: & binds
                                            // looser than !=
    std::cout << "canWrite: " << canWrite << "\n";

    flags &= ~WRITE;  // clear WRITE
    std::cout << "flags after clearing WRITE: 0b"
              << std::bitset<3>(flags) << "\n";

    flags ^= EXEC;  // toggle EXEC on
    std::cout << "flags after toggling EXEC: 0b"
              << std::bitset<3>(flags) << "\n";

    // --- Arithmetic vs logical right shift ------------------------------------
    int negOne = -1;
    unsigned int uMax = 0xFFFFFFFFu;
    std::cout << "\n(-1) >> 1 = " << (negOne >> 1)
              << " (arithmetic shift: sign bit replicated, C++20-guaranteed)\n";
    std::cout << "(0xFFFFFFFFu) >> 1 = " << (uMax >> 1)
              << " (logical shift: fills with 0)\n";

    // --- Short-circuit evaluation ----------------------------------------------
    auto noisyTrue = []() {
        std::cout << "(noisyTrue called) ";
        return true;
    };
    auto noisyFalse = []() {
        std::cout << "(noisyFalse called) ";
        return false;
    };
    std::cout << "\nfalse && noisyTrue(): ";
    bool r1 = false && noisyTrue();  // noisyTrue() is never called
    std::cout << "-> " << r1 << "\n";
    std::cout << "true || noisyFalse(): ";
    bool r2 = true || noisyFalse();  // noisyFalse() is never called
    std::cout << "-> " << r2 << "\n";

    // --- Ternary chain ----------------------------------------------------------
    for (int score : {95, 82, 71, 40}) {
        std::string grade = score >= 90 ? "A"
                           : score >= 80 ? "B"
                           : score >= 70 ? "C"
                           :               "F";
        std::cout << "\nscore " << score << " -> grade " << grade;
    }
    std::cout << "\n";

    return 0;
}
