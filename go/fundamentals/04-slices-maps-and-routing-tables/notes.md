# 04. Slices, Arrays & Maps

## Arrays

An array type `[N]T` has a fixed length `N` that is part of the type —
`[4]byte` and `[6]byte` are different types. Arrays are **value types**:
assigning or passing an array copies all its elements.

```go
var a [4]byte         // [0 0 0 0]
b := a                 // b is an independent copy
b[0] = 1               // does not affect a
```

## Slices

A slice `[]T` is a view (pointer, length, capacity) into an underlying
array. The zero value is `nil`, with `len(s) == 0` and `cap(s) == 0` — but
a nil slice is still usable (`append` works on it).

```go
var s []int            // nil, len 0, cap 0
s = append(s, 1, 2, 3)  // allocates a backing array

a := [5]int{1, 2, 3, 4, 5}
s1 := a[1:3]            // [2 3], shares a's backing array
s1[0] = 99              // a is now [1 99 3 4 5] — mutation is visible through a!
```

- **Slicing shares storage.** `s[low:high]` does not copy; it's a new
  view over the same backing array. Mutating elements through one slice
  is visible through any other slice sharing that array.
- **`append` may or may not reallocate.** If `cap(s)` has room, `append`
  writes into the existing backing array (visible to other slices sharing
  it!). If not, `append` allocates a new, larger array — after that, the
  original and the appended slice are independent.
- Use `copy(dst, src)` to make an independent copy when you need one.
- `make([]T, len, cap)` preallocates; useful when you know the final size
  (e.g. building a filtered result: `make([]T, 0, len(input))`).

## Maps

`map[K]V` is a hash table. The zero value is `nil`.

```go
var m map[string]int     // nil map
v := m["missing"]        // reading a nil map is fine: returns the zero value (0)
m["x"] = 1                // PANICS: assignment to entry in nil map

m = make(map[string]int) // writable map
m["x"] = 1
v, ok := m["x"]           // comma-ok idiom: ok reports whether the key exists
delete(m, "x")            // no-op if "x" isn't present
```

**Map iteration order is randomized** — `for k, v := range m` visits keys
in an unspecified (and varying-between-runs) order. If output order
matters, collect keys into a slice and sort it (`sort.Slice`, covered in
topic 2).

A map can hold slice values (`map[uint8][]Route`): the zero value for a
missing key is a `nil` slice, and `append(nil, x)` works fine, so
`m[k] = append(m[k], x)` is a common "group by" idiom without needing to
check `ok` first.

## Bitwise prefix masks (recap)

As in topic 1, `^uint32(0) << (32 - prefixLen)` builds a mask with the top
`prefixLen` bits set. Go defines `x << n` as `0` when `n >= the bit width
of x`, so this formula handles `prefixLen == 0` (mask `0`, matches
anything) and `prefixLen == 32` (mask `0xFFFFFFFF`, exact match) without
special-casing.

---

## Networking: routing tables & longest-prefix-match (RFC 4632 §3)

A **routing table** is a list of entries, each mapping a CIDR prefix to a
next hop: `(prefix, prefixLen, nextHop)`. To route a packet to destination
address `addr`, a router finds all entries whose prefix matches `addr`
(`addr` masked to `prefixLen` bits equals the entry's prefix) and picks
the entry with the **longest `prefixLen`** — the most specific match.

- A **default route** (`0.0.0.0/0`) matches every address and acts as a
  catch-all fallback (lowest possible specificity).
- Two CIDR blocks of the same length are **siblings** if they are the "0"
  half and "1" half of the same parent block — e.g. `10.0.0.0/25` and
  `10.0.0.128/25` are siblings of `10.0.0.0/24`.
- **Route aggregation (supernetting)**: if sibling blocks share the same
  next hop, they can be merged into their parent block, halving the
  number of table entries needed to express the same routing policy. This
  can cascade: merging may produce a new pair of siblings one level up.

## Further Reading

- [Tour: Arrays](https://go.dev/tour/moretypes/6)
- [Tour: Slices](https://go.dev/tour/moretypes/7)
- [Tour: Appending to a slice](https://go.dev/tour/moretypes/13)
- [Tour: Maps](https://go.dev/tour/moretypes/17)
- [Tour: Mutating maps](https://go.dev/tour/moretypes/20)
- [`sort` package](https://pkg.go.dev/sort)
- [RFC 4632 §3 — Routing Considerations & Aggregation](https://www.rfc-editor.org/rfc/rfc4632#section-3)
