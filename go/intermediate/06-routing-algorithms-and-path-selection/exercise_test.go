package routing

import (
	"reflect"
	"testing"
)

// buildSampleGraph returns the 5-node undirected graph used throughout this
// topic's tests, with edges A-B:4, A-C:2, B-C:1, B-D:5, C-D:8, C-E:10,
// D-E:2.
func buildSampleGraph() *Graph {
	g := NewGraph()
	edges := []struct {
		a, b string
		w    int
	}{
		{"A", "B", 4},
		{"A", "C", 2},
		{"B", "C", 1},
		{"B", "D", 5},
		{"C", "D", 8},
		{"C", "E", 10},
		{"D", "E", 2},
	}
	for _, e := range edges {
		g.AddEdge(e.a, e.b, e.w)
		g.AddEdge(e.b, e.a, e.w)
	}
	return g
}

func TestDijkstra(t *testing.T) {
	g := buildSampleGraph()

	dist, prev := g.Dijkstra("A")

	wantDist := map[string]int{"A": 0, "B": 3, "C": 2, "D": 8, "E": 10}
	if !reflect.DeepEqual(dist, wantDist) {
		t.Errorf("dist = %v, want %v", dist, wantDist)
	}

	wantPrev := map[string]string{"B": "C", "C": "A", "D": "B", "E": "D"}
	if !reflect.DeepEqual(prev, wantPrev) {
		t.Errorf("prev = %v, want %v", prev, wantPrev)
	}
}

func TestBellmanFord(t *testing.T) {
	g := buildSampleGraph()

	dist, prev, err := g.BellmanFord("A")
	if err != nil {
		t.Fatalf("BellmanFord() error = %v", err)
	}

	wantDist := map[string]int{"A": 0, "B": 3, "C": 2, "D": 8, "E": 10}
	if !reflect.DeepEqual(dist, wantDist) {
		t.Errorf("dist = %v, want %v", dist, wantDist)
	}

	wantPrev := map[string]string{"B": "C", "C": "A", "D": "B", "E": "D"}
	if !reflect.DeepEqual(prev, wantPrev) {
		t.Errorf("prev = %v, want %v", prev, wantPrev)
	}
}

func TestBellmanFordNegativeWeights(t *testing.T) {
	g := NewGraph()
	g.AddEdge("A", "B", 4)
	g.AddEdge("A", "C", 5)
	g.AddEdge("B", "C", -3)
	g.AddEdge("C", "D", 2)

	dist, prev, err := g.BellmanFord("A")
	if err != nil {
		t.Fatalf("BellmanFord() error = %v", err)
	}

	wantDist := map[string]int{"A": 0, "B": 4, "C": 1, "D": 3}
	if !reflect.DeepEqual(dist, wantDist) {
		t.Errorf("dist = %v, want %v", dist, wantDist)
	}

	wantPrev := map[string]string{"B": "A", "C": "B", "D": "C"}
	if !reflect.DeepEqual(prev, wantPrev) {
		t.Errorf("prev = %v, want %v", prev, wantPrev)
	}
}

func TestBellmanFordNegativeCycle(t *testing.T) {
	g := NewGraph()
	g.AddEdge("A", "B", 1)
	g.AddEdge("B", "C", -3)
	g.AddEdge("C", "A", 1)

	if _, _, err := g.BellmanFord("A"); err == nil {
		t.Fatal("BellmanFord() error = nil, want error for negative cycle")
	}
}

func TestShortestPath(t *testing.T) {
	g := buildSampleGraph()
	_, prev := g.Dijkstra("A")

	tests := []struct {
		name    string
		source  string
		dest    string
		want    []string
		wantErr bool
	}{
		{"same source and dest", "A", "A", []string{"A"}, false},
		{"direct neighbor", "A", "C", []string{"A", "C"}, false},
		{"multi-hop path", "A", "E", []string{"A", "C", "B", "D", "E"}, false},
		{"unreachable node", "A", "Z", nil, true},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got, err := ShortestPath(prev, tt.source, tt.dest)
			if (err != nil) != tt.wantErr {
				t.Fatalf("ShortestPath(%s, %s) error = %v, wantErr %v", tt.source, tt.dest, err, tt.wantErr)
			}
			if tt.wantErr {
				return
			}
			if !reflect.DeepEqual(got, tt.want) {
				t.Errorf("ShortestPath(%s, %s) = %v, want %v", tt.source, tt.dest, got, tt.want)
			}
		})
	}
}

