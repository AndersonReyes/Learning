// Run with: node examples.js

// --- Flags: g, i, m, s ---
console.log("=== Flags ===");
console.log("aAaA".match(/a/g)); // ['a', 'a'] -- g: all matches
console.log("aAaA".match(/a/gi)); // ['a', 'A', 'a', 'A'] -- i: case-insensitive
console.log("line1\nline2".match(/^line\d/gm)); // ['line1', 'line2'] -- m: ^/$ per line
console.log(/a.b/s.test("a\nb")); // true -- s: . matches newline too

// --- Greedy vs lazy quantifiers ---
console.log("\n=== Greedy vs lazy ===");
console.log(/".*"/.exec('"a" "b"')[0]); // '"a" "b"' -- greedy spans both
console.log(/".*?"/.exec('"a" "b"')[0]); // '"a"' -- lazy stops at first close

// --- Named groups + backreferences ---
console.log("\n=== Named groups & backreferences ===");
const dateMatch = "2024-01-15".match(/(?<year>\d{4})-(?<month>\d{2})-(?<day>\d{2})/);
console.log(dateMatch.groups); // { year: '2024', month: '01', day: '15' }
console.log(/(\w+) \1\b/.test("the the quick")); // true -- repeated word via \1

// --- Lookahead / lookbehind ---
console.log("\n=== Lookahead & lookbehind ===");
console.log("40 dollars".match(/\d+(?= dollars)/)[0]); // "40"
console.log("price: $40".match(/(?<=\$)\d+/)[0]); // "40"
console.log("foobar".match(/foo(?!baz)/)[0]); // "foo" -- not followed by "baz"

// --- exec loop with a global regex (and the lastIndex gotcha) ---
console.log("\n=== exec loop & lastIndex ===");
{
  const wordPattern = /\w+/g;
  const words = [];
  let match;
  while ((match = wordPattern.exec("the quick brown fox")) !== null) {
    words.push(match[0]);
  }
  console.log(words); // ['the', 'quick', 'brown', 'fox']
}
{
  const re = /a/g;
  console.log(re.test("aaa"), re.lastIndex); // true 1
  console.log(re.test("aaa"), re.lastIndex); // true 2
  console.log(re.test("aaa"), re.lastIndex); // true 3
  console.log(re.test("aaa"), re.lastIndex); // false 0 -- wrapped around
}

// --- match (with/without g) vs matchAll ---
console.log("\n=== match vs matchAll ===");
console.log("a1 b2 c3".match(/(\w)(\d)/)); // single match, includes groups
console.log("a1 b2 c3".match(/(\w)(\d)/g)); // ['a1', 'b2', 'c3'] -- groups dropped
console.log(
  [..."a1 b2 c3".matchAll(/(\w)(\d)/g)].map((m) => [m[1], m[2]]),
); // [['a','1'],['b','2'],['c','3']] -- matchAll keeps groups

// --- replace with $1 / named groups / a function ---
console.log("\n=== replace ===");
console.log("2024-01-15".replace(/(\d{4})-(\d{2})-(\d{2})/, "$3/$2/$1")); // 15/01/2024
console.log(
  "2024-01-15".replace(
    /(?<year>\d{4})-(?<month>\d{2})-(?<day>\d{2})/,
    "$<month>/$<day>/$<year>",
  ),
); // 01/15/2024
console.log("hello world".replace(/\b\w/g, (c) => c.toUpperCase())); // "Hello World"

// --- split with a capturing group ---
console.log("\n=== split ===");
console.log("a, b;c, d".split(/([,;])\s*/)); // includes the captured separators
