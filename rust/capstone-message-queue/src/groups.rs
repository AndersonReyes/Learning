//! Consumer group coordinator.
//!
//! Groups track three things:
//!   1. Members — who is in the group and what topics they subscribe to.
//!   2. Assignments — which `(topic, partition)` pairs each member owns.
//!   3. Committed offsets — per-group, per-`(topic, partition)` last-processed offset.
//!
//! ## Assignment algorithm
//!
//! On every membership change (join / leave), the coordinator rebalances:
//!   1. Collect all `(topic, partition)` pairs across all members' subscriptions.
//!   2. Sort them (topic α, then partition ↑) for determinism.
//!   3. Assign round-robin across members sorted by `member_id`.
//!
//! This is intentionally simple — real Kafka uses a leader-elected SyncGroup
//! protocol where the leader proposes the assignment. For Phase 5 the broker
//! assigns directly.

use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::RwLock;

use crate::concurrent::SharedRegistry;
use crate::error::{Error, Result};

// ── internal helpers ──────────────────────────────────────────────────────────

fn lock_err<T>(_: std::sync::PoisonError<T>) -> Error {
    Error::Io(std::io::Error::new(
        std::io::ErrorKind::Other,
        "group coordinator lock poisoned",
    ))
}

// ── GroupState ────────────────────────────────────────────────────────────────

struct GroupState {
    /// member_id → topic names the member subscribes to.
    members: HashMap<String, Vec<String>>,
    /// member_id → assigned `(topic, partition)` pairs after the last rebalance.
    assignments: HashMap<String, Vec<(String, u32)>>,
    /// `(topic, partition)` → last committed offset.
    committed: HashMap<(String, u32), u64>,
}

impl GroupState {
    fn new() -> Self {
        GroupState {
            members: HashMap::new(),
            assignments: HashMap::new(),
            committed: HashMap::new(),
        }
    }

    fn rebalance(&mut self, registry: &SharedRegistry) {
        // All (topic, partition) pairs across every member's subscriptions.
        let topics: HashSet<String> = self
            .members
            .values()
            .flat_map(|ts| ts.iter().cloned())
            .collect();

        let mut all_partitions: Vec<(String, u32)> = topics
            .iter()
            .flat_map(|topic| {
                let n = registry.num_partitions(topic).unwrap_or(0) as u32;
                (0..n).map(move |p| (topic.clone(), p))
            })
            .collect();
        all_partitions.sort();

        // Sorted member list for determinism.
        let mut member_ids: Vec<String> = self.members.keys().cloned().collect();
        member_ids.sort();

        self.assignments.clear();
        for m in &member_ids {
            self.assignments.insert(m.clone(), vec![]);
        }
        if !member_ids.is_empty() {
            for (i, tp) in all_partitions.into_iter().enumerate() {
                let m = &member_ids[i % member_ids.len()];
                self.assignments.get_mut(m).unwrap().push(tp);
            }
        }
    }
}

// ── GroupCoordinator ──────────────────────────────────────────────────────────

/// Thread-safe group coordinator. Wrap in `Arc` and share across tasks.
pub struct GroupCoordinator {
    groups: RwLock<HashMap<String, GroupState>>,
    member_counter: AtomicU64,
}

impl GroupCoordinator {
    pub fn new() -> Self {
        GroupCoordinator {
            groups: RwLock::new(HashMap::new()),
            member_counter: AtomicU64::new(0),
        }
    }

    fn fresh_member_id(&self) -> String {
        let n = self.member_counter.fetch_add(1, Ordering::Relaxed);
        format!("member-{n}")
    }

    /// Adds a member to `group` subscribing to `topics`.
    ///
    /// Triggers a rebalance and returns `(member_id, assignment)`.
    pub fn join(
        &self,
        group: &str,
        topics: Vec<String>,
        registry: &SharedRegistry,
    ) -> Result<(String, Vec<(String, u32)>)> {
        let member_id = self.fresh_member_id();
        let mut groups = self.groups.write().map_err(lock_err)?;
        let state = groups.entry(group.to_owned()).or_insert_with(GroupState::new);
        state.members.insert(member_id.clone(), topics);
        state.rebalance(registry);
        let assignment = state
            .assignments
            .get(&member_id)
            .cloned()
            .unwrap_or_default();
        Ok((member_id, assignment))
    }

    /// Removes `member_id` from `group` and triggers a rebalance.
    pub fn leave(
        &self,
        group: &str,
        member_id: &str,
        registry: &SharedRegistry,
    ) -> Result<()> {
        let mut groups = self.groups.write().map_err(lock_err)?;
        let state = groups
            .get_mut(group)
            .ok_or_else(|| Error::TopicNotFound(format!("group '{group}' not found")))?;
        if state.members.remove(member_id).is_none() {
            return Err(Error::TopicNotFound(format!(
                "member '{member_id}' not in group '{group}'"
            )));
        }
        state.assignments.remove(member_id);
        state.rebalance(registry);
        Ok(())
    }

    /// Returns the current `(topic, partition)` assignment for `member_id`.
    pub fn assignment(&self, group: &str, member_id: &str) -> Result<Vec<(String, u32)>> {
        let groups = self.groups.read().map_err(lock_err)?;
        let state = groups
            .get(group)
            .ok_or_else(|| Error::TopicNotFound(format!("group '{group}' not found")))?;
        state
            .assignments
            .get(member_id)
            .cloned()
            .ok_or_else(|| Error::TopicNotFound(format!("member '{member_id}' not found")))
    }

    /// Records that `group` has processed up to `offset` on `topic`/`partition`.
    pub fn commit_offset(
        &self,
        group: &str,
        topic: &str,
        partition: u32,
        offset: u64,
    ) -> Result<()> {
        let mut groups = self.groups.write().map_err(lock_err)?;
        let state = groups
            .entry(group.to_owned())
            .or_insert_with(GroupState::new);
        state
            .committed
            .insert((topic.to_owned(), partition), offset);
        Ok(())
    }

    /// Returns the committed offset for `group`/`topic`/`partition`, or `None`
    /// if none has been committed yet.
    pub fn fetch_offset(
        &self,
        group: &str,
        topic: &str,
        partition: u32,
    ) -> Result<Option<u64>> {
        let groups = self.groups.read().map_err(lock_err)?;
        Ok(groups
            .get(group)
            .and_then(|s| s.committed.get(&(topic.to_owned(), partition)))
            .copied())
    }

    /// Returns sorted member IDs in `group`, or empty if the group doesn't exist.
    pub fn members(&self, group: &str) -> Vec<String> {
        let groups = self.groups.read().unwrap_or_else(|e| e.into_inner());
        let mut ids: Vec<String> = groups
            .get(group)
            .map(|s| s.members.keys().cloned().collect())
            .unwrap_or_default();
        ids.sort();
        ids
    }

    /// Returns sorted names of all groups that have ever had activity.
    pub fn group_names(&self) -> Vec<String> {
        let groups = self.groups.read().unwrap_or_else(|e| e.into_inner());
        let mut names: Vec<String> = groups.keys().cloned().collect();
        names.sort();
        names
    }
}

impl Default for GroupCoordinator {
    fn default() -> Self {
        Self::new()
    }
}
