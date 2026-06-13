import { describe, test } from "node:test";
import assert from "node:assert/strict";
import {
  Graph,
  setOp,
  createPrivateStore,
  mostCommon,
  MultiMap,
} from "./exercise.js";

describe("Graph", () => {
  test("addEdge auto-creates nodes and adds edges in both directions", () => {
    const g = new Graph();
    g.addEdge("A", "B");

    assert.equal(g.adjacency.has("A"), true);
    assert.equal(g.adjacency.has("B"), true);
    assert.equal(g.adjacency.get("A").has("B"), true);
    assert.equal(g.adjacency.get("B").has("A"), true);
  });

  test("adding the same edge twice is a no-op (no duplicate neighbors)", () => {
    const g = new Graph();
    g.addEdge("A", "B");
    g.addEdge("A", "B");
    g.addEdge("B", "A");

    assert.equal(g.adjacency.get("A").size, 1);
    assert.equal(g.adjacency.get("B").size, 1);
  });

  test("bfs throws if start is not in the graph", () => {
    const g = new Graph();
    g.addEdge("A", "B");
    assert.throws(() => g.bfs("Z"));
  });

  test("dfs throws if start is not in the graph", () => {
    const g = new Graph();
    g.addEdge("A", "B");
    assert.throws(() => g.dfs("Z"));
  });

  test("bfs on a single node returns just that node", () => {
    const g = new Graph();
    g.addEdge("A", "B");
    g.adjacency.set("Z", new Set());
    assert.deepEqual(g.bfs("Z"), ["Z"]);
  });

  // Graph edges (undirected, added in this order):
  //   A-B, A-C, B-D, C-D, D-E, B-E
  //
  // Adjacency lists (insertion order):
  //   A: [B, C]
  //   B: [A, D, E]
  //   C: [A, D]
  //   D: [B, C, E]
  //   E: [D, B]
  //
  // BFS from A (hand-traced):
  //   visit A -> enqueue neighbors B, C
  //   visit B -> enqueue unvisited neighbors D, E (A already visited)
  //   visit C -> neighbors A, D already visited/enqueued
  //   visit D -> neighbors B, C, E already visited/enqueued
  //   visit E -> neighbors D, B already visited
  //   order: A, B, C, D, E
  //
  // DFS from A, iterative stack, neighbors pushed in REVERSE insertion
  // order (so the first-inserted neighbor is popped/visited first):
  //   stack=[A] -> pop A, visit. neighbors [B,C] reversed -> push C, B
  //   stack=[C,B] -> pop B, visit. neighbors [A,D,E] reversed -> push E, D
  //     (A skipped, already visited)
  //   stack=[C,E,D] -> pop D, visit. neighbors [B,C,E] reversed -> push E, C
  //     (B skipped, already visited)
  //   stack=[C,E,E,C] -> pop C, visit. neighbors [A,D] reversed -> nothing
  //     pushed (both already visited)
  //   stack=[C,E,E] -> pop E, visit. neighbors [D,B] reversed -> nothing
  //     pushed (both already visited)
  //   stack=[C,E] -> pop E, already visited, skip
  //   stack=[C] -> pop C, already visited, skip
  //   order: A, B, D, C, E
  function buildGraph() {
    const g = new Graph();
    g.addEdge("A", "B");
    g.addEdge("A", "C");
    g.addEdge("B", "D");
    g.addEdge("C", "D");
    g.addEdge("D", "E");
    g.addEdge("B", "E");
    return g;
  }

  test("bfs visits nodes in breadth-first order, neighbors in insertion order", () => {
    const g = buildGraph();
    assert.deepEqual(g.bfs("A"), ["A", "B", "C", "D", "E"]);
  });

  test("dfs visits nodes in depth-first order (iterative, hand-verified)", () => {
    const g = buildGraph();
    assert.deepEqual(g.dfs("A"), ["A", "B", "D", "C", "E"]);
  });

  test("bfs/dfs starting from a non-root node", () => {
    const g = buildGraph();
    // From E: neighbors [D, B]
    // BFS: E -> D,B -> (D's neighbors B,C,E; B's neighbors A,D,E) -> C, A
    assert.deepEqual(g.bfs("E"), ["E", "D", "B", "C", "A"]);
  });
});

