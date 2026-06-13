/**
 * Recursively freeze `value` and all nested plain objects/arrays reachable
 * from it (own keys, including symbols, via `Reflect.ownKeys`).
 *
 * - Non-object values (including `null`) pass through unchanged.
 * - Handles circular references — a value that has already been visited is
 *   not processed again (no infinite loop).
 * - Returns the SAME reference passed in (mutated in place via
 *   `Object.freeze`), not a copy.
 *
 * const obj = { a: 1, b: { c: 2 }, arr: [{ x: 1 }] };
 * const result = deepFreeze(obj);
 * result === obj;              // true
 * Object.isFrozen(result);     // true
 * Object.isFrozen(result.b);   // true
 * Object.isFrozen(result.arr[0]); // true
 * result.b.c = 99;             // throws TypeError
 *
 * const cyclic = {};
 * cyclic.self = cyclic;
 * deepFreeze(cyclic); // terminates; Object.isFrozen(cyclic) === true
 *
 * @param {*} value
 * @returns {*}
 */
export function deepFreeze(value) {
  throw new Error("Not implemented");
}

/**
 * Return ALL property names found on `obj` itself and along its prototype
 * chain, stopping BEFORE `Object.prototype` (exclusive).
 *
 * - Uses `Object.getOwnPropertyNames` at each level (so non-enumerable own
 *   properties — like class methods and `constructor` — are included).
 * - String keys only (no symbols).
 * - Order: `obj`'s own names first (in `Object.getOwnPropertyNames` order),
 *   then each prototype's own names, walking up via `Object.getPrototypeOf`.
 * - Duplicates removed — if a name appears at multiple levels, only its
 *   FIRST (closest-to-`obj`) occurrence is kept.
 * - `Object.create(null)` objects (no prototype) are handled without error.
 *
 * class Animal {
 *   constructor(name) { this.name = name; }
 *   speak() { return "..."; }
 * }
 * class Dog extends Animal {
 *   constructor(name) { super(name); this.breed = "?"; }
 *   bark() { return "Woof"; }
 * }
 * getAllPropertyNames(new Dog("Rex"));
 * // -> ["name", "breed", "constructor", "bark", "speak"]
 *
 * getAllPropertyNames({ a: 1, b: 2 }); // -> ["a", "b"]
 *
 * @param {Object} obj
 * @returns {string[]}
 */
export function getAllPropertyNames(obj) {
  throw new Error("Not implemented");
}

/**
 * Return a new plain object containing only the entries of `obj` whose key
 * is BOTH in `keys` AND an own, enumerable property of `obj`
 * (`Object.prototype.propertyIsEnumerable`).
 *
 * - Keys in `keys` that don't exist on `obj`, are non-enumerable, or are
 *   inherited (not own) are silently skipped.
 * - If the property is a getter, its CURRENT VALUE is copied (the getter
 *   itself is invoked once, not preserved as an accessor on the result).
 *
 * pickEnumerable({ a: 1, b: 2, c: 3 }, ["a", "c"]); // -> { a: 1, c: 3 }
 * pickEnumerable({ a: 1 }, ["a", "missing"]);       // -> { a: 1 }
 *
 * const obj = { a: 1 };
 * Object.defineProperty(obj, "hidden", { value: 2, enumerable: false });
 * pickEnumerable(obj, ["a", "hidden"]); // -> { a: 1 } — "hidden" skipped
 *
 * const proto = { inherited: 1 };
 * const child = Object.create(proto);
 * child.own = 2;
 * pickEnumerable(child, ["own", "inherited"]); // -> { own: 2 } — inherited skipped
 *
 * @param {Object} obj
 * @param {string[]} keys
 * @returns {Object}
 */
export function pickEnumerable(obj, keys) {
  throw new Error("Not implemented");
}

/**
 * "Lock the shape" of `obj`: make every existing own property
 * non-configurable (preserving its current `value`/`writable` or
 * `get`/`set`/`enumerable`), and prevent new properties from being added
 * (`Object.preventExtensions`). EXISTING writable properties can still be
 * reassigned. Returns `obj`.
 *
 * const obj = { a: 1, b: 2 };
 * lockShape(obj);
 * obj.a = 99;        // OK — still writable -> obj.a === 99
 * obj.c = 3;         // throws TypeError — not extensible
 * delete obj.a;      // throws TypeError — not configurable
 * Object.defineProperty(obj, "a", { enumerable: false }); // throws TypeError
 *
 * @param {Object} obj
 * @returns {Object}
 */
export function lockShape(obj) {
  throw new Error("Not implemented");
}

/**
 * Categorize `obj`'s string-keyed properties by origin and enumerability:
 *
 * - `ownEnumerable`: own, enumerable keys (`Object.keys` order)
 * - `ownNonEnumerable`: own, non-enumerable keys
 * - `inheritedEnumerable`: enumerable keys found on the prototype chain
 *   (not own), in `for...in` order
 *
 * Returns `{ ownEnumerable, ownNonEnumerable, inheritedEnumerable }`, each
 * an array of strings.
 *
 * const proto = {};
 * Object.defineProperty(proto, "protoHidden", { value: "h", enumerable: false });
 * proto.protoVisible = "v";
 * const obj = Object.create(proto);
 * obj.ownVisible = "a";
 * Object.defineProperty(obj, "ownHidden", { value: "b", enumerable: false });
 *
 * groupKeysByOrigin(obj);
 * // -> {
 * //   ownEnumerable: ["ownVisible"],
 * //   ownNonEnumerable: ["ownHidden"],
 * //   inheritedEnumerable: ["protoVisible"], // protoHidden excluded (non-enumerable)
 * // }
 *
 * @param {Object} obj
 * @returns {{ ownEnumerable: string[], ownNonEnumerable: string[], inheritedEnumerable: string[] }}
 */
export function groupKeysByOrigin(obj) {
  throw new Error("Not implemented");
}
