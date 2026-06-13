package congestion

import (
	"math"
	"reflect"
	"testing"
)

const epsilon = 1e-6

func floatsEqual(a, b float64) bool {
	return math.Abs(a-b) < epsilon
}

func TestRenoOnRTT(t *testing.T) {
	s := NewRenoState(1, 8)

	// Slow start doubles CWND each RTT, capped at SSThresh (8); once CWND
	// reaches SSThresh, congestion avoidance adds 1 per RTT.
	want := []float64{2, 4, 8, 9, 10}
	for i, w := range want {
		s.OnRTT()
		if s.CWND != w {
			t.Errorf("after OnRTT() #%d: CWND = %v, want %v", i+1, s.CWND, w)
		}
	}
}

func TestRenoOnLoss(t *testing.T) {
	s := NewRenoState(10, 100)
	s.OnLoss()

	if s.SSThresh != 5 {
		t.Errorf("SSThresh = %v, want 5", s.SSThresh)
	}
	if s.CWND != 5 {
		t.Errorf("CWND = %v, want 5", s.CWND)
	}
}

func TestCubicOnRTT(t *testing.T) {
	s := NewCubicState(10)

	// W(t) = cubicC*(t-K)^3 + WMax, K = cbrt(WMax*(1-cubicBeta)/cubicC) ≈
	// 1.9574338205844317 for WMax=10.
	want := []float64{9.64893600966, 10.000030849917364, 10.453284520772092}
	for i, w := range want {
		s.OnRTT()
		if !floatsEqual(s.CWND, w) {
			t.Errorf("after OnRTT() #%d: CWND = %v, want %v", i+1, s.CWND, w)
		}
		if s.Epoch != float64(i+1) {
			t.Errorf("after OnRTT() #%d: Epoch = %v, want %v", i+1, s.Epoch, float64(i+1))
		}
	}
}

func TestCubicOnLoss(t *testing.T) {
	s := &CubicState{CWND: 50, WMax: 10, Epoch: 7}
	s.OnLoss()

	if s.WMax != 50 {
		t.Errorf("WMax = %v, want 50", s.WMax)
	}
	if !floatsEqual(s.CWND, 35) {
		t.Errorf("CWND = %v, want 35", s.CWND)
	}
	if s.Epoch != 0 {
		t.Errorf("Epoch = %v, want 0", s.Epoch)
	}
}

func TestSimulateWindowGrowth(t *testing.T) {
	s := NewRenoState(1, 8)
	lossRounds := map[int]bool{6: true}

	got := SimulateWindowGrowth(s, 8, lossRounds)
	want := []float64{2, 4, 8, 9, 10, 5, 6, 7}

	if !reflect.DeepEqual(got, want) {
		t.Errorf("SimulateWindowGrowth() = %v, want %v", got, want)
	}
}
