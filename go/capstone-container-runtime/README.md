# Capstone E: Linux Container Runtime

A minimal "build your own Docker" — a two-process container runtime using
Linux namespaces, `pivot_root`, cgroups, capabilities, and seccomp. Standard
library only, no external dependencies.

Based on:
- [Linux Containers in 500 Lines of Code](https://blog.lizzie.io/linux-containers-in-500-loc.html) (Lizzie Dixon) — C implementation with deep security analysis, exploit PoCs, and the full 28-capability drop list
- [Build Your Own Container Using Less than 100 Lines of Go](https://www.infoq.com/articles/build-a-container-golang/) (Julian Friedman) — Go skeleton with the `/proc/self/exe` re-exec pattern

Source material saved in `sources/diydocker.txt`.

## Architecture

Two-process model (parent + child), connected by the `/proc/self/exe`
re-exec trick:

```
Parent process                          Child process
---------------------------------------------
1. Parse CLI flags                     (spawned by parent with clone flags)
2. Set up cgroups                      1. Set hostname
3. clone() with CLONE_NEW* flags -->   2. Set up mounts (pivot_root)
4. Write uid_map/gid_map               3. User namespace (unshare)
5. Wait for child exit                 4. Drop capabilities
6. Clean up cgroups                    5. Apply seccomp filters
                                       6. execve(command)
```

Order matters: mounts need capabilities, seccomp prevents subsequent
`unshare`/`clone`, `execve` recalculates capabilities from the bounding set.

## Phases

Work one phase at a time. Each phase is a Go package. Later phases import
earlier ones. All tests must pass before moving to the next phase.

| Phase | Package | What you build |
|-------|---------|----------------|
| 1 | `namespace/` | Clone flag computation, hostname generation, namespace metadata |
| 2 | `filesystem/` | Mount operation planning, rootfs validation, pivot_root path setup |
| 3 | `cgroup/` | Cgroup config management, filesystem writes for resource limits |
| 4 | `security/` | Capability drop lists, seccomp rule building, security policy |
| 5 | `container/` + `cmd/` | Full container lifecycle tying phases 1-4 together |

---

## Phase 1: `namespace/` — Namespace Configuration

A package that manages Linux namespace metadata and computes clone flags.

### Functions to implement

- **`AllNamespaces()`** — returns metadata (name, CLONE_NEW* flag name, description, flag value) for all 7 namespace types: UTS, PID, MNT, NET, IPC, USER, CGROUP
- **`CloneFlags(types)`** — takes a list of namespace types, returns the OR'd `uintptr` of their CLONE_NEW* constants. Handle duplicates, empty input.
- **`ParseCloneFlags(flags)`** — reverse of above: decompose raw flags into sorted namespace types, ignore unknown bits like SIGCHLD
- **`GenerateHostname(seed)`** — deterministic hostname from a uint32 seed, format `ct-XXXXXXXX` (8 hex digits)
- **`ReExecArgs(command, args)`** — build the arg list for `/proc/self/exe` child re-exec: prepend `"child"` as argv[0] marker, then command, then args. Must not mutate the input slice.
- **`DefaultTypes()`** — returns the 6 standard namespace types (all except USER, which has complex capability interactions and many kernels restrict it)

### What to test

All pure computation — no root, no Linux required.
- `AllNamespaces` returns exactly 7 entries, ordered by type constant, each with correct flag values matching `syscall.CLONE_NEWUTS`, `syscall.CLONE_NEWPID`, etc.
- `CloneFlags` roundtrips with `ParseCloneFlags` — compute flags, parse back, get same types
- `CloneFlags` with duplicates produces same result as without
- `CloneFlags(nil)` returns 0
- `ParseCloneFlags` returns types sorted by constant value
- `ParseCloneFlags` ignores unknown bits (e.g. SIGCHLD = 0x11)
- `GenerateHostname` format is `ct-XXXXXXXX`, length 11, unique for different seeds
- `ReExecArgs` prepends "child", doesn't mutate input slice
- `DefaultTypes` has 6 entries, excludes USER, includes UTS/PID/MNT/NET/IPC/CGROUP

### Stdlib references

- [`syscall`](https://pkg.go.dev/syscall) — `CLONE_NEWUTS`, `CLONE_NEWPID`, `CLONE_NEWNS`, `CLONE_NEWNET`, `CLONE_NEWIPC`, `CLONE_NEWUSER`. Note: `CLONE_NEWCGROUP` (`0x02000000`) is not in Go's `syscall` package — define it yourself.
- [`fmt`](https://pkg.go.dev/fmt) — `Sprintf` for hostname formatting

---

## Phase 2: `filesystem/` — Rootfs & Mount Planning

A package that plans the mount operations for `pivot_root` filesystem isolation.

### Functions to implement

- **`PivotDirs(rootDir, tempBase)`** — compute `(newRoot, putOld)` paths for pivot_root. `newRoot` is a subdir under `tempBase`, `putOld` is a subdir inside `newRoot` where the old root gets moved.
- **`MountPlan(rootDir, newRoot)`** — returns an ordered list of mount operations: (1) remount `/` with `MS_PRIVATE|MS_REC` to prevent mount propagation, (2) bind-mount rootDir to newRoot, (3) mount proc inside newRoot at `/proc`
- **`UnmountPlan(putOld)`** — returns operations to unmount the old root with `MNT_DETACH` and rmdir it
- **`ValidateRootfs(rootDir)`** — check the directory exists and has at minimum `/bin` or `/usr/bin`
- **`DefaultMountFlags()`** — returns `MS_PRIVATE | MS_REC`

### What to test

Use `t.TempDir()` to create real directory trees.
- `ValidateRootfs` with empty dir fails, dir with `bin/` subdir passes
- `PivotDirs` returns paths where `putOld` is under `newRoot`
- `MountPlan` returns 3 operations in correct order: private remount first, then bind, then proc
- `MountPlan` operations have correct flags (`MS_PRIVATE|MS_REC`, `MS_BIND`, etc.)
- `UnmountPlan` targets `putOld` with `MNT_DETACH`
- `DefaultMountFlags` returns `MS_PRIVATE | MS_REC`

### Stdlib references

- [`syscall`](https://pkg.go.dev/syscall) — `MS_PRIVATE`, `MS_REC`, `MS_BIND`, `MNT_DETACH`, `SYS_PIVOT_ROOT`
- [`os`](https://pkg.go.dev/os) — `Stat`, `MkdirAll` for validation
- [`path/filepath`](https://pkg.go.dev/path/filepath) — `Join` for path construction

---

## Phase 3: `cgroup/` — Resource Limits

A package that manages cgroup v2 resource limits by writing to the cgroup filesystem.

### Functions to implement

- **Config struct** with `MemoryBytes int64`, `CPUWeight int64`, `MaxPIDs int64`, `IOWeight int64`
- **`CgroupPath(root, name)`** — returns the cgroup directory path: `root/name`
- **`ConfigEntries(config)`** — converts Config to file/value pairs: `memory.max`, `cpu.weight`, `pids.max`, `io.weight`. Skip any field set to zero.
- **`WriteCgroup(dir, entries)`** — mkdir the cgroup directory, write each entry's value to its file
- **`ReadCgroup(dir, paths)`** — read values back from cgroup files (for verification)
- **`AddProcess(dir, pid)`** — write PID string to `cgroup.procs`. pid=0 means "the calling process".
- **`CleanupCgroup(dir)`** — remove the cgroup directory

### Key detail

Cgroups are just filesystem writes. `mkdir /sys/fs/cgroup/my-container`, then `echo 1073741824 > memory.max`. Cleanup: move processes out by writing to the parent's `cgroup.procs`, then `rmdir`.

### What to test

Fully testable with `t.TempDir()` as a fake cgroup root.
- `CgroupPath("root", "name")` returns `"root/name"`
- `ConfigEntries` produces correct file names (`memory.max`, `cpu.weight`, etc.)
- `ConfigEntries` skips zero-valued fields
- `WriteCgroup` creates the directory and writes files with correct content
- `ReadCgroup` roundtrips with `WriteCgroup` — write then read back, values match
- `AddProcess` writes `"0"` to `cgroup.procs` when pid=0
- `CleanupCgroup` removes the directory

### Stdlib references

- [`os`](https://pkg.go.dev/os) — `Mkdir`, `WriteFile`, `ReadFile`, `Remove`
- [`strconv`](https://pkg.go.dev/strconv) — `FormatInt` for int-to-string
- [`path/filepath`](https://pkg.go.dev/path/filepath) — `Join`

---

## Phase 4: `security/` — Capabilities & Seccomp Rules

A package that defines the security policy — which capabilities to drop and why, and which syscalls to filter.

### Functions to implement

- **CapInfo struct** with `Cap int`, `Name string`, `Description string`, `DropReason string`
- **`DefaultDropList()`** — returns the 28 capabilities to drop, each with a reason from the article:
  - `CAP_AUDIT_CONTROL/READ/WRITE` — audit system not namespaced, can falsify logs
  - `CAP_BLOCK_SUSPEND` — suspend not namespaced
  - `CAP_DAC_READ_SEARCH` — enables `open_by_handle_at` to read arbitrary host files (the `shocker.c` exploit)
  - `CAP_FSETID` — modify setuid executables without removing the setuid bit
  - `CAP_IPC_LOCK` — lock memory beyond limits, DoS vector
  - `CAP_MAC_ADMIN/OVERRIDE` — circumvent Apparmor/SELinux/SMACK
  - `CAP_MKNOD` — create device files for real hardware, read host disk
  - `CAP_SETFCAP` — set file capabilities on executables
  - `CAP_SYSLOG` — destructive syslog actions, leaks kernel addresses
  - `CAP_SYS_ADMIN` — mount, vm86, etc.
  - `CAP_SYS_BOOT` — reboot, kexec
  - `CAP_SYS_MODULE` — load/unload kernel modules
  - `CAP_SYS_NICE` — set higher priority, DoS other processes
  - `CAP_SYS_RAWIO` — raw memory/port access via iopl/ioperm
  - `CAP_SYS_RESOURCE` — circumvent kernel-wide resource limits
  - `CAP_SYS_TIME` — modify system-wide time (breaks TLS, Kerberos, HSTS)
  - `CAP_WAKE_ALARM` — interfere with suspend
- **`RetainedCaps()`** — capabilities safe to keep inside namespaces, with reasons:
  - `CAP_DAC_OVERRIDE` — standard Unix permission bypass, doesn't enable `open_by_handle_at`
  - `CAP_NET_*` — scoped to network namespace
  - `CAP_SYS_PTRACE`, `CAP_SYS_KILL` — scoped to PID namespace
  - `CAP_SETUID`, `CAP_SETGID` — scoped to user namespace
  - `CAP_SYS_CHROOT` — safe without dynamic libraries in namespace
  - `CAP_FOWNER`, `CAP_IPC_OWNER`, `CAP_LEASE`, `CAP_LINUX_IMMUTABLE`, `CAP_SYS_PACCT`, `CAP_SETPCAP`, `CAP_SYS_TTYCONFIG`
- **`IsCapDropped(cap)`** — lookup against the drop list
- **SeccompRule struct** with `Syscall string`, `Action string`, `ArgIndex int`, `ArgMask uint64`, `ArgValue uint64`, `Description string`
- **`DefaultSeccompRules()`** — returns the seccomp rules from the article:
  - `chmod`/`fchmod`/`fchmodat` with `S_ISUID` or `S_ISGID` bits — prevent setuid binary creation
  - `unshare`/`clone` with `CLONE_NEWUSER` — prevent user namespace nesting
  - `ioctl` with `TIOCSTI` — prevent terminal injection (CVE-2016-7545)
  - `keyctl`/`add_key`/`request_key` — kernel keyring not namespaced
  - `ptrace` — bypasses seccomp before Linux 4.8
  - `mbind`/`migrate_pages`/`move_pages`/`set_mempolicy` — NUMA DoS
  - `userfaultfd` — used in kernel exploits to pause execution
  - `perf_event_open` — leaks kernel addresses

### What to test

Pure data — no syscalls needed.
- `DefaultDropList` has exactly 28 entries with no duplicate Cap values
- Every entry has non-empty Name, Description, DropReason
- `RetainedCaps` and `DefaultDropList` don't overlap (no Cap in both)
- `IsCapDropped` returns true for `CAP_SYS_ADMIN` (cap 21), false for `CAP_NET_ADMIN` (cap 12)
- `DefaultSeccompRules` covers at least: chmod, unshare, clone, ioctl, keyctl, ptrace, userfaultfd, perf_event_open
- Every SeccompRule has non-empty Syscall and Description

### Stdlib references

- [`syscall`](https://pkg.go.dev/syscall) — Go's `syscall` package doesn't export `CAP_*` constants. Define your own matching the kernel values from `linux/capability.h` (e.g. `CAP_SYS_ADMIN = 21`).

---

## Phase 5: `container/` + `cmd/container/` — Lifecycle & CLI

The package that wires phases 1-4 together into a working container, and the CLI binary.

### `container/` — functions to implement

- **ContainerConfig struct** — `Command string`, `Args []string`, `Hostname string`, `RootDir string`, `UID int`, `GID int`, `MemoryMB int64`, `CPUWeight int64`, `MaxPIDs int64`
- **`New(config)`** — validate config (rootDir must exist via `filesystem.ValidateRootfs`, command must be non-empty), return a Container struct
- **`Run()`** — the full lifecycle:
  1. Generate hostname if not set (via `namespace.GenerateHostname`)
  2. Compute clone flags (via `namespace.CloneFlags`)
  3. Set up cgroups (via `cgroup.WriteCgroup`)
  4. Spawn child via `os/exec.Cmd` targeting `/proc/self/exe` with `SysProcAttr.Cloneflags`
  5. **Parent side**: write `/proc/<pid>/uid_map` and `/proc/<pid>/gid_map`, wait for child, clean up cgroups
  6. **Child side** (detected by `os.Args[0] == "child"`): call `syscall.Sethostname` → execute mount plan with `syscall.Mount`/`syscall.PivotRoot` → drop capabilities with `syscall.RawSyscall(SYS_PRCTL, PR_CAPBSET_DROP, ...)` → apply seccomp BPF filter → `syscall.Exec` the target command
- **`ParseArgs(args)`** — parse command-line flags into ContainerConfig

### `cmd/container/main.go`

Route on `os.Args`: if first arg is `"child"` call the child-side code, otherwise parse flags → `container.New` → `container.Run` → `os.Exit(exitCode)`.

### What to test

- `New` rejects empty command, missing rootDir
- `ParseArgs` parses `-rootfs`, `-cmd`, `-uid`, `--memory`, `--cpu-weight`, `--max-pids`
- Full `Run` is an integration test requiring Linux + root — test manually with a busybox rootfs image

### Stdlib references

- [`os/exec`](https://pkg.go.dev/os/exec) — `Command("/proc/self/exe", ...)`, set `cmd.SysProcAttr`
- [`syscall`](https://pkg.go.dev/syscall) — `Sethostname`, `Mount`, `PivotRoot`, `Unmount2`, `RawSyscall` (for prctl/seccomp), `Exec`, `SysProcAttr`
- [`flag`](https://pkg.go.dev/flag) — CLI flag parsing
- [`os`](https://pkg.go.dev/os) — `WriteFile` for uid_map/gid_map writes to `/proc/<pid>/uid_map`

## Running tests

```bash
go test ./namespace/
go test ./filesystem/
go test ./cgroup/
go test ./security/
go test ./container/
go test ./...
```

## Running the container

```bash
sudo go run ./cmd/container/ -rootfs /path/to/busybox-rootfs -cmd /bin/sh
```

Needs Linux + root. Phases 1-4 are testable without privileges.

## Linux primitives reference

| Primitive | What it does | Phase |
|-----------|-------------|-------|
| `clone()` with `CLONE_NEW*` | Create child in new namespaces | 1, 5 |
| `pivot_root()` | Swap root filesystem | 2, 5 |
| `mount()` with `MS_BIND`, `MS_PRIVATE` | Bind mount, prevent propagation | 2, 5 |
| `umount2()` with `MNT_DETACH` | Lazy unmount old root | 2, 5 |
| `sethostname()` | Set container hostname | 1, 5 |
| cgroup v2 filesystem writes | Memory, CPU, PID, I/O limits | 3, 5 |
| `prctl(PR_CAPBSET_DROP)` | Drop capabilities from bounding set | 4, 5 |
| `seccomp(SECCOMP_SET_MODE_FILTER)` | Apply BPF syscall filter | 4, 5 |
| `/proc/<pid>/uid_map`, `gid_map` | Map container UIDs to host UIDs | 5 |
