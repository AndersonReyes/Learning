# Concurrency Internals: `Send`, `Sync` & Atomics

Nomicon "Concurrency" (data races, `Send`/`Sync`, atomics).

---

## Data races vs. races in general

A **data race** (undefined behaviour) requires three things simultaneously:
1. Two or more threads access the same memory location
2. At least one access is a write
3. Accesses are unsynchronized

Rust's ownership system prevents data races at compile time through the
`Send` and `Sync` marker traits. A *race condition* (wrong logical ordering
without UB) is still possible but is not UB.

## `Send` and `Sync`

Both are auto-traits: automatically implemented for a type if all its
fields implement the trait, unless opted out with a negative impl.

### `Send: T: Send`
A type is `Send` if it is safe to **move** to another thread. Almost
all types are `Send`; exceptions include:
- `Rc<T>` — reference counted without atomic operations (not thread-safe)
- `*mut T` / `*const T` — raw pointers (opt-in required via `unsafe impl`)
- `MutexGuard<T>` — must be unlocked on the same thread

### `Sync: T: Sync`
A type is `Sync` if it is safe to **share a reference** (`&T`) across threads.
`T: Sync` iff `&T: Send`. Exceptions:
- `Cell<T>` / `RefCell<T>` — interior mutability without synchronization
- `Rc<T>` — same as above

### Manual impl

```rust
// A newtype over *mut u8 that we know is safe to send.
struct MyPtr(*mut u8);
unsafe impl Send for MyPtr {}
unsafe impl Sync for MyPtr {}
```

Only do this when you can *prove* the invariants hold.

## Atomics

`std::sync::atomic` provides types with atomic operations — guaranteed
indivisible on hardware, no data race UB:

```rust
use std::sync::atomic::{AtomicUsize, Ordering};

static COUNTER: AtomicUsize = AtomicUsize::new(0);
COUNTER.fetch_add(1, Ordering::Relaxed);
```

### Ordering

- `Relaxed` — no ordering guarantees, just atomicity. Cheapest.
- `Acquire` — read; paired with `Release` write. No code before the
  `Release` can be reordered after; no code after the `Acquire` before.
- `Release` — write; pairs with `Acquire`. See above.
- `AcqRel` — both `Acquire` and `Release` (for read-modify-write ops).
- `SeqCst` — total sequential consistency across all threads. Expensive.

Common operations:
```rust
let a = AtomicUsize::new(0);
a.load(Ordering::Relaxed);             // read
a.store(5, Ordering::Release);         // write
a.fetch_add(1, Ordering::AcqRel);     // read+add, returns old value
a.compare_exchange(                    // CAS
    expected, new, Ordering::AcqRel, Ordering::Relaxed
);
```

### `compare_exchange` (CAS)

```rust
let result = counter.compare_exchange(
    old,        // expected current value
    new,        // desired new value
    Ordering::AcqRel,   // success ordering
    Ordering::Relaxed,  // failure ordering
);
// Ok(old) on success, Err(actual) on failure
```

Used to build lock-free algorithms.

## Mutex and Arc recap

`Arc<Mutex<T>>` is the idiomatic safe shared-mutable state pattern:

```rust
use std::sync::{Arc, Mutex};
use std::thread;

let data = Arc::new(Mutex::new(vec![0u32; 10]));
let mut handles = vec![];
for i in 0..4 {
    let data = Arc::clone(&data);
    handles.push(thread::spawn(move || {
        let mut guard = data.lock().unwrap();
        guard[i] += 1;
    }));
}
for h in handles { h.join().unwrap(); }
```

## Further Reading

- [Nomicon — Races](https://doc.rust-lang.org/nomicon/races.html)
- [Nomicon — Send and Sync](https://doc.rust-lang.org/nomicon/send-and-sync.html)
- [Nomicon — Atomics](https://doc.rust-lang.org/nomicon/atomics.html)
- [`std::sync::atomic`](https://doc.rust-lang.org/std/sync/atomic/index.html)
- [`std::sync::Mutex`](https://doc.rust-lang.org/std/sync/struct.Mutex.html)
