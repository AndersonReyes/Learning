package lbproxy

import (
	"fmt"
	"io"
	"net/http"
	"net/http/httptest"
	"net/url"
	"testing"
	"time"
)

func TestNewReverseProxy(t *testing.T) {
	t.Run("forwards requests to the target", func(t *testing.T) {
		backend := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
			fmt.Fprintf(w, "backend received %s", r.URL.Path)
		}))
		defer backend.Close()

		target, err := url.Parse(backend.URL)
		if err != nil {
			t.Fatalf("url.Parse() error = %v", err)
		}

		proxy := NewReverseProxy(target)
		frontend := httptest.NewServer(proxy)
		defer frontend.Close()

		resp, err := http.Get(frontend.URL + "/hello")
		if err != nil {
			t.Fatalf("Get() error = %v", err)
		}
		defer resp.Body.Close()

		body, _ := io.ReadAll(resp.Body)
		if want := "backend received /hello"; string(body) != want {
			t.Errorf("body = %q, want %q", body, want)
		}
	})

	t.Run("returns 502 when the target is unreachable", func(t *testing.T) {
		target, err := url.Parse("http://127.0.0.1:1")
		if err != nil {
			t.Fatalf("url.Parse() error = %v", err)
		}

		proxy := NewReverseProxy(target)
		frontend := httptest.NewServer(proxy)
		defer frontend.Close()

		resp, err := http.Get(frontend.URL + "/hello")
		if err != nil {
			t.Fatalf("Get() error = %v", err)
		}
		defer resp.Body.Close()

		if resp.StatusCode != http.StatusBadGateway {
			t.Errorf("status = %d, want %d", resp.StatusCode, http.StatusBadGateway)
		}
	})
}

func TestNextBackend(t *testing.T) {
	b0, err := NewBackend("http://backend0.invalid")
	if err != nil {
		t.Fatalf("NewBackend() error = %v", err)
	}
	b1, err := NewBackend("http://backend1.invalid")
	if err != nil {
		t.Fatalf("NewBackend() error = %v", err)
	}
	b2, err := NewBackend("http://backend2.invalid")
	if err != nil {
		t.Fatalf("NewBackend() error = %v", err)
	}

	lb := NewLoadBalancer([]*Backend{b0, b1, b2})

	t.Run("round robins through all alive backends", func(t *testing.T) {
		want := []*Backend{b0, b1, b2, b0, b1, b2}
		for i, w := range want {
			got, err := lb.NextBackend()
			if err != nil {
				t.Fatalf("NextBackend() #%d error = %v", i, err)
			}
			if got != w {
				t.Errorf("NextBackend() #%d = %s, want %s", i, got.URL, w.URL)
			}
		}
	})

	t.Run("skips dead backends", func(t *testing.T) {
		b1.SetAlive(false)
		for i := 0; i < 4; i++ {
			got, err := lb.NextBackend()
			if err != nil {
				t.Fatalf("NextBackend() error = %v", err)
			}
			if got == b1 {
				t.Errorf("NextBackend() #%d returned dead backend %s", i, b1.URL)
			}
		}
	})

	t.Run("returns an error when no backend is alive", func(t *testing.T) {
		b0.SetAlive(false)
		b1.SetAlive(false)
		b2.SetAlive(false)

		if _, err := lb.NextBackend(); err == nil {
			t.Error("NextBackend() error = nil, want error when no backend is alive")
		}
	})

	t.Run("returns an error with no backends", func(t *testing.T) {
		empty := NewLoadBalancer(nil)
		if _, err := empty.NextBackend(); err == nil {
			t.Error("NextBackend() error = nil, want error with no backends")
		}
	})
}

func TestLoadBalancerServeHTTP(t *testing.T) {
	t.Run("round robins requests across backends", func(t *testing.T) {
		backendA := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
			fmt.Fprint(w, "A")
		}))
		defer backendA.Close()
		backendB := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
			fmt.Fprint(w, "B")
		}))
		defer backendB.Close()

		a, err := NewBackend(backendA.URL)
		if err != nil {
			t.Fatalf("NewBackend() error = %v", err)
		}
		b, err := NewBackend(backendB.URL)
		if err != nil {
			t.Fatalf("NewBackend() error = %v", err)
		}

		lb := NewLoadBalancer([]*Backend{a, b})
		frontend := httptest.NewServer(lb)
		defer frontend.Close()

		want := []string{"A", "B", "A", "B"}
		for i, w := range want {
			resp, err := http.Get(frontend.URL)
			if err != nil {
				t.Fatalf("Get() #%d error = %v", i, err)
			}
			body, _ := io.ReadAll(resp.Body)
			resp.Body.Close()
			if string(body) != w {
				t.Errorf("request #%d body = %q, want %q", i, body, w)
			}
		}
	})

	t.Run("returns 503 when no backend is alive", func(t *testing.T) {
		a, err := NewBackend("http://backend.invalid")
		if err != nil {
			t.Fatalf("NewBackend() error = %v", err)
		}
		a.SetAlive(false)

		lb := NewLoadBalancer([]*Backend{a})
		frontend := httptest.NewServer(lb)
		defer frontend.Close()

		resp, err := http.Get(frontend.URL)
		if err != nil {
			t.Fatalf("Get() error = %v", err)
		}
		defer resp.Body.Close()

		if resp.StatusCode != http.StatusServiceUnavailable {
			t.Errorf("status = %d, want %d", resp.StatusCode, http.StatusServiceUnavailable)
		}
	})
}

func TestHealthCheck(t *testing.T) {
	healthy := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path == "/health" {
			w.WriteHeader(http.StatusOK)
			return
		}
		w.WriteHeader(http.StatusNotFound)
	}))
	defer healthy.Close()

	unhealthy := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusServiceUnavailable)
	}))
	defer unhealthy.Close()

	client := &http.Client{Timeout: 2 * time.Second}

	healthyBackend, err := NewBackend(healthy.URL)
	if err != nil {
		t.Fatalf("NewBackend() error = %v", err)
	}
	unhealthyBackend, err := NewBackend(unhealthy.URL)
	if err != nil {
		t.Fatalf("NewBackend() error = %v", err)
	}

	if !HealthCheck(healthyBackend, client, "/health") {
		t.Error("HealthCheck() = false for healthy backend, want true")
	}
	if HealthCheck(unhealthyBackend, client, "/health") {
		t.Error("HealthCheck() = true for unhealthy backend, want false")
	}
}

func TestRunHealthChecks(t *testing.T) {
	healthy := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusOK)
	}))
	defer healthy.Close()

	unhealthy := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusServiceUnavailable)
	}))
	defer unhealthy.Close()

	a, err := NewBackend(healthy.URL)
	if err != nil {
		t.Fatalf("NewBackend() error = %v", err)
	}
	b, err := NewBackend(unhealthy.URL)
	if err != nil {
		t.Fatalf("NewBackend() error = %v", err)
	}
	b.SetAlive(true)

	lb := NewLoadBalancer([]*Backend{a, b})
	lb.RunHealthChecks(&http.Client{Timeout: 2 * time.Second}, "/health")

	if !a.IsAlive() {
		t.Error("a.IsAlive() = false after RunHealthChecks, want true")
	}
	if b.IsAlive() {
		t.Error("b.IsAlive() = true after RunHealthChecks, want false")
	}
}
