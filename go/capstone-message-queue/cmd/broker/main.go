// Broker binary.
//
// Usage:
//
//	broker [--data-dir <path>] [--port <port>] [--flush-ms <ms>]
//
// Defaults: data-dir=./data, port=9092, flush-ms=500
package main

import (
	"fmt"
	"net"
	"os"
	"strconv"
	"time"

	"github.com/andersonreyes/learning/go/capstone-message-queue/broker"
	"github.com/andersonreyes/learning/go/capstone-message-queue/concurrent"
	"github.com/andersonreyes/learning/go/capstone-message-queue/server"
)

func usage() {
	fmt.Fprintln(os.Stderr, "Usage: broker [--data-dir <path>] [--port <port>] [--flush-ms <ms>]")
	os.Exit(1)
}

func parseArgs() (dataDir string, port uint16, flushMs uint64) {
	dataDir = "./data"
	port = 9092
	flushMs = 500

	args := os.Args[1:]
	for i := 0; i < len(args); i++ {
		switch args[i] {
		case "--data-dir":
			i++
			if i >= len(args) {
				usage()
			}
			dataDir = args[i]
		case "--port":
			i++
			if i >= len(args) {
				usage()
			}
			p, err := strconv.ParseUint(args[i], 10, 16)
			if err != nil {
				usage()
			}
			port = uint16(p)
		case "--flush-ms":
			i++
			if i >= len(args) {
				usage()
			}
			ms, err := strconv.ParseUint(args[i], 10, 64)
			if err != nil {
				usage()
			}
			flushMs = ms
		default:
			usage()
		}
	}
	return
}

func main() {
	dataDir, port, flushMs := parseArgs()
	addr := fmt.Sprintf("0.0.0.0:%d", port)

	reg, err := broker.Open(dataDir)
	if err != nil {
		fmt.Fprintf(os.Stderr, "failed to open data directory: %v\n", err)
		os.Exit(1)
	}

	sr := concurrent.New(reg, time.Duration(flushMs)*time.Millisecond)
	defer sr.Close()

	listener, err := net.Listen("tcp", addr)
	if err != nil {
		fmt.Fprintf(os.Stderr, "failed to bind %s: %v\n", addr, err)
		os.Exit(1)
	}
	defer listener.Close()

	fmt.Fprintf(os.Stderr, "mini-mq broker listening on %s  (data-dir=%s)\n", addr, dataDir)

	handle := server.NewBrokerHandle(sr)
	if err := server.RunServer(listener, handle); err != nil {
		fmt.Fprintf(os.Stderr, "server error: %v\n", err)
		os.Exit(1)
	}
}
