package jsonapi

import (
	"bytes"
	"encoding/json"
	"errors"
	"fmt"
	"net/http"
	"net/http/httptest"
	"strings"
	"sync"
	"testing"
)

func TestStoreGetSetDelete(t *testing.T) {
	s := NewStore()

	if _, ok := s.Get("missing"); ok {
		t.Error("Get(\"missing\") ok = true, want false")
	}

	s.Set("a", json.RawMessage(`"hello"`))

	got, ok := s.Get("a")
	if !ok {
		t.Fatal(`Get("a") ok = false, want true`)
	}
	if string(got) != `"hello"` {
		t.Errorf(`Get("a") = %q, want %q`, got, `"hello"`)
	}

	if !s.Delete("a") {
		t.Error(`Delete("a") = false, want true`)
	}
	if _, ok := s.Get("a"); ok {
		t.Error(`Get("a") after Delete ok = true, want false`)
	}
	if s.Delete("a") {
		t.Error(`Delete("a") (already deleted) = true, want false`)
	}
}

func TestStoreConcurrent(t *testing.T) {
	s := NewStore()
	const n = 100

	var wg sync.WaitGroup
	for i := 0; i < n; i++ {
		wg.Add(1)
		go func(i int) {
			defer wg.Done()
			s.Set(fmt.Sprintf("key%d", i), json.RawMessage(fmt.Sprintf("%d", i)))
		}(i)
	}
	wg.Wait()

	for i := 0; i < n; i++ {
		key := fmt.Sprintf("key%d", i)
		got, ok := s.Get(key)
		if !ok {
			t.Fatalf("Get(%q) ok = false, want true", key)
		}
		if want := fmt.Sprintf("%d", i); string(got) != want {
			t.Errorf("Get(%q) = %q, want %q", key, got, want)
		}
	}
}

func TestServeHTTP(t *testing.T) {
	s := NewStore()

	t.Run("GET missing key returns 404", func(t *testing.T) {
		req := httptest.NewRequest(http.MethodGet, "/items/foo", nil)
		rec := httptest.NewRecorder()

		s.ServeHTTP(rec, req)

		if rec.Code != http.StatusNotFound {
			t.Errorf("GET missing key status = %d, want %d", rec.Code, http.StatusNotFound)
		}
	})

	t.Run("PUT then GET round trip", func(t *testing.T) {
		putReq := httptest.NewRequest(http.MethodPut, "/items/foo", strings.NewReader(`{"name":"bar"}`))
		putRec := httptest.NewRecorder()
		s.ServeHTTP(putRec, putReq)

		if putRec.Code != http.StatusNoContent {
			t.Fatalf("PUT status = %d, want %d", putRec.Code, http.StatusNoContent)
		}

		getReq := httptest.NewRequest(http.MethodGet, "/items/foo", nil)
		getRec := httptest.NewRecorder()
		s.ServeHTTP(getRec, getReq)

		if getRec.Code != http.StatusOK {
			t.Fatalf("GET status = %d, want %d", getRec.Code, http.StatusOK)
		}
		if ct := getRec.Header().Get("Content-Type"); ct != "application/json" {
			t.Errorf("GET Content-Type = %q, want %q", ct, "application/json")
		}
		if got := getRec.Body.String(); got != `{"name":"bar"}` {
			t.Errorf("GET body = %q, want %q", got, `{"name":"bar"}`)
		}
	})

	t.Run("PUT invalid JSON returns 400", func(t *testing.T) {
		req := httptest.NewRequest(http.MethodPut, "/items/bad", strings.NewReader("not json"))
		rec := httptest.NewRecorder()

		s.ServeHTTP(rec, req)

		if rec.Code != http.StatusBadRequest {
			t.Errorf("PUT invalid JSON status = %d, want %d", rec.Code, http.StatusBadRequest)
		}
	})

	t.Run("DELETE existing key returns 204", func(t *testing.T) {
		putReq := httptest.NewRequest(http.MethodPut, "/items/todelete", strings.NewReader(`1`))
		s.ServeHTTP(httptest.NewRecorder(), putReq)

		req := httptest.NewRequest(http.MethodDelete, "/items/todelete", nil)
		rec := httptest.NewRecorder()
		s.ServeHTTP(rec, req)

		if rec.Code != http.StatusNoContent {
			t.Errorf("DELETE status = %d, want %d", rec.Code, http.StatusNoContent)
		}
	})

	t.Run("DELETE missing key returns 404", func(t *testing.T) {
		req := httptest.NewRequest(http.MethodDelete, "/items/missing", nil)
		rec := httptest.NewRecorder()

		s.ServeHTTP(rec, req)

		if rec.Code != http.StatusNotFound {
			t.Errorf("DELETE missing key status = %d, want %d", rec.Code, http.StatusNotFound)
		}
	})

	t.Run("unsupported method returns 405", func(t *testing.T) {
		req := httptest.NewRequest(http.MethodPatch, "/items/foo", nil)
		rec := httptest.NewRecorder()

		s.ServeHTTP(rec, req)

		if rec.Code != http.StatusMethodNotAllowed {
			t.Errorf("PATCH status = %d, want %d", rec.Code, http.StatusMethodNotAllowed)
		}
	})
}

