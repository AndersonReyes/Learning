import { describe, test } from "node:test";
import assert from "node:assert/strict";
import {
  addDays,
  dateDiffInDays,
  formatDuration,
  parseISODateOnly,
  formatMoney,
} from "./exercise.js";

function ymd(date) {
  return [date.getFullYear(), date.getMonth(), date.getDate()];
}

describe("addDays", () => {
  test("adds days within the same month", () => {
    assert.deepEqual(ymd(addDays(new Date(2024, 0, 15), 5)), [2024, 0, 20]);
  });

  test("rolls over to the next month", () => {
    assert.deepEqual(ymd(addDays(new Date(2024, 0, 31), 1)), [2024, 1, 1]);
  });

  test("rolls over Feb 29 in a leap year", () => {
    assert.deepEqual(ymd(addDays(new Date(2024, 1, 29), 1)), [2024, 2, 1]);
  });

  test("rolls over Feb 28 in a non-leap year", () => {
    assert.deepEqual(ymd(addDays(new Date(2023, 1, 28), 1)), [2023, 2, 1]);
  });

  test("rolls back across a year boundary with negative days", () => {
    assert.deepEqual(ymd(addDays(new Date(2024, 0, 1), -1)), [2023, 11, 31]);
  });

  test("does not mutate the input date", () => {
    const original = new Date(2024, 0, 31);
    addDays(original, 1);
    assert.deepEqual(ymd(original), [2024, 0, 31]);
  });

  test("adding 0 days returns an equal but distinct date", () => {
    const original = new Date(2024, 0, 1);
    const result = addDays(original, 0);
    assert.equal(result.getTime(), original.getTime());
    assert.notEqual(result, original);
  });
});

describe("dateDiffInDays", () => {
  test("counts whole days between two dates", () => {
    assert.equal(dateDiffInDays(new Date(2024, 0, 1), new Date(2024, 0, 10)), 9);
  });

  test("is negative when b is before a", () => {
    assert.equal(dateDiffInDays(new Date(2024, 0, 10), new Date(2024, 0, 1)), -9);
  });

  test("ignores time-of-day, comparing calendar dates", () => {
    assert.equal(
      dateDiffInDays(new Date(2024, 0, 1, 23, 59), new Date(2024, 0, 2, 0, 1)),
      1,
    );
  });

  test("same calendar day with different times returns 0", () => {
    assert.equal(dateDiffInDays(new Date(2024, 0, 1, 5), new Date(2024, 0, 1, 20)), 0);
  });

  test("handles a leap-year February correctly", () => {
    assert.equal(dateDiffInDays(new Date(2024, 1, 28), new Date(2024, 2, 1)), 2);
  });

  test("handles a non-leap-year February correctly", () => {
    assert.equal(dateDiffInDays(new Date(2023, 1, 28), new Date(2023, 2, 1)), 1);
  });

  test("same date returns 0", () => {
    assert.equal(dateDiffInDays(new Date(2024, 5, 15), new Date(2024, 5, 15)), 0);
  });
});

describe("formatDuration", () => {
  test("zero milliseconds", () => {
    assert.equal(formatDuration(0), "0s");
  });

  test("sub-second durations round down to 0s", () => {
    assert.equal(formatDuration(500), "0s");
  });

  test("seconds only", () => {
    assert.equal(formatDuration(1000), "1s");
  });

  test("minutes and seconds", () => {
    assert.equal(formatDuration(61000), "1m 1s");
  });

  test("hours, minutes, and seconds", () => {
    assert.equal(formatDuration(3661000), "1h 1m 1s");
  });

  test("days and hours, with zero minutes/seconds omitted", () => {
    assert.equal(formatDuration(90000000), "1d 1h");
  });

  test("exactly one day, all smaller units omitted", () => {
    assert.equal(formatDuration(86400000), "1d");
  });
});

describe("parseISODateOnly", () => {
  test("parses a valid date in a leap year", () => {
    assert.deepEqual(parseISODateOnly("2024-02-29"), { year: 2024, month: 2, day: 29 });
  });

  test("throws for Feb 29 in a non-leap year", () => {
    assert.throws(() => parseISODateOnly("2023-02-29"));
  });

  test("year 2000 (divisible by 400) is a leap year", () => {
    assert.deepEqual(parseISODateOnly("2000-02-29"), { year: 2000, month: 2, day: 29 });
  });

  test("year 1900 (divisible by 100, not 400) is not a leap year", () => {
    assert.throws(() => parseISODateOnly("1900-02-29"));
  });

  test("throws for a day exceeding the month's length", () => {
    assert.throws(() => parseISODateOnly("2024-04-31"));
  });

  test("throws for month 13", () => {
    assert.throws(() => parseISODateOnly("2024-13-01"));
  });

  test("throws for month 0", () => {
    assert.throws(() => parseISODateOnly("2024-00-10"));
  });

  test("throws for day 0", () => {
    assert.throws(() => parseISODateOnly("2024-01-00"));
  });

  test("throws for non-zero-padded input", () => {
    assert.throws(() => parseISODateOnly("2024-1-5"));
  });

  test("throws for a non-date string", () => {
    assert.throws(() => parseISODateOnly("not-a-date"));
  });

  test("throws for an ISO string with a time component", () => {
    assert.throws(() => parseISODateOnly("2024-01-15T00:00:00"));
  });

  test("returns a 1-indexed month (December)", () => {
    assert.deepEqual(parseISODateOnly("2024-12-25"), { year: 2024, month: 12, day: 25 });
  });
});

describe("formatMoney", () => {
  test("formats USD in en-US", () => {
    assert.equal(formatMoney(1234.5, "USD"), "$1,234.50");
  });

  test("formats EUR in en-US", () => {
    assert.equal(formatMoney(1234.5, "EUR"), "€1,234.50");
  });

  test("formats JPY (no decimal places) in en-US", () => {
    assert.equal(formatMoney(1234.5, "JPY"), "¥1,235");
  });

  test("formats zero", () => {
    assert.equal(formatMoney(0, "USD"), "$0.00");
  });

  test("formats negative amounts", () => {
    assert.equal(formatMoney(-50, "USD"), "-$50.00");
  });

  test("formats EUR in de-DE locale (different separators and symbol placement)", () => {
    assert.equal(formatMoney(1234.5, "EUR", "de-DE"), "1.234,50 €");
  });
});
