package groups

import (
	"testing"
	"time"

	"github.com/andersonreyes/learning/go/capstone-message-queue/broker"
	"github.com/andersonreyes/learning/go/capstone-message-queue/concurrent"
)

// helper: build a SharedRegistry with one topic.
func makeReg(t *testing.T, topic string, partitions int) *concurrent.SharedRegistry {
	t.Helper()
	dir := t.TempDir()
	reg, err := broker.OpenRegistry(dir)
	if err != nil {
		t.Fatalf("OpenRegistry: %v", err)
	}
	if err := reg.CreateTopic(topic, partitions); err != nil {
		t.Fatalf("CreateTopic: %v", err)
	}
	return concurrent.NewSharedRegistry(reg, time.Hour)
}

func closeReg(t *testing.T, s *concurrent.SharedRegistry) {
	t.Helper()
	if err := s.Close(); err != nil {
		t.Fatalf("SharedRegistry.Close(): %v", err)
	}
}

// ── Join ──────────────────────────────────────────────────────────────────────

func TestJoinSoleMemberGetsAllPartitions(t *testing.T) {
	reg := makeReg(t, "events", 3)
	defer closeReg(t, reg)

	gc := NewGroupCoordinator()
	memberID, assignments, err := gc.Join("g", []string{"events"}, reg)
	if err != nil {
		t.Fatalf("Join: %v", err)
	}
	if memberID != "member-0" {
		t.Errorf("memberID = %q, want \"member-0\"", memberID)
	}
	if len(assignments) != 3 {
		t.Fatalf("sole member got %d assignments, want 3 (one per partition)", len(assignments))
	}
	for i, a := range assignments {
		if a.Topic != "events" {
			t.Errorf("assignments[%d].Topic = %q, want \"events\"", i, a.Topic)
		}
		if a.Partition != uint32(i) {
			t.Errorf("assignments[%d].Partition = %d, want %d", i, a.Partition, i)
		}
	}
}

func TestJoinAssignsMemberIDs(t *testing.T) {
	reg := makeReg(t, "t", 4)
	defer closeReg(t, reg)

	gc := NewGroupCoordinator()
	id0, _, err := gc.Join("g", []string{"t"}, reg)
	if err != nil {
		t.Fatal(err)
	}
	id1, _, err := gc.Join("g", []string{"t"}, reg)
	if err != nil {
		t.Fatal(err)
	}
	if id0 != "member-0" {
		t.Errorf("first member ID = %q, want \"member-0\"", id0)
	}
	if id1 != "member-1" {
		t.Errorf("second member ID = %q, want \"member-1\"", id1)
	}
}

func TestJoinTwoMembersSplitRoundRobin(t *testing.T) {
	// 4 partitions, 2 members → member-0 gets [0,2], member-1 gets [1,3].
	reg := makeReg(t, "events", 4)
	defer closeReg(t, reg)

	gc := NewGroupCoordinator()

	_, _, err := gc.Join("g", []string{"events"}, reg)
	if err != nil {
		t.Fatalf("Join member-0: %v", err)
	}
	_, assignments1, err := gc.Join("g", []string{"events"}, reg)
	if err != nil {
		t.Fatalf("Join member-1: %v", err)
	}

	// After the second join, each member holds 2 partitions.
	// member-1 (alphabetically second) gets pairs at indices 1 and 3
	// in the sorted list (events/0, events/1, events/2, events/3).
	if len(assignments1) != 2 {
		t.Fatalf("member-1 assignments = %v, want 2 entries", assignments1)
	}
	if assignments1[0].Partition != 1 {
		t.Errorf("member-1 assignments[0].Partition = %d, want 1", assignments1[0].Partition)
	}
	if assignments1[1].Partition != 3 {
		t.Errorf("member-1 assignments[1].Partition = %d, want 3", assignments1[1].Partition)
	}
}

func TestJoinUnknownTopicErrors(t *testing.T) {
	reg := makeReg(t, "real", 1)
	defer closeReg(t, reg)

	gc := NewGroupCoordinator()
	_, _, err := gc.Join("g", []string{"nonexistent"}, reg)
	if err == nil {
		t.Fatal("Join with unknown topic: expected error, got nil")
	}
}

// ── Members / GroupNames ───────────────────────────────────────────────────────

func TestMembersAfterJoin(t *testing.T) {
	reg := makeReg(t, "t", 1)
	defer closeReg(t, reg)

	gc := NewGroupCoordinator()
	gc.Join("grp", []string{"t"}, reg) //nolint:errcheck
	gc.Join("grp", []string{"t"}, reg) //nolint:errcheck

	members := gc.Members("grp")
	if len(members) != 2 {
		t.Fatalf("Members(grp) = %v, want 2 members", members)
	}
	if members[0] != "member-0" || members[1] != "member-1" {
		t.Errorf("Members(grp) = %v, want [member-0 member-1]", members)
	}
}

func TestMembersUnknownGroupEmpty(t *testing.T) {
	gc := NewGroupCoordinator()
	members := gc.Members("unknown")
	if members == nil {
		t.Fatal("Members(unknown) = nil, want empty slice")
	}
	if len(members) != 0 {
		t.Errorf("Members(unknown) = %v, want []", members)
	}
}

func TestGroupNames(t *testing.T) {
	reg := makeReg(t, "t", 1)
	defer closeReg(t, reg)

	gc := NewGroupCoordinator()
	gc.Join("beta", []string{"t"}, reg)  //nolint:errcheck
	gc.Join("alpha", []string{"t"}, reg) //nolint:errcheck

	names := gc.GroupNames()
	if len(names) != 2 {
		t.Fatalf("GroupNames() = %v, want 2 groups", names)
	}
	if names[0] != "alpha" || names[1] != "beta" {
		t.Errorf("GroupNames() = %v, want [alpha beta]", names)
	}
}

