// Package routing implements graph algorithms behind three routing
// protocols: OSPF's link-state SPF calculation (Dijkstra), RIP's
// distance-vector updates (Bellman-Ford, including negative-cycle/routing-
// loop detection), and BGP's path-vector best-path selection. Dijkstra's
// priority queue is built on container/heap, this topic's new Go concept.
package routing

import "errors"

// Graph is a directed, weighted graph represented as an adjacency map:
// edges[from][to] = weight.
type Graph struct {
	edges map[string]map[string]int
}

// NewGraph returns an empty Graph.
func NewGraph() *Graph {
	return &Graph{edges: make(map[string]map[string]int)}
}

// AddEdge adds a directed edge from -> to with the given weight, overwriting
// any existing edge between the same pair. To represent an undirected link
// (as in an OSPF adjacency), call AddEdge twice with from/to swapped.
func (g *Graph) AddEdge(from, to string, weight int) {
	if g.edges[from] == nil {
		g.edges[from] = make(map[string]int)
	}
	g.edges[from][to] = weight
}

// Neighbors returns the outgoing edges from node as a map of neighbor ->
// weight. It returns nil if node has no outgoing edges.
func (g *Graph) Neighbors(node string) map[string]int {
	return g.edges[node]
}

// Nodes returns every node that appears in the graph, either as the source
// or the destination of an edge, in no particular order.
func (g *Graph) Nodes() []string {
	seen := make(map[string]bool)
	for from, neighbors := range g.edges {
		seen[from] = true
		for to := range neighbors {
			seen[to] = true
		}
	}
	nodes := make([]string, 0, len(seen))
	for n := range seen {
		nodes = append(nodes, n)
	}
	return nodes
}

// LSA (Link-State Advertisement) is the information an OSPF router floods
// about itself: its name and the cost to reach each directly-connected
// neighbor.
type LSA struct {
	Router    string
	Neighbors map[string]int
}

// BGPRoute is a candidate route to a prefix, as a BGP speaker would receive
// it from a peer.
type BGPRoute struct {
	Prefix    string
	NextHop   string
	ASPath    []string
	LocalPref int
}

// BuildGraphFromLSAs builds a Graph from a link-state database: a slice of
// LSAs, one per router, each describing that router's directly-connected
// neighbors and the cost to reach them. The resulting graph has one directed
// edge per (router, neighbor) pair in the input.
func BuildGraphFromLSAs(lsas []LSA) *Graph {
	return nil
}

// Dijkstra computes shortest-path distances from source to every other node
// reachable in g, using a container/heap-based priority queue. It returns
// dist, mapping each reachable node (including source, with distance 0) to
// its shortest distance from source, and prev, mapping each reachable node
// other than source to the previous node on its shortest path. Dijkstra
// assumes all edge weights are non-negative.
func (g *Graph) Dijkstra(source string) (dist map[string]int, prev map[string]string) {
	return nil, nil
}

// BellmanFord computes shortest-path distances from source to every other
// node reachable in g, tolerating negative edge weights. It returns dist and
// prev with the same meaning as Dijkstra. If g contains a negative-weight
// cycle reachable from source, BellmanFord returns a non-nil error.
func (g *Graph) BellmanFord(source string) (dist map[string]int, prev map[string]string, err error) {
	return nil, nil, errors.New("not implemented")
}

// ShortestPath reconstructs the shortest path from source to dest using the
// prev map produced by Dijkstra or BellmanFord. The returned slice starts
// with source and ends with dest. If dest is unreachable (absent from prev
// and not equal to source), ShortestPath returns an error.
func ShortestPath(prev map[string]string, source, dest string) ([]string, error) {
	return nil, errors.New("not implemented")
}

// SelectBestRoute runs a simplified version of the BGP best-path selection
// process over routes, all assumed to be for the same prefix. It returns the
// route with the highest LocalPref; ties are broken by the shortest ASPath,
// and further ties by the lexicographically smallest NextHop. SelectBestRoute
// returns an error if routes is empty.
func SelectBestRoute(routes []BGPRoute) (BGPRoute, error) {
	return BGPRoute{}, errors.New("not implemented")
}
