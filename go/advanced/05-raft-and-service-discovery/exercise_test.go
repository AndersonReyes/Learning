package miniraft

import (
	"reflect"
	"testing"
)

// newTestNode returns a RaftNode with the given id, term, vote, role, and
// commit index, and a log consisting of the index-0 sentinel followed by
// entries.
func newTestNode(id int, currentTerm uint64, votedFor int, state State, entries []LogEntry, commitIndex int) *RaftNode {
	return &RaftNode{
		id:          id,
		state:       state,
		currentTerm: currentTerm,
		votedFor:    votedFor,
		log:         append([]LogEntry{{}}, entries...),
		commitIndex: commitIndex,
	}
}

// TestNewRaftNode checks that NewRaftNode returns a node in the initial
// state described by the Raft paper: a Follower in term 0, having voted for
// no one, with a log containing only the index-0 sentinel, and nothing
// committed.
func TestNewRaftNode(t *testing.T) {
	rn := NewRaftNode(1, []int{2, 3})

	if rn.id != 1 {
		t.Errorf("id = %d, want 1", rn.id)
	}
	if !reflect.DeepEqual(rn.peers, []int{2, 3}) {
		t.Errorf("peers = %v, want [2 3]", rn.peers)
	}
	if rn.state != Follower {
		t.Errorf("state = %v, want Follower", rn.state)
	}
	if rn.currentTerm != 0 {
		t.Errorf("currentTerm = %d, want 0", rn.currentTerm)
	}
	if rn.votedFor != -1 {
		t.Errorf("votedFor = %d, want -1", rn.votedFor)
	}
	if want := []LogEntry{{}}; !reflect.DeepEqual(rn.log, want) {
		t.Errorf("log = %v, want %v", rn.log, want)
	}
	if rn.commitIndex != 0 {
		t.Errorf("commitIndex = %d, want 0", rn.commitIndex)
	}
}

// TestRequestVote checks the RequestVote RPC handler (Raft paper Figure 2,
// §5.2, §5.4.1) against the term, vote, and log-comparison rules that decide
// whether a vote is granted.
func TestRequestVote(t *testing.T) {
	tests := []struct {
		name string

		currentTerm uint64
		votedFor    int
		state       State
		log         []LogEntry

		args RequestVoteArgs

		wantReply RequestVoteReply
		wantTerm  uint64
		wantVoted int
		wantState State
	}{
		{
			name:        "stale term rejected",
			currentTerm: 5, votedFor: -1, state: Follower,
			args:      RequestVoteArgs{Term: 3, CandidateID: 2, LastLogIndex: 0, LastLogTerm: 0},
			wantReply: RequestVoteReply{Term: 5, VoteGranted: false},
			wantTerm:  5, wantVoted: -1, wantState: Follower,
		},
		{
			name:        "higher term grants vote and reverts to follower",
			currentTerm: 2, votedFor: -1, state: Candidate,
			args:      RequestVoteArgs{Term: 3, CandidateID: 2, LastLogIndex: 0, LastLogTerm: 0},
			wantReply: RequestVoteReply{Term: 3, VoteGranted: true},
			wantTerm:  3, wantVoted: 2, wantState: Follower,
		},
		{
			name:        "already voted for different candidate in same term",
			currentTerm: 3, votedFor: 2, state: Follower,
			args:      RequestVoteArgs{Term: 3, CandidateID: 5, LastLogIndex: 0, LastLogTerm: 0},
			wantReply: RequestVoteReply{Term: 3, VoteGranted: false},
			wantTerm:  3, wantVoted: 2, wantState: Follower,
		},
		{
			name:        "retransmitted vote request from same candidate",
			currentTerm: 3, votedFor: 5, state: Follower,
			args:      RequestVoteArgs{Term: 3, CandidateID: 5, LastLogIndex: 0, LastLogTerm: 0},
			wantReply: RequestVoteReply{Term: 3, VoteGranted: true},
			wantTerm:  3, wantVoted: 5, wantState: Follower,
		},
		{
			name:        "candidate log has lower last term",
			currentTerm: 3, votedFor: -1, state: Follower,
			log:       []LogEntry{{1, "a"}, {1, "b"}, {2, "c"}}, // last index 3, last term 2
			args:      RequestVoteArgs{Term: 3, CandidateID: 2, LastLogIndex: 5, LastLogTerm: 1},
			wantReply: RequestVoteReply{Term: 3, VoteGranted: false},
			wantTerm:  3, wantVoted: -1, wantState: Follower,
		},
		{
			name:        "candidate log has same last term but is shorter",
			currentTerm: 3, votedFor: -1, state: Follower,
			log:       []LogEntry{{1, "a"}, {1, "b"}, {2, "c"}}, // last index 3, last term 2
			args:      RequestVoteArgs{Term: 3, CandidateID: 2, LastLogIndex: 2, LastLogTerm: 2},
			wantReply: RequestVoteReply{Term: 3, VoteGranted: false},
			wantTerm:  3, wantVoted: -1, wantState: Follower,
		},
		{
			name:        "candidate log has same last term and is at least as long",
			currentTerm: 3, votedFor: -1, state: Follower,
			log:       []LogEntry{{1, "a"}, {1, "b"}, {2, "c"}}, // last index 3, last term 2
			args:      RequestVoteArgs{Term: 3, CandidateID: 2, LastLogIndex: 3, LastLogTerm: 2},
			wantReply: RequestVoteReply{Term: 3, VoteGranted: true},
			wantTerm:  3, wantVoted: 2, wantState: Follower,
		},
		{
			name:        "candidate log has higher last term despite being shorter",
			currentTerm: 3, votedFor: -1, state: Follower,
			log:       []LogEntry{{1, "a"}, {1, "b"}, {2, "c"}}, // last index 3, last term 2
			args:      RequestVoteArgs{Term: 4, CandidateID: 9, LastLogIndex: 1, LastLogTerm: 3},
			wantReply: RequestVoteReply{Term: 4, VoteGranted: true},
			wantTerm:  4, wantVoted: 9, wantState: Follower,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			rn := newTestNode(1, tt.currentTerm, tt.votedFor, tt.state, tt.log, 0)

			var reply RequestVoteReply
			if err := rn.RequestVote(&tt.args, &reply); err != nil {
				t.Fatalf("RequestVote() error = %v", err)
			}
			if reply != tt.wantReply {
				t.Errorf("RequestVote() reply = %+v, want %+v", reply, tt.wantReply)
			}
			if rn.currentTerm != tt.wantTerm {
				t.Errorf("currentTerm = %d, want %d", rn.currentTerm, tt.wantTerm)
			}
			if rn.votedFor != tt.wantVoted {
				t.Errorf("votedFor = %d, want %d", rn.votedFor, tt.wantVoted)
			}
			if rn.state != tt.wantState {
				t.Errorf("state = %v, want %v", rn.state, tt.wantState)
			}
		})
	}
}

