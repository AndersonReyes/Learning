import { test, describe, before, after } from "node:test";
import assert from "node:assert/strict";
import fs from "node:fs/promises";
import os from "node:os";
import path from "node:path";
import { EventEmitter } from "node:events";
import {
  safeJoin,
  buildDirectoryTree,
  waitForEvents,
  splitLines,
  findFilesByExtension,
} from "./exercise.js";

describe("safeJoin", () => {
  test("joins base with segments into an absolute path", () => {
    assert.equal(safeJoin("/data", "file.txt"), "/data/file.txt");
    assert.equal(safeJoin("/data", "sub", "file.txt"), "/data/sub/file.txt");
  });

  test("normalizes '..' that stays within base", () => {
    assert.equal(safeJoin("/data", "sub/../file.txt"), "/data/file.txt");
  });

  test("requesting base itself (no segments) returns base", () => {
    assert.equal(safeJoin("/data"), "/data");
  });

  test("throws if '..' escapes base", () => {
    assert.throws(() => safeJoin("/data", "../etc/passwd"), /escapes/);
  });

  test("throws if nested '..' escapes base", () => {
    assert.throws(() => safeJoin("/data", "sub", "../../etc/passwd"), /escapes/);
  });

  test("throws if a segment is an absolute path outside base", () => {
    assert.throws(() => safeJoin("/data", "/etc/passwd"), /escapes/);
  });
});

describe("buildDirectoryTree", () => {
  let root;

  before(async () => {
    root = await fs.mkdtemp(path.join(os.tmpdir(), "tree-"));
    await fs.writeFile(path.join(root, "a.txt"), "a");
    await fs.writeFile(path.join(root, "b.md"), "b");
    await fs.mkdir(path.join(root, "sub"));
    await fs.writeFile(path.join(root, "sub", "c.txt"), "c");
    await fs.mkdir(path.join(root, "sub", "nested"));
    await fs.writeFile(path.join(root, "sub", "nested", "d.txt"), "d");
  });

  after(async () => {
    await fs.rm(root, { recursive: true, force: true });
  });

  test("builds a sorted, recursive tree of files and directories", async () => {
    const tree = await buildDirectoryTree(root);
    assert.deepEqual(tree, {
      name: path.basename(root),
      type: "directory",
      children: [
        { name: "a.txt", type: "file" },
        { name: "b.md", type: "file" },
        {
          name: "sub",
          type: "directory",
          children: [
            { name: "c.txt", type: "file" },
            {
              name: "nested",
              type: "directory",
              children: [{ name: "d.txt", type: "file" }],
            },
          ],
        },
      ],
    });
  });

  test("an empty directory has an empty children array", async () => {
    const empty = await fs.mkdtemp(path.join(os.tmpdir(), "empty-"));
    try {
      const tree = await buildDirectoryTree(empty);
      assert.deepEqual(tree, { name: path.basename(empty), type: "directory", children: [] });
    } finally {
      await fs.rm(empty, { recursive: true, force: true });
    }
  });
});

describe("waitForEvents", () => {
  test("resolves with the first value of each event, regardless of emission order", async () => {
    const emitter = new EventEmitter();
    const result = waitForEvents(emitter, ["start", "done"]);
    emitter.emit("done", 42);
    emitter.emit("start", "go");
    assert.deepEqual(await result, { start: "go", done: 42 });
  });

  test("only the first emission of a repeated event is kept", async () => {
    const emitter = new EventEmitter();
    const result = waitForEvents(emitter, ["done"]);
    emitter.emit("done", "first");
    emitter.emit("done", "second");
    assert.deepEqual(await result, { done: "first" });
  });

  test("an empty eventNames array resolves immediately with {}", async () => {
    assert.deepEqual(await waitForEvents(new EventEmitter(), []), {});
  });

  test('rejects if "error" is emitted before all events have fired', async () => {
    const emitter = new EventEmitter();
    const result = waitForEvents(emitter, ["done"]);
    emitter.emit("error", new Error("boom"));
    await assert.rejects(result, /boom/);
  });
});

describe("splitLines", () => {
  function collect(stream) {
    return new Promise((resolve) => {
      const lines = [];
      stream.on("data", (line) => lines.push(line));
      stream.on("end", () => resolve(lines));
    });
  }

  test("splits lines across chunk boundaries", async () => {
    const stream = splitLines();
    const result = collect(stream);
    stream.write("hello\nwor");
    stream.write("ld\nfoo");
    stream.end();
    assert.deepEqual(await result, ["hello", "world", "foo"]);
  });

  test("handles \\r\\n line endings", async () => {
    const stream = splitLines();
    const result = collect(stream);
    stream.write("a\r\nb\r\n");
    stream.end();
    assert.deepEqual(await result, ["a", "b"]);
  });

  test("empty input produces no lines", async () => {
    const stream = splitLines();
    const result = collect(stream);
    stream.end();
    assert.deepEqual(await result, []);
  });

  test("a trailing line without a final newline is still emitted", async () => {
    const stream = splitLines();
    const result = collect(stream);
    stream.write("hello");
    stream.end();
    assert.deepEqual(await result, ["hello"]);
  });
});

describe("findFilesByExtension", () => {
  let root;

  before(async () => {
    root = await fs.mkdtemp(path.join(os.tmpdir(), "find-"));
    await fs.writeFile(path.join(root, "a.txt"), "a");
    await fs.writeFile(path.join(root, "b.md"), "b");
    await fs.mkdir(path.join(root, "sub"));
    await fs.writeFile(path.join(root, "sub", "c.txt"), "c");
    await fs.mkdir(path.join(root, "sub", "nested"));
    await fs.writeFile(path.join(root, "sub", "nested", "d.txt"), "d");
  });

  after(async () => {
    await fs.rm(root, { recursive: true, force: true });
  });

  test("finds matching files recursively, returning sorted relative paths", async () => {
    assert.deepEqual(await findFilesByExtension(root, ".txt"), [
      "a.txt",
      "sub/c.txt",
      "sub/nested/d.txt",
    ]);
  });

  test("a different extension returns a different set of files", async () => {
    assert.deepEqual(await findFilesByExtension(root, ".md"), ["b.md"]);
  });

  test("no matches returns an empty array", async () => {
    assert.deepEqual(await findFilesByExtension(root, ".json"), []);
  });
});