func TestBuildGraphFromLSAs(t *testing.T) {
	lsas := []LSA{
		{Router: "A", Neighbors: map[string]int{"B": 4, "C": 2}},
		{Router: "B", Neighbors: map[string]int{"A": 4, "C": 1, "D": 5}},
		{Router: "C", Neighbors: map[string]int{"A": 2, "B": 1, "D": 8, "E": 10}},
		{Router: "D", Neighbors: map[string]int{"B": 5, "C": 8, "E": 2}},
		{Router: "E", Neighbors: map[string]int{"C": 10, "D": 2}},
	}

	g := BuildGraphFromLSAs(lsas)
	if g == nil {
		t.Fatal("BuildGraphFromLSAs() returned nil graph")
	}

	wantA := map[string]int{"B": 4, "C": 2}
	if got := g.Neighbors("A"); !reflect.DeepEqual(got, wantA) {
		t.Errorf("Neighbors(A) = %v, want %v", got, wantA)
	}

	dist, prev := g.Dijkstra("A")

	wantDist := map[string]int{"A": 0, "B": 3, "C": 2, "D": 8, "E": 10}
	if !reflect.DeepEqual(dist, wantDist) {
		t.Errorf("dist = %v, want %v", dist, wantDist)
	}

	wantPrev := map[string]string{"B": "C", "C": "A", "D": "B", "E": "D"}
	if !reflect.DeepEqual(prev, wantPrev) {
		t.Errorf("prev = %v, want %v", prev, wantPrev)
	}
}

func TestSelectBestRoute(t *testing.T) {
	tests := []struct {
		name    string
		routes  []BGPRoute
		want    BGPRoute
		wantErr bool
	}{
		{
			name: "shortest AS path wins at equal local pref",
			routes: []BGPRoute{
				{Prefix: "10.0.0.0/8", NextHop: "192.0.2.3", ASPath: []string{"65002", "65003", "65004"}, LocalPref: 100},
				{Prefix: "10.0.0.0/8", NextHop: "192.0.2.1", ASPath: []string{"65001"}, LocalPref: 100},
				{Prefix: "10.0.0.0/8", NextHop: "192.0.2.2", ASPath: []string{"65002", "65003"}, LocalPref: 100},
			},
			want: BGPRoute{Prefix: "10.0.0.0/8", NextHop: "192.0.2.1", ASPath: []string{"65001"}, LocalPref: 100},
		},
		{
			name: "higher local pref wins despite longer AS path",
			routes: []BGPRoute{
				{Prefix: "10.0.0.0/8", NextHop: "192.0.2.4", ASPath: []string{"65004"}, LocalPref: 50},
				{Prefix: "10.0.0.0/8", NextHop: "192.0.2.1", ASPath: []string{"65001", "65002", "65003"}, LocalPref: 200},
			},
			want: BGPRoute{Prefix: "10.0.0.0/8", NextHop: "192.0.2.1", ASPath: []string{"65001", "65002", "65003"}, LocalPref: 200},
		},
		{
			name: "lowest next hop wins at equal local pref and AS path length",
			routes: []BGPRoute{
				{Prefix: "10.0.0.0/8", NextHop: "192.0.2.5", ASPath: []string{"65001", "65002"}, LocalPref: 100},
				{Prefix: "10.0.0.0/8", NextHop: "192.0.2.2", ASPath: []string{"65003", "65004"}, LocalPref: 100},
			},
			want: BGPRoute{Prefix: "10.0.0.0/8", NextHop: "192.0.2.2", ASPath: []string{"65003", "65004"}, LocalPref: 100},
		},
		{
			name:    "no routes",
			routes:  nil,
			wantErr: true,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got, err := SelectBestRoute(tt.routes)
			if (err != nil) != tt.wantErr {
				t.Fatalf("SelectBestRoute() error = %v, wantErr %v", err, tt.wantErr)
			}
			if tt.wantErr {
				return
			}
			if !reflect.DeepEqual(got, tt.want) {
				t.Errorf("SelectBestRoute() = %v, want %v", got, tt.want)
			}
		})
	}
}