// TestAppendEntries checks the AppendEntries RPC handler (Raft paper Figure
// 2, §5.3) against the term rules, the log-consistency check on
// PrevLogIndex/PrevLogTerm, conflict resolution by truncation, and the
// commitIndex advancement rule.
func TestAppendEntries(t *testing.T) {
	tests := []struct {
		name string

		currentTerm uint64
		votedFor    int
		state       State
		log         []LogEntry
		commitIndex int

		args AppendEntriesArgs

		wantReply       AppendEntriesReply
		wantTerm        uint64
		wantVoted       int
		wantState       State
		wantLog         []LogEntry
		wantCommitIndex int
	}{
		{
			name:        "stale term rejected",
			currentTerm: 5, votedFor: -1, state: Leader,
			log:         []LogEntry{{1, "a"}, {1, "b"}},
			commitIndex: 0,
			args:        AppendEntriesArgs{Term: 3, LeaderID: 9, PrevLogIndex: 2, PrevLogTerm: 1, LeaderCommit: 0},
			wantReply:   AppendEntriesReply{Term: 5, Success: false},
			wantTerm:    5, wantVoted: -1, wantState: Leader,
			wantLog: []LogEntry{{1, "a"}, {1, "b"}}, wantCommitIndex: 0,
		},
		{
			name:        "heartbeat from current leader advances commit index",
			currentTerm: 3, votedFor: -1, state: Candidate,
			log:         []LogEntry{{1, "a"}, {2, "b"}},
			commitIndex: 0,
			args:        AppendEntriesArgs{Term: 3, LeaderID: 2, PrevLogIndex: 2, PrevLogTerm: 2, LeaderCommit: 2},
			wantReply:   AppendEntriesReply{Term: 3, Success: true},
			wantTerm:    3, wantVoted: -1, wantState: Follower,
			wantLog: []LogEntry{{1, "a"}, {2, "b"}}, wantCommitIndex: 2,
		},
		{
			name:        "higher term updates term, reverts to follower, clears vote",
			currentTerm: 2, votedFor: 7, state: Candidate,
			commitIndex: 0,
			args:        AppendEntriesArgs{Term: 5, LeaderID: 3, PrevLogIndex: 0, PrevLogTerm: 0, Entries: []LogEntry{{5, "x"}}, LeaderCommit: 0},
			wantReply:   AppendEntriesReply{Term: 5, Success: true},
			wantTerm:    5, wantVoted: -1, wantState: Follower,
			wantLog: []LogEntry{{5, "x"}}, wantCommitIndex: 0,
		},
		{
			name:        "PrevLogIndex beyond end of log rejected",
			currentTerm: 3, votedFor: -1, state: Candidate,
			log:         []LogEntry{{1, "a"}},
			commitIndex: 0,
			args:        AppendEntriesArgs{Term: 3, LeaderID: 2, PrevLogIndex: 2, PrevLogTerm: 1, Entries: []LogEntry{{1, "y"}}, LeaderCommit: 0},
			wantReply:   AppendEntriesReply{Term: 3, Success: false},
			wantTerm:    3, wantVoted: -1, wantState: Follower,
			wantLog: []LogEntry{{1, "a"}}, wantCommitIndex: 0,
		},
		{
			name:        "PrevLogTerm mismatch rejected",
			currentTerm: 3, votedFor: -1, state: Follower,
			log:         []LogEntry{{1, "a"}, {2, "b"}},
			commitIndex: 0,
			args:        AppendEntriesArgs{Term: 3, LeaderID: 2, PrevLogIndex: 2, PrevLogTerm: 1, LeaderCommit: 0},
			wantReply:   AppendEntriesReply{Term: 3, Success: false},
			wantTerm:    3, wantVoted: -1, wantState: Follower,
			wantLog: []LogEntry{{1, "a"}, {2, "b"}}, wantCommitIndex: 0,
		},
		{
			name:        "appends new entries to empty log",
			currentTerm: 1, votedFor: -1, state: Follower,
			commitIndex: 0,
			args:        AppendEntriesArgs{Term: 1, LeaderID: 2, PrevLogIndex: 0, PrevLogTerm: 0, Entries: []LogEntry{{1, "a"}, {1, "b"}}, LeaderCommit: 0},
			wantReply:   AppendEntriesReply{Term: 1, Success: true},
			wantTerm:    1, wantVoted: -1, wantState: Follower,
			wantLog: []LogEntry{{1, "a"}, {1, "b"}}, wantCommitIndex: 0,
		},
		{
			name:        "truncates conflicting entries and advances commit index",
			currentTerm: 2, votedFor: -1, state: Follower,
			log:         []LogEntry{{1, "a"}, {2, "stale"}},
			commitIndex: 0,
			args:        AppendEntriesArgs{Term: 2, LeaderID: 2, PrevLogIndex: 1, PrevLogTerm: 1, Entries: []LogEntry{{1, "c"}}, LeaderCommit: 2},
			wantReply:   AppendEntriesReply{Term: 2, Success: true},
			wantTerm:    2, wantVoted: -1, wantState: Follower,
			wantLog: []LogEntry{{1, "a"}, {1, "c"}}, wantCommitIndex: 2,
		},
		{
			name:        "idempotent retransmission leaves matching entries untouched",
			currentTerm: 2, votedFor: -1, state: Follower,
			log:         []LogEntry{{1, "a"}, {1, "c"}},
			commitIndex: 1,
			args:        AppendEntriesArgs{Term: 2, LeaderID: 2, PrevLogIndex: 1, PrevLogTerm: 1, Entries: []LogEntry{{1, "c"}}, LeaderCommit: 2},
			wantReply:   AppendEntriesReply{Term: 2, Success: true},
			wantTerm:    2, wantVoted: -1, wantState: Follower,
			wantLog: []LogEntry{{1, "a"}, {1, "c"}}, wantCommitIndex: 2,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			rn := newTestNode(1, tt.currentTerm, tt.votedFor, tt.state, tt.log, tt.commitIndex)

			var reply AppendEntriesReply
			if err := rn.AppendEntries(&tt.args, &reply); err != nil {
				t.Fatalf("AppendEntries() error = %v", err)
			}
			if reply != tt.wantReply {
				t.Errorf("AppendEntries() reply = %+v, want %+v", reply, tt.wantReply)
			}
			if rn.currentTerm != tt.wantTerm {
				t.Errorf("currentTerm = %d, want %d", rn.currentTerm, tt.wantTerm)
			}
			if rn.votedFor != tt.wantVoted {
				t.Errorf("votedFor = %d, want %d", rn.votedFor, tt.wantVoted)
			}
			if rn.state != tt.wantState {
				t.Errorf("state = %v, want %v", rn.state, tt.wantState)
			}
			wantLog := append([]LogEntry{{}}, tt.wantLog...)
			if !reflect.DeepEqual(rn.log, wantLog) {
				t.Errorf("log = %v, want %v", rn.log, wantLog)
			}
			if rn.commitIndex != tt.wantCommitIndex {
				t.Errorf("commitIndex = %d, want %d", rn.commitIndex, tt.wantCommitIndex)
			}
		})
	}
}

