// Spec for exercise.h/exercise.cpp. No test framework yet (that's built in
// fundamentals/06) -- just assert(). Compile and run:
//
//   g++ -std=c++20 -Wall -Wextra -o /tmp/test exercise_test.cpp exercise.cpp && /tmp/test
//
// Every assert() must pass. If one fails, the program aborts and prints the
// failing file:line -- fix exercise.cpp and re-run.

#include "exercise.h"

#include <climits>
#include <cassert>
#include <iostream>

static void testCountSetBits() {
    assert(countSetBits(0xFFu) == 8);
    assert(countSetBits(1) == 1);
    assert(countSetBits(0) == 0);
    assert(countSetBits(0xFFFFFFFFu) == 32);
    assert(countSetBits(0x80000000u) == 1);
    assert(countSetBits(0b10110110u) == 5);
}

static void testAverageNoOverflow() {
    assert(averageNoOverflow(5, 3) == 4);
    assert(averageNoOverflow(7, 8) == 7);
    assert(averageNoOverflow(-7, -8) == -8);
    assert(averageNoOverflow(-5, -3) == -4);
    assert(averageNoOverflow(-1, 0) == -1);
    assert(averageNoOverflow(INT_MAX, INT_MAX) == INT_MAX);
    assert(averageNoOverflow(INT_MAX, INT_MAX - 1) == INT_MAX - 1);
}

static void testRotateLeft8() {
    assert(rotateLeft8(0b00000001, 1) == 0b00000010);
    assert(rotateLeft8(0b10000000, 1) == 0b00000001);
    assert(rotateLeft8(0b11110000, 4) == 0b00001111);
    assert(rotateLeft8(0b00001111, 4) == 0b11110000);
    assert(rotateLeft8(0b10110110, 0) == 0b10110110);
    assert(rotateLeft8(0b10110110, 8) == 0b10110110);
    assert(rotateLeft8(0b00000001, -1) == 0b10000000);
    assert(rotateLeft8(0b11000000, 2) == 0b00000011);
}

static void testNarrowToByte() {
    assert(narrowToByte(255) == 255);
    assert(narrowToByte(0) == 0);
    assert(narrowToByte(256) == 0);
    assert(narrowToByte(257) == 1);
    assert(narrowToByte(-1) == 255);
    assert(narrowToByte(300) == 44);
    assert(narrowToByte(-300) == 212);
    assert(narrowToByte(1000) == 232);
}

static void testSignExtend() {
    assert(signExtend(0b1101, 4) == -3);
    assert(signExtend(0b0011, 4) == 3);
    assert(signExtend(1, 1) == -1);
    assert(signExtend(0, 1) == 0);
    assert(signExtend(0b01111111, 8) == 127);
    assert(signExtend(0b10000000, 8) == -128);
    assert(signExtend(0b11111111, 8) == -1);
}

int main() {
    testCountSetBits();
    testAverageNoOverflow();
    testRotateLeft8();
    testNarrowToByte();
    testSignExtend();
    std::cout << "All tests passed!\n";
    return 0;
}
