# Advanced 3. Bit-Twiddling Instruction Encoding + Classic BPF (cBPF) Packet Filters

## Why classic BPF, not eBPF/XDP

The roadmap originally scoped this topic as eBPF/XDP via `cilium/ebpf`. That
library (and a real eBPF workflow generally) needs `bpftool`/`vmlinux.h` for
CO-RE, a clang toolchain to compile C to BPF bytecode, and a new module
dependency. None of that is available here, so this topic instead covers
**classic BPF (cBPF)** — eBPF's direct ancestor, still fully supported by the
kernel, expressible as data (no compiler needed), and reachable entirely
through the standard library's `syscall` package. It's the same mechanism
`tcpdump`/libpcap use (`tcpdump -dd` prints a cBPF program), and the
instruction model (load/jump/return) is the conceptual core that eBPF
generalized into a full register machine. If you want to come back to
real eBPF/XDP later with `cilium/ebpf` and a clang toolchain, this topic's
`Match`/`AssembleFilter` types are a small enough surface to extend or
replace.

## New: bit-twiddling an instruction encoding

A cBPF instruction is a fixed-size struct (`syscall.SockFilter`):

```go
type SockFilter struct {
    Code uint16 // opcode: class | size/mode | operand-source
    Jt   uint8  // jump-true offset (instructions to skip)
    Jf   uint8  // jump-false offset
    K    uint32 // generic operand: byte offset, comparison value, return value
}
```

`Code` packs three bitfields with OR, the same pattern as the IPv4/TCP flag
bytes from `fundamentals/03`:

```
BPF_LD  | BPF_ABS | BPF_H   =  0x00 | 0x20 | 0x08  = 0x28   // load 2 bytes from a fixed offset
BPF_JMP | BPF_JEQ | BPF_K   =  0x05 | 0x10 | 0x00  = 0x15   // jump if accumulator == K
BPF_RET | BPF_K             =  0x06 | 0x00         = 0x06   // return K as the verdict
```

`syscall` exports the field constants (`BPF_LD`, `BPF_ABS`, `BPF_H`,
`BPF_JMP`, `BPF_JEQ`, `BPF_K`, `BPF_RET`, ...) — `LoadAbsolute`,
`JumpIfEqual`, and `Return` are thin, validated constructors over these
bitwise-OR combinations.

## Networking: the cBPF virtual machine

A cBPF **program** is a flat `[]SockFilter` run by an in-kernel interpreter
against each packet, with one accumulator register and an implicit program
counter:

- **`BPF_LD|BPF_ABS|size`** loads `size` bytes (1/2/4 — `BPF_B`/`BPF_H`/`BPF_W`)
  from a fixed byte offset (`K`) in the packet into the accumulator, as a
  big-endian integer. "Fixed offset" means the same Ethernet/IPv4/TCP header
  layouts from `fundamentals/03` and `advanced/01` — EtherType is 2 bytes at
  offset 12, IPv4's protocol field is 1 byte at offset 23 (14-byte Ethernet
  header + 9), and so on.
- **`BPF_JMP|BPF_JEQ|BPF_K`** compares the accumulator to `K`, then jumps
  **forward** `Jt` instructions if equal or `Jf` instructions if not (`0`
  falls through to the next instruction). Jumps are always forward — cBPF
  programs are loops-free DAGs, which is part of how the kernel verifies they
  terminate.
- **`BPF_RET|BPF_K`** ends the program with verdict `K`: `0` drops the
  packet/datagram; any other value is how many bytes of it to deliver (the
  conventional "accept everything" value is `0xffff`).

### `AssembleFilter`: compiling an AND-chain

`AssembleFilter` takes `[]Match{Offset, Size, Value}` — "packet at this
offset equals this value" — and emits a program that delivers the packet
only if **every** `Match` holds, short-circuiting like `&&`. For `N` matches
the program is `2N+2` instructions: a `Load`/`Jump` pair per match, then
`Return(0)` (drop) at index `2N`, then `Return(0xffff)` (accept) at `2N+1`.

The interesting part is computing each jump's `Jt`/`Jf` — a small instance of
the same "compute relative offsets" problem any assembler/compiler backend
has for branches:

- For the **last** match (`i == N-1`): on success jump forward `1`
  instruction (over the drop, landing on accept); on failure fall through
  (`0`, landing on drop).
- For every **earlier** match (`i < N-1`): on success fall through (`0`, to
  the next match's load); on failure jump forward `2*(N-1-i)` instructions —
  skipping every remaining `Load`/`Jump` pair — landing on the drop
  instruction.

### `AttachFilter` and `SO_ATTACH_FILTER`

`SO_ATTACH_FILTER` (a `SOL_SOCKET`-level socket option) attaches a compiled
program to **any** socket — `AF_PACKET`/`SOCK_RAW` for "real" packet
filtering as in `tcpdump`, but also ordinary `AF_INET` sockets (which is what
`exercise_test.go` and `examples/main.go` use, so the tests need no special
privileges). As in `advanced/01`, crossing into a raw syscall means building
the kernel's `struct sock_fprog` (`syscall.SockFprog{Len, Filter}`) and
passing `unsafe.Pointer(&fprog)` to `syscall.Syscall6(SYS_SETSOCKOPT, ...)` —
`syscall` has typed helpers for common `setsockopt` options
(`SetsockoptInt`, ...) but not for `SockFprog`, so this one is raw.

## Further Reading

- [`syscall`](https://pkg.go.dev/syscall) (`SockFilter`, `SockFprog`, `BPF_*`
  constants), [`unsafe`](https://pkg.go.dev/unsafe)
- [Linux kernel docs: BPF and XDP reference guide](https://docs.cilium.io/en/latest/bpf/) —
  background on how cBPF relates to eBPF/XDP
- [Linux kernel docs: socket filtering](https://www.kernel.org/doc/Documentation/networking/filter.txt) —
  the cBPF instruction set and `SO_ATTACH_FILTER`
- `man 7 socket` (`SO_ATTACH_FILTER`, `SO_ATTACH_BPF`)
