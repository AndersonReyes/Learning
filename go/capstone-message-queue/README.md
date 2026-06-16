# Capstone D: Distributed Message Queue (mini-Kafka)

A "mini-Kafka" built in Go — append-only log storage, topic/partition routing,
concurrent access, a JSON-over-TCP network protocol, and consumer groups.

## How to work through the phases

Work one phase at a time. Each phase is a Go package under this module.
After completing a phase, run its tests — **all tests must pass** before
moving to the next phase. Later phases import earlier ones, so each phase
builds on a working foundation.

| Phase | Package | What you build |
|-------|---------|----------------|
| 1 | `storage/` | Append-only log with binary records and sparse index |
| 2 | `broker/` | Topic/partition registry with FNV-1a key routing |
| 3 | `concurrent/` | Thread-safe registry wrapper with background flush |
| 4 | `protocol/` + `server/` | JSON-over-TCP network server |
| 5 | `groups/` + `server/` | Consumer groups with round-robin rebalancing |

## Running tests

Each package has its own test file. Run a single phase:

```bash
go test ./storage/
go test ./broker/
go test ./concurrent/
go test ./groups/
go test ./server/
```

Run all at once:

```bash
go test ./...
```

For race detection (important for `concurrent/` and `server/`):

```bash
go test -race ./...
```

## Running the binary

```bash
go run ./cmd/broker/
```

(The binary is a stub — implement the server before running it for real.)

## On-disk format

### `data.log` — record storage

Each record is a 12-byte header followed by the payload:

```
[offset  uint64 BE]  8 bytes
[length  uint32 BE]  4 bytes
[payload bytes]      `length` bytes
```

### `data.idx` — sparse index

One 16-byte entry every 64 records:

```
[offset   uint64 BE]  8 bytes
[file_pos uint64 BE]  8 bytes
```

Lets `Read` and `Scan` seek to approximately the right position without
scanning from the beginning of the log.

## Wire protocol

Newline-delimited JSON over TCP. Every message has a `"type"` field.

### Requests (client → broker)

```json
{"type":"create_topic","topic":"events","partitions":3}
{"type":"produce","topic":"events","key":"dXNlcjE=","payload":"aGVsbG8="}
{"type":"fetch","topic":"events","partition":0,"offset":5}
{"type":"fetch_batch","topic":"events","partition":0,"offset":5,"max_count":100}
{"type":"metadata"}
{"type":"join_group","group":"g","topics":["events"]}
{"type":"leave_group","group":"g","member_id":"member-0"}
{"type":"commit_offset","group":"g","topic":"events","partition":0,"offset":42}
{"type":"fetch_offset","group":"g","topic":"events","partition":0}
```

Payloads and keys are standard base64 encoded.

### Responses (broker → client)

Each request yields one JSON object response (except errors, which yield
`{"type":"error","message":"..."}`).

## FNV-1a routing

When `Produce` is called with a non-nil key, the partition is:

```
hash = 14695981039346656037
for each byte b in key:
    hash ^= uint64(b)
    hash *= 1099511628211
partition = hash % numPartitions
```

## Consumer group rebalance

On `Join` or `Leave`:
1. Collect all (topic, partition) pairs across all members' subscribed topics.
2. Sort: topic alphabetically, then partition index ascending.
3. Sort member IDs alphabetically.
4. Assign round-robin: pair[i] → member[i % len(members)].
