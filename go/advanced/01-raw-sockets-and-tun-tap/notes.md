# Advanced 1. Raw Sockets & TUN/TAP + Reading/Writing Raw IP Packets

## New: the `syscall` package — raw syscalls and `ioctl`

Everything so far has gone through `net` (`Dial`, `Listen`, ...), which hides
the OS underneath. This topic drops one level: creating a virtual network
interface means talking to the kernel directly via `syscall`.

```go
fd, err := syscall.Open("/dev/net/tun", syscall.O_RDWR, 0)
defer syscall.Close(fd)

r1, r2, errno := syscall.Syscall(syscall.SYS_IOCTL, uintptr(fd), uintptr(request), uintptr(argp))
if errno != 0 {
    // errno is a syscall.Errno, which implements error
    return err
}
```

`ioctl` ("I/O control") is a catch-all syscall for device-specific operations
that don't fit `read`/`write`. Each operation has a numeric **request code**
and an argument — usually a pointer to a struct the kernel reads and/or
writes in place. Request codes are built from a packed bitfield (direction,
size, type, number):

```
_IOC(dir, type, nr, size) = dir<<30 | size<<16 | type<<8 | nr
```

`TUNSETIFF` is `_IOW('T', 202, sizeof(int))` = `0x400454CA` — the constant
this topic's `tunSetIff` is set to. You don't need to memorize these; the
exercise gives you the derived constants, but `examples/main.go` shows the
arithmetic so the "magic number" isn't mysterious.

### `unsafe.Pointer`: passing a Go struct to the kernel

`ioctl`'s third argument is a pointer the kernel reads as a C struct. Go's
type system doesn't know about C structs, so crossing this boundary requires
`unsafe.Pointer`:

```go
var req ifReq
copy(req.Name[:], "tun0")
req.Flags = iffTUN | iffNoPI
syscall.Syscall(syscall.SYS_IOCTL, uintptr(fd), tunSetIff, uintptr(unsafe.Pointer(&req)))
```

**Gotchas**:

- The Go struct's field order, sizes, and padding must match the C struct
  **exactly** — the kernel reads raw bytes at the pointer, with no knowledge
  of Go types. A mismatched layout corrupts memory or silently does the
  wrong thing.
- `unsafe.Pointer` bypasses Go's type safety entirely. Outside of syscalls
  and cgo-style interop, you should essentially never need it.
- `unsafe.Sizeof(T{})` reports the size Go computes for a struct, including
  padding for alignment — useful for checking your struct matches the C
  side's size (here, 40 bytes for `struct ifreq` on Linux/amd64).

### `os.NewFile`: wrapping a raw fd

Once you have a file descriptor from `syscall.Open` + `ioctl`, wrap it in
`*os.File` to get ordinary `Read`/`Write`/`Close`:

```go
f := os.NewFile(uintptr(fd), name)
n, err := f.Read(buf)
```

## Networking: TUN devices

A **TUN device** is a virtual point-to-point network interface backed by a
file descriptor instead of a NIC. Opening `/dev/net/tun` and calling
`ioctl(fd, TUNSETIFF, &ifreq{Name: "tun0", Flags: IFF_TUN|IFF_NO_PI})` creates
(or attaches to) an interface named `tun0` and a file descriptor for talking
to it:

- **`IFF_TUN`** vs **`IFF_TAP`**: a TUN device operates at layer 3 (you
  read/write raw IP packets); a TAP device operates at layer 2 (you
  read/write Ethernet frames). This topic uses TUN.
- **`IFF_NO_PI`**: without it, every packet is prefixed with a 4-byte
  "packet information" header (flags + protocol). With it, `Read`/`Write`
  deal in raw IP packets directly — what this topic's functions expect.
- **Direction is from the kernel's point of view**: `Read(fd)` returns
  packets the kernel is sending **out** this interface; `Write(fd, pkt)`
  injects a packet as if it had been **received** on this interface from the
  network.
- **Creating the device requires `CAP_NET_ADMIN`** (in practice, root).
  `/dev/net/tun` must exist (it does on any standard Linux kernel with TUN
  support).

### Configuring the interface

`OpenTUN` only creates the fd and the (down, unconfigured) interface.
Assigning an address and bringing the link up is ordinary interface
configuration, done the same way as for any NIC — shell out to `ip` via
`os/exec`:

```go
exec.Command("ip", "addr", "add", "10.200.0.1/24", "dev", "tun0").Run()
exec.Command("ip", "link", "set", "tun0", "up").Run()
```

Once `10.200.0.1/24` is assigned, the kernel adds a connected route for
`10.200.0.0/24` via `tun0`. Any packet the kernel needs to send to an address
in that subnet — including a reply it generates itself — gets written to the
TUN fd.

### Why writing a "ping" into the TUN device produces a reply

If you `Write` an IPv4/ICMP Echo Request packet into the TUN fd, addressed
**to** the interface's own configured address (`10.200.0.1`) and claiming to
be **from** some other address in the subnet (`10.200.0.2`), the kernel's IP
stack receives it as if it arrived from the network, sees the destination is
a local address, and its ICMP handling answers its own Echo Request with an
Echo Reply — addressed back to `10.200.0.2`. The routing table says
`10.200.0.2` is reachable via `tun0`, so the kernel writes that reply packet
to the TUN fd, where a `Read` picks it up. This loop — build a packet, write
it in, read the kernel's response back out — is exactly how a userspace
"ping" or VPN client exchanges packets with the kernel's network stack.

## Networking: the Internet checksum (RFC 1071)

IPv4 and ICMP headers carry a 16-bit checksum: the one's-complement sum of
the data's 16-bit words (network byte order), then complemented
(`^sum`). Summing must fold carries out of bit 16 back into the low 16 bits
("end-around carry"):

```go
var sum uint32
for i := 0; i+1 < len(data); i += 2 {
    sum += uint32(binary.BigEndian.Uint16(data[i : i+2]))
}
if len(data)%2 == 1 {
    sum += uint32(data[len(data)-1]) << 8 // odd trailing byte = high byte of a zero-padded word
}
for sum > 0xFFFF {
    sum = (sum & 0xFFFF) + (sum >> 16)
}
return ^uint16(sum)
```

A header is valid exactly when `Checksum(header) == 0` — the checksum field
itself is part of the summed data, so a correctly-checksummed header sums to
`0xFFFF`, and `^0xFFFF == 0`.

## Networking: IPv4 packets and ICMP Echo (RFC 791, RFC 792)

`fundamentals/03` parsed IPv4 headers field-by-field with pointers and
methods. This topic adds the piece that was missing there: **computing** a
valid header checksum, and combining header + payload into one packet that a
TUN device can write/read directly.

An ICMP Echo Request/Reply (RFC 792) is 8 header bytes — type (`8` = request,
`0` = reply), code (`0`), checksum, identifier, sequence number — followed by
an arbitrary payload, with the same RFC 1071 checksum covering the whole
message (header + payload).

## Further Reading

- [`syscall`](https://pkg.go.dev/syscall), [`unsafe`](https://pkg.go.dev/unsafe),
  [`os.NewFile`](https://pkg.go.dev/os#NewFile)
- [Linux kernel docs: Universal TUN/TAP driver](https://www.kernel.org/doc/html/latest/networking/tuntap.html)
- [RFC 1071 (Internet checksum)](https://www.rfc-editor.org/rfc/rfc1071)
- [RFC 791 (IPv4)](https://www.rfc-editor.org/rfc/rfc791) §3.1 (header format)
- [RFC 792 (ICMP)](https://www.rfc-editor.org/rfc/rfc792) (Echo Request/Reply)
