#include "exercise.h"

// All four function-template exercises (swapViaMove, rotateLeft,
// forwardingRefKind, firstElement) are fully defined in exercise.h --
// template definitions must be visible at the point of instantiation, same
// reasoning as intermediate/01. Only IntBuffer's member functions are
// defined here.

IntBuffer::IntBuffer(size_t size) {
    (void)size;
    throw std::logic_error("not implemented");
}

IntBuffer::IntBuffer(const IntBuffer& other) {
    (void)other;
    throw std::logic_error("not implemented");
}

IntBuffer::IntBuffer(IntBuffer&& other) {
    (void)other;
    throw std::logic_error("not implemented");
}

IntBuffer& IntBuffer::operator=(const IntBuffer& other) {
    (void)other;
    throw std::logic_error("not implemented");
}

IntBuffer& IntBuffer::operator=(IntBuffer&& other) {
    (void)other;
    throw std::logic_error("not implemented");
}

IntBuffer::~IntBuffer() { delete[] data_; }

size_t IntBuffer::size() const { throw std::logic_error("not implemented"); }

int& IntBuffer::operator[](size_t i) {
    (void)i;
    throw std::logic_error("not implemented");
}

const int& IntBuffer::operator[](size_t i) const {
    (void)i;
    throw std::logic_error("not implemented");
}

long IntBuffer::sum() const { throw std::logic_error("not implemented"); }
