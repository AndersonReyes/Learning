// Package miniraft implements the core decision logic of the Raft
// consensus algorithm (Ongaro & Ousterhout, "In Search of an Understandable
// Consensus Algorithm" — see Figure 2 of the extended paper at
// https://raft.github.io/raft.pdf): the RequestVote and AppendEntries RPC
// handlers, the candidate state transition that starts an election, and the
// leader-side call that appends a new command to the replicated log.
//
// RequestVote and AppendEntries are written as net/rpc-compatible methods
// (exported method, two pointer-shaped arguments, error return) so a real
// cluster can expose a *RaftNode directly via net/rpc — see
// examples/main.go for a two-node election running over a real TCP
// connection, which is how nodes "discover" and call each other in this
// sketch.
package miniraft

import "errors"

// State is a Raft node's role (paper §5.1).
type State int

const (
	Follower State = iota
	Candidate
	Leader
)

// LogEntry is one entry in a Raft node's replicated log (paper §5.3).
type LogEntry struct {
	Term    uint64
	Command string
}

// RequestVoteArgs is the argument to the RequestVote RPC (paper Figure 2).
type RequestVoteArgs struct {
	Term         uint64
	CandidateID  int
	LastLogIndex int
	LastLogTerm  uint64
}

// RequestVoteReply is the reply from the RequestVote RPC (paper Figure 2).
type RequestVoteReply struct {
	Term        uint64
	VoteGranted bool
}

// AppendEntriesArgs is the argument to the AppendEntries RPC (paper Figure
// 2). Entries is empty for a heartbeat.
type AppendEntriesArgs struct {
	Term         uint64
	LeaderID     int
	PrevLogIndex int
	PrevLogTerm  uint64
	Entries      []LogEntry
	LeaderCommit int
}

// AppendEntriesReply is the reply from the AppendEntries RPC (paper Figure
// 2).
type AppendEntriesReply struct {
	Term    uint64
	Success bool
}

// RaftNode is a single node's state (paper Figure 2's "State" box). Timers,
// networking, and the leader's replication loop are out of scope: RaftNode
// implements the RPC handlers and state transitions that decide what a node
// does in response to an event, and a caller (test, or a goroutine driven by
// timers and net/rpc in a real deployment) is responsible for triggering
// those events and delivering the resulting RPCs to peers.
type RaftNode struct {
	id    int
	peers []int

	state       State
	currentTerm uint64
	votedFor    int // -1 if this node has not voted in currentTerm

	// log[0] is a sentinel entry with Term 0, so log indices match the
	// paper's 1-based indexing and PrevLogIndex 0 always matches.
	log []LogEntry

	commitIndex int
}

// NewRaftNode returns a new Raft node with the given id and the ids of its
// peers, initialized as a Follower in term 0 with an empty log (containing
// only the index-0 sentinel) and no vote cast.
func NewRaftNode(id int, peers []int) *RaftNode {
	return &RaftNode{}
}

// RequestVote implements the RequestVote RPC handler (paper Figure 2,
// §5.2, §5.4.1): a candidate calls this on each peer to request its vote.
//
//   - If args.Term > rn.currentTerm, rn updates its term to args.Term,
//     reverts to Follower, and clears any vote already cast in the old term.
//   - If args.Term < rn.currentTerm, RequestVote sets reply.VoteGranted =
//     false (a stale candidate) without granting a vote.
//   - Otherwise, rn grants its vote (reply.VoteGranted = true and rn records
//     votedFor = args.CandidateID) if and only if it has not already voted
//     for a different candidate in this term, AND the candidate's log is at
//     least as up-to-date as rn's own: the candidate's last log entry has a
//     strictly higher term, or the same term and an index >= rn's last log
//     index.
//
// reply.Term is always set to rn.currentTerm (after any update above).
// RequestVote returns a non-nil error only on internal failure; a rejected
// vote is reported via reply.VoteGranted = false with a nil error.
func (rn *RaftNode) RequestVote(args *RequestVoteArgs, reply *RequestVoteReply) error {
	return errors.New("not implemented")
}

// AppendEntries implements the AppendEntries RPC handler (paper Figure 2,
// §5.3): a leader calls this on each follower both to replicate log entries
// and as a heartbeat (with Entries empty).
//
//   - If args.Term > rn.currentTerm, rn updates its term to args.Term and
//     clears any vote cast in the old term.
//   - If args.Term < rn.currentTerm, AppendEntries sets reply.Success =
//     false (a stale leader) and makes no other changes.
//   - Otherwise, rn reverts to Follower (a valid leader exists for this
//     term) and:
//   - If rn's log does not contain an entry at args.PrevLogIndex whose
//     term is args.PrevLogTerm (including if rn's log is shorter than
//     args.PrevLogIndex), AppendEntries sets reply.Success = false
//     without modifying rn's log.
//   - Otherwise, for each entry in args.Entries (in order, appended
//     starting at index args.PrevLogIndex+1): if rn already has an entry
//     at that index with a different term, rn deletes that entry and all
//     entries after it before appending args.Entries from that point on;
//     if rn already has an entry at that index with the same term, rn
//     leaves it as-is (idempotent retransmission). reply.Success = true.
//   - If args.LeaderCommit > rn.commitIndex, rn sets rn.commitIndex to the
//     minimum of args.LeaderCommit and the index of the last entry in
//     args.Entries (or args.PrevLogIndex if Entries is empty).
//
// reply.Term is always set to rn.currentTerm (after any update above).
// AppendEntries returns a non-nil error only on internal failure; a
// rejected append is reported via reply.Success = false with a nil error.
func (rn *RaftNode) AppendEntries(args *AppendEntriesArgs, reply *AppendEntriesReply) error {
	return errors.New("not implemented")
}

// BecomeCandidate starts a new election (paper §5.2): it increments rn's
// term, transitions rn to the Candidate state, votes for itself
// (votedFor = rn.id), and returns the RequestVoteArgs to send to every peer
// — populated with rn's new term, rn's id, and the index and term of rn's
// last log entry.
func (rn *RaftNode) BecomeCandidate() *RequestVoteArgs {
	return &RequestVoteArgs{}
}

// Propose appends command to rn's log as a new entry in rn's current term,
// but only if rn is currently the Leader. It returns the new entry's index
// and rn's current term, and whether rn was the Leader. If rn is not the
// Leader, Propose returns (-1, rn.currentTerm, false) and leaves rn's log
// unchanged. Replicating the new entry to peers (via AppendEntries) is the
// caller's responsibility.
func (rn *RaftNode) Propose(command string) (index int, term uint64, isLeader bool) {
	return 0, 0, false
}
