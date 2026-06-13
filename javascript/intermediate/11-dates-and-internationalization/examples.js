// Run with: node examples.js

// --- Creating dates ---
console.log("Date.now():", Date.now());
console.log("new Date(2024, 0, 15):", new Date(2024, 0, 15).toString());
console.log("new Date('2024-01-15') UTC hours:", new Date("2024-01-15").getUTCHours());
console.log("new Date('2024-01-15T00:00:00') local hours:", new Date("2024-01-15T00:00:00").getHours());

// --- Getting components ---
const d = new Date(2024, 0, 15, 10, 30, 0);
console.log("getFullYear:", d.getFullYear());
console.log("getMonth (0-indexed):", d.getMonth());
console.log("getDate:", d.getDate());
console.log("getDay (0=Sunday):", d.getDay());
console.log("getHours:", d.getHours());
console.log("getTime:", d.getTime());

// --- Setters roll over ---
const endOfJan = new Date(2024, 0, 31);
endOfJan.setDate(endOfJan.getDate() + 1);
console.log("Jan 31 + 1 day rolls to:", endOfJan.toDateString());

const lastDayOfPrevMonth = new Date(2024, 2, 0); // March, day 0
console.log("setDate(0) on March -> last day of Feb:", lastDayOfPrevMonth.toDateString());

const rolledMonths = new Date(2024, 0, 15);
rolledMonths.setMonth(rolledMonths.getMonth() + 13);
console.log("Jan 2024 + 13 months:", rolledMonths.toDateString());

// --- Comparing dates ---
const a = new Date(2024, 0, 1);
const b = new Date(2024, 0, 2);
console.log("a < b:", a < b);
console.log("a === b (same instant, different objects):", a.getTime() === new Date(2024, 0, 1).getTime());
console.log("b - a (ms difference):", b - a);
console.log("+a (numeric timestamp):", +a);

// --- Intl.DateTimeFormat ---
const sample = new Date(2024, 0, 15);
console.log(
  "Intl.DateTimeFormat dateStyle medium:",
  new Intl.DateTimeFormat("en-US", { dateStyle: "medium" }).format(sample),
);
console.log(
  "Intl.DateTimeFormat long month:",
  new Intl.DateTimeFormat("en-US", { year: "numeric", month: "long", day: "numeric" }).format(sample),
);

// --- Intl.NumberFormat currency ---
console.log("USD:", new Intl.NumberFormat("en-US", { style: "currency", currency: "USD" }).format(1234.5));
console.log("EUR (de-DE):", new Intl.NumberFormat("de-DE", { style: "currency", currency: "EUR" }).format(1234.5));
console.log("JPY (no decimals):", new Intl.NumberFormat("en-US", { style: "currency", currency: "JPY" }).format(1234.5));

// --- Intl.RelativeTimeFormat (wording is locale/ICU-dependent) ---
const rtf = new Intl.RelativeTimeFormat("en-US", { numeric: "auto" });
console.log("rtf.format(-1, 'day'):", rtf.format(-1, "day"));
console.log("rtf.format(3, 'hour'):", rtf.format(3, "hour"));
