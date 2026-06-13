import { describe, test } from "node:test";
import assert from "node:assert/strict";
import {
  triangleType,
  validatePassword,
  ticTacToeWinner,
  romanNumeral,
  leapYearCategory,
} from "./exercise.js";

describe("triangleType", () => {
  test("equilateral triangle", () => {
    assert.equal(triangleType(3, 3, 3), "equilateral");
  });

  test("isosceles triangle", () => {
    assert.equal(triangleType(3, 3, 4), "isosceles");
  });

  test("scalene triangle", () => {
    assert.equal(triangleType(3, 4, 5), "scalene");
  });

  test("invalid: fails triangle inequality", () => {
    assert.equal(triangleType(1, 1, 3), "invalid");
  });

  test("invalid: non-positive side", () => {
    assert.equal(triangleType(0, 1, 1), "invalid");
  });
});

describe("validatePassword", () => {
  test("a fully valid password", () => {
    assert.deepEqual(validatePassword("Abcdef1!"), { valid: true, errors: [] });
  });

  test("a password missing everything", () => {
    assert.deepEqual(validatePassword("abc"), {
      valid: false,
      errors: [
        "Password must be at least 8 characters",
        "Password must contain an uppercase letter",
        "Password must contain a digit",
      ],
    });
  });

  test("a password missing only a lowercase letter", () => {
    assert.deepEqual(validatePassword("ALLCAPS123"), {
      valid: false,
      errors: ["Password must contain a lowercase letter"],
    });
  });

  test("a valid password without special characters", () => {
    assert.deepEqual(validatePassword("Password1"), { valid: true, errors: [] });
  });
});

describe("ticTacToeWinner", () => {
  test("a winning row", () => {
    const board = [
      ["X", "X", "X"],
      ["O", "O", null],
      [null, null, null],
    ];
    assert.equal(ticTacToeWinner(board), "X");
  });

  test("a winning column", () => {
    const board = [
      ["X", "O", null],
      ["X", "O", null],
      ["X", null, null],
    ];
    assert.equal(ticTacToeWinner(board), "X");
  });

  test("a winning diagonal", () => {
    const board = [
      ["X", "O", null],
      ["O", "X", null],
      [null, null, "X"],
    ];
    assert.equal(ticTacToeWinner(board), "X");
  });

  test("no winner", () => {
    const board = [
      ["X", "O", "X"],
      ["O", "X", "O"],
      ["O", "X", "O"],
    ];
    assert.equal(ticTacToeWinner(board), null);
  });

  test("empty board", () => {
    const board = [
      [null, null, null],
      [null, null, null],
      [null, null, null],
    ];
    assert.equal(ticTacToeWinner(board), null);
  });
});

describe("romanNumeral", () => {
  test("single-symbol numbers", () => {
    assert.equal(romanNumeral(1), "I");
  });

  test("subtractive notation", () => {
    assert.equal(romanNumeral(4), "IV");
    assert.equal(romanNumeral(9), "IX");
  });

  test("a multi-part number", () => {
    assert.equal(romanNumeral(58), "LVIII");
  });

  test("a number using multiple subtractive pairs", () => {
    assert.equal(romanNumeral(1994), "MCMXCIV");
  });

  test("the maximum supported value", () => {
    assert.equal(romanNumeral(3999), "MMMCMXCIX");
  });
});

describe("leapYearCategory", () => {
  test("divisible by 4 but not 100 is a leap year", () => {
    assert.equal(leapYearCategory(2024), "leap year");
  });

  test("divisible by 100 but not 400 is a common year", () => {
    assert.equal(leapYearCategory(1900), "common year");
  });

  test("divisible by 400 is a leap year", () => {
    assert.equal(leapYearCategory(2000), "leap year");
  });

  test("not divisible by 4 is a common year", () => {
    assert.equal(leapYearCategory(2023), "common year");
  });
});