describe("setOp", () => {
  test("union returns all values from both sets", () => {
    const result = setOp(new Set([1, 2, 3]), new Set([2, 3, 4]), "union");
    assert.deepEqual([...result].sort(), [1, 2, 3, 4]);
  });

  test("intersection returns values in both sets", () => {
    const result = setOp(
      new Set([1, 2, 3]),
      new Set([2, 3, 4]),
      "intersection",
    );
    assert.deepEqual([...result].sort(), [2, 3]);
  });

  test("difference returns values in a but not b", () => {
    const result = setOp(new Set([1, 2, 3]), new Set([2, 3, 4]), "difference");
    assert.deepEqual([...result], [1]);
  });

  test("difference is not symmetric", () => {
    const result = setOp(new Set([2, 3, 4]), new Set([1, 2, 3]), "difference");
    assert.deepEqual([...result], [4]);
  });

  test("symmetricDifference returns values in exactly one set", () => {
    const result = setOp(
      new Set([1, 2, 3]),
      new Set([2, 3, 4]),
      "symmetricDifference",
    );
    assert.deepEqual([...result].sort(), [1, 4]);
  });

  test("does not modify the input sets", () => {
    const a = new Set([1, 2, 3]);
    const b = new Set([2, 3, 4]);
    setOp(a, b, "union");
    assert.deepEqual([...a], [1, 2, 3]);
    assert.deepEqual([...b], [2, 3, 4]);
  });

  test("returns a new Set instance, not a or b", () => {
    const a = new Set([1, 2]);
    const b = new Set([2, 3]);
    const result = setOp(a, b, "union");
    assert.notEqual(result, a);
    assert.notEqual(result, b);
    assert.ok(result instanceof Set);
  });

  test("throws on an unknown operation", () => {
    assert.throws(
      () => setOp(new Set([1]), new Set([2]), "bogus"),
      /[Uu]nknown|[Ii]nvalid|operation/,
    );
  });
});

describe("createPrivateStore", () => {
  test("set/get round-trips data for an object key", () => {
    const store = createPrivateStore();
    const key = {};
    store.set(key, { secret: 42 });
    assert.deepEqual(store.get(key), { secret: 42 });
  });

  test("get returns undefined for an object never set", () => {
    const store = createPrivateStore();
    assert.equal(store.get({}), undefined);
  });

  test("has reflects presence of an entry", () => {
    const store = createPrivateStore();
    const key = {};
    assert.equal(store.has(key), false);
    store.set(key, "data");
    assert.equal(store.has(key), true);
  });

  test("set overwrites a previous association for the same object", () => {
    const store = createPrivateStore();
    const key = {};
    store.set(key, "first");
    store.set(key, "second");
    assert.equal(store.get(key), "second");
  });

  test("different object instances are different keys, even if structurally equal", () => {
    const store = createPrivateStore();
    store.set({ id: 1 }, "a");
    assert.equal(store.get({ id: 1 }), undefined);
  });

  test("set throws TypeError for a primitive obj", () => {
    const store = createPrivateStore();
    assert.throws(() => store.set("not an object", "x"), TypeError);
    assert.throws(() => store.set(42, "x"), TypeError);
    assert.throws(() => store.set(null, "x"), TypeError);
    assert.throws(() => store.set(undefined, "x"), TypeError);
  });

  test("get throws TypeError for a primitive obj", () => {
    const store = createPrivateStore();
    assert.throws(() => store.get("not an object"), TypeError);
  });

  test("has throws TypeError for a primitive obj", () => {
    const store = createPrivateStore();
    assert.throws(() => store.has(42), TypeError);
  });

  test("separate stores have independent data", () => {
    const storeA = createPrivateStore();
    const storeB = createPrivateStore();
    const key = {};
    storeA.set(key, "in A");
    assert.equal(storeB.has(key), false);
  });
});

