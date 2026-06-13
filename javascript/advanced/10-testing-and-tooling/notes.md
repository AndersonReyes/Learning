# Advanced 10. Testing & Tooling

> Adapted scope: ESLint, bundlers (esbuild/Vite/webpack), and CI pipelines
> aren't `node:test`-able directly — and MDN doesn't cover them (they're
> ecosystem tooling, not part of the JS language or Web platform). This
> topic's exercises instead build small pieces of a **testing framework**
> (the same primitives `node:test`/Jest/Vitest are built from): custom
> assertions, a `mockFn`, a spy, a mini test runner, and snapshot matching.
> The conceptual sections below cover linters/bundlers/CI with links to their
> own docs.

## Assertion libraries

At their core, `assert.equal(a, b)` etc. are just: compare, and `throw` with
a descriptive message if the comparison fails. Everything else (test
runners, reporters) is built on "a thrown error = failed test".

```js
function equal(actual, expected, message) {
  if (!Object.is(actual, expected)) {
    throw new Error(message ?? `Expected ${actual} to equal ${expected}`);
  }
}
```

`Object.is` (not `===`) is the right comparison for an `equal` assertion —
it treats `NaN` as equal to `NaN` and distinguishes `+0`/`-0`, matching how
most assertion libraries define "equal" for primitives. Deep equality
(`deepEqual`) recursively compares object/array structure rather than
reference identity.

## Mock functions (`mockFn`)

A **mock function** replaces a real function in a test, recording how it was
called so you can assert on the calls afterward (instead of, or in addition
to, its return value):

```js
const mock = createMockFn();
doSomething(mock);
mock.calls; // [[arg1, arg2], ...] -- every call's arguments
```

Configurable behavior (`mockReturnValue`, `mockImplementation`) lets a mock
stand in for a real dependency (a database call, an API client) without
making it do real work.

## Spies

A **spy** wraps an *existing* method on a real object, recording calls while
(by default) still calling the original implementation through — useful for
asserting "was this called?" without changing behavior. `restore()` puts the
original method back, which matters because spies **mutate the object they
spy on** — forgetting to restore leaks the spy into later tests.

```js
const spy = spyOn(console, "log");
doSomethingThatLogs();
spy.calls.length; // assert it was called
spy.restore(); // put the real console.log back
```

## Test runners

A test runner collects `(name, fn)` pairs, runs each one (catching thrown
errors / rejected promises as failures), and reports a summary. This is what
`describe`/`test` + `node --test` do under the hood — register, then run,
then aggregate `{ total, passed, failed, failures }`.

Key design point: **tests run independently** — one test's thrown error
shouldn't stop the runner from running the rest, just gets recorded as that
test's failure.

## Snapshot testing

Instead of hand-writing the expected value, **snapshot tests** record the
actual output the first time, then compare against that recorded snapshot on
future runs — failing if the output changes unexpectedly:

```js
matcher.match("user", getUser()); // first run: records the value
matcher.match("user", getUser()); // later runs: compares against the recording
```

Useful for large/complex outputs (rendered HTML, serialized objects) where
writing out the full expected value by hand is impractical. The tradeoff:
snapshots can mask real bugs if a developer blindly "updates" a failing
snapshot without checking *why* it changed.

## ESLint (conceptual)

A **linter** statically analyzes code for likely bugs and style issues
without running it — e.g. unused variables, `==` vs `===`, unreachable code.
Configured via `eslint.config.js`; rules can be `"error"`, `"warn"`, or
`"off"`. Often run in CI and as a pre-commit hook (catches issues before
they reach review).

## Bundlers (conceptual)

A **bundler** (esbuild, Vite, webpack, Rollup) combines many ES modules into
fewer output files for the browser: resolves `import`/`export` graphs,
**tree-shakes** (removes unused exports), transpiles modern syntax for older
targets, and minifies. Node's native ESM support means bundling is mostly a
*browser/deployment* concern, not a development-time necessity for
Node-only code.

## CI (conceptual)

**Continuous Integration** runs your test suite (and lint/build) automatically
on every push/PR — e.g. GitHub Actions. A typical CI config: install deps,
run `npm test`, run `npm run lint`, fail the build (and block merge) if any
step fails. Catches regressions before they reach `main`.

## Gotchas

- **`Object.is` vs `===`**: `Object.is(NaN, NaN)` is `true`,
  `NaN === NaN` is `false`. `Object.is(0, -0)` is `false`,
  `0 === -0` is `true`. Pick the right one for an `equal` assertion.
- **Spies mutate shared state**: a `spyOn(obj, method)` that's never
  `restore()`d affects every later test that touches `obj`.
- **One test's failure shouldn't crash the runner** — wrap each test in its
  own `try`/`catch` (or `await`+`catch` for async tests) so the rest still
  run and report.
- **Snapshot drift**: a snapshot that's "updated" without review can hide a
  real regression — treat snapshot diffs as something to *read*, not
  rubber-stamp.

## Further Reading

- [Node.js test runner](https://nodejs.org/api/test.html)
- [ESLint](https://eslint.org/docs/latest/)
- [Vitest (snapshot testing)](https://vitest.dev/guide/snapshot)
- [GitHub Actions documentation](https://docs.github.com/en/actions)
