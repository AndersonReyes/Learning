# Intermediate 7. The `math` Package + TCP Congestion Control (Reno & CUBIC)

## New: the `math` package

Go's `math` package provides standard floating-point functions. This topic's
exercise needs three:

```go
math.Pow(x, y)   // x^y, both float64
math.Cbrt(x)     // cube root of x (handles negative x correctly, unlike
                 // math.Pow(x, 1.0/3.0), which returns NaN for negative x)
math.Abs(x)      // |x|
math.Max(a, b)   // larger of two float64 — NOT the builtin max[T] for ints
math.Min(a, b)   // smaller of two float64
```

**Gotchas**:

- All `math` functions operate on (and return) `float64`. Converting `int`
  to `float64` and back is explicit: `float64(n)`, `int(f)` (the latter
  truncates toward zero).
- `math.Cbrt(-8)` correctly returns `-2`. `math.Pow(-8, 1.0/3.0)` returns
  `NaN`, because `Pow` computes general real exponentiation
  (`exp(y * log(x))`), and `log` of a negative number is undefined. Always
  use `Cbrt` for cube roots that might be negative.
- **Never compare floats with `==`.** Floating-point arithmetic accumulates
  rounding error. `var a, b float64 = 0.1, 0.2; a+b == 0.3` is `false` in Go
  — `a+b` is computed at runtime in float64 and lands on `0.30000000000000004`,
  one bit off from the nearest float64 to `0.3`. (Note: the *constant
  expression* `0.1 + 0.2 == 0.3` is `true` in Go, because untyped constants
  are evaluated exactly at compile time before either side is rounded to
  float64 — but this doesn't help once values come from variables,
  computation, or — as here — a cube root.) Compare with a tolerance
  instead:
  ```go
  func floatsEqual(a, b, epsilon float64) bool {
      return math.Abs(a-b) < epsilon
  }
  ```
  This topic's test suite uses exactly this pattern for CUBIC's window
  values, which involve a cube root and are not exact in binary
  floating-point.
- `math.MaxInt`, `math.MaxFloat64`, etc. are useful sentinel "infinity"
  values when an algorithm needs "no value yet" without using a pointer or a
  separate `bool`.

## Networking: TCP congestion control

A TCP sender doesn't transmit as fast as the application produces data — it
maintains a **congestion window (cwnd)**, a self-imposed limit on how many
unacknowledged segments may be in flight, and adapts cwnd based on signals of
network congestion (packet loss, ECN). Two algorithms, two very different
growth curves for the same cwnd-over-time picture.

### TCP Reno (RFC 5681): AIMD — Additive Increase, Multiplicative Decrease

Reno alternates between two phases:

- **Slow start**: cwnd starts small (historically 1 segment, RFC 5681
  recommends up to 4) and *doubles* every RTT (in practice, +1 segment per
  ACK received — since a full window of ACKs arrives per RTT, this is
  exponential growth) until cwnd reaches the **slow-start threshold**
  (`ssthresh`).
- **Congestion avoidance**: once `cwnd >= ssthresh`, growth becomes *linear*
  — +1 segment per RTT (in practice, `cwnd += 1/cwnd` per ACK, which sums to
  +1 per RTT's worth of ACKs).
- **On loss** (multiplicative decrease): `ssthresh = cwnd / 2`, and — under
  Fast Retransmit/Fast Recovery (a loss signaled by duplicate ACKs, not a
  timeout) — `cwnd` drops directly to the new `ssthresh` rather than
  collapsing to 1. A full retransmission timeout is more drastic
  (`cwnd = 1`), restarting slow start from scratch; this exercise models only
  the fast-recovery case.

Plotted over time, repeated loss events produce Reno's famous **sawtooth**:
linear climb, halve, climb, halve.

### TCP CUBIC (RFC 8312): a cubic function of time since the last loss

CUBIC, the default congestion control in Linux since 2.6.19, grows cwnd as a
**cubic function of wall-clock time since the last congestion event** (RTT
count, in this exercise's simplified per-RTT model) rather than per-ACK:

```
K = cbrt(W_max * (1 - beta) / C)
W(t) = C * (t - K)^3 + W_max
```

- `W_max` is the window size at the last congestion event.
- `beta` (default 0.7) is the multiplicative decrease factor: on loss,
  `W_max = cwnd` and `cwnd *= beta`.
- `C` (default 0.4) is a scaling constant controlling how aggressively `W(t)`
  grows.
- `K` is chosen so that `W(K) = W_max` — i.e., `t = K` is exactly when the
  curve returns to the pre-loss window size.

The shape of `W(t)` is the key idea: for `t < K`, `(t-K)` is negative, so
`(t-K)^3` is negative and `W(t) < W_max` — cwnd approaches `W_max` with
*decreasing* slope (concave, "cautious" growth, since the network was
recently congested near this point). For `t > K`, `(t-K)^3 > 0` and growth
becomes *increasingly* aggressive (convex) — CUBIC probes for new available
bandwidth, since the network has had time to drain the queue that caused the
earlier loss. This concave-then-convex shape is CUBIC's signature, and is
exactly why it's called "CUBIC."

### Why this matters for "TCP-friendliness"

RFC 8312 designs CUBIC's constants so that, under the same conditions, CUBIC
and Reno achieve roughly the same average throughput on networks with
small bandwidth-delay products (where Reno's linear growth is already
adequate) — while CUBIC's cubic curve grows much faster than Reno's linear
one on the high-bandwidth, high-latency links where Reno's +1-per-RTT growth
is far too slow to use available capacity.

## Further Reading

- [`math`](https://pkg.go.dev/math) — `Cbrt`, `Pow`, `Abs`, `Max`, `Min`
- [RFC 5681 (TCP Congestion Control)](https://www.rfc-editor.org/rfc/rfc5681)
  — §2 (slow start, congestion avoidance), §3 (fast retransmit/fast recovery)
- [RFC 8312 (CUBIC)](https://www.rfc-editor.org/rfc/rfc8312) — §4 (the window
  growth function and its concave/convex regions)
