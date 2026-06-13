# 04. Control Flow

## `if` / `else if` / `else`

```js
if (score >= 90) {
  grade = "A";
} else if (score >= 80) {
  grade = "B";
} else {
  grade = "C";
}
```

The condition is converted to boolean using truthy/falsy rules
([Topic 01](../01-variables-and-data-types/notes.md)).

## `switch`

Compares with **strict equality (`===`)**, no coercion.

```js
switch (dayIndex) {
  case 0:
    return "Sunday";
  case 1:
    return "Monday";
  default:
    return "Invalid day";
}
```

- Without `break`, execution **falls through** to the next case.
- `return` inside a `case` exits immediately — no `break` needed.
- Falling through intentionally (shared code for multiple cases) is a valid
  pattern:

```js
switch (fruit) {
  case "apple":
  case "pear":
    return "pome fruit"; // shared for both cases
  default:
    return "unknown";
}
```

## Choosing `if` vs `switch` vs ternary

- **Ternary**: single, simple either/or value.
- **`if`/`else if`**: range checks (`>=`, `<`), multiple `&&`/`||` conditions.
- **`switch`**: one value against many discrete possibilities.

## Combining conditions

```js
if (age >= 13 && age <= 19) { /* teenager */ }
if (status === "admin" || status === "owner") { /* elevated access */ }
```

`&&`/`||` short-circuit — see [Topic 02](../02-operators-and-type-coercion/notes.md).

## Further Reading (MDN)

- [Control flow and error handling](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Control_flow_and_error_handling)
- [`if...else`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/if...else)
- [`switch`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/switch)
