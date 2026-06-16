// Package groups implements the consumer group coordinator.
//
// Groups track three things:
//  1. Members — who is in the group and what topics they subscribe to.
//  2. Assignments — which (topic, partition) pairs each member owns.
//  3. Committed offsets — per-group, per-(topic, partition) last-processed offset.
//
// # Assignment algorithm
//
// On every membership change (join / leave), the coordinator rebalances:
//  1. Collect all (topic, partition) pairs across all members' subscriptions.
//  2. Sort them (topic alpha, then partition ascending) for determinism.
//  3. Assign round-robin across members sorted by member_id.
//
// This is intentionally simple — real Kafka uses a leader-elected SyncGroup
// protocol. For Phase 5 the broker assigns directly.
package groups

import (
	"fmt"
	"sort"
	"sync"
	"sync/atomic"

	"github.com/andersonreyes/learning/go/capstone-message-queue/concurrent"
	"github.com/andersonreyes/learning/go/capstone-message-queue/protocol"
)

// ── errors ────────────────────────────────────────────────────────────────────

// ErrNotFound is returned when a group or member is not found.
type ErrNotFound struct{ Msg string }

func (e *ErrNotFound) Error() string { return e.Msg }

// ── topicPartitionKey is a (topic, partition) pair used as a map key. ─────────

type topicPartitionKey struct {
	topic     string
	partition uint32
}

// ── groupState holds state for a single consumer group. ───────────────────────

type groupState struct {
	// member_id → topic names the member subscribes to.
	members map[string][]string
	// member_id → assigned (topic, partition) pairs after the last rebalance.
	assignments map[string][]protocol.AssignedPartition
	// (topic, partition) → last committed offset.
	committed map[topicPartitionKey]uint64
}

func newGroupState() *groupState {
	return &groupState{
		members:     make(map[string][]string),
		assignments: make(map[string][]protocol.AssignedPartition),
		committed:   make(map[topicPartitionKey]uint64),
	}
}

func (gs *groupState) rebalance(reg *concurrent.SharedRegistry) {
	// Collect all unique topics across all members.
	topicSet := make(map[string]struct{})
	for _, topics := range gs.members {
		for _, t := range topics {
			topicSet[t] = struct{}{}
		}
	}

	// Build all (topic, partition) pairs.
	var allPartitions []protocol.AssignedPartition
	for topic := range topicSet {
		n, err := reg.NumPartitions(topic)
		if err != nil {
			n = 0
		}
		for p := uint32(0); p < uint32(n); p++ {
			allPartitions = append(allPartitions, protocol.AssignedPartition{Topic: topic, Partition: p})
		}
	}

	// Sort for determinism: topic alpha, then partition ascending.
	sort.Slice(allPartitions, func(i, j int) bool {
		if allPartitions[i].Topic != allPartitions[j].Topic {
			return allPartitions[i].Topic < allPartitions[j].Topic
		}
		return allPartitions[i].Partition < allPartitions[j].Partition
	})

	// Sorted member IDs for determinism.
	memberIDs := make([]string, 0, len(gs.members))
	for id := range gs.members {
		memberIDs = append(memberIDs, id)
	}
	sort.Strings(memberIDs)

	// Reset assignments.
	gs.assignments = make(map[string][]protocol.AssignedPartition)
	for _, id := range memberIDs {
		gs.assignments[id] = nil
	}

	if len(memberIDs) > 0 {
		for i, ap := range allPartitions {
			m := memberIDs[i%len(memberIDs)]
			gs.assignments[m] = append(gs.assignments[m], ap)
		}
	}
}

// ── GroupCoordinator ──────────────────────────────────────────────────────────

// GroupCoordinator is the thread-safe group coordinator.
type GroupCoordinator struct {
	mu            sync.RWMutex
	groups        map[string]*groupState
	memberCounter atomic.Uint64
}

// New returns a new GroupCoordinator.
func New() *GroupCoordinator {
	return &GroupCoordinator{
		groups: make(map[string]*groupState),
	}
}

