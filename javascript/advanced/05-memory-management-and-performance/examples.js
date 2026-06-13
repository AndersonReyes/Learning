// Run with: node examples.js

const sleep = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

// --- WeakMap: metadata tied to an object's lifetime ---
console.log("=== WeakMap for object-keyed caching ===");
{
  const cache = new WeakMap();
  function expensiveLength(obj) {
    if (cache.has(obj)) {
      console.log("  cache hit");
      return cache.get(obj);
    }
    console.log("  cache miss, computing...");
    const result = obj.items.length;
    cache.set(obj, result);
    return result;
  }
  const data = { items: [1, 2, 3] };
  console.log("length:", expensiveLength(data)); // miss, 3
  console.log("length:", expensiveLength(data)); // hit, 3
  // WeakMap is not enumerable -- no .size, .keys(), etc.
  console.log("typeof cache.size:", typeof cache.size); // "undefined"
}

// --- WeakRef and FinalizationRegistry (conceptual) ---
console.log("\n=== WeakRef (conceptual) ===");
{
  let obj = { label: "temporary" };
  const ref = new WeakRef(obj);
  console.log("deref while reachable:", ref.deref()?.label); // "temporary"
  // After `obj = null` and a GC pass (NOT triggered here -- non-deterministic),
  // `ref.deref()` would eventually return `undefined`. Don't rely on the
  // timing -- only use WeakRef for OPTIONAL cleanup/diagnostics.
  obj = null;
  console.log("(GC timing is non-deterministic -- not demonstrated here)");
}

// --- LRU cache in action ---
console.log("\n=== LRU cache ===");
{
  function createLRUCache(capacity) {
    const map = new Map();
    return {
      get(key) {
        if (!map.has(key)) return undefined;
        const value = map.get(key);
        map.delete(key);
        map.set(key, value);
        return value;
      },
      set(key, value) {
        if (map.has(key)) map.delete(key);
        map.set(key, value);
        if (map.size > capacity) {
          map.delete(map.keys().next().value);
        }
      },
      has(key) {
        return map.has(key);
      },
    };
  }

  const cache = createLRUCache(2);
  cache.set("a", 1);
  cache.set("b", 2);
  cache.get("a"); // "a" now most-recently-used
  cache.set("c", 3); // evicts "b"
  console.log("has a:", cache.has("a")); // true
  console.log("has b:", cache.has("b")); // false (evicted)
  console.log("has c:", cache.has("c")); // true
}

// --- Object pool: reuse instead of reallocate ---
console.log("\n=== Object pool ===");
{
  let created = 0;
  const pool = [];
  function acquire() {
    if (pool.length > 0) return pool.pop();
    created++;
    return { id: created, dirty: true };
  }
  function release(obj) {
    obj.dirty = false;
    pool.push(obj);
  }

  const a = acquire();
  release(a);
  const b = acquire();
  console.log("reused same object:", a === b); // true
  console.log("objects created:", created); // 1
}

// --- Batching: amortize the cost of many small operations ---
console.log("\n=== Batching ===");
{
  const flushedBatches = [];
  let batch = [];
  function add(item) {
    batch.push(item);
    if (batch.length >= 3) {
      flushedBatches.push(batch);
      batch = [];
    }
  }
  [1, 2, 3, 4, 5, 6, 7].forEach(add);
  console.log("flushed batches:", flushedBatches); // [[1,2,3],[4,5,6]]
  console.log("remaining (unflushed):", batch); // [7]
}

// --- performance.now() for measuring durations ---
console.log("\n=== performance.now() ===");
{
  const start = performance.now();
  await sleep(10);
  const elapsed = performance.now() - start;
  console.log(`elapsed >= 10ms: ${elapsed >= 10}`);
}
