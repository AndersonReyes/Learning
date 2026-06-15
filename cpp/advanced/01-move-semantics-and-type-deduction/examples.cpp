#include <iostream>
#include <string>
#include <type_traits>
#include <utility>
#include <vector>

// Topic 21 (Advanced 01): Move Semantics, Value Categories & Type Deduction
//
// Different illustrative examples than exercise.h's swapViaMove/rotateLeft/
// forwardingRefKind/firstElement/IntBuffer -- same concepts, different
// functions/classes, so the exercises stay unspoiled.

// --- Value categories: overload resolution picks lvalue vs. rvalue ref --------------------

void who(int&) { std::cout << "  int&       overload (lvalue)\n"; }
void who(int&&) { std::cout << "  int&&      overload (rvalue/xvalue)\n"; }
void who(const int&) { std::cout << "  const int& overload (fallback)\n"; }

// --- Move-only resource handle: Rule of 5 with copy DELETED ---------------------------------
//
// IntBuffer (exercise.h) is copyable AND movable. FileHandle is movable-only
// -- a common pattern for resources that can't (or shouldn't) be duplicated
// (file descriptors, sockets, mutex locks, ...).
class FileHandle {
public:
    explicit FileHandle(int fd) : fd_(fd) { std::cout << "  FileHandle(" << fd_ << ") opened\n"; }

    FileHandle(const FileHandle&) = delete;
    FileHandle& operator=(const FileHandle&) = delete;

    FileHandle(FileHandle&& other) noexcept : fd_(other.fd_) {
        other.fd_ = -1;
        std::cout << "  FileHandle move-constructed (fd=" << fd_ << ")\n";
    }
    FileHandle& operator=(FileHandle&& other) noexcept {
        if (this == &other) return *this;
        closeIfOpen();
        fd_ = other.fd_;
        other.fd_ = -1;
        std::cout << "  FileHandle move-assigned (fd=" << fd_ << ")\n";
        return *this;
    }
    ~FileHandle() { closeIfOpen(); }

    int fd() const { return fd_; }

private:
    void closeIfOpen() {
        if (fd_ >= 0) std::cout << "  FileHandle(" << fd_ << ") closed\n";
        fd_ = -1;
    }
    int fd_;
};

// --- Forwarding references + reference collapsing -------------------------------------------

template <typename T>
void describeForwardingRef(T&& /*x*/) {
    if constexpr (std::is_lvalue_reference_v<T>) {
        std::cout << "  T deduced as " << (std::is_const_v<std::remove_reference_t<T>> ? "const " : "")
                  << "T& -> lvalue argument\n";
    } else {
        std::cout << "  T deduced as plain T -> rvalue argument\n";
    }
}

// --- std::forward + perfect forwarding --------------------------------------------------------

struct Widget {
    Widget() { std::cout << "  Widget() default-constructed\n"; }
    Widget(const Widget&) { std::cout << "  Widget(const Widget&) copy-constructed\n"; }
    Widget(Widget&&) noexcept { std::cout << "  Widget(Widget&&) move-constructed\n"; }
};

// Forwards its argument to Widget's constructor preserving lvalue/rvalue-ness.
template <typename T>
Widget relayToWidget(T&& arg) {
    return Widget(std::forward<T>(arg));
}

// --- auto vs. decltype vs. decltype(auto) -------------------------------------------------------

int& firstOf(std::vector<int>& v) { return v[0]; }

// --- Trailing return type: decltype(c[i]) preserves reference-ness ------------------------------

template <typename Container>
auto at(Container& c, size_t i) -> decltype(c[i]) {
    return c[i];
}

// --- Copy elision / RVO -----------------------------------------------------------------------

Widget makeWidget() {
    return Widget();  // C++17: mandatory elision -- constructed directly in caller's storage
}

int main() {
    std::cout << "-- value categories: overload resolution --\n";
    {
        int x = 10;
        std::cout << "who(x):            "; who(x);              // lvalue -> int&
        std::cout << "who(5):            "; who(5);              // prvalue -> int&&
        std::cout << "who(std::move(x)): "; who(std::move(x));   // xvalue -> int&&
        const int cx = 20;
        std::cout << "who(cx):           "; who(cx);             // const lvalue -> const int&
    }

    std::cout << "\n-- move-only FileHandle (Rule of 5, copy deleted) --\n";
    {
        FileHandle a(3);
        FileHandle b = std::move(a);  // move ctor; a.fd() now -1
        std::cout << "a.fd()=" << a.fd() << " b.fd()=" << b.fd() << "\n";

        FileHandle c(4);
        c = std::move(b);  // move assignment; closes c's old fd=4 first
        std::cout << "b.fd()=" << b.fd() << " c.fd()=" << c.fd() << "\n";
        // a, b, c destroyed at end of scope -- only c (fd=3) prints "closed"
    }

    std::cout << "\n-- forwarding references + reference collapsing --\n";
    {
        int x = 5;
        const int cx = 10;
        std::cout << "describeForwardingRef(x):            "; describeForwardingRef(x);
        std::cout << "describeForwardingRef(5):            "; describeForwardingRef(5);
        std::cout << "describeForwardingRef(std::move(x)): "; describeForwardingRef(std::move(x));
        std::cout << "describeForwardingRef(cx):           "; describeForwardingRef(cx);
    }

    std::cout << "\n-- std::forward + perfect forwarding --\n";
    {
        Widget w;
        std::cout << "relayToWidget(w) (lvalue arg):\n";
        Widget w2 = relayToWidget(w);  // T = Widget&  -> forward<Widget&>(w) -> lvalue -> copy ctor
        std::cout << "relayToWidget(Widget{}) (rvalue arg):\n";
        Widget w3 = relayToWidget(Widget{});  // T = Widget -> forward<Widget>(arg) -> rvalue -> move ctor
        (void)w2;
        (void)w3;
    }

    std::cout << "\n-- auto vs. decltype vs. decltype(auto) --\n";
    {
        std::vector<int> nums = {1, 2, 3};

        auto a = firstOf(nums);          // int  -- auto decays the int& to int (a copy)
        decltype(auto) b = firstOf(nums); // int& -- decltype(auto) preserves the reference
        static_assert(std::is_same_v<decltype(a), int>);
        static_assert(std::is_same_v<decltype(b), int&>);

        a = 999;  // does not affect nums
        std::cout << "after a=999 (copy):      nums[0]=" << nums[0] << " (unchanged)\n";

        b = 100;  // mutates through the reference
        std::cout << "after b=100 (reference): nums[0]=" << nums[0] << "\n";
    }

    std::cout << "\n-- trailing return type: decltype(c[i]) --\n";
    {
        std::vector<int> nums = {10, 20, 30};
        at(nums, 1) = 222;  // at() returns int& -- assignable
        std::cout << "after at(nums,1)=222: nums[1]=" << nums[1] << "\n";

        const std::vector<int> cnums = {1, 2, 3};
        std::cout << "at(cnums, 0) = " << at(cnums, 0) << "\n";
        static_assert(std::is_same_v<decltype(at(nums, 0)), int&>);
        static_assert(std::is_same_v<decltype(at(cnums, 0)), const int&>);
    }

    std::cout << "\n-- copy elision / RVO --\n";
    {
        std::cout << "Widget w = makeWidget(): ";
        Widget w = makeWidget();  // prints only "Widget() default-constructed" -- no copy/move
        (void)w;
    }
}
