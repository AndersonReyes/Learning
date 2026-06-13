// Command main demonstrates net/http's idiomatic HTTPS support —
// httptest.NewTLSServer (an HTTPS server with an auto-generated
// certificate), making requests with http.Client, and inspecting
// resp.TLS (the negotiated tls.ConnectionState) — as the "then net/http"
// counterpart to this topic's hand-built TLS listener/dialer
// (NewTLSListener/DialTLS in exercise.go), mirroring how topic 10 followed
// a from-scratch HTTP/1.1 parser with net/http.
package main

import (
	"crypto/tls"
	"fmt"
	"io"
	"net/http"
	"net/http/httptest"
)

func main() {
	srv := httptest.NewTLSServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		fmt.Fprintln(w, "hello over TLS")
	}))
	defer srv.Close()

	fmt.Println("server URL:", srv.URL)

	// srv.Client() is pre-configured to trust srv's certificate.
	resp, err := srv.Client().Get(srv.URL)
	if err != nil {
		fmt.Println("Get error:", err)
		return
	}
	defer resp.Body.Close()

	body, _ := io.ReadAll(resp.Body)
	fmt.Print("response body: ", string(body))
	fmt.Println("negotiated TLS version:", tls.VersionName(resp.TLS.Version))
	fmt.Println("negotiated cipher suite:", tls.CipherSuiteName(resp.TLS.CipherSuite))

	// A plain client doesn't trust srv's self-signed certificate.
	_, err = http.Get(srv.URL)
	fmt.Println("plain client error (expected, untrusted cert):", err != nil)

	// InsecureSkipVerify disables certificate verification entirely —
	// convenient for local testing, but provides no protection against a
	// man-in-the-middle in production.
	insecureClient := &http.Client{
		Transport: &http.Transport{
			TLSClientConfig: &tls.Config{InsecureSkipVerify: true},
		},
	}
	resp2, err := insecureClient.Get(srv.URL)
	if err != nil {
		fmt.Println("insecure client error:", err)
		return
	}
	defer resp2.Body.Close()
	fmt.Println("insecure client status:", resp2.Status)
}
