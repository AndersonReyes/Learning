# 09. Strings & Template Literals

## Strings are immutable

```js
const greeting = "hello";
greeting.toUpperCase(); // "HELLO"
greeting;               // "hello" — unchanged
```

## Common methods

```js
const text = "  Hello, World!  ";

text.trim();              // "Hello, World!"
text.toUpperCase();       // "  HELLO, WORLD!  "
text.toLowerCase();       // "  hello, world!  "

text.includes("World");   // true
text.indexOf("World");    // 9 (-1 if not found)

text.slice(2, 7);          // "Hello" — [start, end)
text.slice(-1);            // " "

"a,b,c".split(",");        // ["a", "b", "c"]
["a", "b", "c"].join("-"); // "a-b-c"

text.replace("World", "JS");  // first match only
text.replaceAll("l", "L");    // all matches

"5".padStart(3, "0"); // "005"
"5".padEnd(3, "0");   // "500"
```

## Template literals

```js
const name = "Ada", age = 30;

`${name} is ${age} years old.`          // interpolation, any expression
`Next year, ${name} will be ${age + 1}.`

const message = `Line one
Line two`; // multi-line, no \n needed

const items = ["a", "b"];
`Items: ${items.map((i) => `[${i}]`).join(", ")}`; // nesting -> "Items: [a], [b]"
```

Prefer template literals over `+` concatenation once more than one value is
involved — fewer quotes, easier to read.

## Char codes

```js
"a".charCodeAt(0);       // 97
String.fromCharCode(97); // "a"
```

Useful for shifting letters (e.g. a Caesar cipher): convert a char to its
code, do arithmetic, convert back.

## Regular expressions with strings

```js
/[^a-z0-9]/gi.test("a-b"); // true — contains a non-alphanumeric char

"a-b-c".replace(/-/g, "_");  // "a_b_c" — g flag replaces all matches
"hello  world".split(/\s+/); // ["hello", "world"] — split on whitespace runs

// replace with a function: called per match, return value is the replacement
"{{name}}".replace(/\{\{(\w+)\}\}/, (match, key) => key.toUpperCase()); // "NAME"
```

`\w` = word character (`[A-Za-z0-9_]`), `\s` = whitespace, `g` flag = match
all occurrences (not just the first).

## Further Reading (MDN)

- [String reference](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String)
- [Template literals](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Template_literals)
- [`String.prototype.replace`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/replace)
- [`String.prototype.split`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/split)
- [`RegExp`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/RegExp)
