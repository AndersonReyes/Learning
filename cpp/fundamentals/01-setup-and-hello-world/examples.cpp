// Run with:
//   g++ -std=c++20 -Wall -Wextra -o /tmp/ex examples.cpp && /tmp/ex

#include <iomanip>
#include <iostream>
#include <sstream>
#include <string>

int main() {
    // --- Anatomy of a minimal program -------------------------------------
    std::cout << "Hello, world!\n";

    // '\n' vs std::endl: '\n' just writes a newline; std::endl also flushes
    // the stream's buffer. Prefer '\n' unless you specifically need a flush.
    std::cout << "line one\n";
    std::cout << "line two" << std::endl;

    // std::cerr is unbuffered -- useful for diagnostics that must appear
    // even if the program crashes right after.
    std::cerr << "(diagnostic) starting manipulator demo\n";

    // --- <iomanip> manipulators ---------------------------------------------
    int value = 4000;

    // std::hex/std::dec/std::oct and std::uppercase are "sticky": once set,
    // they apply to every subsequent operation until changed.
    std::cout << "decimal: " << value << "\n";
    std::cout << std::hex << "hex:     " << value << "\n";
    std::cout << std::uppercase << "HEX:     " << value << "\n";
    std::cout << std::dec << "back to decimal: " << value << "\n";

    // std::setw applies only to the NEXT field; std::setfill is sticky.
    std::cout << std::setfill('0');
    std::cout << std::setw(6) << 42 << "\n";  // "000042"
    std::cout << std::setw(6) << 7 << "\n";   // "000007"
    std::cout << 99 << "\n";                  // setw already consumed: "99"

    // std::fixed + std::setprecision: exact decimal places (both sticky).
    double pi = 3.14159265358979;
    std::cout << "default precision: " << pi << "\n";
    std::cout << std::fixed << std::setprecision(2) << "fixed(2): " << pi << "\n";
    std::cout << std::setprecision(5) << "fixed(5): " << pi << "\n";

    // std::left/std::right justify padded fields (sticky).
    std::cout << std::setfill(' ') << std::left << std::setw(10) << "left" << "|\n";
    std::cout << std::right << std::setw(10) << "right" << "|\n";

    // std::fixed/setprecision are sticky -- reset them before continuing,
    // otherwise the price printed below would inherit "fixed(5)" and show
    // as "3.50000" instead of "3.5".
    std::cout << std::defaultfloat << std::setprecision(6);

    // --- String streams (<sstream>) -----------------------------------------
    // ostringstream: build a string piece by piece, then call .str().
    std::ostringstream out;
    out << "user=" << "alice" << ", id=" << 42;
    std::string built = out.str();
    std::cout << "\nbuilt string: " << built << "\n";

    // istringstream: split/parse a line into typed fields.
    std::istringstream in("100 3.5 widget");
    int count;
    double price;
    std::string name;
    in >> count >> price >> name;
    std::cout << "parsed: count=" << count << ", price=" << price
              << ", name=" << name << "\n";

    // --- Stream state & the extraction-failure gotcha ------------------------
    // ">>" into an int stops at the first non-numeric character. If the very
    // next token isn't numeric at all, failbit is set and that character is
    // NOT consumed -- clear() + ignore(1) is needed to move past it.
    std::istringstream mixed("10 oops 20");
    int a = 0;
    int b = 0;

    mixed >> a;  // reads 10
    std::cout << "\nfirst value: " << a << "\n";

    if (!(mixed >> b)) {
        std::cout << "extraction failed on non-numeric token\n";
        mixed.clear();          // clear failbit so the stream is usable again
        std::string skipped;
        mixed >> skipped;       // consume the whole "oops" token
        std::cout << "skipped token: " << skipped << "\n";
    }

    mixed >> b;  // now reads 20
    std::cout << "second value: " << b << "\n";

    return 0;
}
