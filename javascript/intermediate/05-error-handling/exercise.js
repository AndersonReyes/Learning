/**
 * Base class for application errors.
 *
 * - Calls `super(message, options)` so `options.cause` (if provided) is
 *   wired up via the standard `Error: cause` mechanism.
 * - Sets `this.name = this.constructor.name` so subclasses report their
 *   own name (e.g. "ValidationError"), not the inherited "Error".
 * - Stores `options.code` as `this.code` (a short machine-readable string,
 *   e.g. "NOT_FOUND") — `undefined` if not provided.
 *
 * new AppError("boom", { code: "BOOM", cause: new Error("root") })
 *   .name    -> "AppError"
 *   .message -> "boom"
 *   .code    -> "BOOM"
 *   .cause.message -> "root"
 */
export class AppError extends Error {
  /**
   * @param {string} message
   * @param {{ cause?: unknown, code?: string }} [options]
   */
  constructor(message, { cause, code } = {}) {
    super(message, { cause });
    this.name = this.constructor.name;
    this.code = code;
  }

  /**
   * Serialize this error to a plain object suitable for JSON logging.
   *
   * - `name`: `this.name`
   * - `message`: `this.message`
   * - `code`: `this.code`
   * - `cause`: the cause's `.message` if `this.cause` is set and has a
   *   `.message`, otherwise `undefined`
   *
   * new AppError("boom", { code: "BOOM", cause: new Error("root") }).toJSON()
   *   -> { name: "AppError", message: "boom", code: "BOOM", cause: "root" }
   *
   * new AppError("boom").toJSON()
   *   -> { name: "AppError", message: "boom", code: undefined, cause: undefined }
   *
   * @returns {{ name: string, message: string, code: string|undefined, cause: string|undefined }}
   */
  toJSON() {
    return {
      name: this.name,
      message: this.message,
      code: this.code,
      cause: this.cause?.message,
    };
  }
}

/**
 * Error for invalid input / failed validation. Distinct class from
 * `NotFoundError` so callers can branch with `instanceof`.
 */
export class ValidationError extends AppError {
  /**
   * @param {string} message
   * @param {{ cause?: unknown, code?: string }} [options]
   */
  constructor(message, options) {
    super(message, options);
  }
}

/**
 * Error for "the requested thing doesn't exist". Distinct class from
 * `ValidationError` so callers can branch with `instanceof`.
 */
export class NotFoundError extends AppError {
  /**
   * @param {string} message
   * @param {{ cause?: unknown, code?: string }} [options]
   */
  constructor(message, options) {
    super(message, options);
  }
}

/**
 * Parse `text` as JSON without ever throwing.
 *
 * safeJsonParse('{"a":1}') -> { ok: true, value: { a: 1 } }
 * safeJsonParse('not json') -> { ok: false, error: SyntaxError }
 *
 * @param {string} text
 * @returns {{ ok: true, value: * } | { ok: false, error: Error }}
 */
export function safeJsonParse(text) {
  try {
    return { ok: true, value: JSON.parse(text) };
  } catch (err) {
    return { ok: false, error: err };
  }
}

/**
 * Call `fn` (sync or async — may return a promise) and retry on failure.
 *
 * - Calls `fn()`. If it returns successfully (or its returned promise
 *   resolves), returns that result immediately.
 * - If it throws (or its returned promise rejects), waits `delayMs`
 *   (via `setTimeout`) then retries.
 * - Up to `attempts` total calls to `fn`. If every attempt fails, throws
 *   the error from the LAST attempt.
 *
 * let n = 0;
 * await retry(() => {
 *   n++;
 *   if (n < 3) throw new Error("fail");
 *   return "ok";
 * }, { attempts: 5, delayMs: 10 });
 * -> "ok" (after 2 retries, 3 total calls)
 *
 * @param {() => *} fn
 * @param {{ attempts?: number, delayMs?: number }} [options]
 * @returns {Promise<*>}
 */
export async function retry(fn, { attempts = 3, delayMs = 0 } = {}) {
  let lastError;
  for (let attempt = 1; attempt <= attempts; attempt++) {
    try {
      return await fn();
    } catch (err) {
      lastError = err;
      if (attempt < attempts) {
        await new Promise((resolve) => setTimeout(resolve, delayMs));
      }
    }
  }
  throw lastError;
}

/**
 * Call each zero-arg sync function in `fns`, collecting results and errors
 * without stopping at the first failure.
 *
 * - `results[i]` is `fns[i]()`'s return value, or `undefined` if `fns[i]`
 *   threw.
 * - `errors` contains `{ index, error }` for each `i` where `fns[i]` threw,
 *   in `fns` order.
 *
 * aggregateErrors([
 *   () => 1,
 *   () => { throw new Error("bad"); },
 *   () => 3,
 * ])
 * -> {
 *      results: [1, undefined, 3],
 *      errors: [{ index: 1, error: Error("bad") }],
 *    }
 *
 * @param {Array<() => *>} fns
 * @returns {{ results: Array<*>, errors: Array<{ index: number, error: Error }> }}
 */
export function aggregateErrors(fns) {
  const results = [];
  const errors = [];
  fns.forEach((fn, index) => {
    try {
      results.push(fn());
    } catch (error) {
      results.push(undefined);
      errors.push({ index, error });
    }
  });
  return { results, errors };
}

/**
 * Walk an error's `.cause` chain, starting with `error` itself.
 *
 * Returns an array of `{ name, message }`, one per error in the chain,
 * ending when `.cause` is missing or is not an `Error`.
 *
 * To guard against a (pathological) circular `.cause` chain, stops once
 * either:
 *   - 10 errors have been collected, or
 *   - an error object already seen earlier in the chain is encountered again
 * (whichever comes first).
 *
 * const root = new Error("root");
 * const mid = new Error("mid", { cause: root });
 * const top = new Error("top", { cause: mid });
 * getErrorChain(top)
 * -> [
 *      { name: "Error", message: "top" },
 *      { name: "Error", message: "mid" },
 *      { name: "Error", message: "root" },
 *    ]
 *
 * @param {Error} error
 * @returns {Array<{ name: string, message: string }>}
 */
export function getErrorChain(error) {
  const chain = [];
  const seen = new Set();
  let current = error;
  while (current instanceof Error && !seen.has(current) && chain.length < 10) {
    seen.add(current);
    chain.push({ name: current.name, message: current.message });
    current = current.cause;
  }
  return chain;
}
