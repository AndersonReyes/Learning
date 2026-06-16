# mini-mq — Capstone B: Distributed Message Queue

A "mini-Kafka" built in phases as a single growing Rust project. Each phase
is a runnable milestone. The goal is to exercise the full Rust curriculum —
file I/O, generics, concurrency, async networking — in a realistic system.

## Protocol

Custom **newline-delimited JSON over TCP** instead of the real Kafka wire
protocol. This keeps the scope on the storage/concurrency/async work rather
than protocol compliance. Messages look like:

```json
// Produce request
{"type":"produce","topic":"events","payload":"aGVsbG8="}  // payload is base64

// Fetch request
{"type":"fetch","topic":"events","partition":0,"offset":5}

// Fetch response
{"type":"record","offset":5,"payload":"aGVsbG8="}
{"type":"end"}
```

## Phases

| # | Description | Status |
|---|-------------|--------|
| 1 | **Storage engine** — append-only log, sparse offset index, `Log::append`/`read`/`scan`, crash recovery | done |
| 2 | **Topics & partitions** — topic/partition registry, producer/consumer APIs | planned |
| 3 | **Concurrency** — `Arc<RwLock<...>>` multi-thread access, background flush thread | planned |
| 4 | **Network server** — tokio async TCP, JSON framing, produce/fetch/metadata requests | planned |
| 5 | **Consumer groups** — group membership, per-group committed offsets, rebalancing | planned |
| 6 | **Replication (stretch)** — leader/follower, log compaction | planned |

## Running

```bash
# From rust/
cargo build -p mini-mq

# Broker (Phase 4+)
cargo run --bin broker

# Producer CLI (Phase 4+)
cargo run --bin producer -- --topic events --message "hello"

# Consumer CLI (Phase 4+)
cargo run --bin consumer -- --topic events --group my-group
```

## Testing

```bash
cargo test -p mini-mq
```

## On-disk format (Phase 1)

**`<dir>/data.log`** — fixed-header records, one per message:
```
[offset: u64 BE][length: u32 BE][payload: length bytes]
```

**`<dir>/data.idx`** — sparse index (one entry every 64 records) mapping
logical offset → byte position in `.log`:
```
[offset: u64 BE][file_position: u64 BE]   (16 bytes each)
```

Read at offset O: binary-search index for the largest indexed offset ≤ O,
seek to that byte position, scan forward record-by-record until O is found.
