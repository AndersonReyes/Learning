# Advanced 5. `net/rpc` + Raft Consensus: Leader Election, Log Replication & Service Discovery

## New: `net/rpc`

Every server-side method this curriculum has written so far has had a
hand-rolled wire format (HTTP handlers, WebSocket frames, the JSON API,
QUIC's CRYPTO frames...). `net/rpc` is the stdlib's answer to "I just want to
call a Go method on another process": register any value whose methods have
the shape

```go
func (t *T) MethodName(args *ArgType, reply *ReplyType) error
```

and `net/rpc` exposes every such method as a remote call, encoded with
`encoding/gob` by default:

```go
// server
rpc.Register(node)             // node's exported MethodName(args, reply) methods become RPCs
l, _ := net.Listen("tcp", ":0")
rpc.Accept(l)                  // blocks, serving one goroutine per connection

// client
client, _ := rpc.Dial("tcp", addr)
var reply ReplyType
client.Call("RaftNode.MethodName", &args, &reply)
```

**Gotchas**:

- The method must be **exported** (capitalized), take exactly **two
  arguments** (the second a pointer the callee fills in), and **return
  `error`** — `net/rpc` reflects over registered values to find methods
  matching this shape and ignores everything else.
- `rpc.Call` is synchronous; `client.Go` is the async/non-blocking variant
  (returns a `*rpc.Call` with a `Done` channel).
- Types crossed over RPC must be exported with exported fields —
  `encoding/gob` (like `encoding/json`) only encodes what it can see via
  reflection.

This exercise's `RequestVote` and `AppendEntries` methods are written with
exactly this shape *on purpose*: `examples/main.go` registers a `*RaftNode`
and calls these methods over a real TCP loopback connection, with no
changes needed to `exercise.go`.

## Networking/Distributed systems: Raft (Ongaro & Ousterhout, 2014)

[Raft](https://raft.github.io/) is a consensus algorithm: it keeps a
replicated log identical across a cluster of nodes despite crashes and
message loss, by electing a single leader per **term** (a logical clock) and
having that leader replicate its log to followers. This topic implements the
RPC handlers and state transitions from the paper's Figure 2 — the part that
decides *what a node does* in response to an RPC or a timeout — without the
timers, persistence, or networking loop around them.

### Node states (§5.1)

```
Follower --(election timeout, BecomeCandidate)--> Candidate
Candidate --(receives AppendEntries from a Leader)--> Follower
Candidate --(majority of RequestVote replies)--> Leader
any state --(sees a higher Term)--> Follower
```

### RequestVote (§5.2, §5.4.1)

A candidate increments its term, votes for itself, and asks every peer to
vote. A peer grants its vote only if **both**:

1. it hasn't already voted for someone else this term, and
2. the candidate's log is **at least as up-to-date**: compare
   `(LastLogTerm, LastLogIndex)` lexicographically — higher term wins
   regardless of length; equal term means the longer (or equal) log wins.

Rule 2 is Raft's core safety mechanism: it ensures a candidate can only win
an election if its log contains every entry that's been committed, because a
committed entry has been replicated to a majority, and the candidate also
needs a majority of votes — the two majorities must overlap.

### AppendEntries (§5.3)

The leader sends `PrevLogIndex`/`PrevLogTerm` — "here's what I think your log
looks like right before the entries I'm sending." A follower rejects
(`Success = false`) if its log doesn't have a matching entry there. On
rejection, a real leader retries with an earlier `PrevLogIndex` (decrementing
`nextIndex` for that follower) until it finds a point both logs agree on,
then overwrites everything after that point — the **conflict resolution by
truncation** this exercise's `AppendEntries` implements.

`LeaderCommit` tells followers how far the leader has committed (replicated
to a majority); a follower advances its own `commitIndex` to match, capped at
how much of the leader's log it has actually received.

A heartbeat is just an `AppendEntries` with `Entries` empty — the same
handler, same rules, just nothing to append.

### The index-0 sentinel

`log[0]` is a zero-value `LogEntry{Term: 0}` that's never replicated or
applied. It exists purely so `PrevLogIndex = 0, PrevLogTerm = 0` — "the
follower's log is empty" — is a normal, always-true case in the
`AppendEntries` consistency check, instead of a special case for an
out-of-range index.

### "Service discovery" in this sketch

A production Raft deployment needs nodes to find each other's addresses
(via DNS, a config file, Consul/etcd, Kubernetes services, ...). This sketch
keeps that part as simple as possible: each `RaftNode` is constructed with
the **ids** of its peers (`NewRaftNode(id, peers)`), and `examples/main.go`
maps those ids to `net/rpc` addresses directly. Swapping in real discovery
means changing only how that id-to-address map is built — the RPC handlers
in `exercise.go` don't change.

## Further Reading

- [`net/rpc`](https://pkg.go.dev/net/rpc), [`encoding/gob`](https://pkg.go.dev/encoding/gob)
- [Raft paper, extended version](https://raft.github.io/raft.pdf) — Figure 2
  (RPC summary and rules), §5.2 (leader election), §5.3 (log replication),
  §5.4.1 (election restriction / safety)
- [The Raft website](https://raft.github.io/) — visualization and links to
  implementations
