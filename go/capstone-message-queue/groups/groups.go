// Package groups implements consumer group coordination.
//
// Rebalance algorithm (triggered on Join and Leave):
//  1. Collect all (topic, partition) pairs across all members' subscriptions,
//     using reg.NumPartitions to enumerate partitions per topic.
//  2. Sort pairs: topic alphabetically, then partition index ascending.
//  3. Sort member IDs alphabetically.
//  4. Assign round-robin: pair[i] → member[i % len(members)].
//
// Member IDs are server-assigned: "member-0", "member-1", … based on join order.
package groups

import (
	"errors"

	"github.com/andersonreyes/learning/go/capstone-message-queue/concurrent"
)

// Assignment is a (topic, partition) pair held by a group member.
type Assignment struct {
	Topic     string
	Partition uint32
}

// GroupCoordinator manages consumer groups.
type GroupCoordinator struct {
	// unexported fields
}

// NewGroupCoordinator creates an empty GroupCoordinator.
func NewGroupCoordinator() *GroupCoordinator {
	return &GroupCoordinator{}
}

// Join adds a new member to group, subscribing to topics.
// The server assigns a member ID ("member-0", "member-1", …) in join order.
// Triggers a rebalance so all members' assignments are updated.
// Returns the new member's ID and their assigned partitions after rebalance.
// Returns an error if any topic in topics does not exist in reg.
func (g *GroupCoordinator) Join(group string, topics []string, reg *concurrent.SharedRegistry) (memberID string, assignments []Assignment, err error) {
	return "", nil, errors.New("not implemented")
}

// Leave removes memberID from group and triggers a rebalance.
// If memberID is not a member of group, returns an error.
// If the group becomes empty, it is removed entirely.
func (g *GroupCoordinator) Leave(group, memberID string, reg *concurrent.SharedRegistry) error {
	return errors.New("not implemented")
}

// CommitOffset records that group has consumed up to offset on topic/partition.
// Subsequent FetchOffset calls return this value.
func (g *GroupCoordinator) CommitOffset(group, topic string, partition uint32, offset uint64) error {
	return errors.New("not implemented")
}

// FetchOffset returns the last committed offset for group/topic/partition, or
// nil if no offset has been committed yet.
// Never returns an error (unknown groups/topics/partitions simply return nil).
func (g *GroupCoordinator) FetchOffset(group, topic string, partition uint32) (*uint64, error) {
	return nil, errors.New("not implemented")
}

// Members returns sorted member IDs in group.
// Returns an empty slice (not an error) if the group does not exist.
func (g *GroupCoordinator) Members(group string) []string {
	return nil
}

// GroupNames returns sorted names of all known groups.
func (g *GroupCoordinator) GroupNames() []string {
	return nil
}
