package poolcache

import (
	"errors"
	"net"
	"sync"
	"sync/atomic"
	"testing"
)

func TestPool(t *testing.T) {
	t.Run("Get calls New when empty", func(t *testing.T) {
		var calls int32
		p := NewPool(2, func() (int, error) {
			n := atomic.AddInt32(&calls, 1)
			return int(n), nil
		})

		v, err := p.Get()
		if err != nil {
			t.Fatalf("Get() error = %v", err)
		}
		if v != 1 {
			t.Errorf("Get() = %d, want 1", v)
		}
		if calls != 1 {
			t.Errorf("New called %d times, want 1", calls)
		}
	})

	t.Run("Put then Get reuses without calling New again", func(t *testing.T) {
		var calls int32
		p := NewPool(2, func() (int, error) {
			n := atomic.AddInt32(&calls, 1)
			return int(n), nil
		})

		v, err := p.Get()
		if err != nil {
			t.Fatalf("Get() error = %v", err)
		}
		p.Put(v)

		got, err := p.Get()
		if err != nil {
			t.Fatalf("Get() error = %v", err)
		}
		if got != v {
			t.Errorf("Get() after Put = %d, want %d", got, v)
		}
		if calls != 1 {
			t.Errorf("New called %d times, want 1", calls)
		}
	})

	t.Run("Put discards values beyond max idle", func(t *testing.T) {
		var calls int32
		p := NewPool(1, func() (int, error) {
			n := atomic.AddInt32(&calls, 1)
			return int(n), nil
		})

		a, err := p.Get() // calls = 1, a = 1
		if err != nil {
			t.Fatalf("Get() error = %v", err)
		}
		b, err := p.Get() // calls = 2, b = 2
		if err != nil {
			t.Fatalf("Get() error = %v", err)
		}
		p.Put(a)
		p.Put(b) // pool already holds 1 idle value (a); b is discarded

		got, err := p.Get()
		if err != nil {
			t.Fatalf("Get() error = %v", err)
		}
		if got != a {
			t.Errorf("Get() = %d, want %d (reused idle value)", got, a)
		}

		got2, err := p.Get()
		if err != nil {
			t.Fatalf("Get() error = %v", err)
		}
		if got2 != 3 {
			t.Errorf("Get() = %d, want 3 (New called again)", got2)
		}
	})

	t.Run("New error is propagated", func(t *testing.T) {
		wantErr := errors.New("boom")
		p := NewPool(1, func() (int, error) {
			return 0, wantErr
		})

		_, err := p.Get()
		if !errors.Is(err, wantErr) {
			t.Errorf("Get() error = %v, want %v", err, wantErr)
		}
	})

	t.Run("concurrent Get and Put", func(t *testing.T) {
		var calls int32
		p := NewPool(10, func() (int, error) {
			return int(atomic.AddInt32(&calls, 1)), nil
		})

		var wg sync.WaitGroup
		for i := 0; i < 50; i++ {
			wg.Add(1)
			go func() {
				defer wg.Done()
				v, err := p.Get()
				if err != nil {
					t.Errorf("Get() error = %v", err)
					return
				}
				p.Put(v)
			}()
		}
		wg.Wait()
	})
}

func TestLRU(t *testing.T) {
	t.Run("Get on empty cache returns false", func(t *testing.T) {
		c := NewLRU[string, int](2)
		if _, ok := c.Get("a"); ok {
			t.Error(`Get("a") ok = true, want false`)
		}
	})

	t.Run("Put then Get round trip", func(t *testing.T) {
		c := NewLRU[string, int](2)
		c.Put("a", 1)

		got, ok := c.Get("a")
		if !ok {
			t.Fatal(`Get("a") ok = false, want true`)
		}
		if got != 1 {
			t.Errorf(`Get("a") = %d, want 1`, got)
		}
	})

	t.Run("Put updates an existing key without growing Len", func(t *testing.T) {
		c := NewLRU[string, int](2)
		c.Put("a", 1)
		c.Put("a", 2)

		got, ok := c.Get("a")
		if !ok {
			t.Fatal(`Get("a") ok = false, want true`)
		}
		if got != 2 {
			t.Errorf(`Get("a") = %d, want 2`, got)
		}
		if got := c.Len(); got != 1 {
			t.Errorf("Len() = %d, want 1", got)
		}
	})

	t.Run("eviction order: least-recently-used is evicted", func(t *testing.T) {
		c := NewLRU[string, int](2)
		c.Put("a", 1)
		c.Put("b", 2)
		c.Put("c", 3) // evicts "a" (least recently used)

		if _, ok := c.Get("a"); ok {
			t.Error(`Get("a") ok = true, want false (evicted)`)
		}
		if got, ok := c.Get("b"); !ok || got != 2 {
			t.Errorf(`Get("b") = (%d, %v), want (2, true)`, got, ok)
		}
		if got, ok := c.Get("c"); !ok || got != 3 {
			t.Errorf(`Get("c") = (%d, %v), want (3, true)`, got, ok)
		}
		if got := c.Len(); got != 2 {
			t.Errorf("Len() = %d, want 2", got)
		}
	})

	t.Run("Get refreshes recency", func(t *testing.T) {
		c := NewLRU[string, int](2)
		c.Put("a", 1)
		c.Put("b", 2)
		c.Get("a")    // "a" is now most-recently-used
		c.Put("c", 3) // evicts "b" instead of "a"

		if _, ok := c.Get("b"); ok {
			t.Error(`Get("b") ok = true, want false (evicted)`)
		}
		if _, ok := c.Get("a"); !ok {
			t.Error(`Get("a") ok = false, want true`)
		}
		if _, ok := c.Get("c"); !ok {
			t.Error(`Get("c") ok = false, want true`)
		}
	})

	t.Run("caches DNS-style lookup results", func(t *testing.T) {
		c := NewLRU[string, []net.IP](4)
		want := []net.IP{net.IPv4(192, 0, 2, 1), net.IPv4(192, 0, 2, 2)}
		c.Put("example.com", want)

		got, ok := c.Get("example.com")
		if !ok {
			t.Fatal(`Get("example.com") ok = false, want true`)
		}
		if len(got) != len(want) {
			t.Fatalf("len(got) = %d, want %d", len(got), len(want))
		}
		for i := range want {
			if !got[i].Equal(want[i]) {
				t.Errorf("got[%d] = %v, want %v", i, got[i], want[i])
			}
		}
	})

	t.Run("concurrent Get and Put", func(t *testing.T) {
		c := NewLRU[int, int](50)

		var wg sync.WaitGroup
		for i := 0; i < 100; i++ {
			wg.Add(1)
			go func(i int) {
				defer wg.Done()
				c.Put(i, i*2)
				c.Get(i)
			}(i)
		}
		wg.Wait()

		if got := c.Len(); got > 50 {
			t.Errorf("Len() = %d, want <= 50", got)
		}
	})
}
