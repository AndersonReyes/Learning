# Fearless Concurrency: Threads, Channels, `Mutex`/`Arc`

Book ch. 16. "Fearless" because the ownership/borrowing rules that prevent
data races at compile time *also* apply across threads — most concurrency
bugs become compile errors instead of runtime heisenbugs.

## Threads (ch. 16.1)

`std::thread::spawn(closure)` starts an OS thread running `closure`, and
returns a `JoinHandle<T>` where `T` is the closure's return type.

```rust
use std::thread;

let handle = thread::spawn(|| {
    println!("from a new thread");
    42
});

let result: i32 = handle.join().unwrap(); // blocks until the thread finishes
```

- `join()` returns `Result<T, Box<dyn Any + Send>>` — `Err` if the thread
  **panicked**. `.unwrap()` propagates that panic to the joining thread.
- If the main thread ends before a spawned thread finishes (and it isn't
  joined), the spawned thread is killed early — always `join()` threads you
  depend on.
- **`move` closures**: a closure passed to `thread::spawn` must own
  everything it captures (it might outlive the stack frame that created it).
  `thread::spawn(move || { ... })` moves captured variables into the closure.
  Without `move`, capturing a reference to a local would be a lifetime error
  (the spawned thread's lifetime isn't tied to the caller's stack frame).
- Each `thread::spawn` call has overhead (OS thread creation) — for N tasks,
  spawn N threads, store the `JoinHandle`s in a `Vec`, then join them all
  afterward (joining one at a time, in order, doesn't serialize the *work*,
  since all threads are already running concurrently by the time you join).

```rust
let handles: Vec<_> = (0..4)
    .map(|i| thread::spawn(move || i * i))
    .collect();

let results: Vec<i32> = handles.into_iter().map(|h| h.join().unwrap()).collect();
```

## Message Passing: `mpsc` channels (ch. 16.2)

`std::sync::mpsc::channel()` returns `(Sender<T>, Receiver<T>)` —
**m**ultiple-**p**roducer, **s**ingle-**c**onsumer. `Sender<T>` is `Clone`
(one per producer thread); `Receiver<T>` is not.

```rust
use std::sync::mpsc;
use std::thread;

let (tx, rx) = mpsc::channel();

for i in 0..3 {
    let tx = tx.clone(); // each producer gets its own clone
    thread::spawn(move || {
        tx.send(i).unwrap();
    });
}
drop(tx); // drop the original — otherwise rx never sees "no more senders"

for received in rx {
    println!("got {received}");
}
```

- `tx.send(value)` moves `value` into the channel — returns `Err` only if
  the `Receiver` was already dropped.
- `rx.recv()` blocks until a value arrives or **all** `Sender`s (including
  clones) are dropped, then returns `Err`. `for x in rx` is sugar for
  repeatedly calling `recv()` until that `Err`, i.e. it terminates exactly
  when every `Sender` clone has been dropped.
- **Gotcha**: if you `clone()` the sender into each spawned thread but also
  keep the *original* `tx` alive in the spawning thread (e.g. it's still in
  scope when you start the `for` loop over `rx`), the channel never closes —
  `rx` blocks forever waiting for a value that will never come, because `tx`
  (the original) is still a live sender. Either `drop(tx)` explicitly or let
  it go out of scope *before* iterating `rx`.
- Order across producers is **not guaranteed** — if you need a deterministic
  result from a multi-producer channel, sort the collected values or tag
  them with enough info to reconstruct order.

## Shared State: `Mutex<T>` and `Arc<T>` (ch. 16.3)

`Mutex<T>` (mutual exclusion) gives **runtime-checked** exclusive access —
the multithreaded analog of `RefCell<T>`. `Arc<T>` (**a**tomic **r**eference
**c**ounted) is the multithreaded analog of `Rc<T>`.

```rust
use std::sync::{Arc, Mutex};
use std::thread;

let counter = Arc::new(Mutex::new(0));
let mut handles = vec![];

for _ in 0..10 {
    let counter = Arc::clone(&counter);
    handles.push(thread::spawn(move || {
        let mut num = counter.lock().unwrap();
        *num += 1;
    })); // MutexGuard dropped here, unlocking
}

for handle in handles {
    handle.join().unwrap();
}

println!("{}", *counter.lock().unwrap()); // 10
```

- `mutex.lock()` returns `Result<MutexGuard<T>, PoisonError<..>>`. It's
  `Err` only if **another thread panicked while holding the lock**
  ("poisoned") — `.unwrap()` is standard, propagating that panic.
- `MutexGuard<T>` implements `Deref`/`DerefMut` to `T` (like `RefCell`'s
  `Ref`/`RefMut`) and **unlocks on drop**. Keep the guard's scope as small as
  possible — holding it across a `.join()` or another `.lock()` on the same
  mutex deadlocks (a single thread can't acquire the same `Mutex` twice; two
  threads each holding a lock the other wants deadlock the same way as any
  language).
- **Why not just `Rc<RefCell<T>>`?** Neither `Rc<T>` nor `RefCell<T>` is
  `Sync`/`Send` — the compiler rejects sharing them across threads at
  compile time. `Arc`/`Mutex` are the thread-safe equivalents: `Arc::clone`
  bumps an *atomic* refcount (safe under concurrent access), and `Mutex`
  enforces exclusivity with real OS-level locking instead of a `Cell<bool>`.

## `Send` and `Sync` (ch. 16.4)

Two marker traits (no methods) describing thread-safety, auto-implemented by
the compiler for types composed entirely of `Send`/`Sync` parts:

- **`Send`**: safe to *move* to another thread. Almost everything is `Send`;
  `Rc<T>` is **not** (its refcount isn't atomic — two threads incrementing it
  concurrently could race).
- **`Sync`**: safe to *share* (`&T`) across threads. `Cell<T>`/`RefCell<T>`
  are **not** `Sync` (their interior mutability has no synchronization —
  concurrent `borrow_mut()` from two threads would violate the
  one-mutable-XOR-many-immutable invariant with no runtime check spanning
  threads).
- `thread::spawn`'s closure bound is `F: Send + 'static` (and its return type
  `T: Send + 'static`) — the closure and its result must be safely movable to
  the new thread and must not borrow anything that could be dropped first.
- You almost never implement `Send`/`Sync` yourself — they're auto-derived
  from fields, and manually implementing them (`unsafe impl`) is only for
  low-level primitives that the compiler can't reason about (out of scope
  here; see the Nomicon).

