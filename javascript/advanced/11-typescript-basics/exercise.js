/**
 * Create a set of composable runtime type guards (the runtime analog of
 * TypeScript type predicates / narrowing).
 *
 * Returns an object with:
 * - `isString(value)`: `true` iff `typeof value === "string"`.
 * - `isNumber(value)`: `true` iff `typeof value === "number"` AND `value` is
 *   not `NaN`.
 * - `isBoolean(value)`: `true` iff `typeof value === "boolean"`.
 * - `isArrayOf(itemGuard)`: returns a guard that is `true` iff `value` is an
 *   `Array` and every element satisfies `itemGuard` (an empty array passes).
 * - `isObjectOf(shape)`: `shape` is an object mapping keys to guards. Returns
 *   a guard that is `true` iff `value` is a non-null, non-array object AND,
 *   for every key in `shape`, `shape[key](value[key])` is `true`. Extra keys
 *   on `value` not present in `shape` are ignored.
 * - `isOptional(guard)`: returns a guard that is `true` iff `value ===
 *   undefined` OR `guard(value)` is `true`.
 * - `isUnion(...guards)`: returns a guard that is `true` iff ANY of `guards`
 *   returns `true` for `value`.
 * - `isLiteral(...literals)`: returns a guard that is `true` iff `value ===`
 *   one of `literals`.
 *
 * @returns {{
 *   isString: (value: any) => boolean,
 *   isNumber: (value: any) => boolean,
 *   isBoolean: (value: any) => boolean,
 *   isArrayOf: (itemGuard: (value: any) => boolean) => (value: any) => boolean,
 *   isObjectOf: (shape: Record<string, (value: any) => boolean>) => (value: any) => boolean,
 *   isOptional: (guard: (value: any) => boolean) => (value: any) => boolean,
 *   isUnion: (...guards: Array<(value: any) => boolean>) => (value: any) => boolean,
 *   isLiteral: (...literals: any[]) => (value: any) => boolean,
 * }}
 *
 * @example
 * const g = createTypeGuards();
 * const isUser = g.isObjectOf({
 *   name: g.isString,
 *   role: g.isUnion(g.isLiteral("admin"), g.isLiteral("user")),
 *   tags: g.isArrayOf(g.isString),
 *   nickname: g.isOptional(g.isString),
 * });
 * isUser({ name: "Ada", role: "admin", tags: ["x"] }); // true
 */
export function createTypeGuards() {
  throw new Error("Not implemented");
}

/**
 * Recursively validate `value` against `schema` (a simplified JSON-Schema-like
 * description), collecting ALL errors (does not stop at the first one).
 *
 * `schema` is one of:
 * - `{ type: "string" | "number" | "boolean" }`: `value` must have that
 *   `typeof`.
 * - `{ type: "literal", value: <any> }`: `value` must be `===` to
 *   `schema.value`.
 * - `{ type: "array", items: <schema> }`: `value` must be an `Array`; every
 *   element is validated against `items`.
 * - `{ type: "object", properties: Record<string, schema>, required:
 *   string[] }`: `value` must be a non-null, non-array object. Every key in
 *   `required` must be present on `value`. Every key present in BOTH
 *   `properties` and `value` is validated against its schema. Extra keys on
 *   `value` not in `properties` are ignored.
 * - `{ type: "union", options: <schema>[] }`: `value` must satisfy AT LEAST
 *   ONE schema in `options`.
 *
 * Returns `{ valid: boolean, errors: string[] }`. `errors` is `[]` iff
 * `valid` is `true`. Each error is a string starting with a "path" rooted at
 * `"value"` (e.g. `"value.tags[1]"`, `"value.user.name"`), using these exact
 * formats:
 * - Primitive type mismatch: `` `${path}: expected ${type}, got ${actualType}` ``
 *   where `actualType` is `"null"` for `null`, `"array"` for arrays, else
 *   `typeof value`.
 * - Literal mismatch: `` `${path}: expected ${JSON.stringify(schema.value)}, got ${JSON.stringify(value)}` ``
 * - Array, when `value` is not an array: `` `${path}: expected array, got ${actualType}` ``
 * - Object, when `value` is not an object (or is `null`/an array): `` `${path}: expected object, got ${actualType}` ``
 * - Missing required property: `` `${path}.${key}: missing required property` ``
 * - Union, when no option matches: `` `${path}: value did not match any option in union` ``
 *
 * @param {object} schema
 * @param {any} value
 * @returns {{ valid: boolean, errors: string[] }}
 *
 * @example
 * validate({ type: "string" }, "x");   // { valid: true, errors: [] }
 * validate({ type: "string" }, 1);     // { valid: false, errors: ["value: expected string, got number"] }
 */
