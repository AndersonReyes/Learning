// Command main demonstrates container/heap directly — implementing
// heap.Interface for a small priority queue and using heap.Push/heap.Pop —
// and shows Dijkstra's "lazy deletion" pattern on a tiny inline graph. Both
// are illustrative warm-ups for this topic's exercise, which builds a
// similar priority queue inside Graph.Dijkstra.
package main

import (
	"container/heap"
	"fmt"
)

// task is a unit of work with a priority; lower priority values run first.
type task struct {
	name     string
	priority int
}

// taskQueue is a min-heap of tasks ordered by priority, implementing
// container/heap.Interface.
type taskQueue []task

func (tq taskQueue) Len() int           { return len(tq) }
func (tq taskQueue) Less(i, j int) bool { return tq[i].priority < tq[j].priority }
func (tq taskQueue) Swap(i, j int)      { tq[i], tq[j] = tq[j], tq[i] }

func (tq *taskQueue) Push(x any) {
	*tq = append(*tq, x.(task))
}

func (tq *taskQueue) Pop() any {
	old := *tq
	n := len(old)
	item := old[n-1]
	*tq = old[:n-1]
	return item
}

func main() {
	fmt.Println("--- container/heap: priority queue of tasks ---")

	tq := &taskQueue{
		{"send keepalive", 5},
		{"flood LSA", 1},
		{"recompute SPF tree", 2},
		{"log statistics", 9},
	}
	heap.Init(tq)

	heap.Push(tq, task{"handle BGP withdrawal", 0})

	for tq.Len() > 0 {
		t := heap.Pop(tq).(task)
		fmt.Printf("priority %d: %s\n", t.priority, t.name)
	}

	fmt.Println("\n--- Dijkstra's lazy deletion, illustrated ---")

	// A tiny graph where B is reachable two ways: directly from A (cost 5)
	// and via C (cost 1 + 1 = 2). The heap will contain a stale entry for B
	// with dist=5 alongside the better dist=2 entry; lazy deletion skips
	// the stale one once B has already been finalized.
	type edge struct {
		to     string
		weight int
	}
	graph := map[string][]edge{
		"A": {{"B", 5}, {"C", 1}},
		"C": {{"B", 1}},
		"B": {},
	}

	dist := map[string]int{"A": 0}
	visited := map[string]bool{}

	pq := &taskQueue{}
	push := func(node string, d int) {
		heap.Push(pq, task{name: node, priority: d})
	}
	push("A", 0)

	for pq.Len() > 0 {
		item := heap.Pop(pq).(task)
		node, d := item.name, item.priority

		if visited[node] {
			fmt.Printf("skipping stale entry: %s at dist=%d (already finalized)\n", node, d)
			continue
		}
		visited[node] = true
		fmt.Printf("finalizing %s at dist=%d\n", node, d)

		for _, e := range graph[node] {
			newDist := d + e.weight
			if existing, ok := dist[e.to]; !ok || newDist < existing {
				dist[e.to] = newDist
				push(e.to, newDist)
			}
		}
	}

	fmt.Printf("final distances: %v\n", dist)
}
