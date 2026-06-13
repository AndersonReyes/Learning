import { test, describe } from "node:test";
import assert from "node:assert/strict";
import {
  tokenize,
  extractLinks,
  redact,
  renderTemplate,
  parseLogLine,
} from "./exercise.js";

describe("tokenize", () => {
  test("tokenizes a full expression with numbers, identifiers, operators, and parens", () => {
    assert.deepEqual(tokenize("3.14 * (x + 42)"), [
      { type: "number", value: "3.14" },
      { type: "operator", value: "*" },
      { type: "paren", value: "(" },
      { type: "identifier", value: "x" },
      { type: "operator", value: "+" },
      { type: "number", value: "42" },
      { type: "paren", value: ")" },
    ]);
  });

  test("empty and whitespace-only input produce no tokens", () => {
    assert.deepEqual(tokenize(""), []);
    assert.deepEqual(tokenize("   \t  "), []);
  });

  test("consecutive operators each become their own token", () => {
    assert.deepEqual(tokenize("--"), [
      { type: "operator", value: "-" },
      { type: "operator", value: "-" },
    ]);
  });

  test("identifiers may contain digits and underscores after the first character", () => {
    assert.deepEqual(tokenize("var_1 % 2"), [
      { type: "identifier", value: "var_1" },
      { type: "operator", value: "%" },
      { type: "number", value: "2" },
    ]);
  });

  test("an underscore-prefixed identifier is recognized", () => {
    assert.deepEqual(tokenize("_foo + 1"), [
      { type: "identifier", value: "_foo" },
      { type: "operator", value: "+" },
      { type: "number", value: "1" },
    ]);
  });

  test("an invalid character throws an Error mentioning that character", () => {
    assert.throws(() => tokenize("3 $ 4"), /\$/);
  });
});

describe("extractLinks", () => {
  test("extracts a single link with text and url", () => {
    assert.deepEqual(extractLinks("See [MDN](https://developer.mozilla.org) for docs."), [
      { text: "MDN", url: "https://developer.mozilla.org" },
    ]);
  });

  test("extracts multiple links in order", () => {
    assert.deepEqual(extractLinks("[a](1) middle [b](2)"), [
      { text: "a", url: "1" },
      { text: "b", url: "2" },
    ]);
  });

  test("handles empty text and/or empty url", () => {
    assert.deepEqual(extractLinks("[docs]() and []( /path )"), [
      { text: "docs", url: "" },
      { text: "", url: " /path " },
    ]);
  });

  test("returns an empty array when there are no links", () => {
    assert.deepEqual(extractLinks("no links here"), []);
  });

  test("url may contain query-string characters", () => {
    assert.deepEqual(
      extractLinks("[search](https://example.com?q=test&page=1)"),
      [{ text: "search", url: "https://example.com?q=test&page=1" }],
    );
  });
});

describe("redact", () => {
  test("redacts an email and a space-separated card number", () => {
    assert.equal(
      redact("Contact john@example.com or pay with 4111 1111 1111 1111."),
      "Contact [EMAIL] or pay with [CARD].",
    );
  });

  test("redacts multiple emails", () => {
    assert.equal(redact("From a@b.com to c@d.org"), "From [EMAIL] to [EMAIL]");
  });

  test("redacts a hyphen-separated card number", () => {
    assert.equal(redact("Card: 4111-1111-1111-1111"), "Card: [CARD]");
  });

  test("redacts a card number with no separators", () => {
    assert.equal(redact("4111111111111111"), "[CARD]");
  });

  test("leaves text with no sensitive data unchanged", () => {
    assert.equal(redact("Hello world"), "Hello world");
  });

  test("does not redact short digit runs like phone numbers", () => {
    assert.equal(redact("Call 555-1234"), "Call 555-1234");
  });

  test("redacts an email with a multi-part domain and a plus sign", () => {
    assert.equal(
      redact("Reach jane.doe+test@sub.example.co.uk anytime"),
      "Reach [EMAIL] anytime",
    );
  });
});

describe("renderTemplate", () => {
  test("substitutes top-level and nested placeholders", () => {
    assert.equal(
      renderTemplate("Hello {{user.name}}, you have {{count}} messages.", {
        user: { name: "Ada" },
        count: 3,
      }),
      "Hello Ada, you have 3 messages.",
    );
  });

  test("tolerates whitespace inside the braces", () => {
    assert.equal(renderTemplate("{{ name }}", { name: "Grace" }), "Grace");
  });

  test("returns the template unchanged when there are no placeholders", () => {
    assert.equal(renderTemplate("no placeholders", {}), "no placeholders");
  });

  test("throws an Error mentioning the path for a missing top-level key", () => {
    assert.throws(() => renderTemplate("{{missing}}", {}), /missing/);
  });

  test("throws an Error mentioning the full path for a missing nested key", () => {
    assert.throws(() => renderTemplate("{{a.b.c}}", { a: { b: {} } }), /a\.b\.c/);
  });

  test("stringifies non-string values", () => {
    assert.equal(renderTemplate("{{flag}} and {{n}}", { flag: true, n: 0 }), "true and 0");
  });
});

describe("parseLogLine", () => {
  test("parses a well-formed log line into its parts", () => {
    assert.deepEqual(
      parseLogLine("[2024-01-15T10:30:00Z] ERROR (auth-service): login failed"),
      {
        timestamp: "2024-01-15T10:30:00Z",
        level: "ERROR",
        service: "auth-service",
        message: "login failed",
      },
    );
  });

  test("returns null for a line that doesn't match the format", () => {
    assert.equal(parseLogLine("not a log line"), null);
  });

  test("supports an empty message", () => {
    assert.deepEqual(parseLogLine("[2024-01-15T10:30:00Z] INFO (api): "), {
      timestamp: "2024-01-15T10:30:00Z",
      level: "INFO",
      service: "api",
      message: "",
    });
  });

  test("trims leading and trailing whitespace from the message", () => {
    assert.deepEqual(
      parseLogLine("[2024-01-15T10:30:00Z] WARN (svc):    trailing spaces   "),
      {
        timestamp: "2024-01-15T10:30:00Z",
        level: "WARN",
        service: "svc",
        message: "trailing spaces",
      },
    );
  });

  test("supports other log levels", () => {
    assert.deepEqual(parseLogLine("[2024-01-15T10:30:00Z] DEBUG (worker): tick"), {
      timestamp: "2024-01-15T10:30:00Z",
      level: "DEBUG",
      service: "worker",
      message: "tick",
    });
  });
});
