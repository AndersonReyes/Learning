package groups_test

import (
	"sort"
	"testing"
	"time"

	"github.com/andersonreyes/learning/go/capstone-message-queue/broker"
	"github.com/andersonreyes/learning/go/capstone-message-queue/concurrent"
	"github.com/andersonreyes/learning/go/capstone-message-queue/groups"
	"github.com/andersonreyes/learning/go/capstone-message-queue/protocol"
)

func makeRegistry(t *testing.T, topics []struct {
	name  string
	parts uint32
}) *concurrent.SharedRegistry {
	t.Helper()
	dir := t.TempDir()
	reg, err := broker.Open(dir)
	if err != nil {
		t.Fatalf("Open: %v", err)
	}
	for _, tp := range topics {
		if err := reg.CreateTopic(tp.name, tp.parts); err != nil {
			t.Fatalf("CreateTopic: %v", err)
		}
	}
	sr := concurrent.New(reg, 100*time.Millisecond)
	t.Cleanup(sr.Close)
	return sr
}

func sortedAssignments(a []protocol.AssignedPartition) []protocol.AssignedPartition {
	result := make([]protocol.AssignedPartition, len(a))
	copy(result, a)
	sort.Slice(result, func(i, j int) bool {
		if result[i].Topic != result[j].Topic {
			return result[i].Topic < result[j].Topic
		}
		return result[i].Partition < result[j].Partition
	})
	return result
}

// ── join ──────────────────────────────────────────────────────────────────────

func TestJoinAssignsAllPartitionsToSoleMember(t *testing.T) {
	reg := makeRegistry(t, []struct {
		name  string
		parts uint32
	}{{"events", 3}})
	gc := groups.New()

	memberID, assignment, err := gc.Join("g1", []string{"events"}, reg)
	if err != nil {
		t.Fatalf("Join: %v", err)
	}
	if memberID != "member-0" {
		t.Fatalf("expected member-0, got %q", memberID)
	}
	sorted := sortedAssignments(assignment)
	expected := []protocol.AssignedPartition{
		{Topic: "events", Partition: 0},
		{Topic: "events", Partition: 1},
		{Topic: "events", Partition: 2},
	}
	if len(sorted) != len(expected) {
		t.Fatalf("expected %v, got %v", expected, sorted)
	}
	for i, ap := range sorted {
		if ap != expected[i] {
			t.Errorf("assignment[%d] = %v, want %v", i, ap, expected[i])
		}
	}
}

func TestJoinTwoMembersSplitPartitionsRoundRobin(t *testing.T) {
	reg := makeRegistry(t, []struct {
		name  string
		parts uint32
	}{{"t", 4}})
	gc := groups.New()

	m0, _, err := gc.Join("g", []string{"t"}, reg)
	if err != nil {
		t.Fatalf("Join m0: %v", err)
	}
	m1, _, err := gc.Join("g", []string{"t"}, reg)
	if err != nil {
		t.Fatalf("Join m1: %v", err)
	}

	// Fetch current assignments after the rebalance triggered by the second join.
	a0, _ := gc.Assignment("g", m0)
	a1, _ := gc.Assignment("g", m1)

	// Combined must cover all 4 partitions exactly once.
	all := append(a0, a1...)
	sort.Slice(all, func(i, j int) bool {
		if all[i].Topic != all[j].Topic {
			return all[i].Topic < all[j].Topic
		}
		return all[i].Partition < all[j].Partition
	})
	expected := []protocol.AssignedPartition{
		{Topic: "t", Partition: 0},
		{Topic: "t", Partition: 1},
		{Topic: "t", Partition: 2},
		{Topic: "t", Partition: 3},
	}
	if len(all) != 4 {
		t.Fatalf("expected 4 assignments, got %d: %v", len(all), all)
	}
	for i, ap := range all {
		if ap != expected[i] {
			t.Errorf("all[%d] = %v, want %v", i, ap, expected[i])
		}
	}

	// Sorted member IDs should be member-0 and member-1.
	members := gc.Members("g")
	memberSet := map[string]bool{m0: true, m1: true}
	for _, m := range members {
		if !memberSet[m] {
			t.Errorf("unexpected member %q", m)
		}
	}
}

func TestJoinUnknownTopicAssignsEmpty(t *testing.T) {
	reg := makeRegistry(t, nil)
	gc := groups.New()

	_, assignment, err := gc.Join("g", []string{"ghost"}, reg)
	if err != nil {
		t.Fatalf("Join: %v", err)
	}
	if len(assignment) != 0 {
		t.Fatalf("expected empty assignment, got %v", assignment)
	}
}

func TestJoinMultipleTopicsDistributesAll(t *testing.T) {
	reg := makeRegistry(t, []struct {
		name  string
		parts uint32
	}{{"a", 2}, {"b", 2}})
	gc := groups.New()

	_, assignment, err := gc.Join("g", []string{"a", "b"}, reg)
	if err != nil {
		t.Fatalf("Join: %v", err)
	}
	if len(assignment) != 4 {
		t.Fatalf("expected 4 assignments, got %d", len(assignment))
	}
	has := func(topic string, partition uint32) bool {
		for _, ap := range assignment {
			if ap.Topic == topic && ap.Partition == partition {
				return true
			}
		}
		return false
	}
	if !has("a", 0) || !has("a", 1) || !has("b", 0) || !has("b", 1) {
		t.Fatalf("missing expected assignments: %v", assignment)
	}
}