func TestHandleRPC(t *testing.T) {
	methods := map[string]func(json.RawMessage) (any, error){
		"echo": func(params json.RawMessage) (any, error) {
			var s string
			if err := json.Unmarshal(params, &s); err != nil {
				return nil, err
			}
			return s, nil
		},
		"add": func(params json.RawMessage) (any, error) {
			var nums []int
			if err := json.Unmarshal(params, &nums); err != nil {
				return nil, err
			}
			sum := 0
			for _, n := range nums {
				sum += n
			}
			return sum, nil
		},
		"fail": func(params json.RawMessage) (any, error) {
			return nil, errors.New("boom")
		},
	}

	t.Run("echo", func(t *testing.T) {
		resp := HandleRPC(RPCRequest{Method: "echo", Params: json.RawMessage(`"hi"`), ID: 1}, methods)

		if resp.Error != "" {
			t.Fatalf("HandleRPC() error = %q, want empty", resp.Error)
		}
		if !bytes.Equal(resp.Result, []byte(`"hi"`)) {
			t.Errorf("HandleRPC() result = %s, want %s", resp.Result, `"hi"`)
		}
		if resp.ID != 1 {
			t.Errorf("HandleRPC() id = %d, want %d", resp.ID, 1)
		}
	})

	t.Run("add", func(t *testing.T) {
		resp := HandleRPC(RPCRequest{Method: "add", Params: json.RawMessage(`[1,2,3]`), ID: 2}, methods)

		if resp.Error != "" {
			t.Fatalf("HandleRPC() error = %q, want empty", resp.Error)
		}
		if !bytes.Equal(resp.Result, []byte(`6`)) {
			t.Errorf("HandleRPC() result = %s, want %s", resp.Result, `6`)
		}
		if resp.ID != 2 {
			t.Errorf("HandleRPC() id = %d, want %d", resp.ID, 2)
		}
	})

	t.Run("unknown method", func(t *testing.T) {
		resp := HandleRPC(RPCRequest{Method: "missing", ID: 3}, methods)

		if resp.Error == "" {
			t.Error("HandleRPC() error = empty, want non-empty")
		}
		if resp.Result != nil {
			t.Errorf("HandleRPC() result = %s, want nil", resp.Result)
		}
		if resp.ID != 3 {
			t.Errorf("HandleRPC() id = %d, want %d", resp.ID, 3)
		}
	})

	t.Run("method returns error", func(t *testing.T) {
		resp := HandleRPC(RPCRequest{Method: "fail", ID: 4}, methods)

		if resp.Error != "boom" {
			t.Errorf("HandleRPC() error = %q, want %q", resp.Error, "boom")
		}
		if resp.ID != 4 {
			t.Errorf("HandleRPC() id = %d, want %d", resp.ID, 4)
		}
	})
}
