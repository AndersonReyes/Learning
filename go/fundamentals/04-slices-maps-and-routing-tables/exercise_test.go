package routing

import (
	"reflect"
	"testing"
)

func TestMatches(t *testing.T) {
	tests := []struct {
		name string
		r    Route
		addr uint32
		want bool
	}{
		{"matches /8", Route{Prefix: 167772160, PrefixLen: 8}, 167838213, true},          // 10.0.0.0/8 contains 10.1.2.5
		{"does not match /8", Route{Prefix: 167772160, PrefixLen: 8}, 3232235776, false}, // 10.0.0.0/8 vs 192.168.1.0
		{"default route matches anything", Route{Prefix: 0, PrefixLen: 0}, 134744072, true},
		{"exact /32 match", Route{Prefix: 167772161, PrefixLen: 32}, 167772161, true},
		{"exact /32 mismatch", Route{Prefix: 167772161, PrefixLen: 32}, 167772162, false},
		{"matches /24", Route{Prefix: 3232235776, PrefixLen: 24}, 3232235781, true},
		{"does not match /24", Route{Prefix: 3232235776, PrefixLen: 24}, 3232236037, false},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := Matches(tt.r, tt.addr); got != tt.want {
				t.Errorf("Matches(%+v, %d) = %v, want %v", tt.r, tt.addr, got, tt.want)
			}
		})
	}
}

func TestLongestPrefixMatch(t *testing.T) {
	routes := []Route{
		{Prefix: 0, PrefixLen: 0, NextHop: "default"},
		{Prefix: 167772160, PrefixLen: 8, NextHop: "A"},  // 10.0.0.0/8
		{Prefix: 167837696, PrefixLen: 16, NextHop: "B"}, // 10.1.0.0/16
		{Prefix: 167837952, PrefixLen: 24, NextHop: "C"}, // 10.1.1.0/24
	}

	tests := []struct {
		name   string
		routes []Route
		addr   uint32
		want   Route
		wantOk bool
	}{
		{"most specific /24", routes, 167837957, Route{167837952, 24, "C"}, true},  // 10.1.1.5
		{"falls back to /16", routes, 167838213, Route{167837696, 16, "B"}, true},  // 10.1.2.5
		{"falls back to /8", routes, 167903237, Route{167772160, 8, "A"}, true},    // 10.2.0.5
		{"falls back to default", routes, 134744072, Route{0, 0, "default"}, true}, // 8.8.8.8
		{
			"no match without default",
			[]Route{{Prefix: 167772160, PrefixLen: 8, NextHop: "A"}},
			134744072, // 8.8.8.8
			Route{},
			false,
		},
		{
			"tie broken by order",
			[]Route{
				{Prefix: 3232235776, PrefixLen: 24, NextHop: "X"},
				{Prefix: 3232235776, PrefixLen: 24, NextHop: "Y"},
			},
			3232235781, // 192.168.1.5
			Route{3232235776, 24, "X"},
			true,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got, ok := LongestPrefixMatch(tt.routes, tt.addr)
			if ok != tt.wantOk {
				t.Fatalf("LongestPrefixMatch() ok = %v, want %v", ok, tt.wantOk)
			}
			if ok && got != tt.want {
				t.Errorf("LongestPrefixMatch() = %+v, want %+v", got, tt.want)
			}
		})
	}
}

func TestGroupByPrefixLen(t *testing.T) {
	routes := []Route{
		{Prefix: 1, PrefixLen: 8, NextHop: "a"},
		{Prefix: 2, PrefixLen: 16, NextHop: "b"},
		{Prefix: 3, PrefixLen: 8, NextHop: "c"},
		{Prefix: 4, PrefixLen: 24, NextHop: "d"},
	}
	want := map[uint8][]Route{
		8:  {{1, 8, "a"}, {3, 8, "c"}},
		16: {{2, 16, "b"}},
		24: {{4, 24, "d"}},
	}
	if got := GroupByPrefixLen(routes); !reflect.DeepEqual(got, want) {
		t.Errorf("GroupByPrefixLen(%+v) = %+v, want %+v", routes, got, want)
	}

	if got := GroupByPrefixLen(nil); len(got) != 0 {
		t.Errorf("GroupByPrefixLen(nil) = %+v, want empty", got)
	}
}

