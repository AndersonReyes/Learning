#include <iostream>
#include <memory>
#include <string>
#include <vector>

// Topic 26-27 (Advanced 06): Software Design Principles, Idioms & Patterns
//
// Different illustrative examples than exercise.h's MaxStack/IdRegistry/
// Validator hierarchy/EventBus/Comparable<Version> -- same idioms, different
// code, so the exercises stay unspoiled.

// --- Rule of Zero: composition needs no custom special members ------------------------------------

struct Person {
    std::string name;
    std::vector<std::string> hobbies;
    // No destructor/copy/move written -- std::string and std::vector already
    // manage their own resources correctly, so the compiler-generated
    // special members (deep copy, efficient move) are exactly right.
};

// --- PIMPL: Logger hides its storage behind an incomplete Impl --------------------------------------

class Logger {
public:
    Logger();
    ~Logger();
    Logger(const Logger& other);
    Logger(Logger&& other) noexcept;
    Logger& operator=(const Logger& other);
    Logger& operator=(Logger&& other) noexcept;

    void log(std::string message);
    size_t entryCount() const;
    std::string lastEntry() const;

private:
    struct Impl;
    std::unique_ptr<Impl> impl_;
};

struct Logger::Impl {
    std::vector<std::string> entries;
};

Logger::Logger() : impl_(std::make_unique<Impl>()) {}
Logger::~Logger() = default;
Logger::Logger(const Logger& other) : impl_(std::make_unique<Impl>(*other.impl_)) {}
Logger::Logger(Logger&& other) noexcept : impl_(std::move(other.impl_)) {
    other.impl_ = std::make_unique<Impl>();
}
Logger& Logger::operator=(const Logger& other) {
    if (this != &other) impl_ = std::make_unique<Impl>(*other.impl_);
    return *this;
}
Logger& Logger::operator=(Logger&& other) noexcept {
    if (this != &other) {
        impl_ = std::move(other.impl_);
        other.impl_ = std::make_unique<Impl>();
    }
    return *this;
}
void Logger::log(std::string message) { impl_->entries.push_back(std::move(message)); }
size_t Logger::entryCount() const { return impl_->entries.size(); }
std::string Logger::lastEntry() const { return impl_->entries.empty() ? "" : impl_->entries.back(); }

// --- Singleton: Meyer's Singleton for a process-wide counter -----------------------------------------

class GlobalCounter {
public:
    static GlobalCounter& instance() {
        static GlobalCounter inst;
        return inst;
    }
    GlobalCounter(const GlobalCounter&) = delete;
    GlobalCounter& operator=(const GlobalCounter&) = delete;

    int increment() { return ++value_; }
    int value() const { return value_; }

private:
    GlobalCounter() = default;
    int value_ = 0;
};

// --- CRTP: instance-counting mixin ---------------------------------------------------------------

// Each Derived gets its OWN static counter, because count()'s static local
// belongs to the CountedInstances<Derived> instantiation -- Widget and
// Gadget below don't share a count.
template <typename Derived>
class CountedInstances {
public:
    CountedInstances() { ++count(); }
    CountedInstances(const CountedInstances&) { ++count(); }
    ~CountedInstances() { --count(); }
    static int liveCount() { return count(); }

private:
    static int& count() {
        static int c = 0;
        return c;
    }
};

class Widget : public CountedInstances<Widget> {};
class Gadget : public CountedInstances<Gadget> {};

// --- NVI / Template Method: Exporter hierarchy --------------------------------------------------

class Exporter {
public:
    virtual ~Exporter() = default;

    // NVI: fixed shape -- header, then one formatted line per row, then footer.
    std::string exportRows(const std::vector<std::vector<std::string>>& rows) const {
        std::string out = header();
        for (const auto& row : rows) out += formatRow(row);
        out += footer();
        return out;
    }

protected:
    virtual std::string header() const { return ""; }
    virtual std::string formatRow(const std::vector<std::string>& row) const = 0;
    virtual std::string footer() const { return ""; }
};

class CsvExporter : public Exporter {
protected:
    std::string formatRow(const std::vector<std::string>& row) const override {
        std::string line;
        for (size_t i = 0; i < row.size(); ++i) {
            if (i > 0) line += ",";
            line += row[i];
        }
        return line + "\n";
    }
};

class JsonExporter : public Exporter {
protected:
    std::string header() const override { return "[\n"; }
    std::string formatRow(const std::vector<std::string>& row) const override {
        std::string line = "  [";
        for (size_t i = 0; i < row.size(); ++i) {
            if (i > 0) line += ", ";
            line += "\"" + row[i] + "\"";
        }
        return line + "],\n";
    }
    std::string footer() const override { return "]\n"; }
};

int main() {
    std::cout << std::boolalpha;

    std::cout << "-- Rule of Zero: Person --\n";
    {
        Person p{"Ada", {"math", "machines"}};
        Person copy = p;  // compiler-generated deep copy
        copy.hobbies.push_back("writing");
        std::cout << "  original hobbies: " << p.hobbies.size() << ", copy hobbies: " << copy.hobbies.size()
                  << "\n";
    }

    std::cout << "\n-- PIMPL: Logger --\n";
    {
        Logger log1;
        log1.log("starting up");
        log1.log("listening on port 8080");

        Logger log2 = log1;  // deep copy
        log2.log("log2 only");

        std::cout << "  log1: " << log1.entryCount() << " entries, last=\"" << log1.lastEntry() << "\"\n";
        std::cout << "  log2: " << log2.entryCount() << " entries, last=\"" << log2.lastEntry() << "\"\n";
        std::cout << "  sizeof(Logger) == sizeof(void*): " << (sizeof(Logger) == sizeof(void*)) << "\n";
    }

    std::cout << "\n-- Singleton: GlobalCounter --\n";
    {
        GlobalCounter::instance().increment();
        GlobalCounter::instance().increment();
        GlobalCounter::instance().increment();
        std::cout << "  value = " << GlobalCounter::instance().value() << "\n";
        std::cout << "  same instance: " << (&GlobalCounter::instance() == &GlobalCounter::instance()) << "\n";
    }

    std::cout << "\n-- CRTP: instance-counting mixin --\n";
    {
        std::cout << "  Widget::liveCount() before: " << Widget::liveCount() << "\n";
        {
            Widget w1;
            Widget w2;
            Gadget g1;
            std::cout << "  Widget::liveCount() with 2 alive: " << Widget::liveCount() << "\n";
            std::cout << "  Gadget::liveCount() with 1 alive: " << Gadget::liveCount() << "\n";
        }
        std::cout << "  Widget::liveCount() after scope: " << Widget::liveCount() << "\n";
        std::cout << "  Gadget::liveCount() after scope: " << Gadget::liveCount() << "\n";
    }

    std::cout << "\n-- NVI / Template Method: Exporter --\n";
    {
        std::vector<std::vector<std::string>> rows = {{"id", "name"}, {"1", "Ada"}, {"2", "Grace"}};
        CsvExporter csv;
        JsonExporter json;
        std::cout << "  CSV:\n" << csv.exportRows(rows);
        std::cout << "  JSON:\n" << json.exportRows(rows);
    }
}