// ── Leave ─────────────────────────────────────────────────────────────────────

func TestLeaveReassignsToSurvivor(t *testing.T) {
	// 3 partitions, 2 members. After member-0 leaves, member-1 should hold all 3.
	reg := makeReg(t, "events", 3)
	defer closeReg(t, reg)

	gc := NewGroupCoordinator()
	gc.Join("g", []string{"events"}, reg) //nolint:errcheck
	gc.Join("g", []string{"events"}, reg) //nolint:errcheck

	if err := gc.Leave("g", "member-0", reg); err != nil {
		t.Fatalf("Leave member-0: %v", err)
	}

	members := gc.Members("g")
	if len(members) != 1 || members[0] != "member-1" {
		t.Errorf("after leave Members(g) = %v, want [member-1]", members)
	}
}

func TestLeaveUnknownMemberErrors(t *testing.T) {
	reg := makeReg(t, "t", 1)
	defer closeReg(t, reg)

	gc := NewGroupCoordinator()
	gc.Join("g", []string{"t"}, reg) //nolint:errcheck

	if err := gc.Leave("g", "member-99", reg); err == nil {
		t.Fatal("Leave with unknown member: expected error, got nil")
	}
}

func TestLeaveLastMemberRemovesGroup(t *testing.T) {
	reg := makeReg(t, "t", 1)
	defer closeReg(t, reg)

	gc := NewGroupCoordinator()
	gc.Join("g", []string{"t"}, reg) //nolint:errcheck

	if err := gc.Leave("g", "member-0", reg); err != nil {
		t.Fatalf("Leave last member: %v", err)
	}

	names := gc.GroupNames()
	for _, n := range names {
		if n == "g" {
			t.Error("group g still listed in GroupNames after all members left")
		}
	}
	members := gc.Members("g")
	if len(members) != 0 {
		t.Errorf("Members of removed group = %v, want []", members)
	}
}

// ── CommitOffset / FetchOffset ────────────────────────────────────────────────

func TestCommitAndFetchOffset(t *testing.T) {
	gc := NewGroupCoordinator()

	if err := gc.CommitOffset("g", "events", 0, 42); err != nil {
		t.Fatalf("CommitOffset: %v", err)
	}

	p, err := gc.FetchOffset("g", "events", 0)
	if err != nil {
		t.Fatalf("FetchOffset: %v", err)
	}
	if p == nil {
		t.Fatal("FetchOffset returned nil after commit")
	}
	if *p != 42 {
		t.Errorf("FetchOffset = %d, want 42", *p)
	}
}

func TestFetchOffsetBeforeCommitIsNil(t *testing.T) {
	gc := NewGroupCoordinator()

	p, err := gc.FetchOffset("g", "events", 0)
	if err != nil {
		t.Fatalf("FetchOffset before commit: %v", err)
	}
	if p != nil {
		t.Errorf("FetchOffset before commit = %v, want nil", p)
	}
}

func TestCommitOffsetOverwrite(t *testing.T) {
	gc := NewGroupCoordinator()

	gc.CommitOffset("g", "t", 0, 10) //nolint:errcheck
	gc.CommitOffset("g", "t", 0, 20) //nolint:errcheck

	p, _ := gc.FetchOffset("g", "t", 0)
	if p == nil || *p != 20 {
		t.Errorf("FetchOffset after two commits = %v, want 20", p)
	}
}

func TestCommitOffsetDifferentGroupsIsolated(t *testing.T) {
	gc := NewGroupCoordinator()

	gc.CommitOffset("group-a", "t", 0, 100) //nolint:errcheck
	gc.CommitOffset("group-b", "t", 0, 200) //nolint:errcheck

	pa, _ := gc.FetchOffset("group-a", "t", 0)
	pb, _ := gc.FetchOffset("group-b", "t", 0)

	if pa == nil || *pa != 100 {
		t.Errorf("group-a offset = %v, want 100", pa)
	}
	if pb == nil || *pb != 200 {
		t.Errorf("group-b offset = %v, want 200", pb)
	}
}

func TestFetchOffsetUnknownPartitionNil(t *testing.T) {
	gc := NewGroupCoordinator()

	// Commit to partition 0, then fetch partition 1 — should be nil (not error).
	gc.CommitOffset("g", "t", 0, 5) //nolint:errcheck

	p, err := gc.FetchOffset("g", "t", 1)
	if err != nil {
		t.Fatalf("FetchOffset(partition=1): %v", err)
	}
	if p != nil {
		t.Errorf("FetchOffset for uncommitted partition = %v, want nil", p)
	}
}

// ── Multiple topics in a group ────────────────────────────────────────────────

func TestJoinMultipleTopics(t *testing.T) {
	dir := t.TempDir()
	reg, err := broker.OpenRegistry(dir)
	if err != nil {
		t.Fatal(err)
	}
	reg.CreateTopic("alpha", 2) //nolint:errcheck
	reg.CreateTopic("beta", 2)  //nolint:errcheck
	s := concurrent.NewSharedRegistry(reg, time.Hour)
	defer closeReg(t, s)

	gc := NewGroupCoordinator()
	// Single member subscribing to both topics (4 total partitions).
	_, assignments, err := gc.Join("g", []string{"alpha", "beta"}, s)
	if err != nil {
		t.Fatalf("Join multi-topic: %v", err)
	}
	// alpha/0, alpha/1, beta/0, beta/1 → 4 pairs, all assigned to the sole member.
	if len(assignments) != 4 {
		t.Fatalf("assignments = %v (%d entries), want 4", assignments, len(assignments))
	}
}
