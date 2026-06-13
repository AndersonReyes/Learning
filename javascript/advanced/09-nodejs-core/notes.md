# Advanced 09. Node.js Core (fs, streams, events, modules)

> Node-specific topic — `fs`, `streams`, and `events` are Node.js APIs, not
> covered by MDN (which documents the JS language and browser Web APIs).
> "Further Reading" below links to the official Node.js API docs instead.

## `fs/promises` — file I/O

Three flavors of the `fs` module: synchronous (`fs.readFileSync`, blocks the
event loop), callback-based (`fs.readFile(path, cb)`), and promise-based
(`fs.promises` / `node:fs/promises`, used with `async`/`await`). Prefer the
promise API in modern code.

```js
import fs from "node:fs/promises";

const text = await fs.readFile("data.txt", "utf8");
await fs.writeFile("out.txt", "hello");
await fs.mkdir("logs", { recursive: true });
await fs.rm("tmp", { recursive: true, force: true });

const entries = await fs.readdir("src", { withFileTypes: true });
for (const entry of entries) {
  entry.isDirectory(); // boolean
  entry.isFile();      // boolean
  entry.name;          // string
}
```

## `path` — cross-platform path manipulation

```js
import path from "node:path";

path.join("a", "b", "..", "c");     // "a/c" (normalizes . and ..)
path.resolve("a", "b");             // absolute path, resolved against cwd
path.basename("/a/b/file.txt");     // "file.txt"
path.dirname("/a/b/file.txt");      // "/a/b"
path.extname("file.txt");           // ".txt"
path.sep;                           // "/" (POSIX) or "\\" (Windows)
```

`path.resolve` walks arguments right-to-left until it builds an absolute
path — any absolute-looking argument **discards everything before it**:

```js
path.resolve("/data", "/etc/passwd"); // "/etc/passwd" -- "/data" is discarded!
```

### Path traversal — a security gotcha

If a path segment comes from user input, `..` (or an absolute path) can
escape an intended directory:

```js
// DANGEROUS if `userPath` is attacker-controlled:
const filePath = path.join("/data/uploads", userPath);
// userPath = "../../etc/passwd" -> "/etc/passwd"
```

Always validate that a resolved path stays **inside** its intended base
directory before using it (see `safeJoin` in `exercise.js`): resolve both to
absolute paths and check the result starts with `base + path.sep` (or equals
`base` exactly).

## `events` — `EventEmitter`

The base class behind streams, and a general pub/sub primitive:

```js
import { EventEmitter } from "node:events";

const emitter = new EventEmitter();
emitter.on("data", (chunk) => console.log("got", chunk));
emitter.once("end", () => console.log("done"));
emitter.emit("data", "hello"); // "got hello"
emitter.off("data", handlerFn); // removeListener (needs same reference)
```

**The `"error"` event is special**: if `emit("error", ...)` is called and
there is **no `"error"` listener registered**, Node throws the error
(crashing the process for an unhandled error). Always attach an `"error"`
listener on anything that might emit one.

## `stream` — `Readable`, `Writable`, `Transform`

Streams process data in chunks instead of loading everything into memory.
A `Transform` stream both reads and writes, transforming data as it passes
through:

```js
import { Transform } from "node:stream";

const upper = new Transform({
  transform(chunk, encoding, callback) {
    this.push(chunk.toString().toUpperCase());
    callback(); // signals "ready for the next chunk"
  },
});

readable.pipe(upper).pipe(writable);
```

- `readableObjectMode: true` lets a stream emit non-Buffer values (e.g.
  parsed objects, or strings as discrete "records" rather than raw bytes).
- **Chunk boundaries are arbitrary** — a stream may split a single logical
  record (e.g. one line of text) across two `transform` calls. Stateful
  parsers must buffer incomplete data between calls (see `splitLines` in
  `exercise.js`).
- `flush(callback)` runs once, after the last chunk, for any
  buffered/trailing output.
- **Backpressure**: `writable.write()` returns `false` when its internal
  buffer is full — well-behaved producers should pause until `"drain"`
  fires. `pipe()`/`pipeline()` handle this automatically.

## Modules — CommonJS vs ES Modules

Node supports both module systems:

| | CommonJS | ES Modules |
|---|---|---|
| Import | `require("./mod")` | `import x from "./mod.js"` |
| Export | `module.exports = ...` | `export default ...` / `export const ...` |
| File marker | `.cjs`, or no `"type": "module"` | `.mjs`, or `"type": "module"` in `package.json` |
| Loading | synchronous | static (analyzed before execution) + dynamic `import()` |
| Current file path | `__filename`, `__dirname` | `import.meta.url` |

```js
// ESM equivalent of __dirname:
import { fileURLToPath } from "node:url";
import path from "node:path";
const __dirname = path.dirname(fileURLToPath(import.meta.url));

// Dynamic import (works in both systems, always returns a Promise):
const mod = await import("./plugin.js");
```

## Gotchas

- **Unhandled `"error"` events crash the process** — always listen for
  `"error"` on emitters/streams that can fail (file streams, sockets,
  parsers).
- **Streams split data arbitrarily** — never assume a chunk boundary aligns
  with a logical record boundary (line, JSON object, etc.).
- **`path.resolve`/`path.join` don't validate** that a result stays within a
  directory — that's on you, especially with user-supplied path segments.
- **Sync fs calls (`fs.readFileSync`) block the event loop** — fine for
  startup/CLI scripts, avoid in request-handling code.
- **`fs.rm`/`fs.mkdir` need `{ recursive: true }`** for non-empty directories
  / nested paths respectively — without it they throw on existing
  directories or missing parents.

## Further Reading

- [Node.js `fs` (Promises API)](https://nodejs.org/api/fs.html#promises-api)
- [Node.js `path`](https://nodejs.org/api/path.html)
- [Node.js `events`](https://nodejs.org/api/events.html)
- [Node.js `stream`](https://nodejs.org/api/stream.html)
- [Node.js Modules: ECMAScript modules](https://nodejs.org/api/esm.html)