func TestRemoveRoute(t *testing.T) {
	routes := []Route{
		{Prefix: 167772160, PrefixLen: 24, NextHop: "A"},
		{Prefix: 167837696, PrefixLen: 16, NextHop: "B"},
		{Prefix: 167772160, PrefixLen: 24, NextHop: "C"},
	}

	t.Run("removes all matching prefix+len", func(t *testing.T) {
		got := RemoveRoute(routes, 167772160, 24)
		want := []Route{{Prefix: 167837696, PrefixLen: 16, NextHop: "B"}}
		if !reflect.DeepEqual(got, want) {
			t.Errorf("RemoveRoute() = %+v, want %+v", got, want)
		}
	})

	t.Run("no match leaves routes unchanged", func(t *testing.T) {
		got := RemoveRoute(routes, 999, 9)
		if !reflect.DeepEqual(got, routes) {
			t.Errorf("RemoveRoute() = %+v, want %+v", got, routes)
		}
	})

	t.Run("empty input", func(t *testing.T) {
		if got := RemoveRoute(nil, 1, 8); len(got) != 0 {
			t.Errorf("RemoveRoute(nil, ...) = %+v, want empty", got)
		}
	})
}

func TestAggregateRoutes(t *testing.T) {
	tests := []struct {
		name string
		in   []Route
		want []Route
	}{
		{
			"single sibling pair merges",
			[]Route{
				{Prefix: 167772160, PrefixLen: 25, NextHop: "A"}, // 10.0.0.0/25
				{Prefix: 167772288, PrefixLen: 25, NextHop: "A"}, // 10.0.0.128/25
			},
			[]Route{{Prefix: 167772160, PrefixLen: 24, NextHop: "A"}}, // 10.0.0.0/24
		},
		{
			"cascading merge to /24",
			[]Route{
				{Prefix: 167772160, PrefixLen: 26, NextHop: "A"}, // 10.0.0.0/26
				{Prefix: 167772224, PrefixLen: 26, NextHop: "A"}, // 10.0.0.64/26
				{Prefix: 167772288, PrefixLen: 26, NextHop: "A"}, // 10.0.0.128/26
				{Prefix: 167772352, PrefixLen: 26, NextHop: "A"}, // 10.0.0.192/26
			},
			[]Route{{Prefix: 167772160, PrefixLen: 24, NextHop: "A"}}, // 10.0.0.0/24
		},
		{
			"different next hops do not merge",
			[]Route{
				{Prefix: 167772160, PrefixLen: 25, NextHop: "A"},
				{Prefix: 167772288, PrefixLen: 25, NextHop: "B"},
			},
			[]Route{
				{Prefix: 167772160, PrefixLen: 25, NextHop: "A"},
				{Prefix: 167772288, PrefixLen: 25, NextHop: "B"},
			},
		},
		{
			"non-siblings do not merge",
			[]Route{
				{Prefix: 167772160, PrefixLen: 25, NextHop: "A"}, // 10.0.0.0/25
				{Prefix: 167772416, PrefixLen: 25, NextHop: "A"}, // 10.0.1.0/25
			},
			[]Route{
				{Prefix: 167772160, PrefixLen: 25, NextHop: "A"},
				{Prefix: 167772416, PrefixLen: 25, NextHop: "A"},
			},
		},
		{
			"default route passes through, sorted before merged result",
			[]Route{
				{Prefix: 0, PrefixLen: 0, NextHop: "default"},
				{Prefix: 167772160, PrefixLen: 25, NextHop: "A"},
				{Prefix: 167772288, PrefixLen: 25, NextHop: "A"},
			},
			[]Route{
				{Prefix: 0, PrefixLen: 0, NextHop: "default"},
				{Prefix: 167772160, PrefixLen: 24, NextHop: "A"},
			},
		},
		{
			"single route unchanged",
			[]Route{{Prefix: 167772160, PrefixLen: 24, NextHop: "A"}},
			[]Route{{Prefix: 167772160, PrefixLen: 24, NextHop: "A"}},
		},
		{"empty input", nil, nil},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := AggregateRoutes(tt.in)
			if len(tt.want) == 0 {
				if len(got) != 0 {
					t.Errorf("AggregateRoutes(%v) = %v, want empty", tt.in, got)
				}
				return
			}
			if !reflect.DeepEqual(got, tt.want) {
				t.Errorf("AggregateRoutes(%v) = %+v, want %+v", tt.in, got, tt.want)
			}
		})
	}
}
