# JavaScript

This directory is a self-contained JavaScript learning track: notes, runnable
examples, and exercises with tests. No external dependencies are required —
everything runs on Node's built-in test runner.

## How it's organized

```
javascript/
  ROADMAP.md            # full curriculum, fundamentals -> intermediate -> advanced
  package.json
  fundamentals/
    01-variables-and-data-types/
      notes.md          # explanation of the concept, with MDN links for deep dives
      examples.js       # runnable demo code
      exercise.js       # function stubs for you to implement
      exercise.test.js  # tests that define the expected behavior
    02-operators-and-type-coercion/
    ...
```

Each topic folder follows the same four-file pattern.

## How to work through a topic

1. Read `notes.md` for the concept explanation and gotchas.
2. Run the examples to see the concepts in action:
   ```
   node fundamentals/01-variables-and-data-types/examples.js
   ```
3. Open `exercise.js`. Each exported function has a JSDoc comment describing
   what it should do, and a body that throws `Not implemented`.
4. Open `exercise.test.js` to see the test cases — these define exactly what
   each function must do. There are no separate solution files; the tests
   *are* the spec.
5. Implement the functions in `exercise.js` until the tests pass:
   ```
   node --test fundamentals/01-variables-and-data-types/exercise.test.js
   ```
   Or run every topic's tests at once from this directory:
   ```
   npm test
   ```

All exercises start failing — that's expected. Work through them in order,
since later topics build on earlier ones.

## Roadmap

See [`ROADMAP.md`](./ROADMAP.md) for the full curriculum, including
intermediate and advanced topics planned for later.
