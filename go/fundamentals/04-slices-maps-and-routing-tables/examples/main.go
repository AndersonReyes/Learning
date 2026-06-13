// Command main demonstrates arrays, slices, and maps concepts used in this
// topic's exercise: array value semantics, slices sharing a backing array,
// append's reallocation behavior, nil maps, the comma-ok idiom, and
// "group by" with map[K][]V — applied to small examples that are
// deliberately *not* the exercise (routing tables) in exercise.go.
package main

import "fmt"

func main() {
	// Arrays are value types: assigning copies all elements.
	a := [4]byte{192, 168, 1, 1}
	b := a
	b[0] = 10
	fmt.Println("array a:", a) // unchanged
	fmt.Println("array b:", b) // [10 168 1 1]

	// Slicing shares the underlying array.
	s := []int{10, 20, 30, 40, 50}
	mid := s[1:3] // [20 30], shares s's backing array
	mid[0] = 99
	fmt.Println("s after mutating mid:", s) // [10 99 30 40 50]

	// append within capacity writes into the shared backing array...
	fmt.Println("len(mid), cap(mid):", len(mid), cap(mid))
	withinCap := append(mid, -1)
	fmt.Println("s after append within cap:", s) // s[3] overwritten with -1

	// ...but append beyond capacity allocates a new array, breaking the link.
	grown := append(withinCap, -2, -3, -4)
	grown[0] = 1000
	fmt.Println("s after append beyond cap:", s) // unaffected now

	// Nil maps: reading is safe, writing panics.
	var nilMap map[string]int
	fmt.Println("read from nil map:", nilMap["missing"]) // 0, no panic

	// comma-ok idiom
	m := map[string]int{"http": 80, "https": 443}
	if port, ok := m["http"]; ok {
		fmt.Println("http port:", port)
	}
	if _, ok := m["ftp"]; !ok {
		fmt.Println("ftp not found")
	}
	delete(m, "http")
	fmt.Println("after delete:", m)

	// "group by" with map[K][]V: append works on a nil slice value.
	type entry struct {
		proto string
		port  int
	}
	entries := []entry{{"tcp", 80}, {"udp", 53}, {"tcp", 443}, {"udp", 123}}
	byProto := make(map[string][]int)
	for _, e := range entries {
		byProto[e.proto] = append(byProto[e.proto], e.port)
	}
	fmt.Println("tcp ports:", byProto["tcp"])
	fmt.Println("udp ports:", byProto["udp"])
}
