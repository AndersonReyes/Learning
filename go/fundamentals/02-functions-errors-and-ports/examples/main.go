// Command main demonstrates functions/errors concepts used in this topic's
// exercises: multiple & named returns, variadic functions, custom error
// types, sentinel errors with wrapping, errors.Is/As, sort.Slice, and
// case-insensitive map lookups — applied to small examples that are
// deliberately *not* the exercises in exercise.go.
package main

import (
	"errors"
	"fmt"
	"sort"
	"strings"
)

// divmod demonstrates multiple return values.
func divmod(a, b int) (int, int) {
	return a / b, a % b
}

// splitHostPort demonstrates named return values and a "naked" return.
func splitHostPort(s string) (host string, port string) {
	i := strings.LastIndex(s, ":")
	if i < 0 {
		host = s
		return // naked return: host=s, port="" (zero value)
	}
	host, port = s[:i], s[i+1:]
	return
}

// sum demonstrates a variadic function.
func sum(nums ...int) int {
	total := 0
	for _, n := range nums {
		total += n
	}
	return total
}

// ProtocolError is a custom error type carrying structured info.
type ProtocolError struct {
	Number int
}

func (e *ProtocolError) Error() string {
	return fmt.Sprintf("unknown IP protocol number %d", e.Number)
}

// ErrNotFound is a sentinel error for "valid but unrecognized" lookups.
var ErrNotFound = errors.New("not found")

// protocolName demonstrates sentinel errors, wrapping with %w, and a custom
// error type for a different failure mode.
func protocolName(number int) (string, error) {
	names := map[int]string{1: "icmp", 6: "tcp", 17: "udp"}
	if name, ok := names[number]; ok {
		return name, nil
	}
	if number < 0 || number > 255 {
		return "", &ProtocolError{Number: number}
	}
	return "", fmt.Errorf("protocol %d: %w", number, ErrNotFound)
}

func main() {
	q, r := divmod(17, 5)
	fmt.Println("divmod(17,5) =", q, r)

	h, p := splitHostPort("example.com:8080")
	fmt.Printf("host=%q port=%q\n", h, p)
	h2, p2 := splitHostPort("example.com")
	fmt.Printf("host=%q port=%q\n", h2, p2)

	fmt.Println("sum() =", sum())
	fmt.Println("sum(1,2,3) =", sum(1, 2, 3))
	nums := []int{4, 5, 6}
	fmt.Println("sum(nums...) =", sum(nums...))

	for _, n := range []int{6, 99, 300} {
		name, err := protocolName(n)
		switch {
		case err == nil:
			fmt.Printf("protocol %d = %s\n", n, name)
		case errors.Is(err, ErrNotFound):
			fmt.Printf("protocol %d: not found (%v)\n", n, err)
		default:
			var protoErr *ProtocolError
			if errors.As(err, &protoErr) {
				fmt.Printf("protocol %d: invalid (%v)\n", n, protoErr)
			}
		}
	}

	// sort.Slice over a slice of structs
	type entry struct {
		name string
		port int
	}
	entries := []entry{{"https", 443}, {"dns", 53}, {"http", 80}}
	sort.Slice(entries, func(i, j int) bool { return entries[i].port < entries[j].port })
	fmt.Println("sorted by port:", entries)

	// case-insensitive map lookup
	lookup := map[string]int{"http": 80, "https": 443}
	query := "HTTP"
	if port, ok := lookup[strings.ToLower(query)]; ok {
		fmt.Printf("%s -> port %d\n", query, port)
	}
}
