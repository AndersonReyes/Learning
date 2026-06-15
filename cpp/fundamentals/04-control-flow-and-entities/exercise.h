#pragma once

#include <string>
#include <vector>

// Converts `n` (1 <= n <= 3999) to a Roman numeral using the standard
// subtractive notation. Examples:
//   romanNumeral(1)    -> "I"
//   romanNumeral(58)   -> "LVIII"
//   romanNumeral(1994) -> "MCMXCIV"
std::string romanNumeral(int n);

// Returns the number of steps for `n` (n >= 1) to reach 1 under the Collatz
// rules: if n is even, n -> n/2; if n is odd, n -> 3n+1. collatzSteps(1) is
// 0 (already at 1). Examples:
//   collatzSteps(1) -> 0
//   collatzSteps(2) -> 1
//   collatzSteps(6) -> 8
int collatzSteps(int n);

// Rock-paper-scissors moves and outcomes.
enum class RPSMove { Rock, Paper, Scissors };
enum class RPSResult { Tie, FirstWins, SecondWins };

// Determines the outcome of `first` vs `second` under standard
// rock-paper-scissors rules (Rock beats Scissors, Scissors beats Paper,
// Paper beats Rock). Examples:
//   playRockPaperScissors(RPSMove::Rock, RPSMove::Rock)     -> RPSResult::Tie
//   playRockPaperScissors(RPSMove::Rock, RPSMove::Scissors) -> RPSResult::FirstWins
//   playRockPaperScissors(RPSMove::Rock, RPSMove::Paper)    -> RPSResult::SecondWins
RPSResult playRockPaperScissors(RPSMove first, RPSMove second);

// A closed integer interval [low, high].
struct Interval {
    int low;
    int high;
    bool operator==(const Interval&) const = default;
};

// Merges overlapping (and touching) intervals, returning the result sorted
// by `low`. Input intervals may be unsorted and may overlap arbitrarily.
// Two intervals [a,b] and [c,d] are merged if they overlap OR touch
// (b >= c, after sorting). Examples:
//   mergeIntervals({{1,3},{2,6},{8,10},{15,18}}) -> {{1,6},{8,10},{15,18}}
//   mergeIntervals({{1,4},{4,5}})                -> {{1,5}}
std::vector<Interval> mergeIntervals(std::vector<Interval> intervals);

// Returns the number of days in `month` (1-12) for the Gregorian `year`,
// applying the leap-year rule (divisible by 4, except centuries unless also
// divisible by 400). Returns 0 for an invalid month. Examples:
//   daysInMonth(2, 2024) -> 29  (2024 is a leap year)
//   daysInMonth(2, 1900) -> 28  (divisible by 100, not by 400)
//   daysInMonth(2, 2000) -> 29  (divisible by 400)
int daysInMonth(int month, int year);
