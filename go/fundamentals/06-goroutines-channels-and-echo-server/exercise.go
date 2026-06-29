// Package echoserver models a concurrent RFC 862 Echo server using only
// goroutines and channels: simulated connections (channel pairs) in place
// of net.Conn, and the fan-in/fan-out/pipeline patterns used to build one.
package echoserver

import (
	"fmt"
)

// Conn is a simulated bidirectional connection: In delivers messages
// arriving from the client, and Out is where a handler writes responses
// back to the client.
type Conn struct {
	In  <-chan []byte
	Out chan<- []byte
}

// EchoConn implements the RFC 862 Echo protocol for a single connection:
// every message received on c.In is written back unchanged, in order, to
// c.Out. EchoConn returns once c.In is closed, after closing c.Out, and
// reports the number of messages it echoed.
//
// EchoConn is synchronous; call it with `go EchoConn(c)` to run it
// concurrently with other connections.
func EchoConn(c Conn) int {
	var count = 0
	for m := range c.In {
		c.Out <- m
		count++
	}
	close(c.Out)
	return count
}

// RunEchoServer starts EchoConn for every connection in conns,
// concurrently, and returns immediately. The returned channel delivers
// exactly one int per connection — the number of messages that
// connection echoed — in the order connections finish (not necessarily
// the order of conns). The returned channel is closed after all
// len(conns) counts have been sent.
//
// RunEchoServer(nil) and RunEchoServer() return an already-closed
// channel.
func RunEchoServer(conns []Conn) <-chan int {
	msgs := make(chan int)

	go func() {
		for _, c := range conns {
			count := EchoConn(c)
			msgs <- count
		}
		close(msgs)
	}()

	return msgs
}

// FanIn merges zero or more read-only int channels into a single output
// channel. Every value received from any input channel is forwarded to
// the output (interleaving across inputs is non-deterministic, but no
// value is dropped or duplicated). The output channel is closed once
// every input channel has been closed and drained.
//
// FanIn() with no inputs returns an already-closed channel.
func FanIn(inputs ...<-chan int) <-chan int {
	out := make(chan int, len(inputs)*2)

	for _, ch := range inputs {
		if ch != nil {
			for v := range ch {
				fmt.Printf("wrote value=%d to out channel\n", v)
				out <- v
			}
		}
	}
	close(out)
	return out
}

// Pipeline runs nums through a three-stage processing pipeline connected
// by channels:
//
//  1. generate:   emits each value in nums, in order
//  2. square:     replaces each value with its square
//  3. filterEven: drops values whose square is odd
//
// Pipeline returns the output channel of the final stage, which is
// closed once every value from nums has been processed. Because each
// stage forwards values one at a time and in order, the values received
// from the returned channel are the even squares from nums, in the same
// relative order as the corresponding inputs in nums.
func Pipeline(nums []int) <-chan int {
	generate := make(chan int, len(nums)+1)
	squares := make(chan int, len(nums)+1)
	filterEven := make(chan int, len(nums)+1)

	go func() {
		// stage 1
		for _, v := range nums {
			generate <- v
			fmt.Printf("stage 1 v=%d done\n", v)
		}
		close(generate)

	}()

	go func() {

		// stage 2
		for v := range generate {
			squares <- v * v
			fmt.Printf("stage 2 v=%d done\n", v)
		}
		close(squares)
	}()

	go func() {
		// stage 3
		for v := range squares {
			if v%2 == 0 {
				filterEven <- v
				fmt.Printf("stage 3 v=%d done\n", v)
			}
		}
		close(filterEven)
	}()
	return filterEven
}

// WorkerPool applies work to each element of jobs using numWorkers
// concurrent goroutines, and returns the results in the same order as
// jobs (result[i] == work(jobs[i])), regardless of the order in which
// workers finish. If numWorkers < 1, WorkerPool uses a single worker.
func WorkerPool(jobs []int, numWorkers int, work func(int) int) []int {
	if numWorkers < 1 {
		numWorkers = 1
	}

	out := make([]int, len(jobs))
	workerReadChannels := make([]chan int, numWorkers)
	workerDoneChannels := make([]chan bool, numWorkers)

	for i := range numWorkers {
		workerReadChannels[i] = make(chan int)
		workerDoneChannels[i] = make(chan bool)
	}

	// Do work
	for workerId, ch := range workerReadChannels {
		// start worker thread
		go func() {
			fmt.Printf("Starting worker=%d\n", workerId)
			for jobId := range ch {
				result := work(jobs[jobId])
				out[jobId] = result
				fmt.Printf("Worker=%d completed JobId=%d Job=%d with result=%d\n", workerId, jobId, jobs[jobId], result)
			}
			close(workerDoneChannels[workerId])

		}()
	}
	// Separate work for each worker
	for jobId := range jobs {
		workerId := jobId % numWorkers
		fmt.Printf("Sending JobId=%d to Worker=%d\n", jobId, workerId)
		workerReadChannels[workerId] <- jobId
	}

	// allow workers to start working
	for _, ch := range workerReadChannels {
		close(ch)
	}

	// wait for channels to be done
	for workerId, ch := range workerDoneChannels {
		for range ch {
		}

		fmt.Printf("worker %d is done\n", workerId)
	}

	return out
}
