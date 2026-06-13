# Intermediate 1. Generics + Generic Connection Pool & DNS-Cache LRU

## Type parameters

A generic function or type declares **type parameters** in square brackets
before its regular parameter list:

```go
func Max[T cmp.Ordered](a, b T) T {
    if a > b {
        return a
    }
    return b
}

m := Max(3, 7)       // T inferred as int
s := Max("a", "b")   // T inferred as string
```

- `T` is a placeholder type, constrained by `cmp.Ordered` (anything with
  `<`, `>`, etc. — ints, floats, strings).
- The compiler usually **infers** `T` from the argument types — you rarely
  write `Max[int](3, 7)` explicitly.

## Constraints

A constraint is an interface that limits which types `T` can be:

| Constraint | Meaning |
|---|---|
| `any` | alias for `interface{}` — no methods, no operators required |
| `comparable` | supports `==` and `!=` — required for map keys |
| `cmp.Ordered` (stdlib, Go 1.21+) | supports `<`, `<=`, `>`, `>=` |
| custom interface | e.g. `interface { ~int \| ~int64 }` — a **type set** using `~T` to also match named types whose underlying type is `T` |

`comparable` matters for this topic: a generic cache keyed by `K` needs
`K comparable` so it can be used as a map key (`map[K]V`).

## Generic types

```go
type Stack[T any] struct {
    items []T
}

func (s *Stack[T]) Push(v T) {
    s.items = append(s.items, v)
}

func (s *Stack[T]) Pop() (T, bool) {
    if len(s.items) == 0 {
        var zero T
        return zero, false
    }
    v := s.items[len(s.items)-1]
    s.items = s.items[:len(s.items)-1]
    return v, true
}
```

- A generic type's methods repeat the type parameter (`func (s *Stack[T])
  ...`), but don't redeclare its constraint.
- `var zero T` is the idiomatic way to get T's zero value — you can't write
  `nil` or `0` directly since T could be any type.
- Instantiate with `Stack[int]{}` or let inference handle it from usage.

## Two generic types, one shared shape

This topic builds two generic types that both wrap a `sync.Mutex` around an
internal data structure — the concurrency-safe-wrapper pattern from topic 7
(`select`/`sync`), now parameterized over the element type:

- **`Pool[T any]`**: a bag of reusable values of type `T`, created on
  demand by a `New func() (T, error)` and returned with `Put` for reuse.
- **`LRU[K comparable, V any]`**: a fixed-capacity cache that evicts the
  **least-recently-used** entry when a `Put` would exceed `capacity`. `Get`
  and `Put` both count as "use" — they move the entry to the front of an
  internal `container/list` (a doubly-linked list), so the back of the
  list is always the next eviction candidate.

`container/list.List` stores `*list.Element`, each holding an `any` (its
`Value` field) — combined with a `map[K]*list.Element` for O(1) lookup,
this gives O(1) `Get`/`Put` for the LRU.

---

## Networking: pooling connections, caching DNS lookups

**Connection pooling.** Topic 8 showed `net.Dial` opening a fresh TCP
connection per call — each `Dial` pays a full handshake (topic 2's TCP
three-way handshake; with TLS, topic 13's handshake adds more round trips
on top). A pool amortizes that cost: `Get` returns an idle connection if
one exists, or dials a new one; `Put` returns a still-usable connection to
the pool instead of closing it. `Pool[T]` here is generic so the same code
works whether `T` is `net.Conn`, `*sql.DB` connection, or any other
expensive-to-create resource — only the `New` function changes.

**DNS-cache LRU.** Topic 12's `Resolve` does a full UDP round trip for
every lookup. Real DNS resolvers cache answers using the **TTL** field from
each resource record (RFC 1035 §3.2.1 — "the time interval... that the
resource record may be cached before it should be discarded"): an entry is
valid until its TTL expires, regardless of how often it's used.

`LRU[K, V]` here implements a complementary, simpler policy: a fixed
**capacity** rather than a time-based expiry. Caching `Resolve("example.
com")`'s `[]net.IP` result under the key `"example.com"` means repeated
lookups for the same hostname skip the network entirely, as long as the
entry hasn't been evicted to make room for more-recently-used hostnames. A
production resolver cache combines both: LRU (or similar) eviction bounds
memory, while each entry's remaining TTL bounds how long it can be served
before a fresh query is required.

## Further Reading

- [Tour: Generics](https://go.dev/tour/generics/1)
- [`cmp.Ordered`](https://pkg.go.dev/cmp#Ordered)
- [`container/list`](https://pkg.go.dev/container/list)
- [`slices`](https://pkg.go.dev/slices), [`maps`](https://pkg.go.dev/maps) (generic stdlib helpers, Go 1.21+)
- [RFC 1035 §3.2.1 (TTL)](https://www.rfc-editor.org/rfc/rfc1035#section-3.2.1)
