# 11. Dates & Internationalization

## Creating dates

```js
new Date();                          // now
new Date(1700000000000);             // from epoch milliseconds
new Date("2024-01-15");              // parsed as UTC midnight
new Date("2024-01-15T00:00:00");     // parsed as LOCAL time — no Z/offset!
new Date(2024, 0, 15);               // year, MONTH (0-indexed!), day — local time
new Date(2024, 0, 15, 9, 30);        // + hours, minutes, ...
```

**Month is 0-indexed** (`0` = January, `11` = December) in the constructor
and in `getMonth`/`setMonth` — but `parseISODateOnly` in this topic's
exercises returns a human 1-12 month deliberately, to contrast with this.

## The string-parsing gotcha

```js
new Date("2024-01-15").getUTCHours();   // 0  — parsed as UTC
new Date("2024-01-15T00:00:00").getHours(); // 0 in local time, but the
                                              // underlying instant differs
                                              // from the line above by your
                                              // UTC offset.
```

A **date-only** ISO string (`"YYYY-MM-DD"`) is parsed as **UTC midnight**. A
string with a time but **no timezone** (`"...T00:00:00"`) is parsed as
**local time**. Mixing the two formats when comparing dates silently
introduces an offset-sized bug. Prefer constructing dates explicitly
(`new Date(year, month, day)`) or always including a `Z`/offset.

## Getting & setting components

```js
const d = new Date(2024, 0, 15, 10, 30, 0);
d.getFullYear();  // 2024
d.getMonth();     // 0 (January)
d.getDate();      // 15 (day of month, 1-31)
d.getDay();       // 0-6 (day of week, 0 = Sunday)
d.getHours();     // 10
d.getTime();      // ms since epoch
Date.now();       // current ms since epoch, no Date object needed
```

Every getter has a `getUTC*` counterpart (`getUTCFullYear`, `getUTCDate`,
...) that reads the value in UTC instead of the local timezone — use these
for timezone-independent calendar logic.

## Setters roll over

`setDate`/`setMonth`/etc. roll over to adjacent months/years when given an
out-of-range value — this is how you add days/months safely:

```js
const d = new Date(2024, 0, 31); // Jan 31, 2024
d.setDate(d.getDate() + 1);      // rolls to Feb 1, 2024 (Jan has 31 days)

const d2 = new Date(2024, 0, 15);
d2.setMonth(d2.getMonth() + 13); // rolls 13 months forward -> Feb 2025
```

`setDate(0)` rolls back to the **last day of the previous month** — useful
for "end of last month" calculations.

## Comparing dates

```js
const a = new Date(2024, 0, 1);
const b = new Date(2024, 0, 2);

a < b;        // true  — Date coerces to its numeric timestamp for < > <= >=
a === b;      // false — even for equal instants, objects compare by identity
a.getTime() === b.getTime(); // correct way to compare instants
+a;           // numeric timestamp via unary + (calls valueOf)
b - a;        // 86400000 — subtraction gives a millisecond difference (number)
```

## `Intl` for locale-aware formatting

`toString()`/`toISOString()` produce fixed formats. For human-facing output,
use the `Intl` namespace, which is locale-aware:

```js
new Intl.DateTimeFormat("en-US", { dateStyle: "medium" }).format(new Date(2024, 0, 15));
// "Jan 15, 2024"

new Intl.DateTimeFormat("en-US", { year: "numeric", month: "long", day: "numeric" })
  .format(new Date(2024, 0, 15));
// "January 15, 2024"

new Intl.NumberFormat("en-US", { style: "currency", currency: "USD" }).format(1234.5);
// "$1,234.50"

new Intl.NumberFormat("de-DE", { style: "currency", currency: "EUR" }).format(1234.5);
// "1.234,50 €"  — different grouping/decimal separators AND symbol placement

new Intl.NumberFormat("en-US", { style: "currency", currency: "JPY" }).format(1234.5);
// "¥1,235" — JPY has 0 minor units, so it rounds
```

`Intl.RelativeTimeFormat` produces "in 3 days" / "2 hours ago" style strings
from a numeric offset + unit — useful for activity feeds, but exact wording
is locale/ICU-dependent so it's not exercised here with strict assertions.

## Further Reading (MDN)

- [Representing dates and times](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Representing_dates_times)
- [Internationalization](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Internationalization)
- [`Date`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date)
- [`Intl.DateTimeFormat`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat)
- [`Intl.NumberFormat`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat)
