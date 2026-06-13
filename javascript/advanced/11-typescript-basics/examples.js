// Run with: node examples.js
//
// Standalone demo implementations of the runtime patterns from notes.md (NOT
// imported from exercise.js, so this file runs regardless of exercise.js's
// state). Each section's TS-equivalent syntax is shown in notes.md.

// --- Type guards (runtime narrowing) ---
console.log("=== type guards ===");
{
  const isString = (value) => typeof value === "string";
  const isNumber = (value) => typeof value === "number" && !Number.isNaN(value);
  const isArrayOf = (itemGuard) => (value) => Array.isArray(value) && value.every(itemGuard);

  function describe(value) {
    if (isString(value)) return value.toUpperCase();
    if (isNumber(value)) return value.toFixed(2);
    return "unknown";
  }

  console.log("  describe('hi'):", describe("hi"));
  console.log("  describe(3.5):", describe(3.5));
  console.log("  isArrayOf(isString)(['a','b']):", isArrayOf(isString)(["a", "b"]));
  console.log("  isArrayOf(isString)(['a', 1]):", isArrayOf(isString)(["a", 1]));
}

// --- Discriminated unions + exhaustiveness via a runtime throw ---
console.log("\n=== discriminated unions ===");
{
  function match(value, handlers) {
    const handler = handlers[value.kind];
    if (!handler) throw new Error(`Unhandled kind: "${value.kind}"`);
    return handler(value);
  }

  const shapes = [
    { kind: "circle", radius: 2 },
    { kind: "rectangle", width: 3, height: 4 },
  ];

  const area = (shape) =>
    match(shape, {
      circle: (s) => Math.PI * s.radius ** 2,
      rectangle: (s) => s.width * s.height,
    });

  for (const shape of shapes) {
    console.log(`  area(${shape.kind}):`, area(shape).toFixed(2));
  }

  try {
    match({ kind: "triangle" }, { circle: () => {}, rectangle: () => {} });
  } catch (err) {
    console.log("  unhandled kind threw:", err.message);
  }
}

// --- Result<T, E>: typed error handling without exceptions ---
console.log("\n=== Result<T, E> ===");
{
  const ok = (value) => ({ kind: "ok", value });
  const err = (error) => ({ kind: "err", error });
  const isOk = (result) => result.kind === "ok";

  function parseInt10(s) {
    const n = Number(s);
    return Number.isNaN(n) ? err(`not a number: "${s}"`) : ok(n);
  }

  for (const input of ["42", "abc"]) {
    const result = parseInt10(input);
    console.log(`  parseInt10(${JSON.stringify(input)}):`, isOk(result) ? `ok(${result.value})` : `err(${result.error})`);
  }
}

// --- Structural validation (runtime check for shapes TS would check statically) ---
console.log("\n=== structural validation ===");
{
  function actualType(value) {
    if (value === null) return "null";
    if (Array.isArray(value)) return "array";
    return typeof value;
  }

  function validateUser(value) {
    const errors = [];
    if (typeof value !== "object" || value === null) {
      errors.push(`value: expected object, got ${actualType(value)}`);
      return { valid: false, errors };
    }
    if (typeof value.name !== "string") {
      errors.push(`value.name: expected string, got ${actualType(value.name)}`);
    }
    if (typeof value.age !== "number") {
      errors.push(`value.age: expected number, got ${actualType(value.age)}`);
    }
    return { valid: errors.length === 0, errors };
  }

  console.log("  validateUser({name:'Ada', age:30}):", JSON.stringify(validateUser({ name: "Ada", age: 30 })));
  console.log("  validateUser({name:1, age:'30'}):", JSON.stringify(validateUser({ name: 1, age: "30" })));
}

// --- Generic-style constrained collection ---
console.log("\n=== generic-style TypedList ===");
{
  function createTypedList(guard, typeName) {
    const items = [];
    return {
      push(value) {
        if (!guard(value)) throw new TypeError(`Expected ${typeName}, got ${JSON.stringify(value)}`);
        items.push(value);
      },
      toArray() {
        return [...items];
      },
    };
  }

  const numbers = createTypedList((v) => typeof v === "number", "number");
  numbers.push(1);
  numbers.push(2);
  console.log("  numbers.toArray():", numbers.toArray());

  try {
    numbers.push("x");
  } catch (err) {
    console.log("  numbers.push('x') threw:", err.message);
  }
}
