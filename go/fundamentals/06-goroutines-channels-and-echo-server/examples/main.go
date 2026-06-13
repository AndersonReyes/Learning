// Command main demonstrates goroutines and channels concepts used in
// this topic's exercise: launching goroutines, unbuffered vs buffered
// channels, directional channel types, and the generator pattern via
// close+range — applied to small examples that are deliberately *not*
// the exercise (the echo-server/fan-in/pipeline/worker-pool patterns) in
// exercise.go.
package main

import "fmt"

// greet runs in its own goroutine and sends a single message on ch. ch
// is declared chan<- string (send-only) since greet only ever sends.
func greet(name string, ch chan<- string) {
	ch <- fmt.Sprintf("hello, %s", name)
}

// naturals returns a channel that yields 1..n in order and then closes —
// the generator pattern enabled by close+range.
func naturals(n int) <-chan int {
	out := make(chan int)
	go func() {
		for i := 1; i <= n; i++ {
			out <- i
		}
		close(out)
	}()
	return out
}

// sum reads every value from in (a receive-only channel) until it's
// closed, and returns their total.
func sum(in <-chan int) int {
	total := 0
	for v := range in {
		total += v
	}
	return total
}

func main() {
	// Unbuffered channel: greet's send blocks until main receives —
	// the two goroutines rendezvous at the channel operation.
	ch := make(chan string)
	go greet("gopher", ch)
	fmt.Println(<-ch)

	// Buffered channel: sends up to capacity don't block, even with no
	// receiver running yet.
	buf := make(chan int, 3)
	buf <- 1
	buf <- 2
	buf <- 3
	fmt.Println("buffered channel length before receive:", len(buf))
	fmt.Println(<-buf, <-buf, <-buf)

	// Generator + range/close: naturals(5) yields 1,2,3,4,5 then closes,
	// ending the range loop.
	var got []int
	for v := range naturals(5) {
		got = append(got, v)
	}
	fmt.Println("naturals(5):", got)

	// Directional channel types compose: sum takes <-chan int, and
	// naturals returns <-chan int.
	fmt.Println("sum(naturals(100)):", sum(naturals(100)))
}
