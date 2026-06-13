/**
 * Return a NEW `Date` that is `date` advanced by `days` calendar days
 * (`days` may be negative). Does not mutate `date`. Handles month/year
 * rollover correctly (including leap years), using local-time semantics.
 *
 * addDays(new Date(2024, 0, 31), 1);  // -> Date(2024, 1, 1)  (Feb 1)
 * addDays(new Date(2024, 1, 29), 1);  // -> Date(2024, 2, 1)  (2024 is leap)
 * addDays(new Date(2023, 1, 28), 1);  // -> Date(2023, 2, 1)  (2023 not leap)
 * addDays(new Date(2024, 0, 1), -1);  // -> Date(2023, 11, 31)
 *
 * @param {Date} date
 * @param {number} days
 * @returns {Date}
 */
export function addDays(date, days) {
  throw new Error("Not implemented");
}

/**
 * Return the number of whole CALENDAR days between `a` and `b` (`b - a`),
 * based on each date's year/month/day components — ignoring time-of-day.
 * Negative if `b` is before `a`. Computed via `Date.UTC` so it's
 * DST-independent.
 *
 * dateDiffInDays(new Date(2024, 0, 1), new Date(2024, 0, 10)); // -> 9
 * dateDiffInDays(new Date(2024, 0, 10), new Date(2024, 0, 1)); // -> -9
 *
 * // ignores time-of-day: only ~2 minutes apart in real time, but different
 * // calendar days:
 * dateDiffInDays(new Date(2024, 0, 1, 23, 59), new Date(2024, 0, 2, 0, 1)); // -> 1
 *
 * // same calendar day, different times -> 0
 * dateDiffInDays(new Date(2024, 0, 1, 5), new Date(2024, 0, 1, 20)); // -> 0
 *
 * @param {Date} a
 * @param {Date} b
 * @returns {number}
 */
export function dateDiffInDays(a, b) {
  throw new Error("Not implemented");
}

/**
 * Format a duration of `ms` milliseconds as a human-readable string of
 * non-zero units from largest to smallest: days (`d`), hours (`h`), minutes
 * (`m`), seconds (`s`), space-separated. Sub-second remainders are dropped
 * (floored). Zero-valued units are omitted — EXCEPT if the entire duration
 * is zero (or rounds down to zero seconds), in which case returns `"0s"`.
 *
 * formatDuration(0);        // -> "0s"
 * formatDuration(500);      // -> "0s"   (rounds down to 0 seconds)
 * formatDuration(1000);     // -> "1s"
 * formatDuration(61000);    // -> "1m 1s"
 * formatDuration(3661000);  // -> "1h 1m 1s"
 * formatDuration(90000000); // -> "1d 1h"  (= 25h exactly, no leftover m/s)
 * formatDuration(86400000); // -> "1d"
 *
 * @param {number} ms
 * @returns {string}
 */
export function formatDuration(ms) {
  throw new Error("Not implemented");
}

/**
 * Parse a STRICT `"YYYY-MM-DD"` date-only string (4-digit year, 2-digit
 * month, 2-digit day, all zero-padded) into `{ year, month, day }`, where
 * `month` is 1-12 (human-readable, NOT JS's 0-indexed `Date` month).
 *
 * Throws an `Error` if:
 * - the string doesn't match `/^\d{4}-\d{2}-\d{2}$/` exactly
 * - `month` is not in `1..12`
 * - `day` is not valid for that month/year (accounting for leap years —
 *   divisible by 4, except centuries unless divisible by 400)
 *
 * parseISODateOnly("2024-02-29"); // -> { year: 2024, month: 2, day: 29 } (2024 leap)
 * parseISODateOnly("2023-02-29"); // throws (2023 not leap)
 * parseISODateOnly("2000-02-29"); // -> { year: 2000, month: 2, day: 29 } (div by 400)
 * parseISODateOnly("1900-02-29"); // throws (div by 100, not 400)
 * parseISODateOnly("2024-04-31"); // throws (April has 30 days)
 * parseISODateOnly("2024-1-5");   // throws (not zero-padded)
 * parseISODateOnly("garbage");    // throws
 *
 * @param {string} str
 * @returns {{ year: number, month: number, day: number }}
 */
export function parseISODateOnly(str) {
  throw new Error("Not implemented");
}

/**
 * Format `amount` as currency using `Intl.NumberFormat` with
 * `{ style: "currency", currency: currencyCode }`, in the given `locale`
 * (default `"en-US"`).
 *
 * formatMoney(1234.5, "USD");            // -> "$1,234.50"
 * formatMoney(1234.5, "EUR");            // -> "€1,234.50"
 * formatMoney(1234.5, "JPY");            // -> "¥1,235" (JPY has 0 decimals)
 * formatMoney(0, "USD");                 // -> "$0.00"
 * formatMoney(-50, "USD");               // -> "-$50.00"
 * formatMoney(1234.5, "EUR", "de-DE");   // -> "1.234,50 €"
 *
 * @param {number} amount
 * @param {string} currencyCode
 * @param {string} [locale]
 * @returns {string}
 */
export function formatMoney(amount, currencyCode, locale = "en-US") {
  throw new Error("Not implemented");
}
