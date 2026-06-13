# Intermediate 6. `container/heap` + Routing Algorithms (Dijkstra/Bellman-Ford/BGP)

## New: `container/heap`

`container/heap` turns any type implementing `heap.Interface` into a binary
min-heap (priority queue). You implement five methods; the package provides
`Init`, `Push`, `Pop`, `Fix`, `Remove` as free functions operating on your type.

```go
type heap.Interface interface {
    sort.Interface // Len() int, Less(i, j int) bool, Swap(i, j int)
    Push(x any)     // add x as element Len()
    Pop() any       // remove and return element Len() - 1
}
```

A typical priority queue of `(node, distance)` pairs:

```go
type pqItem struct {
    node string
    dist int
}

type priorityQueue []pqItem

func (pq priorityQueue) Len() int            { return len(pq) }
func (pq priorityQueue) Less(i, j int) bool  { return pq[i].dist < pq[j].dist }
func (pq priorityQueue) Swap(i, j int)       { pq[i], pq[j] = pq[j], pq[i] }

func (pq *priorityQueue) Push(x any) {
    *pq = append(*pq, x.(pqItem))
}

func (pq *priorityQueue) Pop() any {
    old := *pq
    n := len(old)
    item := old[n-1]
    *pq = old[:n-1]
    return item
}
```

Usage:

```go
pq := &priorityQueue{}
heap.Init(pq)               // not needed if pq starts empty
heap.Push(pq, pqItem{"A", 0})
heap.Push(pq, pqItem{"B", 4})
item := heap.Pop(pq).(pqItem) // lowest dist first
```

**Gotchas**:

- `Push`/`Pop` take/return `any` (interface{}) — you must type-assert.
- `Push` and `Pop` use **pointer receivers** (`*priorityQueue`) because they
  resize the underlying slice; `Len`/`Less`/`Swap` can use value receivers.
- `heap.Pop` swaps the root with the last element, shrinks the slice by one,
  then sifts down — it does NOT just slice off the end. Always call
  `heap.Pop`, never index/slice the heap directly while it's in use.
- **Lazy deletion**: `container/heap` has no efficient "decrease-key"
  operation. Dijkstra's algorithm pushes a new `(node, dist)` entry every time
  a shorter distance is found, leaving stale (larger-distance) entries in the
  heap. Handle this by checking, when popping, whether the popped distance is
  still the best known one for that node (skip if a `visited` set already
  marked it finalized, or if `dist > distances[node]`).

## Networking: Routing protocols and path selection

Three protocols, three different ways of picking the "best" path — all
reducible to graph algorithms.

### OSPF (Open Shortest Path First) — link-state, Dijkstra

Each OSPF router floods **Link-State Advertisements (LSAs)**: "I am router X,
and my neighbors are Y (cost 4) and Z (cost 2)". Every router collects LSAs
from the whole area into an identical **link-state database** — effectively
a weighted graph of the network — then independently runs **Dijkstra's
algorithm** from itself as the source to compute a shortest-path tree
(the SPF, Shortest Path First, tree). The next hop toward any destination is
read off this tree.

Dijkstra requires **non-negative edge weights** (OSPF costs are configured as
positive integers, conventionally `reference-bandwidth / interface-bandwidth`).
The algorithm:

1. `dist[source] = 0`, all others `= infinity`.
2. Push `(source, 0)` onto a min-priority-queue keyed by distance.
3. Pop the lowest-distance unvisited node `u`. Mark visited.
4. For each neighbor `v` of `u`: if `dist[u] + weight(u,v) < dist[v]`, update
   `dist[v]` and `prev[v] = u`, push `(v, dist[v])`.
5. Repeat until the queue is empty. `prev` now encodes a shortest-path tree;
   walk it backward from any destination to `source` to get the path.

### RIP (Routing Information Protocol) — distance-vector, Bellman-Ford

RIP routers don't build a full topology map. Instead each router tells its
neighbors "my distance to network N is D" (a **distance vector**), and each
router relaxes its own table: `dist[N] = min(dist[N], neighbor_dist[N] +
cost_to_neighbor)`. Repeated across the whole network, this converges to the
same result as running **Bellman-Ford** from every node.

Bellman-Ford (single-source version):

1. `dist[source] = 0`, all others `= infinity`.
2. Repeat `|V| - 1` times: for every edge `(u, v, w)`, if `dist[u] + w <
   dist[v]`, set `dist[v] = dist[u] + w` and `prev[v] = u`.
3. One more pass over all edges: if any edge can still be relaxed, the graph
   has a **negative-weight cycle** reachable from `source` — report an error.

Unlike Dijkstra, Bellman-Ford tolerates negative edge weights and *detects*
negative cycles — which in distance-vector routing correspond to **routing
loops**. RIP's classic failure mode, "count-to-infinity" (a loop's distance
slowly climbing by 1 each exchange instead of converging), is exactly what a
negative cycle does to naive Bellman-Ford: it never stabilizes. RIP bounds
this with a max metric of 16 ("infinity"); split-horizon and route poisoning
are further mitigations RFC 2453 layers on top of the base algorithm.

### BGP (Border Gateway Protocol) — path-vector, attribute-based best-path

BGP routers exchange entire **paths** (sequences of Autonomous System
numbers, the `AS_PATH`), not just distances — this is why it's called
path-vector, and why it can detect loops trivially (a router rejects any
route whose `AS_PATH` already contains its own AS number).

When a router learns multiple routes to the same prefix, it runs the **BGP
best-path selection algorithm** — a sequence of tie-breaking attribute
comparisons (RFC 4271 §9.1.2.2, simplified here to the two most commonly
configured):

1. Highest **`LOCAL_PREF`** wins (a locally-configured preference — "prefer
   this exit point over that one").
2. If tied, shortest **`AS_PATH` length** wins (fewer AS hops = "closer").
3. Further tiebreakers exist (origin type, MED, eBGP over iBGP, lowest IGP
   metric to next hop, lowest router ID, lowest neighbor address) — this
   exercise stops at AS_PATH length plus a final lowest-`NEXT_HOP`-address
   tiebreak for determinism.

Note the contrast with OSPF/RIP: BGP doesn't pick the path with the lowest
"cost" in a metric sense at all — `LOCAL_PREF` is a policy knob, letting an
AS's operators override pure path length for business reasons (e.g. prefer a
cheaper transit provider even if it's an extra AS hop away).

## Further Reading

- [`container/heap`](https://pkg.go.dev/container/heap)
- [RFC 2328 (OSPF Version 2)](https://www.rfc-editor.org/rfc/rfc2328) — §16.1
  describes the Dijkstra-based SPF calculation.
- [RFC 2453 (RIP Version 2)](https://www.rfc-editor.org/rfc/rfc2453) — §2.1
  describes the distance-vector update algorithm and count-to-infinity.
- [RFC 4271 (BGP-4)](https://www.rfc-editor.org/rfc/rfc4271) — §9.1.2.2
  (Phase 2: Route Selection) describes the best-path tie-breaking process.
