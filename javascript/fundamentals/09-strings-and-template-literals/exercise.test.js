import { describe, test } from "node:test";
import assert from "node:assert/strict";
import {
  isPalindrome,
  titleCase,
  wordFrequency,
  template,
  caesarCipher,
} from "./exercise.js";

describe("isPalindrome", () => {
  test("ignores case and punctuation", () => {
    assert.equal(isPalindrome("A man, a plan, a canal: Panama"), true);
  });

  test("simple palindrome", () => {
    assert.equal(isPalindrome("racecar"), true);
  });

  test("non-palindrome", () => {
    assert.equal(isPalindrome("hello"), false);
  });

  test("empty string is a palindrome", () => {
    assert.equal(isPalindrome(""), true);
  });
});

describe("titleCase", () => {
  test("capitalizes each word", () => {
    assert.equal(titleCase("hello world"), "Hello World");
  });

  test("lowercases the rest of each word", () => {
    assert.equal(titleCase("THE QUICK BROWN FOX"), "The Quick Brown Fox");
  });

  test("collapses repeated whitespace and trims", () => {
    assert.equal(titleCase("  multiple   spaces  "), "Multiple Spaces");
  });
});

describe("wordFrequency", () => {
  test("counts words case-insensitively, ignoring punctuation", () => {
    assert.deepEqual(
      wordFrequency("The quick brown fox jumps over the lazy dog. The dog barks."),
      {
        the: 3,
        quick: 1,
        brown: 1,
        fox: 1,
        jumps: 1,
        over: 1,
        lazy: 1,
        dog: 2,
        barks: 1,
      },
    );
  });

  test("returns an empty object for an empty string", () => {
    assert.deepEqual(wordFrequency(""), {});
  });

  test("counts repeated single words", () => {
    assert.deepEqual(wordFrequency("a a a"), { a: 3 });
  });
});

describe("template", () => {
  test("replaces a single placeholder", () => {
    assert.equal(template("Hello, {{name}}!", { name: "Ada" }), "Hello, Ada!");
  });

  test("replaces multiple placeholders", () => {
    assert.equal(template("{{a}} + {{b}} = {{sum}}", { a: 1, b: 2, sum: 3 }), "1 + 2 = 3");
  });

  test("allows whitespace inside the braces", () => {
    assert.equal(template("{{ name }}", { name: "Bob" }), "Bob");
  });

  test("replaces a missing key with an empty string", () => {
    assert.equal(
      template("Hi {{name}}, you have {{count}} messages", { name: "X" }),
      "Hi X, you have  messages",
    );
  });
});

describe("caesarCipher", () => {
  test("shifts letters forward", () => {
    assert.equal(caesarCipher("abc", 1), "bcd");
  });

  test("wraps around the end of the alphabet", () => {
    assert.equal(caesarCipher("xyz", 3), "abc");
  });

  test("preserves case and leaves non-letters unchanged", () => {
    assert.equal(caesarCipher("Hello, World!", 5), "Mjqqt, Btwqi!");
  });

  test("supports a negative shift", () => {
    assert.equal(caesarCipher("abc", -1), "zab");
  });

  test("handles mixed case with wraparound", () => {
    assert.equal(caesarCipher("ABC xyz", 2), "CDE zab");
  });
});
