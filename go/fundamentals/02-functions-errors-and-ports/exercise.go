// Package ports implements port-number parsing, classification, well-known
// service lookup, and CIDR-style port-range arithmetic.
package ports

import "errors"

// OutOfRangeError indicates a syntactically valid integer that falls outside
// the valid port range 0-65535.
type OutOfRangeError struct {
	Value int64
}

func (e *OutOfRangeError) Error() string {
	return "not implemented"
}

// ErrUnknownService is returned (wrapped) by LookupService when name is not
// a recognized well-known service.
var ErrUnknownService = errors.New("unknown service")

// Service describes a well-known network service.
type Service struct {
	Name      string
	Port      uint16
	Transport string // "tcp" or "udp"
}

// ParsePort parses s as a decimal port number in the range 0-65535.
//
//	ParsePort("80")    -> 80, nil
//	ParsePort("65535") -> 65535, nil
//	ParsePort("65536") -> 0, *OutOfRangeError (errors.As)
//	ParsePort("-1")    -> 0, *OutOfRangeError (errors.As)
//	ParsePort("abc")   -> 0, error (NOT an *OutOfRangeError — a wrapped
//	                       syntax error)
func ParsePort(s string) (uint16, error) {
	return 0, errors.New("not implemented")
}

// ClassifyPort categorizes port per RFC 6335:
//
//	0-1023:      "well-known"
//	1024-49151:  "registered"
//	49152-65535: "dynamic"
func ClassifyPort(port uint16) string {
	return ""
}

// LookupService looks up a well-known service by name, case-insensitively.
//
//	LookupService("http")  -> Service{"http", 80, "tcp"}, nil
//	LookupService("HTTPS") -> Service{"https", 443, "tcp"}, nil
//	LookupService("carrier-pigeon") -> Service{}, error (errors.Is(err, ErrUnknownService))
//
// Recognized names: ftp, ssh, telnet, smtp, dns, http, ntp, https.
func LookupService(name string) (Service, error) {
	return Service{}, errors.New("not implemented")
}

// PortRangeSize returns the total number of ports across one or more
// inclusive [start, end] ranges, WITHOUT deduplicating overlaps.
//
//	PortRangeSize([2]uint16{80, 80})                      -> 1, nil
//	PortRangeSize([2]uint16{1024, 2048})                  -> 1025, nil
//	PortRangeSize([2]uint16{80, 80}, [2]uint16{443, 443}) -> 2, nil
//	PortRangeSize()                                        -> 0, nil
//
// Returns an error if any range has start > end.
func PortRangeSize(ranges ...[2]uint16) (uint64, error) {
	return 0, errors.New("not implemented")
}

// MergePortRanges merges overlapping or adjacent inclusive [start, end]
// port ranges into the minimal sorted set of non-overlapping ranges. Two
// ranges are merged if they overlap OR are adjacent (e.g. [80,100] and
// [101,200] merge into [80,200]).
//
//	MergePortRanges([][2]uint16{{80, 100}, {90, 120}})   -> [][2]uint16{{80, 120}}
//	MergePortRanges([][2]uint16{{80, 100}, {101, 120}})  -> [][2]uint16{{80, 120}}
//	MergePortRanges([][2]uint16{{80, 100}, {200, 300}})  -> [][2]uint16{{80, 100}, {200, 300}}
//	MergePortRanges(nil)                                  -> nil, nil
//
// Returns an error if any range has start > end.
func MergePortRanges(ranges [][2]uint16) ([][2]uint16, error) {
	return nil, errors.New("not implemented")
}
