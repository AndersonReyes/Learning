#pragma once

// Counts the number of set (1) bits in `n`, using only bitwise operators —
// no std::popcount, no compiler builtins. Examples:
//   countSetBits(0)           -> 0
//   countSetBits(0xFFu)       -> 8
//   countSetBits(0x80000000u) -> 1
int countSetBits(unsigned int n);

// Returns floor((a + b) / 2.0) without ever computing the intermediate sum
// `a + b` (which could overflow `int`), using only bitwise operators. Must
// be correct for negative operands too — C++20 guarantees that `>>` on a
// negative signed int is an arithmetic (sign-extending) shift. Examples:
//   averageNoOverflow(5, 3)   -> 4
//   averageNoOverflow(7, 8)   -> 7
//   averageNoOverflow(-7, -8) -> -8
int averageNoOverflow(int a, int b);

// Rotates the 8 bits of `value` left by `shift` positions. `shift` may be
// negative (rotates right) or have absolute value >= 8 (wraps modulo 8).
// Examples:
//   rotateLeft8(0b00000001, 1)  -> 0b00000010
//   rotateLeft8(0b10000000, 1)  -> 0b00000001
//   rotateLeft8(0b00000001, -1) -> 0b10000000
unsigned char rotateLeft8(unsigned char value, int shift);

// Converts `value` to unsigned char following the standard's modulo-2^8
// narrowing-conversion rule for unsigned destination types (well-defined for
// any long value, unlike signed arithmetic overflow). Examples:
//   narrowToByte(255)  -> 255
//   narrowToByte(256)  -> 0
//   narrowToByte(-1)   -> 255
//   narrowToByte(300)  -> 44
unsigned char narrowToByte(long value);

// Treats the low `bits` bits of `value` as a two's-complement signed integer
// of that width and sign-extends it to a full `int`. `bits` is in [1, 32].
// Examples:
//   signExtend(0b1101, 4) -> -3   (4-bit 1101 is -3 in two's complement)
//   signExtend(0b0011, 4) -> 3
//   signExtend(1, 1)      -> -1
int signExtend(unsigned int value, int bits);
