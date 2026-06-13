// Run with: node examples.js
//
// Standalone demo implementations of the concepts from notes.md (NOT
// imported from exercise.js, so this file runs regardless of exercise.js's
// state). Simplified versions of what you'll build in exercise.js.

// --- A minimal `equal` assertion ---
console.log("=== assertion: Object.is vs === ===");
{
  function equal(actual, expected, message) {
    if (!Object.is(actual, expected)) {
      throw new Error(message ?? `Expected ${actual} to equal ${expected}`);
    }
  }

  equal(1, 1);
  equal(NaN, NaN); // passes -- Object.is(NaN, NaN) is true
  console.log("  equal(1, 1) and equal(NaN, NaN) passed");

  try {
    equal(0, -0); // Object.is(0, -0) is false
  } catch (err) {
    console.log("  equal(0, -0) threw:", err.message);
  }

  console.log("  0 === -0:", 0 === -0, "  Object.is(0, -0):", Object.is(0, -0));
}

// --- A minimal mock function ---
console.log("\n=== mock functions ===");
{
  function createMockFn(impl = () => undefined) {
    const mock = (...args) => {
      mock.calls.push(args);
      const value = impl(...args);
      mock.results.push({ type: "return", value });
      return value;
    };
    mock.calls = [];
    mock.results = [];
    return mock;
  }

  const onSave = createMockFn();
  function save(record, callback) {
    // ... real save logic would go here ...
    callback(record.id);
  }

  save({ id: 42 }, onSave);
  save({ id: 43 }, onSave);

  console.log("  calls:", JSON.stringify(onSave.calls));
  console.log("  results:", JSON.stringify(onSave.results));
}

// --- A minimal spy ---
console.log("\n=== spies ===");
{
  function spyOn(obj, methodName) {
    const original = obj[methodName];
    const calls = [];
    obj[methodName] = (...args) => {
      calls.push(args);
      return original.apply(obj, args);
    };
    obj[methodName].calls = calls;
    obj[methodName].restore = () => {
      obj[methodName] = original;
    };
    return obj[methodName];
  }

  const greeter = {
    greet(name) {
      return `Hello, ${name}!`;
    },
  };

  const spy = spyOn(greeter, "greet");
  console.log("  greeter.greet('Ada'):", greeter.greet("Ada"));
  console.log("  spy.calls:", JSON.stringify(spy.calls));

  spy.restore();
  console.log("  after restore, greeter.greet is the original:", greeter.greet("Bob"));
}

// --- A minimal test runner ---
console.log("\n=== test runner ===");
{
  function createTestRunner() {
    const tests = [];
    return {
      test(name, fn) {
        tests.push({ name, fn });
      },
      async run() {
        const failures = [];
        let passed = 0;
        for (const { name, fn } of tests) {
          try {
            await fn();
            passed++;
          } catch (error) {
            failures.push({ name, error });
          }
        }
        return { total: tests.length, passed, failed: failures.length, failures };
      },
    };
  }

  const runner = createTestRunner();
  runner.test("1 + 1 is 2", () => {
    if (1 + 1 !== 2) throw new Error("math is broken");
  });
  runner.test("1 + 1 is 3 (intentionally wrong)", () => {
    if (1 + 1 !== 3) throw new Error("1 + 1 !== 3");
  });

  const summary = await runner.run();
  console.log("  summary:", JSON.stringify({ ...summary, failures: summary.failures.map((f) => f.name) }));
}

// --- A minimal snapshot matcher ---
console.log("\n=== snapshot matcher ===");
{
  function createSnapshotMatcher() {
    const snapshots = new Map();
    return {
      match(name, value) {
        const serialized = JSON.stringify(value);
        if (!snapshots.has(name)) {
          snapshots.set(name, serialized);
          return "recorded";
        }
        if (snapshots.get(name) !== serialized) {
          throw new Error(`Snapshot mismatch for "${name}"`);
        }
        return "matched";
      },
    };
  }

  const matcher = createSnapshotMatcher();
  console.log("  first call:", matcher.match("user", { id: 1, name: "Ada" }));
  console.log("  same value again:", matcher.match("user", { id: 1, name: "Ada" }));

  try {
    matcher.match("user", { id: 1, name: "Bob" });
  } catch (err) {
    console.log("  changed value threw:", err.message);
  }
}
