// Command main demonstrates the math package gotchas this topic's exercise
// relies on (math.Cbrt vs math.Pow for negative numbers, and
// epsilon-based float comparison), then prints CUBIC's W(t) curve for a
// couple of beta values to illustrate its concave-then-convex shape — a
// different angle on the same formula the exercise implements, using
// values not used by exercise_test.go.
package main

import (
	"fmt"
	"math"
)

func main() {
	fmt.Println("--- math.Cbrt vs math.Pow for negative numbers ---")
	fmt.Printf("math.Cbrt(-8)        = %v\n", math.Cbrt(-8))
	fmt.Printf("math.Pow(-8, 1.0/3)  = %v\n", math.Pow(-8, 1.0/3.0))

	fmt.Println("\n--- float equality: == vs epsilon ---")
	var x, y float64 = 0.1, 0.2
	sum := x + y // computed at runtime in float64, unlike a constant expression
	fmt.Printf("0.1 + 0.2            = %v\n", sum)
	fmt.Printf("(0.1 + 0.2) == 0.3   -> %v\n", sum == 0.3)
	const epsilon = 1e-9
	fmt.Printf("within %.0e of 0.3?  -> %v\n", epsilon, math.Abs(sum-0.3) < epsilon)

	fmt.Println("\n--- CUBIC's W(t) = C*(t-K)^3 + Wmax, two beta values ---")
	const (
		c    = 0.4
		wmax = 20.0
	)
	for _, beta := range []float64{0.7, 0.5} {
		k := math.Cbrt(wmax * (1 - beta) / c)
		fmt.Printf("\nbeta=%.1f -> K=%.4f, Wmax=%.1f\n", beta, k, wmax)
		fmt.Println("  t\tW(t)")
		for t := 0.0; t <= 6; t++ {
			w := c*math.Pow(t-k, 3) + wmax
			fmt.Printf("  %.0f\t%.4f\n", t, w)
		}
	}
	fmt.Println("\nNote the concave region (W(t) < Wmax, t < K) followed by")
	fmt.Println("the convex region (W(t) > Wmax, t > K) in both columns —")
	fmt.Println("a smaller beta means less window was lost, so K (and the")
	fmt.Println("cautious region) is smaller.")
}
