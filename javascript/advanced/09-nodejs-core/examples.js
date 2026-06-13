// Run with: node examples.js
import fs from "node:fs/promises";
import os from "node:os";
import path from "node:path";
import { EventEmitter } from "node:events";
import { Transform } from "node:stream";
import { fileURLToPath } from "node:url";

// --- fs/promises + path ---
console.log("=== fs/promises + path ===");
{
  const dir = await fs.mkdtemp(path.join(os.tmpdir(), "examples-"));
  const file = path.join(dir, "hello.txt");
  await fs.writeFile(file, "hello world");
  console.log("  read back:", await fs.readFile(file, "utf8"));
  console.log("  basename:", path.basename(file));
  console.log("  dirname matches dir:", path.dirname(file) === dir);

  console.log("  -- path.resolve discards earlier args after an absolute one --");
  console.log("  ", path.resolve("/data", "/etc/passwd"));

  await fs.rm(dir, { recursive: true, force: true });
}

// --- EventEmitter, including the special "error" event ---
console.log("\n=== EventEmitter ===");
{
  const emitter = new EventEmitter();
  emitter.on("data", (chunk) => console.log("  got:", chunk));
  emitter.once("end", () => console.log("  stream ended"));
  emitter.emit("data", "first");
  emitter.emit("data", "second");
  emitter.emit("end");
  emitter.emit("end"); // once -- no second "stream ended"

  console.log("  -- unhandled 'error' would throw; with a listener it doesn't --");
  emitter.on("error", (err) => console.log("  caught error:", err.message));
  emitter.emit("error", new Error("boom"));
}

// --- Transform stream ---
console.log("\n=== Transform stream (uppercase) ===");
{
  const upper = new Transform({
    transform(chunk, encoding, callback) {
      this.push(chunk.toString().toUpperCase());
      callback();
    },
  });

  const chunks = [];
  upper.on("data", (chunk) => chunks.push(chunk.toString()));
  upper.write("hello ");
  upper.write("world");
  upper.end();
  await new Promise((resolve) => upper.on("end", resolve));
  console.log("  result:", chunks.join(""));
}

// --- Modules: import.meta.url, fileURLToPath, dynamic import ---
console.log("\n=== Modules (ESM) ===");
{
  console.log("  import.meta.url:", import.meta.url);
  const __filename = fileURLToPath(import.meta.url);
  console.log("  __filename (via fileURLToPath):", path.basename(__filename));

  const path2 = await import("node:path");
  console.log("  dynamic import of 'node:path' works:", typeof path2.join === "function");
}
