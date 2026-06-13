# 01. Go Basics & IP Addressing

## Program structure

```go
package main // every Go file declares its package; "main" + func main() = executable

import (
	"fmt"
	"strconv"
	"strings"
)

func main() {
	fmt.Println("hello")
}
```

Library code (like `exercise.go`) uses a non-`main` package name
(`package ipaddr`) and is imported by other packages instead of run directly.

## Variables, constants & zero values

```go
var x int        // zero value: 0
var s string      // zero value: ""
var ok bool       // zero value: false
var arr [4]byte   // zero value: [0 0 0 0]

y := 42           // short declaration, type inferred
const MaxOctet = 255 // untyped constant
```

- `:=` only works for new variables inside a function — package-level
  declarations always need `var`/`const`.
- Unused local variables and imports are **compile errors** in Go.

## Basic types & explicit conversion

`int`, `int32`, `int64`, `uint`, `uint8` (alias `byte`), `uint16`, `uint32`,
`uint64`, `float64`, `string`, `bool`.

```go
var n uint32 = 10
var b byte = 5
total := n + uint32(b) // no implicit conversion — uint32(b) required
```

Mixing types without an explicit conversion is a compile error, even between
`int` and `uint32`.

## Operators, including bitwise

| Op | Meaning |
|----|---------|
| `&` | bitwise AND |
| `\|` | bitwise OR |
| `^` | bitwise XOR (binary) / bitwise NOT (unary) |
| `&^` | AND NOT ("bit clear") |
| `<<`, `>>` | left/right shift |

```go
var mask uint32 = 0xFFFFFFFF << 8 // 0xFFFFFF00
```

**Gotcha — shift counts are well-defined in Go**, unlike C: shifting an
unsigned value by >= its bit width yields `0` (left shift) or `0` (right
shift), not undefined behavior. `uint32(0xFFFFFFFF) << 32 == 0`. This makes
"shift by `32 - prefixLen`" safe for `prefixLen` from 0 to 32 *without* a
special case for the endpoints.

## Control flow

```go
// if/else: no parens around condition, braces mandatory
if x > 0 {
	// ...
} else if x == 0 {
	// ...
} else {
	// ...
}

// for: the only loop keyword — 3 forms
for i := 0; i < 10; i++ { /* classic */ }
for cond { /* while-style */ }
for { /* infinite, use break */ }
for i, v := range slice { /* index + value */ }
for _, v := range slice { /* value only, _ discards index */ }

// switch: cases don't fall through by default (no break needed)
switch {
case x < 0:
	fmt.Println("negative")
case x == 0:
	fmt.Println("zero")
default:
	fmt.Println("positive")
}
```

## Functions, multiple returns & errors

Go has no exceptions for normal control flow. Functions that can fail return
an extra `error` value as the **last** return value:

```go
func ParseThing(s string) (Thing, error) {
	if invalid {
		return Thing{}, errors.New("descriptive message")
	}
	return thing, nil
}

// caller:
t, err := ParseThing(s)
if err != nil {
	// handle/return err — Go has no try/catch
}
```

- `error` is an interface (`Error() string`). `errors.New("msg")` creates a
  simple one. `fmt.Errorf("context: %w", err)` wraps an underlying error
  (covered more in topic 5).
- Always check `err != nil` **before** using the other return value — on
  error, the other value is typically its zero value (`0`, `""`, `nil`),
  not meaningful data.

## Strings & parsing

Strings are immutable byte sequences (UTF-8). For this topic:

```go
strings.Split("192.168.1.1", ".") // []string{"192","168","1","1"}
strings.Join(parts, ".")           // "192.168.1.1"

n, err := strconv.Atoi("42")              // string -> int
v, err := strconv.ParseUint("255", 10, 8) // string -> uint64, base 10, fits in 8 bits
                                            // ParseUint validates BOTH "is it a
                                            // number" AND "does it fit" in one call —
                                            // ParseUint("256", 10, 8) errors (> 255)
```

## `fmt` verbs used here

`%d` (decimal), `%s` (string), `%v` (default format), `%T` (type), `%b`
(binary), `%032b` (binary, zero-padded to 32 digits — handy for visualizing
32-bit IPv4 addresses and masks).

---

## Networking: IPv4 addressing & CIDR

**The Internet layering model** (bottom to top): Link (Ethernet/Wi-Fi) → Internet
(IP) → Transport (TCP/UDP) → Application (HTTP/DNS/...). This topic lives at
the **Internet layer**.

### IPv4 address = 32 bits

Written as 4 dotted-decimal **octets** (0-255), e.g. `192.168.1.1`. Internally
it's one 32-bit unsigned integer — `192.168.1.1` is
`192*2^24 + 168*2^16 + 1*2^8 + 1 = 3232235777`. Packing/unpacking octets
into a `uint32` is exactly the shift/OR/AND pattern from the bitwise section
above.

### CIDR notation & subnet masks

`192.168.1.0/24` — the `/24` is the **prefix length**: the number of leading
bits that identify the network; the remaining `32 - prefixLen` bits identify
hosts within that network.

- **Subnet mask**: `prefixLen` leading 1-bits followed by 0-bits, e.g. `/24`
  → `11111111.11111111.11111111.00000000` (`255.255.255.0`).
- **Network address** = `ip & mask` (clear all host bits).
- **Broadcast address** = `network | ^mask` (set all host bits) — `^mask` is
  the "wildcard"/host mask.

Example: `192.168.1.130/26` → mask `/26` covers blocks of `2^(32-26) = 64`
addresses (`.0`, `.64`, `.128`, `.192`). `130` falls in the `.128-.191`
block, so network = `192.168.1.128`, broadcast = `192.168.1.191`.

### Usable host count & edge cases

A subnet's usable hosts = `2^(32-prefixLen) - 2` (network + broadcast
reserved) — **except**:

- `/31`: [RFC 3021](https://www.rfc-editor.org/rfc/rfc3021) — both addresses
  are usable on point-to-point links (2 usable, no network/broadcast
  reservation).
- `/32`: a single host route (1 usable — itself).

## Further Reading

- [A Tour of Go: Basics](https://go.dev/tour/basics/1)
- [Effective Go: Control structures](https://go.dev/doc/effective_go#control-structures)
- [Effective Go: Errors](https://go.dev/doc/effective_go#errors)
- [`strconv` package](https://pkg.go.dev/strconv)
- [`strings` package](https://pkg.go.dev/strings)
- [RFC 791 — Internet Protocol](https://www.rfc-editor.org/rfc/rfc791)
- [RFC 4632 — Classless Inter-domain Routing (CIDR)](https://www.rfc-editor.org/rfc/rfc4632)
- [RFC 3021 — Using 31-Bit Prefixes on IPv4 Point-to-Point Links](https://www.rfc-editor.org/rfc/rfc3021)
