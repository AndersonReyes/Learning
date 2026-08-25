# System Design: Caching

## Table of Contents

- [1. Caching Fundamentals](#1-caching-fundamentals)
- [2. Where to Cache](#2-where-to-cache)
- [3. Cache-Aside Pattern](#3-cache-aside-pattern)
- [4. Eviction and Expiration](#4-eviction-and-expiration)
  - [4.1 LRU](#41-lru--least-recently-used)
  - [4.2 LFU](#42-lfu--least-frequently-used)
  - [4.3 TTL](#43-ttl--time-to-live)
- [5. Cache Freshness and Invalidation](#5-cache-freshness-and-invalidation)
- [6. Cache Write Strategies](#6-cache-write-strategies)
- [7. Hot Keys](#7-hot-keys)
- [8. Cache Stampede / Thundering Herd](#8-cache-stampede--thundering-herd)
- [9. Partitioning and Consistent Hashing](#9-partitioning-and-consistent-hashing)
- [10. Replication](#10-replication)
- [11. Concurrency](#11-concurrency)
- [12. Cache Failures](#12-cache-failures)
- [13. Capacity Estimation](#13-capacity-estimation)
- [14. Monitoring and Metrics](#14-monitoring-and-metrics)
- [15. Putting It Together](#15-putting-it-together)
- [16. Interview Framework](#16-interview-framework)
- [17. One-Page Cheat Sheet](#17-one-page-cheat-sheet)
- [References](#references)

---

# 1. Caching Fundamentals

A cache stores frequently accessed data closer to the application so that repeated reads can avoid expensive work.

The two primary goals are:

1. **Reduce latency**
2. **Reduce load on the underlying data store**

Typical architecture:

```mermaid
flowchart LR
    A[Application] --> C[Cache]
    C --> DB[(Database)]
