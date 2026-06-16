# mini-mq — Go port of the Distributed Message Queue Capstone

A "mini-Kafka" distributed message queue implemented in Go, ported from the
Rust capstone. Uses only the Go standard library — no external dependencies.

## Architecture (5 phases)

| Phase | Package | What it builds |
|-------|---------|----------------|
| 1 | `storage/` | Append-only log with sparse index |
| 2 | `broker/` | Partition, Topic, Registry with FNV-1a routing |
| 3 | `concurrent/` | `SharedRegistry` with RWMutex + background flush goroutine |
| 4 | `server/` | TCP server — newline-delimited JSON, one goroutine per conn |
| 5 | `groups/` | Consumer group coordinator with round-robin rebalancing |

## Wire protocol

Newline-delimited JSON over TCP. Binary payloads are base64-encoded.

### Request types

```json
{"type":"create_topic","topic":"events","partitions":3}
{"type":"produce","topic":"events","payload":"aGVsbG8="}
{"type":"produce","topic":"events","key":"dXNlcjE=","payload":"aGVsbG8="}
{"type":"fetch","topic":"events","partition":0,"offset":5}
{"type":"fetch_batch","topic":"events","partition":0,"offset":5,"max_count":100}
{"type":"metadata"}
{"type":"join_group","group":"my-group","topics":["events","orders"]}
{"type":"leave_group","group":"my-group","member_id":"member-0"}
{"type":"commit_offset","group":"my-group","topic":"events","partition":0,"offset":42}
{"type":"fetch_offset","group":"my-group","topic":"events","partition":0}
```

### Response types

```json
{"type":"topic_created","topic":"events","partitions":3}
{"type":"produced","partition":0,"offset":42}
{"type":"record","offset":5,"payload":"aGVsbG8="}
{"type":"end"}
{"type":"metadata","topics":[{"name":"events","partitions":3}]}
{"type":"joined","group":"my-group","member_id":"member-0","assignments":[{"topic":"events","partition":0}]}
{"type":"left_group","group":"my-group","member_id":"member-0"}
{"type":"offset_committed","group":"my-group","topic":"events","partition":0,"offset":42}
{"type":"committed_offset","group":"my-group","topic":"events","partition":0,"offset":42}
{"type":"committed_offset","group":"my-group","topic":"events","partition":0,"offset":null}
{"type":"error","message":"topic not found: orders"}
```

## On-disk storage format

**`<dir>/data.log`** — sequence of fixed-header records:
```
[offset: uint64 BE][length: uint32 BE][payload: length bytes]
```

**`<dir>/data.idx`** — sparse index, one entry every 64 records:
```
[offset: uint64 BE][file_position: uint64 BE]  (16 bytes each)
```

Directory layout:
```
<data-dir>/<topic-name>/<partition-id>/data.log
<data-dir>/<topic-name>/<partition-id>/data.idx
```

## Running

### Broker

```sh
go run ./cmd/broker [--data-dir <path>] [--port <port>] [--flush-ms <ms>]
# Defaults: data-dir=./data, port=9092, flush-ms=500
```

### Producer

```sh
go run ./cmd/producer --topic events --message "hello world" [--key user-42]
```

### Consumer

```sh
go run ./cmd/consumer --topic events [--group mygroup] [--partition 0] [--offset 0]
```

### Interactive with nc

```sh
# Start broker
go run ./cmd/broker --data-dir /tmp/mq-data

# In another terminal
nc localhost 9092
{"type":"create_topic","topic":"events","partitions":3}
{"type":"produce","topic":"events","payload":"aGVsbG8="}
{"type":"fetch","topic":"events","partition":0,"offset":0}
{"type":"metadata"}
```

## Testing

```sh
go test ./...          # all packages
go test ./storage/...  # storage only
go test ./server/... -v  # server with verbose output
```

## Key Go idioms used

- `sync.RWMutex` on the struct instead of `Arc<RwLock<T>>`
- goroutines + `go func()` instead of `tokio::spawn`
- `chan struct{}` for shutdown signalling
- `(T, error)` return pairs instead of `Result<T, E>`
- `bufio.Scanner` for line-buffered TCP reads
- `encoding/json.Marshal`/`Unmarshal` with struct tags
- `net.Listener` + `net.Conn` for TCP
