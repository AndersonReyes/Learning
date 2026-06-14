// Command main demonstrates this topic's net/rpc-based "service discovery"
// (see notes.md): two RaftNode values, one registered as a net/rpc service
// and served over a real TCP loopback listener, the other dialing it as a
// client. The only "discovery" step is mapping a peer id to that listener's
// address — exercise.go's RequestVote/AppendEntries don't change to make
// this work.
//
// Against the stub exercise.go, the RPC round trip still succeeds; the
// handlers themselves return their "not implemented" errors, which this
// program prints instead of treating as fatal.
package main

import (
	"fmt"
	"log"
	"net"
	"net/rpc"

	miniraft "github.com/andersonreyes/learning/go/advanced/05-raft-and-service-discovery"
)

const (
	node0ID = 0
	node1ID = 1
)

func main() {
	// Node 1 is the RPC server: register it and start accepting connections.
	node1 := miniraft.NewRaftNode(node1ID, []int{node0ID})
	if err := rpc.Register(node1); err != nil {
		log.Fatalf("register: %v", err)
	}

	listener, err := net.Listen("tcp", "127.0.0.1:0")
	if err != nil {
		log.Fatalf("listen: %v", err)
	}
	go rpc.Accept(listener)

	// "Service discovery" in this sketch: node 0 only needs node 1's id
	// mapped to a dialable address. A real deployment would populate this
	// map from DNS/Consul/etcd/Kubernetes; here it's the listener's actual
	// address.
	peerAddrs := map[int]string{node1ID: listener.Addr().String()}

	node0 := miniraft.NewRaftNode(node0ID, []int{node1ID})

	client, err := rpc.Dial("tcp", peerAddrs[node1ID])
	if err != nil {
		log.Fatalf("dial: %v", err)
	}
	defer client.Close()

	fmt.Println("--- node 0 starts an election: BecomeCandidate, then RequestVote on node 1 ---")
	args := node0.BecomeCandidate()
	fmt.Printf("RequestVoteArgs sent to node 1: %+v\n", args)

	var voteReply miniraft.RequestVoteReply
	if err := client.Call("RaftNode.RequestVote", args, &voteReply); err != nil {
		fmt.Printf("RaftNode.RequestVote error: %v\n", err)
	} else {
		fmt.Printf("RequestVoteReply from node 1: %+v\n", voteReply)
	}

	fmt.Println("\n--- a heartbeat: AppendEntries with no entries ---")
	heartbeat := &miniraft.AppendEntriesArgs{
		Term:     args.Term,
		LeaderID: node0ID,
	}
	var appendReply miniraft.AppendEntriesReply
	if err := client.Call("RaftNode.AppendEntries", heartbeat, &appendReply); err != nil {
		fmt.Printf("RaftNode.AppendEntries error: %v\n", err)
	} else {
		fmt.Printf("AppendEntriesReply from node 1: %+v\n", appendReply)
	}
}
