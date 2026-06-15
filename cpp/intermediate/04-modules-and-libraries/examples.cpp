#include <iostream>
#include <string>

// Topic 14 (Intermediate 04): Multi-File Projects, #include, Modules & Libraries
//
// examples.cpp is a single translation unit (per this track's convention), so
// it can't show a real multi-FILE project -- exercise.h/.cpp/lib/shapes.* (3
// .cpp files, built together) is this topic's live multi-file demonstration;
// see notes.md for that build command. This file instead covers two related
// ideas that ARE demonstrable in one TU: forward declarations (incomplete
// types) and extern "C" linkage for C libraries.

// --- Forward declaration: incomplete types ----------------------------------------------------

// A forward declaration: `Engine` is an incomplete type here -- the compiler
// knows it EXISTS (so `Engine*`/`Engine&` are valid types) but not its size or
// members. In a real multi-file project, Engine's full definition would live
// in its own header, #include'd only by the .cpp files that need to
// create/use Engine objects directly -- callers that only ever pass Engine*
// around (without dereferencing) don't need that header at all, reducing
// compile-time coupling.
class Engine;

// Fine with only the forward declaration above: a pointer type, no member
// access, no sizeof(Engine).
void describeEngine(const Engine* engine);

// --- extern "C": linkage for C libraries -------------------------------------------------------

// C++ "mangles" function names (encoding parameter types into the symbol) so
// overloads can coexist at the link level; C does not. To call a function
// compiled as C (e.g. declared in a C library's header), wrap its declaration
// in extern "C" so the compiler emits/expects the unmangled C name instead.
extern "C" {
int c_library_add(int a, int b);
}

// A definition, here, just so this file links and runs on its own. Normally
// this would live in a precompiled C library (a .a/.so); only the extern "C"
// declaration above would appear on the C++ side.
extern "C" int c_library_add(int a, int b) { return a + b; }

// Now Engine is a complete type -- everything above that only used `Engine*`
// still compiles unchanged.
class Engine {
public:
    explicit Engine(std::string name) : name_(std::move(name)) {}
    const std::string& name() const { return name_; }

private:
    std::string name_;
};

void describeEngine(const Engine* engine) {
    if (engine == nullptr) {
        std::cout << "(no engine)\n";
        return;
    }
    std::cout << "Engine: " << engine->name() << "\n";
}

int main() {
    std::cout << "-- Forward declaration (Engine* before Engine is defined) --\n";
    describeEngine(nullptr);
    Engine engine("V8");
    describeEngine(&engine);

    std::cout << "\n-- extern \"C\" linkage --\n";
    std::cout << "c_library_add(2, 3) = " << c_library_add(2, 3) << "\n";
}
