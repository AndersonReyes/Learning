// Command main demonstrates select, sync.Mutex/WaitGroup, and context
// concepts used in this topic's exercise: a mutex-protected counter
// updated from many goroutines, a non-blocking select with default, a
// select racing a result against time.After, and context cancellation —
// applied to small examples that are deliberately *not* the exercise
// (Stats/WaitAll/DialWithTimeout/MergeWithContext) in exercise.go.
package main

import (
	"context"
	"fmt"
	"sync"
	"time"
)

// counter is a sync.Mutex-protected shared counter.
type counter struct {
	mu sync.Mutex
	n  int
}

func (c *counter) inc() {
	c.mu.Lock()
	defer c.mu.Unlock()
	c.n++
}

func (c *counter) value() int {
	c.mu.Lock()
	defer c.mu.Unlock()
	return c.n
}

func main() {
	// sync.Mutex + sync.WaitGroup: 100 goroutines incrementing one counter.
	c := &counter{}
	var wg sync.WaitGroup
	for i := 0; i < 100; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			c.inc()
		}()
	}
	wg.Wait()
	fmt.Println("counter after 100 increments:", c.value())

	// select + default: a non-blocking check of an empty channel.
	ch := make(chan int)
	select {
	case v := <-ch:
		fmt.Println("received", v)
	default:
		fmt.Println("no value ready")
	}

	// select + time.After: bound how long we wait for a result.
	result := make(chan string, 1)
	go func() {
		time.Sleep(10 * time.Millisecond)
		result <- "done"
	}()
	select {
	case r := <-result:
		fmt.Println("result:", r)
	case <-time.After(100 * time.Millisecond):
		fmt.Println("timed out")
	}

	// context.WithTimeout: ctx.Done() closes once the deadline passes.
	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Millisecond)
	defer cancel()
	<-ctx.Done()
	fmt.Println("ctx error after deadline:", ctx.Err())

	// context.WithCancel: canceling sets ctx.Err() to context.Canceled.
	ctx2, cancel2 := context.WithCancel(context.Background())
	cancel2()
	fmt.Println("ctx2 error after cancel:", ctx2.Err())
}
