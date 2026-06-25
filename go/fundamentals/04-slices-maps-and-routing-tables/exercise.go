// Package routing implements a simple IPv4 routing table: prefix
// matching, longest-prefix-match lookup, grouping by prefix length, route
// removal, and CIDR route aggregation (RFC 4632 §3).
package routing

import "fmt"

// Route is a single routing table entry: the network Prefix/PrefixLen
// (CIDR block) and the NextHop that traffic for that block is sent to.
type Route struct {
	Prefix    uint32
	PrefixLen uint8
	NextHop   string
}

// Matches reports whether addr falls within r's CIDR block, i.e. whether
// addr's top r.PrefixLen bits equal r.Prefix's top r.PrefixLen bits.
// PrefixLen 0 (the default route) matches every address; PrefixLen 32
// matches only r.Prefix itself.
//
//	Matches(Route{Prefix: 10.0.0.0, PrefixLen: 8}, 10.1.2.3)  -> true
//	Matches(Route{Prefix: 10.0.0.0, PrefixLen: 8}, 11.0.0.0)  -> false
//	Matches(Route{Prefix: 0, PrefixLen: 0}, anything)         -> true
func Matches(r Route, addr uint32) bool {

	if r.Prefix == 0 && r.PrefixLen == 0 {
		return true
	}

	network := r.Prefix >> uint32(32-r.PrefixLen)

	if (addr >> uint32(32-r.PrefixLen)) == network {
		return true
	}
	return false
}

// LongestPrefixMatch returns the route in routes whose CIDR block contains
// addr and whose PrefixLen is largest (the most specific match). If
// multiple routes tie on PrefixLen, the one appearing earliest in routes
// wins. If no route matches, it returns the zero Route and false.
func LongestPrefixMatch(routes []Route, addr uint32) (Route, bool) {
	largest, matched := Route{}, false

	fmt.Printf("")
	for i, r := range routes {
		if Matches(r, addr) {
			// if its the first match (i == 0), just set it to largest
			if i == 0 || (r.PrefixLen > largest.PrefixLen) {
				largest = r
				matched = true
			}

		}
	}
	return largest, matched
}

// GroupByPrefixLen groups routes by their PrefixLen, preserving the
// relative order of routes within each group.
//
//	GroupByPrefixLen([]Route{{PrefixLen: 8}, {PrefixLen: 16}, {PrefixLen: 8}})
//	  -> map[uint8][]Route{8: {route0, route2}, 16: {route1}}
func GroupByPrefixLen(routes []Route) map[uint8][]Route {
	m := make(map[uint8][]Route)
	for _, r := range routes {
		m[r.PrefixLen] = append(m[r.PrefixLen], r)
	}
	return m
}

// RemoveRoute returns a new slice containing all routes EXCEPT those whose
// Prefix and PrefixLen both equal the given values (NextHop is ignored, so
// this can remove multiple entries that share a CIDR block but differ in
// NextHop). The relative order of the remaining routes is preserved.
func RemoveRoute(routes []Route, prefix uint32, prefixLen uint8) []Route {
	out := []Route{}

	for _, r := range routes {
		if r.Prefix != prefix && r.PrefixLen != prefixLen {
			out = append(out, r)
		}
	}
	return out
}

// AggregateRoutes repeatedly merges pairs of "sibling" routes — same
// PrefixLen > 0, same NextHop, whose Prefixes differ only in the single
// bit that splits their common parent block in half — into that parent
// block (PrefixLen-1), until no more merges are possible. The result is
// sorted by (PrefixLen ascending, then Prefix ascending).
//
//	AggregateRoutes([]Route{
//		{Prefix: 10.0.0.0,   PrefixLen: 25, NextHop: "A"},
//		{Prefix: 10.0.0.128, PrefixLen: 25, NextHop: "A"},
//	}) -> []Route{{Prefix: 10.0.0.0, PrefixLen: 24, NextHop: "A"}}
func AggregateRoutes(routes []Route) []Route {
	return nil
}
