use std::sync::Arc;
use std::time::Duration;

use mini_mq::broker::Registry;
use mini_mq::concurrent::SharedRegistry;
use mini_mq::groups::GroupCoordinator;
use tempfile::TempDir;

fn make_registry(dir: &TempDir) -> Arc<SharedRegistry> {
    Arc::new(SharedRegistry::new(
        Registry::open(dir.path()).unwrap(),
        Duration::from_millis(100),
    ))
}

fn setup(dir: &TempDir, topics: &[(&str, u32)]) -> Arc<SharedRegistry> {
    let reg = make_registry(dir);
    for (name, parts) in topics {
        reg.create_topic(name, *parts).unwrap();
    }
    reg
}

// ── join ─────────────────────────────────────────────────────────────────────

#[test]
fn join_assigns_all_partitions_to_sole_member() {
    let dir = TempDir::new().unwrap();
    let reg = setup(&dir, &[("events", 3)]);
    let gc = GroupCoordinator::new();

    let (member_id, assignment) = gc.join("g1", vec!["events".into()], &reg).unwrap();
    assert_eq!(member_id, "member-0");
    let mut assignment = assignment;
    assignment.sort();
    assert_eq!(
        assignment,
        vec![
            ("events".to_string(), 0),
            ("events".to_string(), 1),
            ("events".to_string(), 2),
        ]
    );
}

#[test]
fn join_two_members_split_partitions_round_robin() {
    let dir = TempDir::new().unwrap();
    let reg = setup(&dir, &[("t", 4)]);
    let gc = GroupCoordinator::new();

    let (m0, _) = gc.join("g", vec!["t".into()], &reg).unwrap();
    let (m1, _) = gc.join("g", vec!["t".into()], &reg).unwrap();

    // Fetch current assignments after the rebalance triggered by the second join.
    let a0 = gc.assignment("g", &m0).unwrap();
    let a1 = gc.assignment("g", &m1).unwrap();

    // Combined must cover all 4 partitions exactly once.
    let mut all: Vec<(String, u32)> = a0.into_iter().chain(a1).collect();
    all.sort();
    assert_eq!(
        all,
        vec![
            ("t".to_string(), 0),
            ("t".to_string(), 1),
            ("t".to_string(), 2),
            ("t".to_string(), 3),
        ]
    );

    // Sorted member IDs should be member-0 and member-1.
    let mut members = gc.members("g");
    members.sort();
    assert_eq!(members, vec![m0, m1]);
}

#[test]
fn join_unknown_topic_assigns_empty() {
    let dir = TempDir::new().unwrap();
    let reg = setup(&dir, &[]);
    let gc = GroupCoordinator::new();

    // Topic doesn't exist — num_partitions returns 0.
    let (_member_id, assignment) = gc.join("g", vec!["ghost".into()], &reg).unwrap();
    assert!(assignment.is_empty());
}

#[test]
fn join_multiple_topics_distributes_all() {
    let dir = TempDir::new().unwrap();
    let reg = setup(&dir, &[("a", 2), ("b", 2)]);
    let gc = GroupCoordinator::new();

    let (_m, assignment) = gc.join("g", vec!["a".into(), "b".into()], &reg).unwrap();
    let mut assignment = assignment;
    assignment.sort();
    // All 4 (topic,partition) pairs go to the sole member.
    assert_eq!(assignment.len(), 4);
    assert!(assignment.contains(&("a".to_string(), 0)));
    assert!(assignment.contains(&("a".to_string(), 1)));
    assert!(assignment.contains(&("b".to_string(), 0)));
    assert!(assignment.contains(&("b".to_string(), 1)));
}

// ── leave ─────────────────────────────────────────────────────────────────────

#[test]
fn leave_reassigns_partitions_to_remaining_member() {
    let dir = TempDir::new().unwrap();
    let reg = setup(&dir, &[("t", 4)]);
    let gc = GroupCoordinator::new();

    let (m0, _) = gc.join("g", vec!["t".into()], &reg).unwrap();
    let (m1, _) = gc.join("g", vec!["t".into()], &reg).unwrap();

    // m0 leaves — m1 should get all 4 partitions.
    gc.leave("g", &m0, &reg).unwrap();

    let assignment = gc.assignment("g", &m1).unwrap();
    let mut assignment = assignment;
    assignment.sort();
    assert_eq!(
        assignment,
        vec![
            ("t".to_string(), 0),
            ("t".to_string(), 1),
            ("t".to_string(), 2),
            ("t".to_string(), 3),
        ]
    );
}

#[test]
fn leave_unknown_group_returns_error() {
    let dir = TempDir::new().unwrap();
    let reg = setup(&dir, &[]);
    let gc = GroupCoordinator::new();

    let result = gc.leave("no-such-group", "member-0", &reg);
    assert!(result.is_err());
}

#[test]
fn leave_unknown_member_returns_error() {
    let dir = TempDir::new().unwrap();
    let reg = setup(&dir, &[("t", 1)]);
    let gc = GroupCoordinator::new();

    gc.join("g", vec!["t".into()], &reg).unwrap();
    let result = gc.leave("g", "member-999", &reg);
    assert!(result.is_err());
}

// ── commit / fetch offset ─────────────────────────────────────────────────────

#[test]
fn fetch_offset_returns_none_before_commit() {
    let dir = TempDir::new().unwrap();
    let reg = setup(&dir, &[("t", 1)]);
    let gc = GroupCoordinator::new();

    gc.join("g", vec!["t".into()], &reg).unwrap();
    let offset = gc.fetch_offset("g", "t", 0).unwrap();
    assert_eq!(offset, None);
}