func (gc *GroupCoordinator) freshMemberID() string {
	n := gc.memberCounter.Add(1) - 1
	return fmt.Sprintf("member-%d", n)
}

// Join adds a member to group subscribing to topics.
// Triggers a rebalance and returns (memberID, assignments).
func (gc *GroupCoordinator) Join(group string, topics []string, reg *concurrent.SharedRegistry) (string, []protocol.AssignedPartition, error) {
	memberID := gc.freshMemberID()

	gc.mu.Lock()
	defer gc.mu.Unlock()

	gs, ok := gc.groups[group]
	if !ok {
		gs = newGroupState()
		gc.groups[group] = gs
	}
	gs.members[memberID] = topics
	gs.rebalance(reg)

	assignment := gs.assignments[memberID]
	// Return a copy.
	result := make([]protocol.AssignedPartition, len(assignment))
	copy(result, assignment)
	return memberID, result, nil
}

// Leave removes memberID from group and triggers a rebalance.
func (gc *GroupCoordinator) Leave(group, memberID string, reg *concurrent.SharedRegistry) error {
	gc.mu.Lock()
	defer gc.mu.Unlock()

	gs, ok := gc.groups[group]
	if !ok {
		return &ErrNotFound{Msg: fmt.Sprintf("group '%s' not found", group)}
	}
	if _, ok := gs.members[memberID]; !ok {
		return &ErrNotFound{Msg: fmt.Sprintf("member '%s' not in group '%s'", memberID, group)}
	}
	delete(gs.members, memberID)
	delete(gs.assignments, memberID)
	gs.rebalance(reg)
	return nil
}

// Assignment returns the current (topic, partition) assignment for memberID.
func (gc *GroupCoordinator) Assignment(group, memberID string) ([]protocol.AssignedPartition, error) {
	gc.mu.RLock()
	defer gc.mu.RUnlock()

	gs, ok := gc.groups[group]
	if !ok {
		return nil, &ErrNotFound{Msg: fmt.Sprintf("group '%s' not found", group)}
	}
	a, ok := gs.assignments[memberID]
	if !ok {
		return nil, &ErrNotFound{Msg: fmt.Sprintf("member '%s' not found", memberID)}
	}
	result := make([]protocol.AssignedPartition, len(a))
	copy(result, a)
	return result, nil
}

// CommitOffset records that group has processed up to offset on topic/partition.
func (gc *GroupCoordinator) CommitOffset(group, topic string, partition uint32, offset uint64) error {
	gc.mu.Lock()
	defer gc.mu.Unlock()

	gs, ok := gc.groups[group]
	if !ok {
		gs = newGroupState()
		gc.groups[group] = gs
	}
	gs.committed[topicPartitionKey{topic, partition}] = offset
	return nil
}

// FetchOffset returns the committed offset for group/topic/partition.
// Returns (nil, nil) if none has been committed yet.
func (gc *GroupCoordinator) FetchOffset(group, topic string, partition uint32) (*uint64, error) {
	gc.mu.RLock()
	defer gc.mu.RUnlock()

	gs, ok := gc.groups[group]
	if !ok {
		return nil, nil
	}
	off, ok := gs.committed[topicPartitionKey{topic, partition}]
	if !ok {
		return nil, nil
	}
	v := off
	return &v, nil
}

// Members returns sorted member IDs in group, or empty if the group doesn't exist.
func (gc *GroupCoordinator) Members(group string) []string {
	gc.mu.RLock()
	defer gc.mu.RUnlock()

	gs, ok := gc.groups[group]
	if !ok {
		return nil
	}
	ids := make([]string, 0, len(gs.members))
	for id := range gs.members {
		ids = append(ids, id)
	}
	sort.Strings(ids)
	return ids
}

// GroupNames returns sorted names of all groups that have ever had activity.
func (gc *GroupCoordinator) GroupNames() []string {
	gc.mu.RLock()
	defer gc.mu.RUnlock()

	names := make([]string, 0, len(gc.groups))
	for name := range gc.groups {
		names = append(names, name)
	}
	sort.Strings(names)
	return names
}
