// Command main demonstrates two reverse-proxy/load-balancer building
// blocks not used by this topic's exercise: a custom
// httputil.ReverseProxy Director that adds an X-Forwarded-For header, and
// a concurrent round-robin counter built directly on sync/atomic.Uint64 —
// the lower-level primitive behind LoadBalancer.NextBackend.
package main

import (
	"fmt"
	"io"
	"net"
	"net/http"
	"net/http/httptest"
	"net/http/httputil"
	"net/url"
	"sync"
	"sync/atomic"
)

func main() {
	// --- Custom Director: record the client's address for the backend ---
	backend := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		fmt.Fprintf(w, "X-Forwarded-For: %s", r.Header.Get("X-Forwarded-For"))
	}))
	defer backend.Close()

	target, _ := url.Parse(backend.URL)
	proxy := httputil.NewSingleHostReverseProxy(target)

	defaultDirector := proxy.Director
	proxy.Director = func(r *http.Request) {
		defaultDirector(r)
		if host, _, err := net.SplitHostPort(r.RemoteAddr); err == nil {
			r.Header.Set("X-Forwarded-For", host)
		}
	}

	frontend := httptest.NewServer(proxy)
	defer frontend.Close()

	resp, err := http.Get(frontend.URL)
	if err != nil {
		fmt.Println("Get error:", err)
		return
	}
	defer resp.Body.Close()
	body, _ := io.ReadAll(resp.Body)
	// ReverseProxy.ServeHTTP also appends the client IP to X-Forwarded-For,
	// so our Director's value appears twice.
	fmt.Println("backend saw:", string(body))

	// --- Concurrent round-robin counter with sync/atomic ---
	var counter atomic.Uint64
	backends := []string{"backend-0", "backend-1", "backend-2"}

	var mu sync.Mutex
	picks := make(map[string]int)

	var wg sync.WaitGroup
	for i := 0; i < 9; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			idx := (counter.Add(1) - 1) % uint64(len(backends))
			mu.Lock()
			picks[backends[idx]]++
			mu.Unlock()
		}()
	}
	wg.Wait()

	for _, b := range backends {
		fmt.Printf("%s: %d requests\n", b, picks[b])
	}
}
