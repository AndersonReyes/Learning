// Package lbproxy builds a reverse proxy and round-robin load balancer on
// top of net/http/httputil.ReverseProxy, using sync/atomic for the
// round-robin counter and each backend's health flag — building on the
// http.Handler interface from topic 5, net/http internals from topic 10,
// and the select/sync patterns from topic 7.
package lbproxy

import (
	"errors"
	"net/http"
	"net/http/httputil"
	"net/url"
	"sync/atomic"
)

// Backend represents one upstream server that requests can be proxied to.
type Backend struct {
	URL   *url.URL
	alive atomic.Bool
}

// NewBackend parses rawURL and returns a Backend marked alive.
func NewBackend(rawURL string) (*Backend, error) {
	target, err := url.Parse(rawURL)
	if err != nil {
		return nil, err
	}
	b := &Backend{URL: target}
	b.SetAlive(true)
	return b, nil
}

// SetAlive sets b's health flag. SetAlive is safe to call concurrently with
// IsAlive and SetAlive.
func (b *Backend) SetAlive(alive bool) {
	b.alive.Store(alive)
}

// IsAlive reports b's current health flag. IsAlive is safe to call
// concurrently with SetAlive.
func (b *Backend) IsAlive() bool {
	return b.alive.Load()
}

// LoadBalancer distributes requests across a fixed set of backends using
// round-robin selection, skipping backends marked as not alive.
type LoadBalancer struct {
	backends []*Backend
	counter  atomic.Uint64
}

// NewLoadBalancer returns a LoadBalancer that distributes requests across
// backends.
func NewLoadBalancer(backends []*Backend) *LoadBalancer {
	return &LoadBalancer{backends: backends}
}

// NewReverseProxy returns an *httputil.ReverseProxy that forwards requests
// to target. If the round trip to target fails, the proxy responds with
// 502 Bad Gateway instead of httputil's default error response.
func NewReverseProxy(target *url.URL) *httputil.ReverseProxy {
	return nil
}

// NextBackend returns the next backend in round-robin order, skipping any
// backend whose IsAlive is false. It returns an error if no backend is
// alive (or the LoadBalancer has no backends). NextBackend is safe to call
// concurrently with NextBackend.
func (lb *LoadBalancer) NextBackend() (*Backend, error) {
	return nil, errors.New("not implemented")
}

// ServeHTTP selects a backend with NextBackend and proxies r to it using
// NewReverseProxy. If NextBackend returns an error, ServeHTTP responds with
// 503 Service Unavailable.
func (lb *LoadBalancer) ServeHTTP(w http.ResponseWriter, r *http.Request) {
}

// HealthCheck sends a GET request to backend.URL joined with path using
// client and reports whether the response status was 200 OK. A request
// error (e.g. connection refused) counts as not healthy.
func HealthCheck(backend *Backend, client *http.Client, path string) bool {
	return false
}

// RunHealthChecks runs HealthCheck for every backend in lb and updates each
// backend's alive flag (via SetAlive) with the result.
func (lb *LoadBalancer) RunHealthChecks(client *http.Client, path string) {
}
