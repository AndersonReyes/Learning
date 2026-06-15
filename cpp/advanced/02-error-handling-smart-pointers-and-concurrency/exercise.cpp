#include "exercise.h"

#include <exception>

// --- BankAccount ---------------------------------------------------------------------------------

BankAccount::BankAccount(double initialBalance) {
    (void)initialBalance;
    throw std::logic_error("not implemented");
}

double BankAccount::balance() const { throw std::logic_error("not implemented"); }

void BankAccount::deposit(double amount) {
    (void)amount;
    throw std::logic_error("not implemented");
}

void BankAccount::withdraw(double amount) {
    (void)amount;
    throw std::logic_error("not implemented");
}

// --- Circle / Rectangle / totalArea / extractLargest ------------------------------------------

Circle::Circle(double radius) {
    (void)radius;
    throw std::logic_error("not implemented");
}

double Circle::area() const { throw std::logic_error("not implemented"); }

Rectangle::Rectangle(double width, double height) {
    (void)width;
    (void)height;
    throw std::logic_error("not implemented");
}

double Rectangle::area() const { throw std::logic_error("not implemented"); }

double totalArea(const std::vector<std::unique_ptr<Shape>>& shapes) {
    (void)shapes;
    throw std::logic_error("not implemented");
}

std::unique_ptr<Shape> extractLargest(std::vector<std::unique_ptr<Shape>>& shapes) {
    (void)shapes;
    throw std::logic_error("not implemented");
}

// --- WeakCache -------------------------------------------------------------------------------------

std::shared_ptr<std::string> WeakCache::get(const std::string& key) {
    (void)key;
    throw std::logic_error("not implemented");
}

size_t WeakCache::size() const { throw std::logic_error("not implemented"); }

// --- parallelSum -----------------------------------------------------------------------------------

long long parallelSum(const std::vector<int>& v, unsigned numThreads) {
    (void)v;
    (void)numThreads;
    throw std::logic_error("not implemented");
}

// --- firstSuccessfulResult --------------------------------------------------------------------------

int firstSuccessfulResult(std::vector<std::function<int()>> tasks) {
    (void)tasks;
    throw std::logic_error("not implemented");
}
