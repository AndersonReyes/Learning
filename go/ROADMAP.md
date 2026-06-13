# Go Roadmap

This curriculum teaches Go **through computer networking**: every topic
pairs a Go language concept with a networking concept, and the exercises
build toward real protocol/network tooling (parsers, servers, clients).

Go reference links point at [A Tour of Go](https://go.dev/tour/),
[Effective Go](https://go.dev/doc/effective_go), and
[pkg.go.dev](https://pkg.go.dev/std) for the standard library. Networking
reference links point at the relevant RFCs (or, where no single RFC fits,
general background).

## Fundamentals (11/12 built — in progress)

Go's core syntax is front-loaded here; later fundamentals topics introduce
fewer new language features and more standard-library/networking depth.

| # | Topic (Go + Networking) | Folder | References |
|---|--------------------------|--------|------------|
| 1 | Variables, Types, Control Flow & Bitwise Ops + IPv4 Addressing & CIDR Subnetting | [`fundamentals/01-go-basics-and-ip-addressing`](./fundamentals/01-go-basics-and-ip-addressing) | [Tour: Basics](https://go.dev/tour/basics/1), [Effective Go: Control structures](https://go.dev/doc/effective_go#control-structures) — [RFC 791 (IPv4)](https://www.rfc-editor.org/rfc/rfc791), [RFC 4632 (CIDR)](https://www.rfc-editor.org/rfc/rfc4632), [RFC 3021 (/31)](https://www.rfc-editor.org/rfc/rfc3021) |
| 2 | Functions, Multiple Returns & Error Handling + Transport-Layer Ports & Protocols (TCP vs UDP) | [`fundamentals/02-functions-errors-and-ports`](./fundamentals/02-functions-errors-and-ports) | [Tour: Basics (functions)](https://go.dev/tour/basics/4), [Effective Go: Errors](https://go.dev/doc/effective_go#errors) — [RFC 793 (TCP)](https://www.rfc-editor.org/rfc/rfc793), [RFC 768 (UDP)](https://www.rfc-editor.org/rfc/rfc768) |
| 3 | Structs, Pointers & Methods + Packet Header Layouts (Ethernet/IPv4/TCP) | [`fundamentals/03-structs-pointers-and-packet-headers`](./fundamentals/03-structs-pointers-and-packet-headers) | [Tour: Structs & pointers](https://go.dev/tour/moretypes/1), [Tour: Methods](https://go.dev/tour/methods/1), [`encoding/binary`](https://pkg.go.dev/encoding/binary) — [RFC 791 §3.1](https://www.rfc-editor.org/rfc/rfc791), [RFC 793 §3.1](https://www.rfc-editor.org/rfc/rfc793) |
| 4 | Slices, Arrays & Maps + Routing Tables & Longest-Prefix-Match | [`fundamentals/04-slices-maps-and-routing-tables`](./fundamentals/04-slices-maps-and-routing-tables) | [Tour: Slices & maps](https://go.dev/tour/moretypes/7) — [RFC 4632 §3 (LPM)](https://www.rfc-editor.org/rfc/rfc4632) |
| 5 | Interfaces & Error Wrapping + Abstracting Transports (`net.Conn`) | [`fundamentals/05-interfaces-and-net-conn`](./fundamentals/05-interfaces-and-net-conn) | [Tour: Interfaces](https://go.dev/tour/methods/9), [`errors`](https://pkg.go.dev/errors) — [`net.Conn`](https://pkg.go.dev/net#Conn) |
| 6 | Goroutines & Channels + Concurrent TCP Echo Server | [`fundamentals/06-goroutines-channels-and-echo-server`](./fundamentals/06-goroutines-channels-and-echo-server) | [Tour: Concurrency](https://go.dev/tour/concurrency/1) — [RFC 862 (Echo)](https://www.rfc-editor.org/rfc/rfc862) |
| 7 | `select`, `sync` & `context` + Connection Timeouts & Cancellation | [`fundamentals/07-select-sync-context-and-timeouts`](./fundamentals/07-select-sync-context-and-timeouts) | [Tour: select](https://go.dev/tour/concurrency/5), [`context`](https://pkg.go.dev/context) — [`net.Conn.SetDeadline`](https://pkg.go.dev/net#Conn) |
| 8 | The `net` Package (Dial/Listen) + TCP Chat Server & UDP Datagram Protocol | [`fundamentals/08-net-dial-listen-and-udp`](./fundamentals/08-net-dial-listen-and-udp) | [`net`](https://pkg.go.dev/net) — [RFC 793](https://www.rfc-editor.org/rfc/rfc793), [RFC 768](https://www.rfc-editor.org/rfc/rfc768) |
| 9 | `bufio`/`io`/`encoding/binary` + Custom Length-Prefixed Binary Wire Protocol | [`fundamentals/09-bufio-io-binary-and-framing`](./fundamentals/09-bufio-io-binary-and-framing) | [`bufio`](https://pkg.go.dev/bufio), [`io`](https://pkg.go.dev/io) — framing concepts |
| 10 | `net/http` Internals + HTTP/1.1 Server From Scratch (then `net/http`) | [`fundamentals/10-http11-from-scratch`](./fundamentals/10-http11-from-scratch) | [`net/http`](https://pkg.go.dev/net/http) — [RFC 9112 (HTTP/1.1)](https://www.rfc-editor.org/rfc/rfc9112) |
| 11 | `encoding/json` + JSON REST/RPC API Over the Network | [`fundamentals/11-json-rest-rpc-api`](./fundamentals/11-json-rest-rpc-api) | [`encoding/json`](https://pkg.go.dev/encoding/json) — [RFC 8259 (JSON)](https://www.rfc-editor.org/rfc/rfc8259) |
| 12 | DNS Protocol + Minimal DNS Resolver Over UDP | planned | [`net`](https://pkg.go.dev/net), `encoding/binary` — [RFC 1035 (DNS)](https://www.rfc-editor.org/rfc/rfc1035) |

## Intermediate (planned)

| Topic (Go + Networking) | References |
|--------------------------|------------|
| Generics + Generic Connection Pool & DNS-Cache LRU | [Tour: Generics](https://go.dev/tour/generics/1) |
| TLS (`crypto/tls`) + HTTPS Client/Server & the TLS 1.3 Handshake | [`crypto/tls`](https://pkg.go.dev/crypto/tls) — [RFC 8446 (TLS 1.3)](https://www.rfc-editor.org/rfc/rfc8446) |
| WebSockets + Real-Time Chat Server | [RFC 6455 (WebSocket)](https://www.rfc-editor.org/rfc/rfc6455) |
| Reverse Proxy / Load Balancer (`net/http/httputil`) | [`net/http/httputil`](https://pkg.go.dev/net/http/httputil) |
| Testing in Go (table-driven tests, `httptest`, fuzzing) + Protocol Parser Test Suites | [`testing`](https://pkg.go.dev/testing), [`net/http/httptest`](https://pkg.go.dev/net/http/httptest) |
| Routing Algorithms (Dijkstra/Bellman-Ford) + Simulating OSPF/BGP Path Selection | [RFC 2328 (OSPF)](https://www.rfc-editor.org/rfc/rfc2328), [RFC 4271 (BGP)](https://www.rfc-editor.org/rfc/rfc4271) |
| Congestion Control + Simulating TCP Reno/CUBIC Window Growth | [RFC 5681 (TCP Congestion Control)](https://www.rfc-editor.org/rfc/rfc5681), [RFC 8312 (CUBIC)](https://www.rfc-editor.org/rfc/rfc8312) |

## Advanced (planned)

| Topic (Go + Networking) | References |
|--------------------------|------------|
| Raw Sockets & TUN/TAP + Reading/Writing Raw IP Packets | `golang.org/x/net/ipv4`, OS TUN/TAP docs |
| QUIC / HTTP/3 | [RFC 9000 (QUIC)](https://www.rfc-editor.org/rfc/rfc9000), [RFC 9114 (HTTP/3)](https://www.rfc-editor.org/rfc/rfc9114) |
| eBPF/XDP Packet Filtering Basics | [cilium/ebpf](https://github.com/cilium/ebpf) |
| gRPC & Protocol Buffers + Service-to-Service Networking | [grpc-go](https://github.com/grpc/grpc-go) |
| Distributed Systems Networking: Consensus (Raft sketch) & Service Discovery | [Raft paper](https://raft.github.io/) |

## Capstone (future — new `rust/` track)

Once the Advanced section above is built out (especially raw sockets/TUN),
add a `rust/` track and build a **TCP/IP stack from scratch in Rust**
(Stanford CS144-style, `smoltcp`-inspired): IP, ARP, TCP handshake,
retransmission and flow control over a TUN device. This is the "go all the
way to advanced, in Rust" project discussed alongside this Go track — roadmap
it in `rust/ROADMAP.md` when that track is created.
