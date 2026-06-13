// Run with: node examples.js

// --- if / else if / else ---
function classify(score) {
  if (score >= 90) {
    return "A";
  } else if (score >= 80) {
    return "B";
  } else {
    return "C or below";
  }
}
console.log("classify(95):", classify(95));
console.log("classify(85):", classify(85));
console.log("classify(50):", classify(50));

// --- switch with strict comparison ---
function dayName(dayIndex) {
  switch (dayIndex) {
    case 0:
      return "Sunday";
    case 1:
      return "Monday";
    case 2:
      return "Tuesday";
    case 3:
      return "Wednesday";
    case 4:
      return "Thursday";
    case 5:
      return "Friday";
    case 6:
      return "Saturday";
    default:
      return "Invalid day";
  }
}
console.log("dayName(0):", dayName(0));
console.log("dayName(6):", dayName(6));
console.log("dayName(7):", dayName(7));
console.log('dayName("0") (string, not number):', dayName("0")); // "Invalid day" — switch uses ===

// --- fallthrough (intentional) ---
function fruitCategory(fruit) {
  switch (fruit) {
    case "apple":
    case "pear":
      return "pome fruit";
    case "banana":
    case "mango":
      return "tropical fruit";
    default:
      return "unknown fruit";
  }
}
console.log("fruitCategory('apple'):", fruitCategory("apple"));
console.log("fruitCategory('pear'):", fruitCategory("pear"));
console.log("fruitCategory('banana'):", fruitCategory("banana"));
console.log("fruitCategory('kiwi'):", fruitCategory("kiwi"));

// --- combining conditions ---
function canRideRollercoaster(age, heightCm) {
  return age >= 8 && heightCm >= 120;
}
console.log("canRideRollercoaster(10, 130):", canRideRollercoaster(10, 130));
console.log("canRideRollercoaster(10, 110):", canRideRollercoaster(10, 110));
console.log("canRideRollercoaster(5, 140):", canRideRollercoaster(5, 140));

// --- FizzBuzz: a classic control-flow exercise ---
function fizzBuzz(n) {
  if (n % 15 === 0) return "FizzBuzz";
  if (n % 3 === 0) return "Fizz";
  if (n % 5 === 0) return "Buzz";
  return String(n);
}
for (const n of [1, 3, 5, 9, 10, 15]) {
  console.log(`fizzBuzz(${n}):`, fizzBuzz(n));
}
