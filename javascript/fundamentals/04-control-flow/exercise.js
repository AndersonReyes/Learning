/**
 * Classify a triangle given its three side lengths.
 *
 * - If any side <= 0, or the triangle inequality fails (the sum of the two
 *   shorter sides must be strictly greater than the longest side), return
 *   "invalid".
 * - If all three sides are equal, return "equilateral".
 * - If exactly two sides are equal, return "isosceles".
 * - Otherwise, return "scalene".
 *
 * @param {number} a
 * @param {number} b
 * @param {number} c
 * @returns {"invalid"|"equilateral"|"isosceles"|"scalene"}
 */
export function triangleType(a, b, c) {
  throw new Error("Not implemented");
}

/**
 * Validate a password against four rules:
 *  - at least 8 characters: "Password must be at least 8 characters"
 *  - contains an uppercase letter: "Password must contain an uppercase letter"
 *  - contains a lowercase letter: "Password must contain a lowercase letter"
 *  - contains a digit: "Password must contain a digit"
 *
 * Return an object listing every rule that was violated, in the order
 * above, and whether the password is valid overall.
 *
 * @param {string} password
 * @returns {{ valid: boolean, errors: string[] }}
 */
export function validatePassword(password) {
  throw new Error("Not implemented");
}

/**
 * Determine the winner of a tic-tac-toe board.
 *
 * `board` is a 3x3 array of rows, each cell being "X", "O", or null.
 * Return "X" or "O" if that player has 3 in a row (horizontally,
 * vertically, or diagonally), otherwise return null.
 *
 * @param {Array<Array<"X"|"O"|null>>} board
 * @returns {"X"|"O"|null}
 */
export function ticTacToeWinner(board) {
  throw new Error("Not implemented");
}

/**
 * Convert an integer (1-3999) to a Roman numeral string.
 *
 * romanNumeral(1)    -> "I"
 * romanNumeral(4)    -> "IV"
 * romanNumeral(9)    -> "IX"
 * romanNumeral(58)   -> "LVIII"
 * romanNumeral(1994) -> "MCMXCIV"
 *
 * @param {number} num
 * @returns {string}
 */
export function romanNumeral(num) {
  throw new Error("Not implemented");
}

/**
 * Classify a year as "leap year" or "common year" using the Gregorian
 * rule: divisible by 4 is a leap year, UNLESS divisible by 100, UNLESS
 * also divisible by 400 (which makes it a leap year again).
 *
 * leapYearCategory(2024) -> "leap year"   (divisible by 4, not 100)
 * leapYearCategory(1900) -> "common year" (divisible by 100, not 400)
 * leapYearCategory(2000) -> "leap year"   (divisible by 400)
 *
 * @param {number} year
 * @returns {"leap year"|"common year"}
 */
export function leapYearCategory(year) {
  throw new Error("Not implemented");
}
