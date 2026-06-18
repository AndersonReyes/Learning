// Package ports implements port-number parsing, classification, well-known
// service lookup, and CIDR-style port-range arithmetic.
package ports

import (
	"errors"
	"fmt"
	"sort"
	"strconv"
	"strings"
)

// OutOfRangeError indicates a syntactically valid integer that falls outside
// the valid port range 0-65535.
type OutOfRangeError struct {
	Value int64
}

func (e *OutOfRangeError) Error() string {
	return fmt.Sprintf("value %d is not in range 0-65635", e.Value)
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
	port, err := strconv.ParseInt(s, 10, 64)

	if err != nil {
		return 0, fmt.Errorf("unable to parse port %s: %w", s, err)
	}

	if port < 0 || port > 65535 {
		return 0, &OutOfRangeError{int64(port)}
	}

	return uint16(port), nil
}

// ClassifyPort categorizes port per RFC 6335:
//
//	0-1023:      "well-known"
//	1024-49151:  "registered"
//	49152-65535: "dynamic"
func ClassifyPort(port uint16) string {
	switch {
	case port < 1024:
		return "well-known"
	case port >= 1024 && port < 49152:
		return "registered"
	case port >= 49152:
		return "dynamic"
	default:
		return ""
	}
}

// LookupService looks up a well-known service by name, case-insensitively.
//
//	LookupService("http")  -> Service{"http", 80, "tcp"}, nil
//	LookupService("HTTPS") -> Service{"https", 443, "tcp"}, nil
//	LookupService("carrier-pigeon") -> Service{}, error (errors.Is(err, ErrUnknownService))
//
// Recognized names: ftp, ssh, telnet, smtp, dns, http, ntp, https.
func LookupService(name string) (Service, error) {
	var sv Service
	switch strings.ToLower(name) {
	case "http":
		sv = Service{"http", 80, "tcp"}
	case "https":
		sv = Service{"https", 443, "tcp"}
	case "ftp":
		sv = Service{"ftp", 21, "tcp"}
	case "ssh":
		sv = Service{"ssh", 22, "tcp"}
	case "telnet":
		sv = Service{"telnet", 23, "tcp"}
	case "smtp":
		sv = Service{"smtp", 25, "tcp"}
	case "dns":
		sv = Service{"dns", 53, "udp"}
	case "ntp":
		sv = Service{"ntp", 123, "udp"}
	default:
		return Service{}, ErrUnknownService
	}
	return sv, nil
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
	var rangeSize uint64 = 0

	for _, r := range ranges {
		if r[0] > r[1] {
			return 0, fmt.Errorf("invalid port range: %+v\n", r)
		}

		rangeSize += uint64(r[1]-r[0]) + 1
	}
	return rangeSize, nil
}

// MergePortRanges merges overlapping or adjacent inclusive [start, end]
// port ranges into the minimal sorted set of non-overlapping ranges. Two
// ranges are merged if they overlap OR are adjacent (e.g. [80,100] and
// [101,200] merge into [80,200]).
//
//	MergePortRanges([][2]uint16{{80, 100}, {90, 120}})   -> [][2]uint16{{80, 120}}
//	MergePortRanges([][2]uint16{{90, 120}, {80, 100}})   -> [][2]uint16{{80, 120}}
//	MergePortRanges([][2]uint16{{80, 100}, {101, 120}})  -> [][2]uint16{{80, 120}}
//	MergePortRanges([][2]uint16{{80, 100}, {200, 300}})  -> [][2]uint16{{80, 100}, {200, 300}}
//	MergePortRanges(nil)                                  -> nil, nil
//
// Returns an error if any range has start > end.
func MergePortRanges(ranges [][2]uint16) ([][2]uint16, error) {

	if len(ranges) < 2 {
		return ranges, nil
	}

	sort.Slice(ranges, func(i, j int) bool {
		return ranges[i][0] < ranges[j][0]
	})

	out := [][2]uint16{
		ranges[0],
	}

	var i = 1
	var last_i = 0

	for i < len(ranges) {
		fmt.Printf("Size of ranges: %d for ranges: %+v\n", len(ranges), ranges)
		left := &out[last_i]
		right := ranges[i]
		if right[0] > right[1] {
			return [][2]uint16{}, fmt.Errorf("invalid port range: %+v\n", right)
		}

		if left[1] >= right[0] || (left[1]+1) == right[0] {
			// there is overlap now
			//

			// min start
			if left[0] > right[0] {
				out[last_i][0] = right[0]
			}

			// max end
			if left[1] < right[1] {
				left[1] = right[1]
			}

		} else {
			// no overlap
			out = append(out, right)
			last_i = i
		}
		i++
	}
	return out, nil
}
