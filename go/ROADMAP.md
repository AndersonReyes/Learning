# Go Roadmap

This curriculum teaches Go **through computer networking**: every topic
pairs a Go language concept with a networking concept, and the exercises
build toward real protocol/network tooling (parsers, servers, clients).

Go reference links point at [A Tour of Go](https://go.dev/tour/),
[Effective Go](https://go.dev/doc/effective_go), and
[pkg.go.dev](https://pkg.go.dev/std) for the standard library. Networking
reference links point at the relevant RFCs (or, where no single RFC fits,
general background).

## Fundamentals (12/12 built)

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
| 12 | DNS Protocol + Minimal DNS Resolver Over UDP | [`fundamentals/12-dns-protocol-and-resolver`](./fundamentals/12-dns-protocol-and-resolver) | [`net`](https://pkg.go.dev/net), `encoding/binary` — [RFC 1035 (DNS)](https://www.rfc-editor.org/rfc/rfc1035) |

## Intermediate (7/7 built)

| # | Topic (Go + Networking) | Folder | References |
|---|--------------------------|--------|------------|
| 1 | Generics + Generic Connection Pool & DNS-Cache LRU | [`intermediate/01-generics-pool-and-cache`](./intermediate/01-generics-pool-and-cache) | [Tour: Generics](https://go.dev/tour/generics/1) |
| 2 | TLS (`crypto/tls`) + HTTPS Client/Server & the TLS 1.3 Handshake | [`intermediate/02-tls-https-and-handshake`](./intermediate/02-tls-https-and-handshake) | [`crypto/tls`](https://pkg.go.dev/crypto/tls) — [RFC 8446 (TLS 1.3)](https://www.rfc-editor.org/rfc/rfc8446) |
| 3 | WebSockets + Real-Time Chat Server | [`intermediate/03-websockets-and-chat-server`](./intermediate/03-websockets-and-chat-server) | [RFC 6455 (WebSocket)](https://www.rfc-editor.org/rfc/rfc6455) |
| 4 | Reverse Proxy / Load Balancer (`net/http/httputil`) | [`intermediate/04-reverse-proxy-and-load-balancer`](./intermediate/04-reverse-proxy-and-load-balancer) | [`net/http/httputil`](https://pkg.go.dev/net/http/httputil) |
| 5 | Testing in Go (table-driven tests, `httptest`, fuzzing) + Protocol Parser Test Suites | [`intermediate/05-testing-fuzzing-and-protocol-parsers`](./intermediate/05-testing-fuzzing-and-protocol-parsers) | [`testing`](https://pkg.go.dev/testing), [`net/http/httptest`](https://pkg.go.dev/net/http/httptest) |
| 6 | `container/heap` + Routing Algorithms (Dijkstra/Bellman-Ford) & Simulating OSPF/BGP Path Selection | [`intermediate/06-routing-algorithms-and-path-selection`](./intermediate/06-routing-algorithms-and-path-selection) | [`container/heap`](https://pkg.go.dev/container/heap) — [RFC 2328 (OSPF)](https://www.rfc-editor.org/rfc/rfc2328), [RFC 2453 (RIP)](https://www.rfc-editor.org/rfc/rfc2453), [RFC 4271 (BGP)](https://www.rfc-editor.org/rfc/rfc4271) |
| 7 | The `math` Package + Simulating TCP Reno/CUBIC Congestion-Window Growth | [`intermediate/07-congestion-control-and-window-growth`](./intermediate/07-congestion-control-and-window-growth) | [`math`](https://pkg.go.dev/math) — [RFC 5681 (TCP Congestion Control)](https://www.rfc-editor.org/rfc/rfc5681), [RFC 8312 (CUBIC)](https://www.rfc-editor.org/rfc/rfc8312) |

## Advanced (5/5 built)

| # | Topic (Go + Networking) | Folder | References |
|---|--------------------------|--------|------------|
| 1 | `syscall`, `unsafe` & ioctl + Raw Sockets, TUN/TAP & Reading/Writing Raw IP Packets | [`advanced/01-raw-sockets-and-tun-tap`](./advanced/01-raw-sockets-and-tun-tap) | [`syscall`](https://pkg.go.dev/syscall), [`unsafe`](https://pkg.go.dev/unsafe) — [Linux TUN/TAP docs](https://www.kernel.org/doc/html/latest/networking/tuntap.html), [RFC 1071 (checksum)](https://www.rfc-editor.org/rfc/rfc1071), [RFC 791 (IPv4)](https://www.rfc-editor.org/rfc/rfc791), [RFC 792 (ICMP)](https://www.rfc-editor.org/rfc/rfc792) |
| 2 | Bit-Twiddling & Event-Driven State Machines + QUIC Varints, CRYPTO Frames & the TLS 1.3 Handshake | [`advanced/02-quic-and-http3`](./advanced/02-quic-and-http3) | [`crypto/tls`](https://pkg.go.dev/crypto/tls) (`QUICConn`), [`encoding/binary`](https://pkg.go.dev/encoding/binary) — [RFC 9000 (QUIC)](https://www.rfc-editor.org/rfc/rfc9000) §16, §19.6, [RFC 9001 (TLS for QUIC)](https://www.rfc-editor.org/rfc/rfc9001), [RFC 9114 (HTTP/3)](https://www.rfc-editor.org/rfc/rfc9114) |
| 3 | Bit-Twiddling Instruction Encoding + Classic BPF (cBPF) Packet Filters | [`advanced/03-cbpf-packet-filters`](./advanced/03-cbpf-packet-filters) | [`syscall`](https://pkg.go.dev/syscall) (`SockFilter`, `SockFprog`, `BPF_*`), [`unsafe`](https://pkg.go.dev/unsafe) — [Linux socket filtering (cBPF)](https://www.kernel.org/doc/Documentation/networking/filter.txt), `man 7 socket` (`SO_ATTACH_FILTER`) |
| 4 | Varint Encoding & Type Switches + the Protocol Buffers Wire Format (gRPC, by Hand) | [`advanced/04-protobuf-wire-format`](./advanced/04-protobuf-wire-format) | [Protocol Buffers Encoding](https://protobuf.dev/programming-guides/encoding/), [`sort`](https://pkg.go.dev/sort), [`encoding/binary`](https://pkg.go.dev/encoding/binary) — gRPC's HTTP/2 framing, `fundamentals/09-bufio-io-binary-and-framing` |
| 5 | `net/rpc` + Raft Consensus: Leader Election, Log Replication & Service Discovery | [`advanced/05-raft-and-service-discovery`](./advanced/05-raft-and-service-discovery) | [`net/rpc`](https://pkg.go.dev/net/rpc), [`encoding/gob`](https://pkg.go.dev/encoding/gob) — [Raft paper, extended version](https://raft.github.io/raft.pdf) (Figure 2, §5.2, §5.3, §5.4.1) |

## Capstones (future)

The Advanced section above is now built out, so all capstones below are
unblocked. Each is a standalone project that ties multiple topics together
into one real tool — pick any (or all), in any order.

### Capstone A: Network Monitoring Tool (Go)

Build a CLI/daemon that combines most of this Go track into one real tool —
roughly "build your own `tcpdump`/`iftop`/`nethogs`", with no new
dependencies beyond what's already used:

- **Packet capture**: cBPF-filtered raw sockets (`advanced/01-raw-sockets-and-tun-tap`,
  `advanced/03-cbpf-packet-filters`) to capture and pre-filter live traffic.
- **Protocol parsing**: Ethernet/IPv4/TCP/UDP header decoding
  (`fundamentals/03-structs-pointers-and-packet-headers`), DNS message parsing
  (`fundamentals/12-dns-protocol-and-resolver`).
- **Flow tracking & stats**: per-connection (5-tuple) state in a
  `container/heap`/map-based table (`intermediate/01`, `intermediate/06`),
  bandwidth/packet counters, and simple RTT/congestion-window estimation
  (`intermediate/07-congestion-control-and-window-growth`).
- **Live view**: an HTTP/JSON or WebSocket dashboard serving real-time
  traffic stats (`fundamentals/11-json-rest-rpc-api`,
  `intermediate/03-websockets-and-chat-server`).

Lives alongside the existing `go/` topics, e.g. `go/capstone-network-monitor/`,
with its own README documenting the architecture and how to run it
(needs raw-socket privileges, per the "personal computer with root access"
note for the Advanced section).

### Capstone B: Peer-to-Peer Protocol (Go)

Build a decentralized P2P node — a BitTorrent/Kademlia-style mini protocol —
that ties together the distributed-systems side of this Go track:

- **Transport & framing**: TCP connections between peers
  (`fundamentals/08-net-dial-listen-and-udp`) with length-prefixed message
  framing (`fundamentals/09-bufio-io-binary-and-framing`) or the protobuf
  wire format (`advanced/04-protobuf-wire-format`) for peer messages.
- **Peer discovery & routing**: a Kademlia-style DHT — XOR distance metric,
  k-buckets, iterative lookups — reusing the routing/shortest-path thinking
  from `intermediate/06-routing-algorithms-and-path-selection`.
- **Gossip / message propagation**: flood or gossip-based broadcast across
  peers using goroutines and channels (`fundamentals/06`, `fundamentals/07`),
  with `context` for per-request cancellation/timeouts.
- **Content exchange**: chunked file transfer with a piece bitfield and
  request pipelining (BitTorrent-style), encoded with
  `advanced/04-protobuf-wire-format` and optionally encrypted with
  `intermediate/02-tls-https-and-handshake`.
- **Optional**: NAT traversal / hole punching using the raw-socket techniques
  from `advanced/01-raw-sockets-and-tun-tap`.

Lives alongside the existing `go/` topics, e.g. `go/capstone-p2p-protocol/`,
with its own README documenting the protocol's message types and how to run
a multi-node demo (e.g. several local processes on different ports).

### Capstone C: TCP/IP Stack From Scratch (Rust)

Add a `rust/` track and build a **TCP/IP stack from scratch in Rust**
(Stanford CS144-style, `smoltcp`-inspired): IP, ARP, TCP handshake,
retransmission and flow control over a TUN device. This is the "go all the
way to advanced, in Rust" project discussed alongside this Go track — roadmap
it in `rust/ROADMAP.md` when that track is created.
