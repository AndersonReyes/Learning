// Command main demonstrates net/http used in this topic's exercise: a
// real HTTP/1.1 server built with http.ServeMux and http.HandlerFunc,
// queried with http.Client — the idiomatic counterpart to the from-scratch
// ReadRequest/WriteResponse/ServeOnce in exercise.go.
package main

import (
	"fmt"
	"io"
	"net/http"
	"net/http/httptest"
	"strings"
)

func main() {
	mux := http.NewServeMux()

	mux.HandleFunc("/ping", func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "text/plain")
		w.WriteHeader(http.StatusOK)
		w.Write([]byte("pong"))
	})

	mux.HandleFunc("/echo", func(w http.ResponseWriter, r *http.Request) {
		body, err := io.ReadAll(r.Body)
		if err != nil {
			http.Error(w, err.Error(), http.StatusInternalServerError)
			return
		}
		w.WriteHeader(http.StatusCreated)
		w.Write(body)
	})

	// httptest.NewServer starts a real net/http server on a loopback
	// ephemeral port (the same net.Listen("tcp", "127.0.0.1:0") pattern
	// from topic 8), and stops it on Close.
	srv := httptest.NewServer(mux)
	defer srv.Close()

	client := srv.Client()

	resp, err := client.Get(srv.URL + "/ping")
	if err != nil {
		fmt.Println("GET /ping error:", err)
		return
	}
	body, _ := io.ReadAll(resp.Body)
	resp.Body.Close()
	fmt.Printf("GET /ping -> %s %q\n", resp.Status, body)

	resp, err = client.Post(srv.URL+"/echo", "text/plain", strings.NewReader("hello, server"))
	if err != nil {
		fmt.Println("POST /echo error:", err)
		return
	}
	body, _ = io.ReadAll(resp.Body)
	resp.Body.Close()
	fmt.Printf("POST /echo -> %s %q\n", resp.Status, body)
}