export function validate(schema, value) {
  throw new Error("Not implemented");
}

/**
 * Dispatch on a discriminated union's `kind` field (the runtime equivalent
 * of an exhaustive `switch` + `never`-typed `default`).
 *
 * `value` is an object with a `kind` string property. `handlers` maps each
 * expected `kind` to a function `(value) => result`. If `handlers` has a
 * function for `value.kind`, calls it with `value` and returns its result.
 * Otherwise throws an `Error` whose message includes `value.kind` (e.g.
 * `Unhandled kind: "triangle"`).
 *
 * @param {{ kind: string, [key: string]: any }} value
 * @param {Record<string, (value: any) => any>} handlers
 * @returns {any}
 *
 * @example
 * const area = (shape) => match(shape, {
 *   circle: (s) => Math.PI * s.radius ** 2,
 *   rectangle: (s) => s.width * s.height,
 * });
 * area({ kind: "rectangle", width: 3, height: 4 }); // 12
 * area({ kind: "triangle" }); // throws Error mentioning "triangle"
 */
export function match(value, handlers) {
  throw new Error("Not implemented");
}

/**
 * Create a `Result<T, E>`-style discriminated union helper for typed error
 * handling without exceptions.
 *
 * Returns an object with:
 * - `ok(value)`: returns `{ kind: "ok", value }`.
 * - `err(error)`: returns `{ kind: "err", error }`.
 * - `isOk(result)`: `true` iff `result.kind === "ok"`.
 * - `isErr(result)`: `true` iff `result.kind === "err"`.
 * - `map(result, fn)`: if `result` is `ok`, returns `ok(fn(result.value))`;
 *   if `err`, returns `result` UNCHANGED (same reference).
 * - `mapErr(result, fn)`: if `result` is `err`, returns
 *   `err(fn(result.error))`; if `ok`, returns `result` UNCHANGED.
 * - `unwrapOr(result, defaultValue)`: returns `result.value` if `ok`, else
 *   `defaultValue`.
 * - `andThen(result, fn)`: if `result` is `ok`, returns `fn(result.value)`
 *   (which must itself return a `Result`); if `err`, returns `result`
 *   UNCHANGED.
 *
 * @returns {{
 *   ok: (value: any) => { kind: "ok", value: any },
 *   err: (error: any) => { kind: "err", error: any },
 *   isOk: (result: { kind: string }) => boolean,
 *   isErr: (result: { kind: string }) => boolean,
 *   map: (result: any, fn: (value: any) => any) => any,
 *   mapErr: (result: any, fn: (error: any) => any) => any,
 *   unwrapOr: (result: any, defaultValue: any) => any,
 *   andThen: (result: any, fn: (value: any) => any) => any,
 * }}
 */
export function createResult() {
  throw new Error("Not implemented");
}

/**
 * Create a list constrained to values matching `guard` (the runtime analog
 * of a generic `class TypedList<T>` that only accepts `T`).
 *
 * Returns an object with:
 * - `push(value)`: if `guard(value)` is falsy, throws a `TypeError` with
 *   message `` `Expected ${typeName}, got ${JSON.stringify(value)}` ``.
 *   Otherwise appends `value`.
 * - `pop()`: removes and returns the last element. If the list is empty,
 *   throws an `Error` whose message contains `"empty"`.
 * - `toArray()`: returns a new array (shallow copy) of the current elements,
 *   in insertion order.
 * - `length` (getter): the current number of elements.
 *
 * @param {(value: any) => boolean} guard
 * @param {string} typeName
 * @returns {{
 *   push: (value: any) => void,
 *   pop: () => any,
 *   toArray: () => any[],
 *   readonly length: number,
 * }}
 *
 * @example
 * const numbers = createTypedList((v) => typeof v === "number", "number");
 * numbers.push(1);
 * numbers.push("x"); // throws TypeError: Expected number, got "x"
 */
export function createTypedList(guard, typeName) {
  throw new Error("Not implemented");
}