// TestBecomeCandidate checks that BecomeCandidate (Raft paper §5.2)
// increments the term, votes for self, transitions to Candidate, and
// returns a RequestVoteArgs describing the new term and the node's log.
func TestBecomeCandidate(t *testing.T) {
	rn := newTestNode(7, 2, -1, Follower, []LogEntry{{1, "a"}, {2, "b"}}, 0) // last index 2, last term 2

	args := rn.BecomeCandidate()

	want := &RequestVoteArgs{Term: 3, CandidateID: 7, LastLogIndex: 2, LastLogTerm: 2}
	if !reflect.DeepEqual(args, want) {
		t.Errorf("BecomeCandidate() = %+v, want %+v", args, want)
	}
	if rn.state != Candidate {
		t.Errorf("state = %v, want Candidate", rn.state)
	}
	if rn.currentTerm != 3 {
		t.Errorf("currentTerm = %d, want 3", rn.currentTerm)
	}
	if rn.votedFor != 7 {
		t.Errorf("votedFor = %d, want 7 (self)", rn.votedFor)
	}
}

// TestPropose checks that Propose only appends to the log when the node is
// the Leader, and reports the new entry's index and term.
func TestPropose(t *testing.T) {
	tests := []struct {
		name  string
		state State

		wantIndex  int
		wantTerm   uint64
		wantLeader bool
		wantLog    []LogEntry
	}{
		{
			name: "not leader", state: Follower,
			wantIndex: -1, wantTerm: 4, wantLeader: false,
			wantLog: []LogEntry{{1, "a"}},
		},
		{
			name: "leader appends entry", state: Leader,
			wantIndex: 2, wantTerm: 4, wantLeader: true,
			wantLog: []LogEntry{{1, "a"}, {4, "y"}},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			rn := newTestNode(1, 4, -1, tt.state, []LogEntry{{1, "a"}}, 0)

			index, term, isLeader := rn.Propose("y")
			if index != tt.wantIndex || term != tt.wantTerm || isLeader != tt.wantLeader {
				t.Errorf("Propose() = (%d, %d, %v), want (%d, %d, %v)", index, term, isLeader, tt.wantIndex, tt.wantTerm, tt.wantLeader)
			}
			wantLog := append([]LogEntry{{}}, tt.wantLog...)
			if !reflect.DeepEqual(rn.log, wantLog) {
				t.Errorf("log = %v, want %v", rn.log, wantLog)
			}
		})
	}
}

