// Package jsonapi applies encoding/json to a small JSON REST API (a
// concurrency-safe key/value Store served over HTTP, topic 10) and a
// JSON-RPC-style dispatcher (HandleRPC), using json.RawMessage to defer
// decoding of values whose shape isn't known in advance.
package jsonapi

import (
	"encoding/json"
	"net/http"
	"sync"
)

// Store is an in-memory, concurrency-safe map of string keys to raw JSON
// values.
type Store struct {
	mu   sync.Mutex
	data map[string]json.RawMessage
}

// NewStore returns an empty Store.
func NewStore() *Store {
	return &Store{data: make(map[string]json.RawMessage)}
}

// Get returns the value stored for key and true, or (nil, false) if key
// is not present. Get is safe to call concurrently with Set and Delete.
func (s *Store) Get(key string) (json.RawMessage, bool) {
	return nil, false
}

// Set stores value under key, replacing any existing value. Set is safe
// to call concurrently with Get, Set, and Delete.
func (s *Store) Set(key string, value json.RawMessage) {
}

// Delete removes key from the store and reports whether it was present.
// Delete is safe to call concurrently with Get, Set, and Delete.
func (s *Store) Delete(key string) bool {
	return false
}

// ServeHTTP implements a REST API over Store for paths of the form
// "/items/{key}":
//
//   - GET /items/{key}: writes the stored value as
//     "application/json" with status 200, or status 404 if key is not
//     present.
//   - PUT /items/{key}: reads the request body, and if it is valid JSON
//     (json.Valid), stores it under key and responds 204. If the body is
//     not valid JSON, responds 400.
//   - DELETE /items/{key}: removes key and responds 204, or 404 if key
//     was not present.
//   - Any other method responds 405.
//
// A request whose path has an empty {key} responds 400.
func (s *Store) ServeHTTP(w http.ResponseWriter, r *http.Request) {
	http.Error(w, "not implemented", http.StatusNotImplemented)
}

// RPCRequest is a JSON-RPC-style request: a method name, method-specific
// parameters, and a caller-supplied ID echoed back in the response.
type RPCRequest struct {
	Method string          `json:"method"`
	Params json.RawMessage `json:"params"`
	ID     int             `json:"id"`
}

// RPCResponse is a JSON-RPC-style response. Exactly one of Result or
// Error is set (Result on success, Error on failure); ID matches the
// request's ID.
type RPCResponse struct {
	Result json.RawMessage `json:"result,omitempty"`
	Error  string          `json:"error,omitempty"`
	ID     int             `json:"id"`
}

// HandleRPC dispatches req to methods[req.Method]. If no method with that
// name exists, HandleRPC returns a response with a non-empty Error. If
// the method returns an error, that error's message becomes Error in the
// response. Otherwise, the method's return value is marshaled with
// json.Marshal into Result. In all cases the response's ID matches
// req.ID.
func HandleRPC(req RPCRequest, methods map[string]func(json.RawMessage) (any, error)) RPCResponse {
	return RPCResponse{}
}
