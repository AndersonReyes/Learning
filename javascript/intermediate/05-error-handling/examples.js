// Run with: node examples.js

// --- try / catch / finally ---
function divide(a, b) {
  try {
    if (b === 0) throw new Error("division by zero");
    return a / b;
  } catch (err) {
    console.log("caught:", err.message);
    return null;
  } finally {
    console.log("divide finally ran");
  }
}
console.log("divide(10, 2):", divide(10, 2));
console.log("divide(10, 0):", divide(10, 0));

// --- catch binding is optional ---
function parseOrDefault(text) {
  try {
    return JSON.parse(text);
  } catch {
    return {}; // don't care what went wrong
  }
}
console.log("parseOrDefault('{\"a\":1}'):", parseOrDefault('{"a":1}'));
console.log("parseOrDefault('not json'):", parseOrDefault("not json"));

// --- finally ALWAYS runs, even with return/throw in try/catch ---
function withFinally() {
  try {
    return "from try";
  } finally {
    console.log("finally runs even though try returned");
  }
}
console.log("withFinally():", withFinally());

// --- Gotcha: finally's return overrides try/catch's return ---
function finallyWins() {
  try {
    return "from try";
  } finally {
    return "from finally"; // wins — discards "from try"
  }
}
console.log("finallyWins():", finallyWins());

// finally also runs on the way out of an uncaught error:
function uncaughtWithFinally() {
  try {
    throw new Error("boom");
  } finally {
    console.log("finally runs before the error propagates");
  }
}
try {
  uncaughtWithFinally();
} catch (err) {
  console.log("caught after finally:", err.message);
}

// --- throw accepts any value, but always throw Error instances ---
try {
  throw "oops"; // valid JS, but no .stack / .message
} catch (err) {
  console.log("typeof thrown string:", typeof err, "value:", err);
}

try {
  throw new Error("real error");
} catch (err) {
  console.log("err.message:", err.message);
  console.log("err.stack starts with:", err.stack.split("\n")[0]);
}

// --- Custom error classes ---
class AppError extends Error {
  constructor(message, options) {
    super(message, options);
    this.name = this.constructor.name; // "AppError" or subclass name
  }
}

class ValidationError extends AppError {}
class NotFoundError extends AppError {}

const validationErr = new ValidationError("email is required");
console.log("validationErr.name:", validationErr.name);
console.log("validationErr instanceof ValidationError:", validationErr instanceof ValidationError);
console.log("validationErr instanceof AppError:", validationErr instanceof AppError);
console.log("validationErr instanceof Error:", validationErr instanceof Error);

// Branching on custom error type:
function handle(err) {
  if (err instanceof NotFoundError) return `404: ${err.message}`;
  if (err instanceof ValidationError) return `400: ${err.message}`;
  return `500: ${err.message}`;
}
console.log("handle(validationErr):", handle(validationErr));
console.log("handle(new NotFoundError('user 42')):", handle(new NotFoundError("user 42")));

// --- Error cause chaining (ES2022) ---
function loadConfig(raw) {
  try {
    return JSON.parse(raw);
  } catch (err) {
    throw new Error("failed to load config", { cause: err });
  }
}
try {
  loadConfig("{bad json");
} catch (err) {
  console.log("err.message:", err.message);
  console.log("err.cause.message:", err.cause.message);
  console.log("err.cause instanceof SyntaxError:", err.cause instanceof SyntaxError);
}

// Custom error + cause together:
const root = new TypeError("expected a number");
const wrapped = new AppError("could not process item", { cause: root });
console.log("wrapped.name:", wrapped.name);
console.log("wrapped.cause.name:", wrapped.cause.name);

// --- Result type pattern (alternative to exceptions) ---
function parseNumber(s) {
  const n = Number(s);
  return Number.isNaN(n)
    ? { ok: false, error: new Error(`invalid number: ${s}`) }
    : { ok: true, value: n };
}

const good = parseNumber("42");
const bad = parseNumber("abc");
console.log("good:", good);
console.log("bad.ok:", bad.ok, "bad.error.message:", bad.error.message);

// Chaining fallible steps without nested try/catch:
function double(result) {
  if (!result.ok) return result; // pass the failure through unchanged
  return { ok: true, value: result.value * 2 };
}
console.log("double(good):", double(good));
console.log("double(bad):", double(bad)); // error passed through, untouched

// --- Gotcha: silently swallowing errors ---
function silentlyWrong() {
  try {
    JSON.parse("not json");
    return "never gets here";
  } catch {
    // BAD: nothing logged, rethrown, or explained — failure vanishes
  }
  return "swallowed";
}
console.log("silentlyWrong():", silentlyWrong());

// Better: log (or rethrow/handle) deliberately.
function loggedFailure() {
  try {
    JSON.parse("not json");
  } catch (err) {
    console.log("logged failure:", err.message);
    return "handled";
  }
}
console.log("loggedFailure():", loggedFailure());

// --- Walking an error's cause chain ---
function errorChain(error) {
  const chain = [];
  let current = error;
  while (current instanceof Error) {
    chain.push({ name: current.name, message: current.message });
    current = current.cause;
  }
  return chain;
}
const rootErr = new Error("root cause");
const midErr = new Error("mid layer", { cause: rootErr });
const topErr = new AppError("top layer", { cause: midErr });
console.log("errorChain(topErr):", errorChain(topErr));
