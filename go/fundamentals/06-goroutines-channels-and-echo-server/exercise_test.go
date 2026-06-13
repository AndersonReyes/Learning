package echoserver

import (
	"bytes"
	"reflect"
	"sort"
	"testing"
	"time"
)

// chanTimeout bounds how long a test waits on a channel produced by the
// function under test. It's generous enough for a correct implementation
// (which resolves in microseconds) but keeps a not-yet-implemented stub
// (which returns a nil channel) from hanging the test run.
const chanTimeout = 200 * time.Millisecond

// collectChan receives every value from ch until it's closed, or fails
// the test if chanTimeout elapses without a value or a close (including
// the case where ch is nil, which never becomes ready).
func collectChan(t *testing.T, ch <-chan int, timeout time.Duration) []int {
	t.Helper()
	var got []int
	for {
		select {
		case v, ok := <-ch:
			if !ok {
				return got
			}
			got = append(got, v)
		case <-time.After(timeout):
			t.Fatalf("timed out waiting for channel to produce a value or close")
			return nil
		}
	}
}

// bufferedIntChan returns a channel pre-loaded with vals and already
// closed, so reading it never blocks regardless of who (if anyone) reads
// it.
func bufferedIntChan(vals ...int) <-chan int {
	ch := make(chan int, len(vals))
	for _, v := range vals {
		ch <- v
	}
	close(ch)
	return ch
}

// bufferedByteChan returns a receive-only channel pre-loaded with n
// copies of a fixed message and already closed.
func bufferedByteChan(n int) <-chan []byte {
	ch := make(chan []byte, n)
	for i := 0; i < n; i++ {
		ch <- []byte("x")
	}
	close(ch)
	return ch
}

// drainBuffered receives exactly len(ch) values from a buffered channel,
// which never blocks.
func drainBuffered(ch <-chan []byte) [][]byte {
	var out [][]byte
	for i, n := 0, len(ch); i < n; i++ {
		out = append(out, <-ch)
	}
	return out
}

func TestEchoConn(t *testing.T) {
	t.Run("echoes messages in order and counts them", func(t *testing.T) {
		msgs := [][]byte{[]byte("a"), []byte("bb"), []byte("ccc")}
		in := make(chan []byte, len(msgs))
		out := make(chan []byte, len(msgs))
		for _, m := range msgs {
			in <- m
		}
		close(in)

		n := EchoConn(Conn{In: in, Out: out})
		if n != len(msgs) {
			t.Errorf("EchoConn() = %d, want %d", n, len(msgs))
		}

		got := drainBuffered(out)
		if len(got) != len(msgs) {
			t.Fatalf("echoed %d messages, want %d", len(got), len(msgs))
		}
		for i, m := range msgs {
			if !bytes.Equal(got[i], m) {
				t.Errorf("got[%d] = %q, want %q", i, got[i], m)
			}
		}
	})

	t.Run("empty connection echoes nothing", func(t *testing.T) {
		in := make(chan []byte)
		out := make(chan []byte)
		close(in)

		if n := EchoConn(Conn{In: in, Out: out}); n != 0 {
			t.Errorf("EchoConn() = %d, want 0", n)
		}
		if len(out) != 0 {
			t.Errorf("EchoConn() wrote %d messages to an empty connection, want 0", len(out))
		}
	})
}

func TestRunEchoServer(t *testing.T) {
	t.Run("no connections", func(t *testing.T) {
		got := collectChan(t, RunEchoServer(nil), chanTimeout)
		if len(got) != 0 {
			t.Errorf("RunEchoServer(nil) = %v, want empty", got)
		}
	})

	t.Run("runs connections concurrently and reports counts", func(t *testing.T) {
		msgCounts := []int{2, 0, 3}
		conns := make([]Conn, len(msgCounts))
		for i, mc := range msgCounts {
			conns[i] = Conn{In: bufferedByteChan(mc), Out: make(chan []byte, mc)}
		}

		got := collectChan(t, RunEchoServer(conns), chanTimeout)

		sort.Ints(got)
		want := append([]int(nil), msgCounts...)
		sort.Ints(want)
		if !reflect.DeepEqual(got, want) {
			t.Errorf("RunEchoServer counts = %v, want %v (as a set)", got, want)
		}
	})
}

func TestFanIn(t *testing.T) {
	t.Run("no inputs", func(t *testing.T) {
		got := collectChan(t, FanIn(), chanTimeout)
		if len(got) != 0 {
			t.Errorf("FanIn() = %v, want empty", got)
		}
	})

	t.Run("single input is passed through", func(t *testing.T) {
		got := collectChan(t, FanIn(bufferedIntChan(42)), chanTimeout)
		want := []int{42}
		if !reflect.DeepEqual(got, want) {
			t.Errorf("FanIn(a) = %v, want %v", got, want)
		}
	})

	t.Run("merges values from multiple channels", func(t *testing.T) {
		a := bufferedIntChan(1, 2, 3)
		b := bufferedIntChan(4, 5)
		c := bufferedIntChan()

		got := collectChan(t, FanIn(a, b, c), chanTimeout)

		sort.Ints(got)
		want := []int{1, 2, 3, 4, 5}
		if !reflect.DeepEqual(got, want) {
			t.Errorf("FanIn merged = %v, want %v (as a set)", got, want)
		}
	})
}

func TestPipeline(t *testing.T) {
	tests := []struct {
		name string
		nums []int
		want []int
	}{
		{"mixed values", []int{1, 2, 3, 4, 5}, []int{4, 16}},
		{"all odd squares", []int{1, 3, 5}, nil},
		{"all even squares", []int{2, 4}, []int{4, 16}},
		{"negative numbers", []int{-2, -4}, []int{4, 16}},
		{"empty input", []int{}, nil},
		{"zero is even", []int{0, 1}, []int{0}},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := collectChan(t, Pipeline(tt.nums), chanTimeout)
			if !reflect.DeepEqual(got, tt.want) {
				t.Errorf("Pipeline(%v) = %v, want %v", tt.nums, got, tt.want)
			}
		})
	}
}

func TestWorkerPool(t *testing.T) {
	square := func(x int) int { return x * x }

	tests := []struct {
		name       string
		jobs       []int
		numWorkers int
		want       []int
	}{
		{"basic", []int{1, 2, 3, 4, 5}, 2, []int{1, 4, 9, 16, 25}},
		{"empty jobs", []int{}, 3, []int{}},
		{"zero workers defaults to one", []int{1, 2, 3}, 0, []int{1, 4, 9}},
		{"negative workers defaults to one", []int{1, 2, 3}, -5, []int{1, 4, 9}},
		{"more workers than jobs", []int{1, 2}, 10, []int{1, 4}},
		{"single worker", []int{5, 6, 7}, 1, []int{25, 36, 49}},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := WorkerPool(tt.jobs, tt.numWorkers, square)
			if !reflect.DeepEqual(got, tt.want) {
				t.Errorf("WorkerPool(%v, %d, square) = %v, want %v", tt.jobs, tt.numWorkers, got, tt.want)
			}
		})
	}
}
