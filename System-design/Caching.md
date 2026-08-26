# System Design: Caching

A cache stores reusable data or computed results so later requests can avoid slower work. It usually reduces latency and load on databases or services. It adds memory cost, invalidation complexity, and failure modes.

## Table of Contents

1. [Fundamentals](#1-fundamentals)
2. [Where to Cache](#2-where-to-cache)
3. [Read Patterns](#3-read-patterns)
4. [Expiration and Eviction](#4-expiration-and-eviction)
5. [Freshness and Invalidation](#5-freshness-and-invalidation)
6. [Write Strategies](#6-write-strategies)
7. [Hot Keys and Cache Stampedes](#7-hot-keys-and-cache-stampedes)
8. [Partitioning and Consistent Hashing](#8-partitioning-and-consistent-hashing)
9. [Replication and Availability](#9-replication-and-availability)
10. [Concurrency and Race Conditions](#10-concurrency-and-race-conditions)
11. [Failure Handling](#11-failure-handling)
12. [Capacity Estimation](#12-capacity-estimation)
13. [Monitoring](#13-monitoring)
14. [Worked Example](#14-worked-example)
15. [Design Interview Approach](#15-design-interview-approach)
16. [Cheat Sheet](#16-cheat-sheet)
17. [References](#17-references)

---

## 1. Fundamentals

### Core terms

- **Source of truth:** Authoritative data store. A cache is normally disposable unless explicitly designed for durability.
- **Cache hit:** The requested value is present and usable.
- **Cache miss:** The value is absent, expired, or unusable.
- **Hit ratio:** `hits / (hits + misses)`.
- **Miss penalty:** Cost of checking the cache and then loading or computing the value.
- **Working set:** Data accessed often enough to benefit from remaining cached.

A high hit ratio helps only when cache hits are cheaper than the work they replace. Average read latency can be estimated as:

```text
average latency
  = hit ratio × hit latency
  + miss ratio × miss latency
```

Example:

```text
95% hits at 2 ms; 5% misses at 102 ms
average = 0.95 × 2 + 0.05 × 102 = 7 ms
```

### Good cache candidates

- Frequently reused data
- Expensive database queries or computations
- Data that tolerates bounded staleness
- Responses from slow or rate-limited dependencies

Poor candidates include rapidly changing data requiring strict freshness, sensitive data without safe isolation, and values larger than the work they save.

### Key and value design

Keys should be predictable and collision-safe:

```text
{namespace}:{version}:{tenant}:{entity}:{id}
catalog:v2:tenant-7:product:123
```

- Include the tenant or authorization scope when values differ by caller.
- Add a schema version when old and new representations may overlap.
- Canonicalize inputs so equivalent requests share one key.
- Avoid unbounded keys built from arbitrary query strings.

Values should be cheap to encode and retrieve. Measure serialized size, set a maximum entry size, and compress only when saved network or memory cost exceeds CPU cost. Large values can increase latency and create uneven memory pressure.

### Security

- Authorize the caller before returning cached protected data.
- Do not share user-specific values under a global key.
- Avoid sensitive data unless encryption, retention, and deletion rules are satisfied.
- Validate data before caching it to reduce cache-poisoning risk.
- For HTTP caches, include every response-varying attribute in the cache key or `Vary` policy.

---

## 2. Where to Cache

### Client, browser, or device

Avoids network requests. Invalidation and version compatibility are harder because clients may remain offline or outdated.

### CDN or reverse proxy

Caches HTTP content near users. Useful for static assets and cacheable responses. Correct cache headers and tenant isolation matter.

### Local application cache

Each application process stores its own entries.

**Advantages**

- Very low latency
- No cache-network request
- Can absorb traffic for hot keys

**Costs**

- Duplicate data across processes
- Less memory for application work
- Inconsistent copies
- Cold cache after each restart

### Shared distributed cache

Applications use a shared service such as Redis or Memcached.

**Advantages**

- Entries are reused across application instances
- Capacity and operations scale separately
- Centralized invalidation and metrics

**Costs**

- Network and serialization overhead
- Another dependency to operate
- Cluster failures can amplify database load

### Multi-level cache

```text
local cache → shared cache → source of truth
```

This combines fast local hits with shared capacity. It also creates two levels that must expire or invalidate correctly.

---

## 3. Read Patterns

### Cache-aside

The application manages cache reads and fills. This is the flow documented by the [Microsoft Cache-Aside pattern](https://learn.microsoft.com/en-us/azure/architecture/patterns/cache-aside).

```text
read(key):
  value = cache.get(key)
  if value exists:
    return value

  value = database.get(key)
  cache.set(key, value, ttl)
  return value
```

Only requested data enters the cache. A miss adds a cache lookup plus a source-of-truth read.

#### Worked example: cache-aside read

Assume a cache operation takes 2 ms and a database read takes 100 ms:

```text
GET product:123       → miss                 2 ms
DB GET product:123    → product data       100 ms
SET product:123       → TTL 60 seconds       2 ms
first read                                  104 ms

next GET product:123  → hit                  2 ms
```

Across one miss and nine hits, average latency is `(104 + 9 × 2) / 10 = 12.2 ms` under these assumptions.

### Read-through

The application reads through a cache abstraction. On a miss, the cache or caching library loads the value. This centralizes loading logic but couples the cache layer to the source of truth.

### Refresh-ahead

Refresh a popular entry before it expires. This reduces user-visible misses but can waste work on entries no longer needed.

### Negative caching

Cache a short-lived “not found” result to prevent repeated requests for missing data.

Use a short TTL. Otherwise, a newly created record may remain hidden until the negative entry expires.

---

## 4. Expiration and Eviction

These mechanisms solve different problems:

- **Expiration:** Removes or rejects entries because they are too old.
- **Eviction:** Frees space when the cache reaches a memory limit.
- **Invalidation:** Removes an entry because its underlying data changed.

### TTL: time to live

An entry becomes invalid after an interval. Systems may remove expired entries when accessed, in background cleanup, or both.

TTL sets an intended maximum residence time; it is not a complete consistency guarantee. A stale fill or failed refresh can insert old data again. Choose TTL from freshness requirements, update rate, and miss cost. Redis, for example, uses both passive and active expiration ([Redis `EXPIRE`](https://redis.io/docs/latest/commands/expire/)).

### LRU: least recently used

Evicts the entry accessed least recently. Exact LRU is commonly represented by a hash map plus an access-ordered doubly linked list, giving `O(1)` lookup, promotion, and eviction. Redis uses an approximation to reduce metadata cost ([Redis eviction](https://redis.io/docs/latest/develop/reference/eviction/)).

A heap can inspect its oldest root in `O(1)`, but insertion, access-time updates, and removal are `O(log n)`.

### LFU: least frequently used

Evicts entries with the lowest access frequency. Practical implementations often approximate counts and decay old history so formerly popular entries do not remain forever.

### Other policies

- FIFO: evict the oldest inserted entry
- Random: simple and sometimes effective
- TTL-aware: prefer entries closest to expiration
- No eviction: reject writes when full

The best policy depends on the workload. Measure hit ratio and eviction behavior rather than assuming LRU is optimal.

---

## 5. Freshness and Invalidation

Cached data becomes stale when the source of truth changes without a corresponding cache change.

### Common strategies

- **TTL only:** Simple; accepts bounded staleness.
- **Explicit invalidation:** Delete affected keys after a successful write.
- **Cache update:** Replace cached values after a successful write.
- **Versioned keys:** Include a version in the key and switch versions on change.
- **Change events:** Publish database changes to cache consumers.

### Database update followed by invalidation

```text
update(key, value):
  database.update(key, value)
  cache.delete(key)
```

This is common with cache-aside ([Microsoft](https://learn.microsoft.com/en-us/azure/architecture/patterns/cache-aside)). If deletion fails, an old cached value may remain, so use retries, an outbox/change stream, and a fallback TTL when the freshness requirement warrants them.

#### Worked example: failed invalidation

```text
t=0   cache price:123 = $10, TTL has 60 seconds left
t=1   database price changes to $12
t=2   cache deletion fails
t=3   reader receives cached $10
```

Without repair, `$10` can remain visible until expiration. A retry at `t=7` reduces the stale window; a durable change event can repair invalidations lost during a process crash.

#### Worked example: stale fill race

| Time | Reader A | Writer B | Cache |
|---|---|---|---|
| 1 | Misses and starts reading version 1 | | Empty |
| 2 | | Writes version 2 to database | Empty |
| 3 | | Deletes key | Empty |
| 4 | Receives version 1 and caches it | | Version 1 |

The invalidation happened before the stale fill, so it did not remove version 1. Possible controls include conditional cache writes using versions, delayed second invalidation, change events, or accepting bounded staleness.

---

## 6. Write Strategies

Names vary between products. State the exact flow and acknowledgment point. AWS, for example, describes application-managed write-through as updating the database and immediately updating the cache ([AWS caching patterns](https://docs.aws.amazon.com/whitepapers/latest/database-caching-strategies-using-redis/caching-patterns.html)).

| Strategy | Example flow | Success returned | Main risk |
|---|---|---|---|
| Cache-aside/write-around | database commit → cache delete | After commit and the configured deletion attempt | Failed deletion leaves stale data |
| Native write-through | cache layer → synchronous database write → cache update | After the database persists and cache accepts the value | Higher latency; implementation must handle rollback |
| Database then cache update | database commit → cache update | After both operations succeed | Database succeeds but cache update fails |
| Write-back/write-behind | durable cache/queue write → asynchronous database write | After the cache or queue accepts the write | Data loss, reordering, and difficult recovery |

Write-through keeps reads warm but adds work to every write, including values never read. Write-around avoids filling unused entries but makes the next read miss. Write-back lowers write latency but makes the cache part of the durability path.

Writing cache and database independently, sequentially or in parallel, is not atomic. Either side can succeed alone. Use an owning write-through component, transactions where supported, durable events, idempotency, or repair logic.

---

## 7. Hot Keys and Cache Stampedes

### Hot key

One key receives disproportionate traffic. Ordinary sharding maps that key to one shard, so adding unrelated shards does not split its load.

Mitigations:

- Small local caches
- Read replicas when stale reads are acceptable
- Replicated or salted copies with routing logic
- Request batching
- Caching a smaller or precomputed representation
- Rate limiting or load shedding

Replication and salting make invalidation harder. Use them only when measurements show a key is hot.

### Cache stampede or thundering herd

Many callers miss the same key and regenerate it concurrently, overloading the source of truth. AWS documents synchronized misses from expiration or cold nodes as causes ([AWS ElastiCache guide](https://docs.aws.amazon.com/pdfs/whitepapers/latest/scale-performance-elasticache/scale-performance-elasticache.pdf)).

Mitigations:

- **Request coalescing:** One loader runs; other callers await its result.
- **Per-key lock:** Serialize regeneration for one key, not the whole cache.
- **Stale-while-revalidate:** Serve a stale value while one caller refreshes it.
- **TTL jitter:** Add randomness so many keys do not expire together.
- **Cache warming:** Populate expected hot entries before traffic arrives.
- **Backpressure:** Bound concurrent source requests.

A cold cache can cause a stampede, but the terms are not identical. A cold cache is empty; a stampede is concurrent regeneration.

#### Worked example: stampede

```text
popular key expires
10,000 requests arrive

without coalescing: up to 10,000 source reads
with coalescing:            1 source read + 9,999 waiters
```

Coalescing must bound wait time and define what happens when the single loader fails.

---

## 8. Partitioning and Consistent Hashing

Partitioning spreads the keyspace across cache nodes.

### Modulo hashing

```text
node = hash(key) mod node_count
```

It is simple and balanced when the hash is good. Changing `node_count` remaps most keys, which can cause a large miss spike.

### Consistent hashing

Hash nodes and keys into the same circular space. A key belongs to the next node clockwise. The original goal is minimal mapping change when membership changes ([Karger et al.](https://people.csail.mit.edu/karger/Papers/web.pdf)).

```text
                 node A
             .------------.
         key 1              node B
            |                |
             '----node C----'
```

When a node is added or removed, only a fraction of keys move instead of nearly the entire keyspace.

#### Worked example: adding one node

Use a ring numbered 0–99:

```text
A=10, B=40, C=70

key positions: 5→A, 20→B, 35→B, 50→C, 65→C, 80→A
```

Add `D=30`. D takes the interval `(10, 30]` previously owned by B:

```text
20: B→D
5, 35, 50, 65, 80: unchanged
```

Only keys in D's new interval move. With `hash(key) mod node_count`, changing the node count generally changes many assignments.

### Virtual nodes

Map each physical node to many ring positions. Virtual nodes improve balance, spread a failed node's ranges across multiple survivors, and allow weighted capacity. Amazon's Dynamo describes these benefits ([Dynamo](https://www.amazon.science/publications/dynamo-amazons-highly-available-key-value-store)).

Consistent hashing is one approach, not a requirement. Redis Cluster uses a fixed set of hash slots that can be reassigned between nodes ([Redis Cluster scaling](https://redis.io/docs/latest/operate/oss_and_stack/management/scaling/)).

Partitioning increases total capacity but does not by itself provide redundancy or solve hot keys.

---

## 9. Replication and Availability

Replication stores additional copies of a shard.

It can provide:

- Failover when a primary fails
- More read capacity
- Better zone availability

It costs extra memory and replication traffic. With asynchronous replication, replicas can lag and recent writes may be lost during failover. Redis documents both properties in its [replication guide](https://redis.io/docs/latest/operate/oss_and_stack/management/replication/).

Do not confuse replication with partitioning:

- Partitioning divides data across nodes.
- Replication copies data across nodes.

For a disposable cache, missing data can be reloaded from the source of truth. If the caching system is also the authoritative store, persistence and recovery requirements are different and must be designed explicitly.

---

## 10. Concurrency and Race Conditions

### Duplicate fills

Several callers miss and load the same value. Use per-key request coalescing when the duplicated work is expensive.

### Stale fill

A slow reader may overwrite a newer cached value. Use version numbers, conditional writes, or invalidation after committed changes.

### Lost update

Read-modify-write operations can overwrite each other. Use atomic cache operations, compare-and-set, transactions, or move the operation to the source of truth.

### Lock cautions

- Prefer per-key locks over global locks.
- Set lock timeouts or leases.
- Define behavior when the lock owner fails.
- Do not assume a distributed lock makes database and cache writes atomic.

Connection pools and worker pools should be bounded. They control resource use but do not replace correctness mechanisms.

---

## 11. Failure Handling

Design separately for **cache empty** and **cache unavailable**.

### Cache empty

Requests reach the cache successfully but miss. Protect the source with warming, coalescing, concurrency limits, and gradual traffic ramp-up.

### Cache unavailable

Requests time out or fail. Possible responses:

- Fall back to the source of truth with strict concurrency limits
- Serve an older local value when allowed
- Return a degraded response
- Fail fast for nonessential requests

Unbounded fallback can turn a cache outage into a database outage.

#### Worked example: cache outage

```text
application reads:       100,000/s
normal hit ratio:             95%
normal database reads:      5,000/s
database capacity:         15,000/s
cache-outage demand:      100,000/s
```

Direct fallback overloads the database by more than 6×. Limit fallback to available database headroom, serve stale or degraded responses where allowed, and reject excess work before it consumes database connections.

### Basic controls

- Short cache timeouts
- Limited retries with backoff and jitter
- Circuit breaking
- Bulkheads or separate resource pools
- Rate limiting and load shedding
- Idempotent recovery operations

---

## 12. Capacity Estimation

Estimate the working set, not only the full dataset.

```text
raw cache size = entry count × average encoded entry size

provisioned memory
  = raw cache size
  + key and metadata overhead
  + allocator/fragmentation headroom
  + replication overhead
  + growth headroom
```

#### Worked example: provisioned memory

Assume:

```text
10 million entries
2,048 bytes serialized value
200 bytes key and metadata
25% fragmentation, growth, and operating headroom
2 total copies: primary + replica
```

```text
base       = 10,000,000 × (2,048 + 200) ≈ 20.94 GiB
headroom   = 20.94 × 1.25                 ≈ 26.18 GiB
replicated = 26.18 × 2                    ≈ 52.36 GiB
```

This is an initial estimate, not a machine count. Measure actual encoding and allocator overhead, then account for shard imbalance and the cache product's failover rules.

For basic steady-state cache-aside reads, estimate source load as:

```text
source read QPS ≈ read QPS × (1 - hit ratio)
```

At 100,000 reads/s and a 95% hit ratio, the source receives about 5,000 demand reads/s. Refreshes, retries, negative caching, request coalescing, and failures change the actual load.

---

## 13. Monitoring

Track:

- Hit and miss ratio by endpoint or key class
- Cache latency percentiles
- Source latency and request volume
- Memory usage and fragmentation
- Evictions and expirations
- Connection count and saturation
- Errors, timeouts, and retries
- Hot keys and skew between shards
- Replication lag and failovers
- Fill duration and concurrent loaders

A global hit ratio can hide a broken endpoint. Segment metrics by workload and compare them with source load and user latency.

---

## 14. Worked Example

Suppose a service reads product content by ID:

```text
product-content:123 → {name, description}
```

Requirements:

- 100,000 reads/s
- 1,000 writes/s
- Product content may be stale for 60 seconds
- Checkout prices must be current

Design:

1. Use a shared distributed cache with `product-content:{id}` keys.
2. Use cache-aside for product-page reads.
3. Apply a TTL near 60 seconds with jitter.
4. After a committed product update, invalidate its cache key.
5. Coalesce concurrent fills per product ID.
6. Partition keys across cache shards.
7. Add replicas for failover if required.
8. Protect the database with timeouts and bounded fallback.
9. Read checkout price from the authoritative pricing service; do not put it in the content entry.

Request flow:

```text
product page → cache content → database on miss
checkout     → authoritative pricing service
```

The split follows the freshness requirement: content tolerates staleness; checkout price does not. During a cache outage, bounded fallback protects the product database while checkout remains independent of cached content.

---

## 15. Design Interview Approach

Use this sequence:

```text
requirements → estimates → placement and pattern → correctness
             → scaling → failures → monitoring → tradeoffs
```

### 1. Clarify requirements

- What data or computation is cached?
- What are read/write rates and object sizes?
- How stale may a value be?
- Is the cache disposable?
- What happens when it is unavailable?

### 2. Estimate

- Read and write QPS
- Working-set memory
- Expected hit ratio
- Source traffic on misses
- Replication and growth overhead

### 3. Choose the design

- Local, distributed, CDN, or multi-level cache
- Cache-aside, read-through, write-through, or write-back
- Key format, value format, TTL, and invalidation

State the exact read and write flows. Pattern names alone are ambiguous.

### 4. Handle scale and correctness

- Partitioning and rebalancing
- Replication and lag
- Hot keys
- Stampedes
- Concurrent reads and writes
- Partial failures

### 5. Define observability

Name the metrics that prove the cache helps: hit ratio, latency, source load, evictions, errors, skew, and memory.

### 6. Explain tradeoffs

- Latency versus freshness
- Availability versus consistency
- Memory cost versus hit ratio
- Simplicity versus stronger guarantees

### Practice questions

1. Why can adding or restarting cache nodes increase database load?
2. Why does ordinary sharding not solve a hot key?
3. What happens when a database write succeeds but cache invalidation fails?
4. When is a local cache better than a shared cache?
5. How does request coalescing change stampede load?
6. Why is hit ratio insufficient without miss cost and source capacity?
7. Which keys move when a node joins a consistent-hashing ring?
8. When is write-back unsafe?

For each answer, state the request flow, failure case, mitigation, and tradeoff.

### Common interview mistakes

- Adding Redis without explaining why
- Assuming a high hit ratio without estimating the working set
- Treating TTL, eviction, and invalidation as the same mechanism
- Ignoring cache-outage load on the database
- Claiming dual writes are atomic
- Saying sharding automatically fixes a hot key
- Giving absolutes without stating assumptions

---

## 16. Cheat Sheet

| Question | Main options |
|---|---|
| Where? | Client, CDN, local, shared, multi-level |
| Read pattern? | Cache-aside, read-through, refresh-ahead |
| Write pattern? | Write-through, write-around, write-back |
| Freshness? | TTL, invalidation, update, versions, events |
| Memory full? | LRU, LFU, FIFO, random, reject writes |
| Stampede? | Coalescing, per-key lock, stale serving, TTL jitter, warming |
| Scale? | Partitioning, consistent hashing/hash slots, replication |
| Failure? | Short timeout, bounded fallback, circuit breaker, load shedding |
| Measure? | Hit ratio, latency, source load, memory, evictions, skew, errors |

Remember:

- Expiration manages age.
- Eviction manages space.
- Invalidation responds to change.
- Partitioning divides data.
- Replication copies data.
- A cache outage must not become a database outage.

---

## 17. References

- [Microsoft: Cache-Aside pattern](https://learn.microsoft.com/en-us/azure/architecture/patterns/cache-aside)
- [AWS: Database caching strategies using Redis](https://docs.aws.amazon.com/whitepapers/latest/database-caching-strategies-using-redis/caching-patterns.html)
- [AWS: Performance at Scale with Amazon ElastiCache](https://docs.aws.amazon.com/pdfs/whitepapers/latest/scale-performance-elasticache/scale-performance-elasticache.pdf)
- [Redis: Key eviction](https://redis.io/docs/latest/develop/reference/eviction/)
- [Redis: Key expiration](https://redis.io/docs/latest/commands/expire/)
- [Redis: Cluster specification](https://redis.io/docs/latest/operate/oss_and_stack/reference/cluster-spec/)
- [Redis: Cluster scaling and hash slots](https://redis.io/docs/latest/operate/oss_and_stack/management/scaling/)
- [Redis: Replication](https://redis.io/docs/latest/operate/oss_and_stack/management/replication/)
- [Karger et al.: Consistent Hashing and Random Trees](https://people.csail.mit.edu/karger/Papers/web.pdf)
- [Amazon: Dynamo](https://www.amazon.science/publications/dynamo-amazons-highly-available-key-value-store)
