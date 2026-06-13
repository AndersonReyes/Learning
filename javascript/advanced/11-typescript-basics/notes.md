# Advanced 11. TypeScript Basics

> Adapted scope: TypeScript is a separate language/toolchain (not testable via
> `node:test` without a build step, and not part of MDN's JS docs).
> `exercise.js`/`exercise.test.js` stay plain `.js`: they implement the
> **runtime patterns** that correspond to what TypeScript checks
> **statically** — type guards, discriminated unions, exhaustiveness checks,
> structural validation, and generic-style constraints. TS code snippets below
> show the equivalent typed syntax (not run). "Further Reading" links to the
> TypeScript Handbook instead of MDN.

## What TypeScript is

TypeScript = JavaScript + a static type system, checked at compile time and
**erased** before running (the output is plain JS — types have zero runtime
cost). `tsc` (or a bundler with a TS plugin) type-checks, then strips types.

```ts
function add(a: number, b: number): number {
  return a + b;
}
add(1, "2"); // compile error: Argument of type 'string' is not assignable
             // to parameter of type 'number'.
```

At runtime, none of this exists — `add` is just `function add(a, b) { return a + b; }`.
**Types describe shapes JS values already have; they don't change JS behavior.**

## Basic types

```ts
let id: number = 1;
let name: string = "Ada";
let active: boolean = true;
let tags: string[] = ["a", "b"];       // array of strings
let pair: [string, number] = ["x", 1]; // tuple: fixed-length, per-position types

// object type ("interface" is the named/reusable form)
interface User {
  id: number;
  name: string;
  nickname?: string; // optional property
}
```

`interface` (and `type`) describe **shape** — TS uses **structural typing**:
any object with a compatible shape matches, regardless of how it was created
(no explicit "implements" needed, unlike Java/C#).

## Union, intersection & literal types

```ts
type Id = string | number;        // union: either type
type Role = "admin" | "user";     // literal union -- like an enum of strings
type Tagged = { id: number } & { createdAt: Date }; // intersection: both
```

A variable typed `Role` can ONLY be assigned `"admin"` or `"user"` — anything
else is a compile error. This is the TS analog of the `isLiteral`/`isUnion`
guards in `exercise.js`.

## Type narrowing

TS narrows a union type within a branch based on runtime checks:

```ts
function describe(value: string | number) {
  if (typeof value === "string") {
    return value.toUpperCase(); // TS knows value: string here
  }
  return value.toFixed(2); // TS knows value: number here
}
```

`typeof`, `instanceof`, `in`, `Array.isArray`, and equality checks against
literals all narrow. **This IS the type guard pattern** — `exercise.js`'s
`createTypeGuards()` builds the runtime predicates (`isString`, `isNumber`,
...) that TS narrows on automatically when their logic is inlined.

### User-defined type guards

```ts
function isUser(value: unknown): value is User {
  return typeof value === "object" && value !== null && "id" in value && "name" in value;
}
```

The `value is User` return type is a **type predicate**: after
`if (isUser(x))`, TS treats `x` as `User` in that branch. At runtime this is
just a function returning `boolean` — exactly `exercise.js`'s guards.

## Discriminated unions

A union of object types that share a common literal-typed field (the
**discriminant** / **tag**), letting TS narrow exhaustively in a `switch`:

```ts
type Shape =
  | { kind: "circle"; radius: number }
  | { kind: "rectangle"; width: number; height: number };

function area(shape: Shape): number {
  switch (shape.kind) {
    case "circle":
      return Math.PI * shape.radius ** 2; // TS knows: { kind: "circle", radius }
    case "rectangle":
      return shape.width * shape.height;  // TS knows: { kind: "rectangle", ... }
  }
}
```

This is exactly the shape `exercise.js`'s `match(value, handlers)` dispatches
on at runtime, using `value.kind` to pick the handler.

## Exhaustiveness checking (`never`)

`never` = "a type with no possible values" — used to make the compiler flag
unhandled union cases:

```ts
function area(shape: Shape): number {
  switch (shape.kind) {
    case "circle": return Math.PI * shape.radius ** 2;
    case "rectangle": return shape.width * shape.height;
    default:
      const _exhaustive: never = shape; // compile error if a case is missing
      throw new Error(`Unhandled kind: ${(shape as any).kind}`);
  }
}
```

If a new variant (`{ kind: "triangle"; ... }`) is added to `Shape` but no
`case` handles it, `shape` in the `default` branch is no longer `never` —
**compile error**, caught before running. `exercise.js`'s `match()` does the
runtime equivalent: throws if `handlers` has no entry for `value.kind`.

## Generics

A generic is a type-level parameter — code that works over a TYPE the same
way a function parameter works over a VALUE:

```ts
function first<T>(items: T[]): T | undefined {
  return items[0];
}
first<number>([1, 2, 3]); // T = number, returns number | undefined
first(["a", "b"]);        // T inferred as string

class TypedList<T> {
  private items: T[] = [];
  push(item: T): void { this.items.push(item); }
}
const numbers = new TypedList<number>();
```

`exercise.js`'s `createTypedList(guard, typeName)` is the runtime analog: TS's
`TypedList<T>` rejects non-`T` values **at compile time** (you can't even
write `numbers.push("x")`); the runtime version rejects them with a thrown
`TypeError` at the `push()` call.

## `Result<T, E>` — typed error handling without exceptions

A common TS pattern: instead of `throw`, return a discriminated union
representing success or failure, forcing callers to handle both:

```ts
type Result<T, E> = { kind: "ok"; value: T } | { kind: "err"; error: E };

function parseInt10(s: string): Result<number, string> {
  const n = Number(s);
  return Number.isNaN(n) ? { kind: "err", error: `not a number: ${s}` } : { kind: "ok", value: n };
}
```

Callers must check `.kind` before accessing `.value`/`.error` — TS won't let
you access `.value` on the `err` branch. `exercise.js`'s `createResult()`
builds `ok`/`err`/`map`/`andThen` etc. for this exact pattern at runtime.

## `unknown` vs `any`

- `any`: opts OUT of type checking entirely — anything goes, no safety.
- `unknown`: type-safe `any` — you can assign anything TO it, but can't USE
  it without narrowing first (`typeof`, type guard, etc.).

```ts
function handle(input: unknown) {
  input.toUpperCase();          // compile error: input is unknown
  if (typeof input === "string") {
    input.toUpperCase();         // OK -- narrowed to string
  }
}
```

Prefer `unknown` for values of uncertain type (parsed JSON, function
arguments from untyped callers); reserve `any` for genuine escape hatches.

## Utility types (conceptual)

Built-in generic helpers that transform existing types — no runtime
equivalent needed (purely compile-time):

| Utility | Effect |
|---|---|
| `Partial<T>` | all properties optional |
| `Required<T>` | all properties required |
| `Pick<T, "a" \| "b">` | subset of `T`'s properties |
| `Omit<T, "a">` | `T` without those properties |
| `Record<K, V>` | object type with keys `K`, values `V` |
| `ReadonlyArray<T>` | array that can't be mutated (no `push`/`splice`) |

## Gotchas

- **Types are erased** — `typeof someTypedValue === "object"` at runtime
  knows nothing about TS types; only the JS value's actual shape matters.
  Validate shapes you don't control (API responses, JSON, user input) at
  runtime — TS types alone don't protect against bad data crossing a
  boundary (this is what `validate(schema, value)` in `exercise.js` is for).
- **Structural, not nominal** — two unrelated `interface`s with the same
  shape are interchangeable in TS. Don't rely on "branding"/class identity
  for type safety unless you add a discriminant field.
- **`any` defeats the type checker silently** — it propagates: anything
  derived from an `any` value is also `any`, with no warning. Prefer
  `unknown` + narrowing.
- **Exhaustiveness needs a `never` check** — without the `default: const _: never = x` 
  pattern (or a `match`-style runtime throw), adding a new union variant
  and forgetting a handler fails SILENTLY (falls through with no value/wrong
  behavior) instead of erroring.
- **Optional (`?`) vs `| undefined`** — `{ a?: string }` allows the KEY to be
  absent; `{ a: string | undefined }` requires the key but allows `undefined`
  as its value. Different for `"a" in obj` checks and `Object.keys`.

## Further Reading

- [TypeScript Handbook — The Basics](https://www.typescriptlang.org/docs/handbook/2/basic-types.html)
- [TypeScript Handbook — Narrowing](https://www.typescriptlang.org/docs/handbook/2/narrowing.html)
- [TypeScript Handbook — Unions and Intersection Types](https://www.typescriptlang.org/docs/handbook/2/everyday-types.html#union-types)
- [TypeScript Handbook — Generics](https://www.typescriptlang.org/docs/handbook/2/generics.html)
- [TypeScript Handbook — Utility Types](https://www.typescriptlang.org/docs/handbook/utility-types.html)