// ── leave ─────────────────────────────────────────────────────────────────────

func TestLeaveReassignsPartitionsToRemainingMember(t *testing.T) {
	reg := makeRegistry(t, []struct {
		name  string
		parts uint32
	}{{"t", 4}})
	gc := groups.New()

	m0, _, _ := gc.Join("g", []string{"t"}, reg)
	m1, _, _ := gc.Join("g", []string{"t"}, reg)

	// m0 leaves — m1 should get all 4 partitions.
	if err := gc.Leave("g", m0, reg); err != nil {
		t.Fatalf("Leave: %v", err)
	}

	assignment, _ := gc.Assignment("g", m1)
	sorted := sortedAssignments(assignment)
	expected := []protocol.AssignedPartition{
		{Topic: "t", Partition: 0},
		{Topic: "t", Partition: 1},
		{Topic: "t", Partition: 2},
		{Topic: "t", Partition: 3},
	}
	if len(sorted) != 4 {
		t.Fatalf("expected 4, got %v", sorted)
	}
	for i, ap := range sorted {
		if ap != expected[i] {
			t.Errorf("assignment[%d] = %v, want %v", i, ap, expected[i])
		}
	}
}

func TestLeaveUnknownGroupReturnsError(t *testing.T) {
	reg := makeRegistry(t, nil)
	gc := groups.New()

	if err := gc.Leave("no-such-group", "member-0", reg); err == nil {
		t.Fatal("expected error, got nil")
	}
}

func TestLeaveUnknownMemberReturnsError(t *testing.T) {
	reg := makeRegistry(t, []struct {
		name  string
		parts uint32
	}{{"t", 1}})
	gc := groups.New()

	gc.Join("g", []string{"t"}, reg)
	if err := gc.Leave("g", "member-999", reg); err == nil {
		t.Fatal("expected error, got nil")
	}
}

// ── commit / fetch offset ─────────────────────────────────────────────────────

func TestFetchOffsetReturnsNilBeforeCommit(t *testing.T) {
	reg := makeRegistry(t, []struct {
		name  string
		parts uint32
	}{{"t", 1}})
	gc := groups.New()

	gc.Join("g", []string{"t"}, reg)
	offset, err := gc.FetchOffset("g", "t", 0)
	if err != nil {
		t.Fatalf("FetchOffset: %v", err)
	}
	if offset != nil {
		t.Fatalf("expected nil, got %d", *offset)
	}
}

func TestCommitAndFetchOffsetRoundtrip(t *testing.T) {
	reg := makeRegistry(t, []struct {
		name  string
		parts uint32
	}{{"t", 2}})
	gc := groups.New()

	gc.Join("g", []string{"t"}, reg)
	gc.CommitOffset("g", "t", 0, 42)
	gc.CommitOffset("g", "t", 1, 7)

	off0, _ := gc.FetchOffset("g", "t", 0)
	if off0 == nil || *off0 != 42 {
		t.Fatalf("expected 42, got %v", off0)
	}
	off1, _ := gc.FetchOffset("g", "t", 1)
	if off1 == nil || *off1 != 7 {
		t.Fatalf("expected 7, got %v", off1)
	}
}

func TestCommitOffsetUpdatesExisting(t *testing.T) {
	reg := makeRegistry(t, []struct {
		name  string
		parts uint32
	}{{"t", 1}})
	gc := groups.New()

	gc.Join("g", []string{"t"}, reg)
	gc.CommitOffset("g", "t", 0, 10)
	gc.CommitOffset("g", "t", 0, 20)

	off, _ := gc.FetchOffset("g", "t", 0)
	if off == nil || *off != 20 {
		t.Fatalf("expected 20, got %v", off)
	}
}

// ── group / member listing ─────────────────────────────────────────────────────

func TestGroupNamesReturnsSortedNames(t *testing.T) {
	reg := makeRegistry(t, []struct {
		name  string
		parts uint32
	}{{"t", 1}})
	gc := groups.New()

	gc.Join("zebra", []string{"t"}, reg)
	gc.Join("alpha", []string{"t"}, reg)

	names := gc.GroupNames()
	if len(names) != 2 || names[0] != "alpha" || names[1] != "zebra" {
		t.Fatalf("expected ['alpha', 'zebra'], got %v", names)
	}
}

func TestMembersReturnsSortedIDs(t *testing.T) {
	reg := makeRegistry(t, []struct {
		name  string
		parts uint32
	}{{"t", 2}})
	gc := groups.New()

	m0, _, _ := gc.Join("g", []string{"t"}, reg)
	m1, _, _ := gc.Join("g", []string{"t"}, reg)
	m2, _, _ := gc.Join("g", []string{"t"}, reg)

	members := gc.Members("g")
	want := []string{m0, m1, m2}
	sort.Strings(want)
	if len(members) != 3 {
		t.Fatalf("expected 3 members, got %v", members)
	}
	for i, m := range members {
		if m != want[i] {
			t.Errorf("members[%d] = %q, want %q", i, m, want[i])
		}
	}
}

func TestMembersReturnsEmptyForUnknownGroup(t *testing.T) {
	gc := groups.New()
	if len(gc.Members("no-such-group")) != 0 {
		t.Fatal("expected empty members list")
	}
}