#[test]
fn commit_and_fetch_offset_roundtrip() {
    let dir = TempDir::new().unwrap();
    let reg = setup(&dir, &[("t", 2)]);
    let gc = GroupCoordinator::new();

    gc.join("g", vec!["t".into()], &reg).unwrap();
    gc.commit_offset("g", "t", 0, 42).unwrap();
    gc.commit_offset("g", "t", 1, 7).unwrap();

    assert_eq!(gc.fetch_offset("g", "t", 0).unwrap(), Some(42));
    assert_eq!(gc.fetch_offset("g", "t", 1).unwrap(), Some(7));
}

#[test]
fn commit_offset_updates_existing() {
    let dir = TempDir::new().unwrap();
    let reg = setup(&dir, &[("t", 1)]);
    let gc = GroupCoordinator::new();

    gc.join("g", vec!["t".into()], &reg).unwrap();
    gc.commit_offset("g", "t", 0, 10).unwrap();
    gc.commit_offset("g", "t", 0, 20).unwrap();

    assert_eq!(gc.fetch_offset("g", "t", 0).unwrap(), Some(20));
}

// ── group / member listing ────────────────────────────────────────────────────

#[test]
fn group_names_returns_sorted_names() {
    let dir = TempDir::new().unwrap();
    let reg = setup(&dir, &[("t", 1)]);
    let gc = GroupCoordinator::new();

    gc.join("zebra", vec!["t".into()], &reg).unwrap();
    gc.join("alpha", vec!["t".into()], &reg).unwrap();

    let names = gc.group_names();
    assert_eq!(names, vec!["alpha", "zebra"]);
}

#[test]
fn members_returns_sorted_ids() {
    let dir = TempDir::new().unwrap();
    let reg = setup(&dir, &[("t", 2)]);
    let gc = GroupCoordinator::new();

    let (m0, _) = gc.join("g", vec!["t".into()], &reg).unwrap();
    let (m1, _) = gc.join("g", vec!["t".into()], &reg).unwrap();
    let (m2, _) = gc.join("g", vec!["t".into()], &reg).unwrap();

    let members = gc.members("g");
    assert_eq!(members, vec![m0, m1, m2]);
}

#[test]
fn members_returns_empty_for_unknown_group() {
    let gc = GroupCoordinator::new();
    assert!(gc.members("no-such-group").is_empty());
}

// ── end-to-end via process_request ───────────────────────────────────────────

#[test]
fn process_request_join_group() {
    use mini_mq::protocol::{AssignedPartition, Request, Response};
    use mini_mq::server::{process_request, BrokerHandle};

    let dir = TempDir::new().unwrap();
    let reg = make_registry(&dir);
    reg.create_topic("events", 2).unwrap();

    let handle = BrokerHandle::new(Arc::clone(&reg));

    let resp = process_request(
        &handle,
        Request::JoinGroup { group: "g".into(), topics: vec!["events".into()] },
    );
    assert_eq!(resp.len(), 1);
    if let Response::Joined { group, member_id, assignments } = &resp[0] {
        assert_eq!(group, "g");
        assert_eq!(member_id, "member-0");
        // Sole member gets both partitions.
        let mut a = assignments.to_vec();
        a.sort();
        assert_eq!(
            a,
            vec![
                AssignedPartition { topic: "events".into(), partition: 0 },
                AssignedPartition { topic: "events".into(), partition: 1 },
            ]
        );
    } else {
        panic!("expected Joined, got {:?}", resp[0]);
    }
}

#[test]
fn process_request_leave_group() {
    use mini_mq::protocol::{Request, Response};
    use mini_mq::server::{process_request, BrokerHandle};

    let dir = TempDir::new().unwrap();
    let reg = make_registry(&dir);
    reg.create_topic("t", 1).unwrap();

    let handle = BrokerHandle::new(Arc::clone(&reg));

    let join_resp = process_request(
        &handle,
        Request::JoinGroup { group: "g".into(), topics: vec!["t".into()] },
    );
    let member_id = if let Response::Joined { member_id, .. } = &join_resp[0] {
        member_id.clone()
    } else {
        panic!("expected Joined");
    };

    let leave_resp = process_request(
        &handle,
        Request::LeaveGroup { group: "g".into(), member_id: member_id.clone() },
    );
    assert_eq!(
        leave_resp,
        vec![Response::LeftGroup { group: "g".into(), member_id }]
    );
}

#[test]
fn process_request_commit_and_fetch_offset() {
    use mini_mq::protocol::{Request, Response};
    use mini_mq::server::{process_request, BrokerHandle};

    let dir = TempDir::new().unwrap();
    let reg = make_registry(&dir);
    let handle = BrokerHandle::new(Arc::clone(&reg));

    // fetch_offset before any commit → null in JSON → offset: None
    let resp = process_request(
        &handle,
        Request::FetchOffset { group: "g".into(), topic: "t".into(), partition: 0 },
    );
    assert_eq!(
        resp,
        vec![Response::CommittedOffset {
            group: "g".into(),
            topic: "t".into(),
            partition: 0,
            offset: None,
        }]
    );

    // commit
    let resp = process_request(
        &handle,
        Request::CommitOffset {
            group: "g".into(),
            topic: "t".into(),
            partition: 0,
            offset: 99,
        },
    );
    assert_eq!(
        resp,
        vec![Response::OffsetCommitted {
            group: "g".into(),
            topic: "t".into(),
            partition: 0,
            offset: 99,
        }]
    );

    // fetch after commit
    let resp = process_request(
        &handle,
        Request::FetchOffset { group: "g".into(), topic: "t".into(), partition: 0 },
    );
    assert_eq!(
        resp,
        vec![Response::CommittedOffset {
            group: "g".into(),
            topic: "t".into(),
            partition: 0,
            offset: Some(99),
        }]
    );
}
