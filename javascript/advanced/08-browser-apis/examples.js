// Run with: node examples.js

// --- EventTarget + CustomEvent ---
console.log("=== EventTarget / CustomEvent ===");
{
  const target = new EventTarget();

  const handler = (event) => console.log(`  greet handler: hello ${event.detail}`);
  target.addEventListener("greet", handler);
  target.dispatchEvent(new CustomEvent("greet", { detail: "world" }));

  console.log("  -- once: only fires on first dispatch --");
  target.addEventListener("ping", (e) => console.log(`  ping #${e.detail}`), { once: true });
  target.dispatchEvent(new CustomEvent("ping", { detail: 1 }));
  target.dispatchEvent(new CustomEvent("ping", { detail: 2 })); // no output -- listener removed

  console.log("  -- removeEventListener requires the same reference --");
  target.removeEventListener("greet", handler);
  target.dispatchEvent(new CustomEvent("greet", { detail: "again" })); // no output
  console.log("  (no 'hello again' printed -- listener was removed)");
}

// --- URL / URLSearchParams ---
console.log("\n=== URL / URLSearchParams ===");
{
  const url = new URL("/users/1?active=true", "https://api.example.com/v1/");
  console.log("  href:", url.href);
  console.log("  origin:", url.origin);
  console.log("  pathname:", url.pathname);
  console.log("  search:", url.search);

  console.log("  -- relative resolution gotcha --");
  console.log("  new URL('users', '.../v1'):  ", new URL("users", "https://api.example.com/v1").href);
  console.log("  new URL('users', '.../v1/'): ", new URL("users", "https://api.example.com/v1/").href);

  const params = new URLSearchParams("a=1&b=2&b=3");
  console.log("  get('a'):", params.get("a"));
  console.log("  getAll('b'):", params.getAll("b"));
  params.append("c", "hello world");
  console.log("  toString() (space -> '+'):", params.toString());
}

// --- fetch via dependency injection (no real network) ---
console.log("\n=== fetch (injected) ===");
{
  async function getJSON(url, fetchFn) {
    const response = await fetchFn(url);
    if (!response.ok) throw new Error(`HTTP ${response.status}`);
    return response.json();
  }

  const fakeFetch = async (url) => ({
    ok: true,
    status: 200,
    json: async () => ({ url, data: "fake response" }),
  });

  const result = await getJSON("https://api.example.com/ping", fakeFetch);
  console.log("  result:", result);

  const failFetch = async () => ({ ok: false, status: 404, json: async () => ({}) });
  try {
    await getJSON("https://api.example.com/missing", failFetch);
  } catch (err) {
    console.log("  caught:", err.message);
  }
}

// --- structuredClone ---
console.log("\n=== structuredClone ===");
{
  const original = { nested: { arr: [1, 2, 3] }, tags: new Set(["a", "b"]) };
  const clone = structuredClone(original);

  clone.nested.arr.push(4);
  console.log("  original.nested.arr:", original.nested.arr); // unaffected
  console.log("  clone.nested.arr:", clone.nested.arr);
  console.log("  clone.tags instanceof Set:", clone.tags instanceof Set);
}

// --- AbortController ---
console.log("\n=== AbortController ===");
{
  const controller = new AbortController();
  controller.signal.addEventListener("abort", () => console.log("  signal aborted"));
  controller.abort();
  console.log("  signal.aborted:", controller.signal.aborted);
}
