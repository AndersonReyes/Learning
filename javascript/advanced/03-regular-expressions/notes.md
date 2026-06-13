# Advanced 03. Regular Expressions

## Creating a regex

- Literal: `/pattern/flags` — compiled once, preferred for static patterns.
- Constructor: `new RegExp(pattern, flags)` — needed when the pattern is
  built from a variable (string interpolation).

```js
const re1 = /\d+/g;
const re2 = new RegExp(`\\d{${n}}`, "g"); // dynamic pattern
```

## Flags

| Flag | Meaning |
|---|---|
| `g` | global — find all matches, not just the first |
| `i` | case-insensitive |
| `m` | multiline — `^`/`$` match start/end of each LINE |
| `s` | dotAll — `.` matches newlines too |
| `u` | unicode — treat pattern/string as code points (required for `\u{...}` and full Unicode property escapes) |
| `y` | sticky — match must start exactly at `lastIndex` |

## Character classes & quantifiers

- `\d \w \s` / `\D \W \S` — digit/word/whitespace and their negations.
- `[abc]`, `[^abc]`, `[a-z0-9]` — custom classes / ranges / negation.
- `.` — any char except newline (unless `s` flag).
- Quantifiers: `*` `+` `?` `{n}` `{n,}` `{n,m}` — GREEDY by default (match as
  much as possible). Add `?` for LAZY: `.*?` matches as little as possible.
  ```js
  /".*"/.exec('"a" "b"')[0]; // '"a" "b"' (greedy spans both quotes)
  /".*?"/.exec('"a" "b"')[0]; // '"a"' (lazy stops at the first closing quote)
  ```

## Groups

- `(...)` — capturing group, available as `match[1]`, `match[2]`, ...
- `(?:...)` — non-capturing group (grouping without indexing).
- `(?<name>...)` — named capturing group, available as `match.groups.name`.
- Backreference: `\1` (by index) or `\k<name>` (by name) — matches the SAME
  text the group captured. E.g. `/(\w+) \1\b/` matches a repeated word
  ("the the").
- In `replace`, `$1` / `$<name>` insert captured groups into the replacement
  string.

## Assertions (zero-width — don't consume characters)

- `^` / `$` — start/end of string (or of each line, with `m`).
- `\b` / `\B` — word boundary / non-boundary.
- `(?=...)` — lookahead: must be followed by `...`.
- `(?!...)` — negative lookahead: must NOT be followed by `...`.
- `(?<=...)` — lookbehind: must be preceded by `...`.
- `(?<!...)` — negative lookbehind: must NOT be preceded by `...`.

```js
"40 dollars".match(/\d+(?= dollars)/)[0]; // "40" — digits followed by " dollars"
"price: $40".match(/(?<=\$)\d+/)[0]; // "40" — digits preceded by "$"
```

## Methods

| Method | On | Returns |
|---|---|---|
| `regex.test(str)` | RegExp | `boolean` — any match? |
| `regex.exec(str)` | RegExp | match array (with `.index`, `.groups`) or `null`. With `g`/`y`, advances `regex.lastIndex` — call in a loop to get every match |
| `str.match(regex)` | String | without `g`: same as one `exec`. With `g`: array of full-match STRINGS only (no groups!) |
| `str.matchAll(regex)` | String | iterator of full match arrays (groups included) — regex MUST have `g` |
| `str.replace(regex, repl)` | String | new string; `repl` can be a string (with `$1` / `$<name>`) or a `(match, ...groups, offset, string, namedGroups) => string` function |
| `str.replaceAll(regex, repl)` | String | like `replace`, but regex MUST have `g` |
| `str.split(regex)` | String | array split on matches; capturing groups in the regex are INCLUDED in the result |

## Gotchas

- **`lastIndex` is mutable state on the regex object** (only with `g`/`y`).
  Reusing the same global regex across calls without resetting `lastIndex`
  causes `test`/`exec` to silently skip ahead — especially dangerous for a
  regex declared at module scope and reused across calls.
  ```js
  const re = /a/g;
  re.test("aaa"); // true, lastIndex now 1
  re.test("aaa"); // true, lastIndex now 2
  re.test("aaa"); // true, lastIndex now 3
  re.test("aaa"); // false, lastIndex reset to 0
  ```
- **`str.match(re)` with `g` drops capture groups** — only full matches come
  back as strings. Use `matchAll` (with `g`) when you need groups for every
  match.
- **Unescaped special characters** (`. * + ? ( ) [ ] { } ^ $ | \`) in a
  dynamically-built pattern can break the regex or match too much — escape
  user input before interpolating into `new RegExp(...)`.
- **Catastrophic backtracking**: nested quantifiers like `(a+)+b` against a
  long non-matching input can blow up exponentially. Prefer specific
  character classes/anchors over broad `.*` for performance-sensitive regexes.
- A capturing group inside `str.split(regex)` is INCLUDED in the output
  array — easy to forget and get unexpected extra elements.
- `exec`/`test` on a NON-global regex never advance `lastIndex` — every call
  re-checks from the start, which is usually what you want for one-off
  `test()` calls.

## Further Reading (MDN)

- [Regular expressions guide](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Regular_expressions)
- [`RegExp`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/RegExp)
- [Groups and backreferences](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Regular_expressions/Groups_and_backreferences)
- [Assertions (lookahead/lookbehind)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Regular_expressions/Assertions)
- [`String.prototype.matchAll`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/matchAll)
- [`String.prototype.replace`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/replace)
