import { test, describe } from "node:test";
import assert from "node:assert/strict";
import {
  createEmitter,
  buildUrl,
  parseQueryString,
  fetchJSON,
  deepCloneWithout,
} from "./exercise.js";

describe("createEmitter", () => {
  test("on() calls the handler with event.detail when emitted", () => {
    const log = [];
    const emitter = createEmitter();
    emitter.on("msg", (detail) => log.push(detail));
    emitter.emit("msg", "hello");
    assert.deepEqual(log, ["hello"]);
  });

  test("multiple handlers for the same type fire in registration order", () => {
    const log = [];
    const emitter = createEmitter();
    emitter.on("msg", (detail) => log.push(`a:${detail}`));
    emitter.on("msg", (detail) => log.push(`b:${detail}`));
    emitter.emit("msg", "x");
    assert.deepEqual(log, ["a:x", "b:x"]);
  });

  test("once() fires only on the first emit", () => {
    const log = [];
    const emitter = createEmitter();
    emitter.once("ping", (detail) => log.push(detail));
    emitter.emit("ping", 1);
    emitter.emit("ping", 2);
    assert.deepEqual(log, [1]);
  });

  test("off() removes a previously registered handler", () => {
    const log = [];
    const emitter = createEmitter();
    const handler = (detail) => log.push(detail);
    emitter.on("msg", handler);
    emitter.emit("msg", "first");
    emitter.off("msg", handler);
    emitter.emit("msg", "second");
    assert.deepEqual(log, ["first"]);
  });

  test("off() for an unregistered handler is a no-op", () => {
    const emitter = createEmitter();
    assert.doesNotThrow(() => emitter.off("msg", () => {}));
  });

  test("off() for one event type doesn't affect the same handler on another type", () => {
    const log = [];
    const emitter = createEmitter();
    const handler = (detail) => log.push(detail);
    emitter.on("a", handler);
    emitter.on("b", handler);
    emitter.off("a", handler);
    emitter.emit("a", "should-not-appear");
    emitter.emit("b", "should-appear");
    assert.deepEqual(log, ["should-appear"]);
  });

  test("emit() with no handlers is a no-op", () => {
    const emitter = createEmitter();
    assert.doesNotThrow(() => emitter.emit("nothing", 1));
  });
});

describe("buildUrl", () => {
  test("returns the base URL when no path or params are given", () => {
    assert.equal(buildUrl("https://api.example.com"), "https://api.example.com/");
  });

  test("appends a path segment to the base path", () => {
    assert.equal(
      buildUrl("https://api.example.com/v1", { path: "users" }),
      "https://api.example.com/v1/users",
    );
  });

  test("path joining is unaffected by a trailing slash on the base", () => {
    assert.equal(
      buildUrl("https://api.example.com/v1/", { path: "users" }),
      "https://api.example.com/v1/users",
    );
  });

  test("path joining is unaffected by a leading slash on the path", () => {
    assert.equal(
      buildUrl("https://api.example.com/v1", { path: "/users/1" }),
      "https://api.example.com/v1/users/1",
    );
  });

  test("appends scalar query parameters", () => {
    assert.equal(
      buildUrl("https://api.example.com", { params: { q: "x" } }),
      "https://api.example.com/?q=x",
    );
  });

  test("array-valued params produce repeated query keys in order", () => {
    assert.equal(
      buildUrl("https://api.example.com/v1", {
        path: "search",
        params: { q: "hello world", tags: ["a", "b"] },
      }),
      "https://api.example.com/v1/search?q=hello+world&tags=a&tags=b",
    );
  });
});

describe("parseQueryString", () => {
  test("returns {} for an empty query string", () => {
    assert.deepEqual(parseQueryString(""), {});
  });

  test("ignores a leading '?'", () => {
    assert.deepEqual(parseQueryString("?a=1"), { a: "1" });
  });

  test("decodes percent-encoded values", () => {
    assert.deepEqual(parseQueryString("q=hello%20world"), { q: "hello world" });
  });

  test("collects repeated keys into an array, preserving order", () => {
    assert.deepEqual(parseQueryString("b=2&b=3"), { b: ["2", "3"] });
  });

  test("mixes single and repeated keys", () => {
    assert.deepEqual(parseQueryString("?a=1&b=2&b=3&q=hello%20world"), {
      a: "1",
      b: ["2", "3"],
      q: "hello world",
    });
  });
});

describe("fetchJSON", () => {
  test("returns parsed JSON when the response is ok", async () => {
    const fakeFetch = async () => ({
      ok: true,
      status: 200,
      json: async () => ({ id: 1, name: "Ada" }),
    });
    const result = await fetchJSON("https://api.example.com/users/1", fakeFetch);
    assert.deepEqual(result, { id: 1, name: "Ada" });
  });

  test("throws an Error mentioning the status and url when not ok", async () => {
    const failFetch = async () => ({
      ok: false,
      status: 404,
      json: async () => ({}),
    });
    await assert.rejects(
      fetchJSON("https://api.example.com/missing", failFetch),
      (err) => {
        assert.match(err.message, /404/);
        assert.match(err.message, /https:\/\/api\.example\.com\/missing/);
        return true;
      },
    );
  });

  test("passes the exact url through to fetchFn", async () => {
    let receivedUrl;
    const fakeFetch = async (url) => {
      receivedUrl = url;
      return { ok: true, status: 200, json: async () => ({}) };
    };
    await fetchJSON("https://api.example.com/ping", fakeFetch);
    assert.equal(receivedUrl, "https://api.example.com/ping");
  });
});

describe("deepCloneWithout", () => {
  test("removes the given top-level keys from the clone", () => {
    const original = { id: 1, password: "secret", name: "Ada" };
    const safe = deepCloneWithout(original, ["password"]);
    assert.deepEqual(safe, { id: 1, name: "Ada" });
  });

  test("does not mutate the original object", () => {
    const original = { id: 1, password: "secret" };
    deepCloneWithout(original, ["password"]);
    assert.deepEqual(original, { id: 1, password: "secret" });
  });

  test("deep-clones nested objects/arrays independently of the original", () => {
    const original = { profile: { name: "Ada", tags: ["x", "y"] } };
    const safe = deepCloneWithout(original, []);
    safe.profile.name = "changed";
    safe.profile.tags.push("z");
    assert.equal(original.profile.name, "Ada");
    assert.deepEqual(original.profile.tags, ["x", "y"]);
  });

  test("a key not present in obj is silently ignored", () => {
    assert.deepEqual(deepCloneWithout({ a: 1 }, ["nonexistent"]), { a: 1 });
  });

  test("preserves Map/Set values via structuredClone", () => {
    const original = { tags: new Set(["a", "b"]), counts: new Map([["a", 1]]) };
    const safe = deepCloneWithout(original, []);
    assert.ok(safe.tags instanceof Set);
    assert.deepEqual([...safe.tags], ["a", "b"]);
    assert.ok(safe.counts instanceof Map);
    assert.equal(safe.counts.get("a"), 1);
  });
});
