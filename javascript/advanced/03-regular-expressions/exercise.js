/**
 * Tokenize a simple arithmetic expression.
 *
 * Recognizes (in order of preference at each position):
 * - `number` — `\d+(\.\d+)?` (integers and decimals, e.g. `42`, `3.14`).
 * - `identifier` — `[A-Za-z_]\w*` (variable names).
 * - `operator` — one of `+ - * / ^ %`.
 * - `paren` — `(` or `)`.
 *
 * Whitespace between tokens is ignored. Any other character is INVALID and
 * causes `tokenize` to throw an `Error` whose message includes that
 * character (e.g. `Unexpected character: "$"`).
 *
 * @param {string} expr
 * @returns {Array<{ type: "number" | "identifier" | "operator" | "paren", value: string }>}
 *
 * @example
 * tokenize("3.14 * (x + 42)")
 * // -> [
 * //   { type: "number", value: "3.14" },
 * //   { type: "operator", value: "*" },
 * //   { type: "paren", value: "(" },
 * //   { type: "identifier", value: "x" },
 * //   { type: "operator", value: "+" },
 * //   { type: "number", value: "42" },
 * //   { type: "paren", value: ")" },
 * // ]
 */
export function tokenize(expr) {
  throw new Error("Not implemented");
}

/**
 * Extract all Markdown-style links `[text](url)` from `markdown`.
 *
 * `text` and `url` may be empty strings. Brackets/parens are matched
 * non-greedily — `text` is everything up to the next `]`, `url` is
 * everything up to the next `)`.
 *
 * @param {string} markdown
 * @returns {Array<{ text: string, url: string }>}
 *
 * @example
 * extractLinks("See [MDN](https://developer.mozilla.org) and [docs]().")
 * // -> [
 * //   { text: "MDN", url: "https://developer.mozilla.org" },
 * //   { text: "docs", url: "" },
 * // ]
 *
 * @example
 * extractLinks("no links here")
 * // -> []
 */
export function extractLinks(markdown) {
  throw new Error("Not implemented");
}

/**
 * Redact email addresses and credit-card-like numbers from `text`.
 *
 * - Email addresses (e.g. `john.doe@example.com`) are replaced with
 *   `[EMAIL]`.
 * - Credit-card-like numbers — 16 digits in 4 groups of 4, separated by
 *   spaces or hyphens (or no separator at all), e.g. `4111 1111 1111 1111`,
 *   `4111-1111-1111-1111`, `4111111111111111` — are replaced with `[CARD]`.
 *   Shorter digit runs (e.g. phone numbers) are left untouched.
 *
 * Everything else in `text` is unchanged.
 *
 * @param {string} text
 * @returns {string}
 *
 * @example
 * redact("Contact john@example.com or pay with 4111 1111 1111 1111.")
 * // -> "Contact [EMAIL] or pay with [CARD]."
 */
export function redact(text) {
  throw new Error("Not implemented");
}

/**
 * Render a template string, replacing `{{path.to.value}}` placeholders with
 * values looked up in `data` via dot-separated paths. Whitespace inside the
 * braces is allowed and ignored (`{{ name }}` === `{{name}}`).
 *
 * Looked-up values are converted to strings with `String(value)`.
 *
 * Throws an `Error` whose message includes the full dotted path if any
 * segment of the path is missing from `data` (or from an intermediate
 * object).
 *
 * @param {string} template
 * @param {Record<string, any>} data
 * @returns {string}
 *
 * @example
 * renderTemplate("Hello {{user.name}}, you have {{count}} messages.", {
 *   user: { name: "Ada" },
 *   count: 3,
 * })
 * // -> "Hello Ada, you have 3 messages."
 *
 * @example
 * renderTemplate("{{missing}}", {}) // throws Error mentioning "missing"
 */
export function renderTemplate(template, data) {
  throw new Error("Not implemented");
}

/**
 * Parse a structured log line of the form:
 * `[<timestamp>] <LEVEL> (<service>): <message>`
 *
 * e.g. `[2024-01-15T10:30:00Z] ERROR (auth-service): login failed`
 *
 * Returns `null` if `line` does not match this format. `message` may be an
 * empty string. Leading/trailing whitespace around `message` is trimmed.
 *
 * @param {string} line
 * @returns {{ timestamp: string, level: string, service: string, message: string } | null}
 *
 * @example
 * parseLogLine("[2024-01-15T10:30:00Z] ERROR (auth-service): login failed")
 * // -> {
 * //   timestamp: "2024-01-15T10:30:00Z",
 * //   level: "ERROR",
 * //   service: "auth-service",
 * //   message: "login failed",
 * // }
 *
 * @example
 * parseLogLine("not a log line") // -> null
 */
export function parseLogLine(line) {
  throw new Error("Not implemented");
}