describe("mostCommon", () => {
  test("returns the n most frequent items, descending by count", () => {
    const result = mostCommon(["a", "b", "a", "c", "b", "a"], 2);
    assert.deepEqual(result, [
      ["a", 3],
      ["b", 2],
    ]);
  });

  test("ties broken by first-occurrence order", () => {
    // "b" and "c" both occur once; "b" appears before "c" in items.
    const result = mostCommon(["a", "a", "b", "c"], 3);
    assert.deepEqual(result, [
      ["a", 2],
      ["b", 1],
      ["c", 1],
    ]);
  });

  test("n exceeding distinct count returns all distinct items", () => {
    const result = mostCommon(["a", "b", "c"], 10);
    assert.deepEqual(result, [
      ["a", 1],
      ["b", 1],
      ["c", 1],
    ]);
  });

  test("n <= 0 returns an empty array", () => {
    assert.deepEqual(mostCommon(["a", "b"], 0), []);
    assert.deepEqual(mostCommon(["a", "b"], -5), []);
  });

  test("empty items array returns an empty array", () => {
    assert.deepEqual(mostCommon([], 5), []);
  });

  test("works with numbers", () => {
    const result = mostCommon([1, 2, 2, 3, 3, 3], 1);
    assert.deepEqual(result, [[3, 3]]);
  });

  test("object items are compared by reference, not deep equality", () => {
    const objA = { id: 1 };
    const objB = { id: 1 }; // structurally identical, different reference
    const result = mostCommon([objA, objA, objB], 2);
    assert.deepEqual(result, [
      [objA, 2],
      [objB, 1],
    ]);
  });
});

describe("MultiMap", () => {
  test("add stores values for a key, get returns them in insertion order", () => {
    const mm = new MultiMap();
    mm.add("fruit", "apple");
    mm.add("fruit", "banana");
    assert.deepEqual(mm.get("fruit"), ["apple", "banana"]);
  });

  test("adding the same value twice is a no-op", () => {
    const mm = new MultiMap();
    mm.add("fruit", "apple");
    mm.add("fruit", "apple");
    assert.deepEqual(mm.get("fruit"), ["apple"]);
  });

  test("get returns [] for a missing key", () => {
    const mm = new MultiMap();
    assert.deepEqual(mm.get("missing"), []);
  });

  test("get never returns the internal Set", () => {
    const mm = new MultiMap();
    mm.add("fruit", "apple");
    const result = mm.get("fruit");
    assert.ok(Array.isArray(result));
    assert.equal(result instanceof Set, false);
  });

  test("has(key) checks key existence; has(key, value) checks a specific value", () => {
    const mm = new MultiMap();
    mm.add("fruit", "apple");

    assert.equal(mm.has("fruit"), true);
    assert.equal(mm.has("missing"), false);
    assert.equal(mm.has("fruit", "apple"), true);
    assert.equal(mm.has("fruit", "kiwi"), false);
    assert.equal(mm.has("missing", "apple"), false);
  });

  test("delete removes a value and returns true", () => {
    const mm = new MultiMap();
    mm.add("fruit", "apple");
    mm.add("fruit", "banana");

    assert.equal(mm.delete("fruit", "apple"), true);
    assert.deepEqual(mm.get("fruit"), ["banana"]);
  });

  test("delete removes the key entirely once its set becomes empty", () => {
    const mm = new MultiMap();
    mm.add("fruit", "apple");

    assert.equal(mm.delete("fruit", "apple"), true);
    assert.equal(mm.has("fruit"), false);
    assert.deepEqual(mm.get("fruit"), []);
  });

  test("delete returns false for a missing key or value", () => {
    const mm = new MultiMap();
    mm.add("fruit", "apple");

    assert.equal(mm.delete("missing", "x"), false);
    assert.equal(mm.delete("fruit", "kiwi"), false);
  });

  test("multiple keys are independent", () => {
    const mm = new MultiMap();
    mm.add("fruit", "apple");
    mm.add("veg", "carrot");

    assert.deepEqual(mm.get("fruit"), ["apple"]);
    assert.deepEqual(mm.get("veg"), ["carrot"]);

    mm.delete("fruit", "apple");
    assert.deepEqual(mm.get("veg"), ["carrot"]);
  });
});
