// Command main demonstrates Go generics concepts used in this topic's
// exercise (type parameters, constraints, generic types) via small
// standalone helpers — generic Max, Filter, Map, and a generic Set[T] —
// applied to illustrative networking data (round-trip times, IP
// addresses, hostnames). These are deliberately *not* the exercise
// (Pool[T]/LRU[K, V] in exercise.go).
package main

import (
	"cmp"
	"fmt"
	"net"
	"time"
)

// Max returns the larger of a and b for any ordered type.
func Max[T cmp.Ordered](a, b T) T {
	if a > b {
		return a
	}
	return b
}

// Filter returns a new slice containing only the elements of s for which
// keep returns true.
func Filter[T any](s []T, keep func(T) bool) []T {
	var out []T
	for _, v := range s {
		if keep(v) {
			out = append(out, v)
		}
	}
	return out
}

// Map returns a new slice with f applied to each element of s.
func Map[T, U any](s []T, f func(T) U) []U {
	out := make([]U, len(s))
	for i, v := range s {
		out[i] = f(v)
	}
	return out
}

// Set is a generic set backed by a map; T must be comparable to be used as
// a map key.
type Set[T comparable] struct {
	m map[T]struct{}
}

// NewSet returns a Set containing items.
func NewSet[T comparable](items ...T) *Set[T] {
	s := &Set[T]{m: make(map[T]struct{}, len(items))}
	for _, it := range items {
		s.m[it] = struct{}{}
	}
	return s
}

// Contains reports whether item is in the set.
func (s *Set[T]) Contains(item T) bool {
	_, ok := s.m[item]
	return ok
}

// Len returns the number of items in the set.
func (s *Set[T]) Len() int {
	return len(s.m)
}

func main() {
	// Generic Max with type inference — no [int]/[float64]/[string]
	// needed at the call site.
	fmt.Println("Max(3, 7):", Max(3, 7))
	fmt.Println("Max(2.5, 1.1):", Max(2.5, 1.1))
	fmt.Println(`Max("a", "b"):`, Max("a", "b"))

	// time.Duration's underlying type is int64, so it satisfies
	// cmp.Ordered too.
	rtts := []time.Duration{12 * time.Millisecond, 45 * time.Millisecond, 8 * time.Millisecond}
	var worst time.Duration
	for _, rtt := range rtts {
		worst = Max(worst, rtt)
	}
	fmt.Println("worst RTT:", worst)

	// Generic Filter + Map chained over a slice of addresses.
	addrs := []net.IP{
		net.IPv4(192, 0, 2, 1),
		net.IPv4(10, 0, 0, 1),
		net.IPv4(198, 51, 100, 7),
		net.IPv4(10, 0, 0, 2),
	}
	private := Filter(addrs, func(ip net.IP) bool {
		return ip.IsPrivate()
	})
	fmt.Println("private addresses:", private)

	strs := Map(private, func(ip net.IP) string {
		return ip.String()
	})
	fmt.Println("private addresses as strings:", strs)

	// Generic Set[T] with a comparable type — here, hostnames.
	seen := NewSet("a.example.com", "b.example.com")
	for _, name := range []string{"a.example.com", "c.example.com"} {
		fmt.Printf("seen(%q) = %v\n", name, seen.Contains(name))
	}
	fmt.Println("set size:", seen.Len())
}
