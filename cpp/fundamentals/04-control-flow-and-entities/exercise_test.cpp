// Spec for exercise.h/exercise.cpp. No test framework yet (that's built in
// fundamentals/06) -- just assert(). Compile and run:
//
//   g++ -std=c++20 -Wall -Wextra -o /tmp/test exercise_test.cpp exercise.cpp && /tmp/test
//
// Every assert() must pass. If one fails, the program aborts and prints the
// failing file:line -- fix exercise.cpp and re-run.

#include "exercise.h"

#include <cassert>
#include <iostream>

static void testRomanNumeral() {
    assert(romanNumeral(1) == "I");
    assert(romanNumeral(4) == "IV");
    assert(romanNumeral(9) == "IX");
    assert(romanNumeral(58) == "LVIII");
    assert(romanNumeral(1994) == "MCMXCIV");
    assert(romanNumeral(2024) == "MMXXIV");
    assert(romanNumeral(3999) == "MMMCMXCIX");
}

static void testCollatzSteps() {
    assert(collatzSteps(2) == 1);
    assert(collatzSteps(1) == 0);
    assert(collatzSteps(6) == 8);
    assert(collatzSteps(7) == 16);
    assert(collatzSteps(27) == 111);
}

static void testPlayRockPaperScissors() {
    assert(playRockPaperScissors(RPSMove::Rock, RPSMove::Scissors) == RPSResult::FirstWins);
    assert(playRockPaperScissors(RPSMove::Rock, RPSMove::Rock) == RPSResult::Tie);
    assert(playRockPaperScissors(RPSMove::Rock, RPSMove::Paper) == RPSResult::SecondWins);
    assert(playRockPaperScissors(RPSMove::Paper, RPSMove::Rock) == RPSResult::FirstWins);
    assert(playRockPaperScissors(RPSMove::Paper, RPSMove::Paper) == RPSResult::Tie);
    assert(playRockPaperScissors(RPSMove::Paper, RPSMove::Scissors) == RPSResult::SecondWins);
    assert(playRockPaperScissors(RPSMove::Scissors, RPSMove::Paper) == RPSResult::FirstWins);
    assert(playRockPaperScissors(RPSMove::Scissors, RPSMove::Scissors) == RPSResult::Tie);
    assert(playRockPaperScissors(RPSMove::Scissors, RPSMove::Rock) == RPSResult::SecondWins);
}

static void testMergeIntervals() {
    assert(mergeIntervals({{1, 3}, {2, 6}, {8, 10}, {15, 18}}) ==
           (std::vector<Interval>{{1, 6}, {8, 10}, {15, 18}}));
    assert(mergeIntervals({{1, 4}, {4, 5}}) == (std::vector<Interval>{{1, 5}}));
    assert(mergeIntervals({}) == std::vector<Interval>{});
    assert(mergeIntervals({{1, 4}}) == (std::vector<Interval>{{1, 4}}));
    assert(mergeIntervals({{1, 4}, {0, 4}}) == (std::vector<Interval>{{0, 4}}));
    assert(mergeIntervals({{1, 4}, {2, 3}}) == (std::vector<Interval>{{1, 4}}));
}

static void testDaysInMonth() {
    assert(daysInMonth(1, 2024) == 31);
    assert(daysInMonth(4, 2024) == 30);
    assert(daysInMonth(2, 2024) == 29);
    assert(daysInMonth(2, 2023) == 28);
    assert(daysInMonth(2, 1900) == 28);
    assert(daysInMonth(2, 2000) == 29);
    assert(daysInMonth(12, 2024) == 31);
    assert(daysInMonth(13, 2024) == 0);
    assert(daysInMonth(0, 2024) == 0);
}

int main() {
    testRomanNumeral();
    testCollatzSteps();
    testPlayRockPaperScissors();
    testMergeIntervals();
    testDaysInMonth();
    std::cout << "All tests passed!\n";
    return 0;
}