## Gotchas

- A spawned thread's panic does **not** crash the main thread — it's only
  observed when you `.join()` and get `Err`. An un-joined panicking thread
  fails silently.
- `for x in rx` only terminates once **every** `Sender` (original + all
  clones) is dropped — a forgotten clone held alive anywhere hangs the loop
  forever.
- `.lock().unwrap()` panics if the mutex is **poisoned** (a previous holder
  panicked while locked) — in these exercises that only happens if your own
  code panics inside a locked section.
- `Mutex<T>::lock()` from the *same* thread twice (e.g. recursively) deadlocks
  — `Mutex` is not reentrant.
- Prefer message passing (channels) when ownership of data should transfer
  between threads; prefer `Arc<Mutex<T>>` when multiple threads need ongoing
  shared access to the *same* data.

## Further Reading (Book)

- [Ch. 16.1 — Using Threads to Run Code Simultaneously](https://doc.rust-lang.org/book/ch16-01-threads.html)
- [Ch. 16.2 — Using Message Passing to Transfer Data Between Threads](https://doc.rust-lang.org/book/ch16-02-message-passing.html)
- [Ch. 16.3 — Shared-State Concurrency](https://doc.rust-lang.org/book/ch16-03-shared-state.html)
- [Ch. 16.4 — Extensible Concurrency with `Sync` and `Send`](https://doc.rust-lang.org/book/ch16-04-extensible-concurrency-sync-and-send.html)
- [`std::thread`](https://doc.rust-lang.org/std/thread/index.html), [`std::sync::mpsc`](https://doc.rust-lang.org/std/sync/mpsc/index.html)
- [`std::sync::Mutex`](https://doc.rust-lang.org/std/sync/struct.Mutex.html), [`std::sync::Arc`](https://doc.rust-lang.org/std/sync/struct.Arc.html)
- [`std::marker::Send`](https://doc.rust-lang.org/std/marker/trait.Send.html), [`std::marker::Sync`](https://doc.rust-lang.org/std/marker/trait.Sync.html)
