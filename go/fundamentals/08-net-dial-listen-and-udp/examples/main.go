// Command main demonstrates net.Listen/Accept/Dial and UDP sockets used in
// this topic's exercise: a TCP chat server that broadcasts each client's
// messages to every other connected client, and a UDP "ping" exchange —
// applied to small examples that are deliberately *not* the exercise
// (Serve/EchoHandler/DialAndSend/ServeUDP/SendUDP in exercise.go).
package main

import (
	"bufio"
	"fmt"
	"net"
	"sync"
	"time"
)

// chatServer broadcasts each line received from a client to every other
// connected client. It's a small extension of the accept-loop pattern from
// exercise.go's Serve: one goroutine per connection, plus a registry of
// connected clients protected by a mutex (topic 7).
type chatServer struct {
	mu      sync.Mutex
	clients map[net.Conn]bool
}

func newChatServer() *chatServer {
	return &chatServer{clients: make(map[net.Conn]bool)}
}

func (s *chatServer) broadcast(from net.Conn, line string) {
	s.mu.Lock()
	defer s.mu.Unlock()
	for c := range s.clients {
		if c == from {
			continue
		}
		fmt.Fprintf(c, "%s\n", line)
	}
}

func (s *chatServer) handle(conn net.Conn) {
	s.mu.Lock()
	s.clients[conn] = true
	s.mu.Unlock()

	defer func() {
		s.mu.Lock()
		delete(s.clients, conn)
		s.mu.Unlock()
		conn.Close()
	}()

	scanner := bufio.NewScanner(conn)
	for scanner.Scan() {
		s.broadcast(conn, fmt.Sprintf("%s says: %s", conn.RemoteAddr(), scanner.Text()))
	}
}

func main() {
	// TCP chat server: accept loop + per-connection goroutine + shared
	// client registry.
	l, err := net.Listen("tcp", "127.0.0.1:0")
	if err != nil {
		fmt.Println("listen error:", err)
		return
	}
	defer l.Close()

	server := newChatServer()
	go func() {
		for {
			conn, err := l.Accept()
			if err != nil {
				return
			}
			go server.handle(conn)
		}
	}()

	alice, err := net.Dial("tcp", l.Addr().String())
	if err != nil {
		fmt.Println("dial error:", err)
		return
	}
	defer alice.Close()

	bob, err := net.Dial("tcp", l.Addr().String())
	if err != nil {
		fmt.Println("dial error:", err)
		return
	}
	defer bob.Close()

	// Give the server a moment to register both connections.
	time.Sleep(10 * time.Millisecond)

	fmt.Fprintln(alice, "hello from alice")

	bobReader := bufio.NewReader(bob)
	line, _ := bobReader.ReadString('\n')
	fmt.Print("bob received: ", line)

	// UDP ping/pong: ListenUDP + ReadFromUDP/WriteToUDP on the server
	// side, DialUDP + Write/Read with a read deadline on the client side.
	uaddr, _ := net.ResolveUDPAddr("udp", "127.0.0.1:0")
	uconn, err := net.ListenUDP("udp", uaddr)
	if err != nil {
		fmt.Println("listenudp error:", err)
		return
	}
	defer uconn.Close()

	go func() {
		buf := make([]byte, 64)
		for {
			n, from, err := uconn.ReadFromUDP(buf)
			if err != nil {
				return
			}
			uconn.WriteToUDP([]byte("pong"), from)
			_ = n
		}
	}()

	client, err := net.DialUDP("udp", nil, uconn.LocalAddr().(*net.UDPAddr))
	if err != nil {
		fmt.Println("dialudp error:", err)
		return
	}
	defer client.Close()

	client.Write([]byte("ping"))
	client.SetReadDeadline(time.Now().Add(time.Second))

	resp := make([]byte, 64)
	n, err := client.Read(resp)
	if err != nil {
		fmt.Println("udp read error:", err)
		return
	}
	fmt.Println("udp response:", string(resp[:n]))
}
