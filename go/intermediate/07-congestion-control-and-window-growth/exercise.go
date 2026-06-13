// Package congestion simulates TCP congestion-control window-growth
// algorithms: Reno's additive-increase/multiplicative-decrease (AIMD, RFC
// 5681) and CUBIC's cubic window-growth function (RFC 8312). CUBIC's cube
// root and exponentiation introduce this topic's new Go concept, the math
// package.
package congestion

const (
	// cubicC and cubicBeta are CUBIC's scaling constant and multiplicative
	// decrease factor, matching RFC 8312's recommended defaults.
	cubicC    = 0.4
	cubicBeta = 0.7
)

// RenoState holds TCP Reno's congestion-control state (RFC 5681): a
// congestion window (CWND) and a slow-start threshold (SSThresh), both
// measured in segments.
type RenoState struct {
	CWND     float64
	SSThresh float64
}

// NewRenoState returns a RenoState with the given initial congestion window
// and slow-start threshold.
func NewRenoState(initialCWND, ssthresh float64) *RenoState {
	return &RenoState{CWND: initialCWND, SSThresh: ssthresh}
}

// Window returns the current congestion window.
func (s *RenoState) Window() float64 {
	return s.CWND
}

// CubicState holds TCP CUBIC's congestion-control state (RFC 8312): the
// current congestion window (CWND), the window size at the last congestion
// event (WMax), and the time elapsed since that event (Epoch), in RTTs.
type CubicState struct {
	CWND  float64
	WMax  float64
	Epoch float64
}

// NewCubicState returns a CubicState with the given initial congestion
// window. WMax is initialized to the same value, as if the connection had
// just recovered from a congestion event at this window size.
func NewCubicState(initialCWND float64) *CubicState {
	return &CubicState{CWND: initialCWND, WMax: initialCWND, Epoch: 0}
}

// Window returns the current congestion window.
func (s *CubicState) Window() float64 {
	return s.CWND
}

// CongestionController is satisfied by both RenoState and CubicState,
// allowing SimulateWindowGrowth to drive either algorithm through a sequence
// of round trips and loss events.
type CongestionController interface {
	OnRTT()
	OnLoss()
	Window() float64
}

// OnRTT advances s by one round trip with no packet loss. While CWND is
// below SSThresh (slow start), CWND doubles, capped at SSThresh. Once CWND
// reaches SSThresh (congestion avoidance), CWND increases by 1 segment per
// round trip.
func (s *RenoState) OnRTT() {
}

// OnLoss applies Reno's multiplicative decrease in response to a detected
// packet loss: SSThresh is set to half of CWND, and CWND drops to the new
// SSThresh (fast recovery).
func (s *RenoState) OnLoss() {
}

// OnRTT advances s by one round trip with no packet loss, incrementing Epoch
// and setting CWND according to RFC 8312's cubic window-growth function:
//
//	K = cbrt(WMax * (1 - cubicBeta) / cubicC)
//	W(t) = cubicC*(t-K)^3 + WMax
//
// where t is Epoch after being incremented by this call.
func (s *CubicState) OnRTT() {
}

// OnLoss applies CUBIC's multiplicative decrease in response to a detected
// packet loss: WMax is set to the current CWND, CWND is reduced to
// cubicBeta times its value before the loss, and Epoch resets to 0.
func (s *CubicState) OnLoss() {
}

// SimulateWindowGrowth drives c through rounds round trips, calling OnLoss
// for any round number present (with value true) in lossRounds and OnRTT
// otherwise, and returns the resulting window size after each round.
func SimulateWindowGrowth(c CongestionController, rounds int, lossRounds map[int]bool) []float64 {
	return nil
}