// TestElectionAndReplication is a small end-to-end sketch of a Raft term:
// node 1 starts an election, nodes 2 and 3 grant their votes (a majority of
// the 3-node cluster), node 1 becomes leader and proposes a command, and
// nodes 2 and 3 accept it via AppendEntries.
func TestElectionAndReplication(t *testing.T) {
	n1 := NewRaftNode(1, []int{2, 3})
	n2 := NewRaftNode(2, []int{1, 3})
	n3 := NewRaftNode(3, []int{1, 2})

	voteArgs := n1.BecomeCandidate()
	if n1.state != Candidate || n1.currentTerm != 1 {
		t.Fatalf("after BecomeCandidate, (state, currentTerm) = (%v, %d), want (Candidate, 1)", n1.state, n1.currentTerm)
	}

	votes := 1 // n1 votes for itself
	for _, peer := range []*RaftNode{n2, n3} {
		var reply RequestVoteReply
		if err := peer.RequestVote(voteArgs, &reply); err != nil {
			t.Fatalf("node %d RequestVote() error = %v", peer.id, err)
		}
		if !reply.VoteGranted {
			t.Fatalf("node %d did not grant its vote", peer.id)
		}
		votes++
	}
	if votes <= len([]int{1, 2, 3})/2 {
		t.Fatalf("got %d votes, want a majority of 3", votes)
	}
	n1.state = Leader // becoming leader on a majority is the caller's responsibility

	index, term, isLeader := n1.Propose("x=1")
	if !isLeader || index != 1 || term != 1 {
		t.Fatalf("Propose() = (%d, %d, %v), want (1, 1, true)", index, term, isLeader)
	}

	appendArgs := &AppendEntriesArgs{
		Term:         n1.currentTerm,
		LeaderID:     n1.id,
		PrevLogIndex: 0,
		PrevLogTerm:  0,
		Entries:      n1.log[1:],
		LeaderCommit: 0,
	}
	for _, peer := range []*RaftNode{n2, n3} {
		var reply AppendEntriesReply
		if err := peer.AppendEntries(appendArgs, &reply); err != nil {
			t.Fatalf("node %d AppendEntries() error = %v", peer.id, err)
		}
		if !reply.Success {
			t.Fatalf("node %d rejected AppendEntries", peer.id)
		}
		if !reflect.DeepEqual(peer.log, n1.log) {
			t.Errorf("node %d log = %v, want %v", peer.id, peer.log, n1.log)
		}
	}
}
