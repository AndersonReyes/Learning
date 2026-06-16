# Advanced 10 — Implementing `Vec` and `Arc` from Scratch

Primary references: Nomicon ch. "Implementing Vec", "Implementing Arc and
Mutex". This topic is the capstone of the Advanced tier: you write the
allocator-facing, pointer-manipulating, thread-safety-proving code that the
standard library hides behind safe APIs.

---

## 1. Raw allocation with `std::alloc`

```rust
use std::alloc::{alloc, dealloc, realloc, Layout};

// Allocate space for `cap` values of type T.
let layout = Layout::array::<T>(cap).unwrap();
let ptr: *mut T = unsafe { alloc(layout) as *mut T };

// Deallocate:
unsafe { dealloc(ptr as *mut u8, layout); }

// Realloc (grow):
let new_layout = Layout::array::<T>(new_cap).unwrap();
let new_ptr = unsafe { realloc(ptr as *mut u8, old_layout, new_layout.size()) as *mut T };
```

`Layout::array::<T>(n)` returns `Err` if `n * size_of::<T>()` overflows
`isize::MAX`. Treat that as a capacity overflow (abort or panic).

Zero-sized types (ZSTs) never need allocation. A common trick is to use
`NonNull::dangling()` as the backing pointer when `size_of::<T>() == 0` or
`cap == 0`.

## 2. `NonNull<T>` and pointer hygiene

`NonNull<T>` is a non-null `*mut T` with variance `+T`. It carries no
aliasing guarantee — the programmer must uphold Rust's aliasing rules. The
canonical pattern for a growable buffer:

```rust
use std::ptr::NonNull;

struct RawBuf<T> {
    ptr: NonNull<T>,
    cap: usize,
}
```

Write through `ptr.as_ptr()` (returns `*mut T`). Read back with
`ptr.as_ptr().add(i).read()` or `&*ptr.as_ptr().add(i)`.

## 3. `ptr::write` vs assignment; `ptr::read` vs move

- **`ptr::write(dst, val)`** — moves `val` into the raw slot without
  dropping what was there. Use this when the slot is uninitialized.
- **`ptr::read(src)`** — copies the bits out without running the source's
  destructor. Use this when you want to logically move out of a slot that
  will later be overwritten or freed.
- **`ptr::drop_in_place(p)`** — runs `T`'s destructor without freeing the
  memory. Use this in your `Vec`'s `drop` impl to destroy live elements
  before deallocating the buffer.

## 4. Building `MyVec<T>`

Minimal interface: `new`, `push`, `pop`, `insert`, `remove`, `len`, `is_empty`, indexing.

Growth strategy: double capacity on push when full (start at 1 if `cap == 0`).

```rust
pub struct MyVec<T> {
    ptr: NonNull<T>,
    len: usize,
    cap: usize,
}

// SAFETY: we manage allocation; T governs Send/Sync like Vec does.
unsafe impl<T: Send> Send for MyVec<T> {}
unsafe impl<T: Sync> Sync for MyVec<T> {}

impl<T> MyVec<T> {
    pub fn new() -> Self {
        MyVec { ptr: NonNull::dangling(), len: 0, cap: 0 }
    }

    fn grow(&mut self) {
        let new_cap = if self.cap == 0 { 1 } else { self.cap * 2 };
        let new_layout = Layout::array::<T>(new_cap).unwrap();
        let new_ptr = if self.cap == 0 {
            unsafe { alloc(new_layout) }
        } else {
            let old_layout = Layout::array::<T>(self.cap).unwrap();
            unsafe { realloc(self.ptr.as_ptr() as *mut u8, old_layout, new_layout.size()) }
        };
        self.ptr = NonNull::new(new_ptr as *mut T).expect("allocation failed");
        self.cap = new_cap;
    }

    pub fn push(&mut self, val: T) {
        if self.len == self.cap { self.grow(); }
        unsafe { self.ptr.as_ptr().add(self.len).write(val); }
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 { return None; }
        self.len -= 1;
        Some(unsafe { self.ptr.as_ptr().add(self.len).read() })
    }
}

impl<T> Drop for MyVec<T> {
    fn drop(&mut self) {
        unsafe {
            // Drop each live element.
            for i in 0..self.len {
                self.ptr.as_ptr().add(i).drop_in_place();
            }
            // Free the buffer if we own one.
            if self.cap != 0 {
                dealloc(self.ptr.as_ptr() as *mut u8,
                        Layout::array::<T>(self.cap).unwrap());
            }
        }
    }
}
```

`insert(index, val)` must shift elements `[index..len]` one slot right using
`ptr::copy` (overlapping is fine). `remove(index)` shifts left.

## 5. Reference-counting with `Arc`

`Arc<T>` stores the value and a reference count in a single heap allocation
("the inner node"). The count is `AtomicUsize`. `Clone` increments,
`Drop` decrements. When the count reaches zero, `Drop` runs the destructor
and frees the allocation.

```rust
use std::sync::atomic::{AtomicUsize, Ordering};

struct ArcInner<T> {
    rc: AtomicUsize,
    data: T,
}

pub struct MyArc<T> {
    ptr: NonNull<ArcInner<T>>,
}

unsafe impl<T: Send + Sync> Send for MyArc<T> {}
unsafe impl<T: Send + Sync> Sync for MyArc<T> {}
```

**Why `T: Send + Sync`?** The standard library requires `T: Send + Sync`
for `Arc<T>` to be `Send + Sync`:

- `Send` — a live reference can be sent to another thread (safe because the
  ref count is atomic).
- `Sync` — multiple threads may hold a reference simultaneously (shared
  access through `&T` is only safe if `T: Sync`).

**Count ordering:** Use `Relaxed` for `fetch_add` on `Clone` (no data needs
to synchronize), and `AcqRel`/`Acquire` for the final `fetch_sub`/load in
`Drop`:

```rust
impl<T> Clone for MyArc<T> {
    fn clone(&self) -> Self {
        unsafe { self.ptr.as_ref() }.rc.fetch_add(1, Ordering::Relaxed);
        MyArc { ptr: self.ptr }
    }
}

impl<T> Drop for MyArc<T> {
    fn drop(&mut self) {
        let inner = unsafe { self.ptr.as_ref() };
        if inner.rc.fetch_sub(1, Ordering::Release) != 1 {
            return; // other copies still live
        }
        // Ensure all stores from other threads are visible before we drop.
        std::sync::atomic::fence(Ordering::Acquire);
        unsafe { Box::from_raw(self.ptr.as_ptr()); } // drops ArcInner<T>
    }
}
```

Using `Box::from_raw` to free the inner node lets us drop `T` and free the
allocation in one step (Box's own `Drop`).

## 6. `Deref` for `MyArc<T>`

```rust
impl<T> std::ops::Deref for MyArc<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &unsafe { self.ptr.as_ref() }.data
    }
}
```

## 7. `IntoIterator` for `MyVec<T>`

Consuming iteration requires moving elements out one at a time. A simple
approach: keep a `start` and `end` index and `ptr::read` each element:

```rust
pub struct MyVecIntoIter<T> {
    buf: NonNull<T>,
    cap: usize,
    start: usize,
    end: usize,
}

impl<T> Iterator for MyVecIntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        if self.start == self.end { return None; }
        let val = unsafe { self.buf.as_ptr().add(self.start).read() };
        self.start += 1;
        Some(val)
    }
}

impl<T> Drop for MyVecIntoIter<T> {
    fn drop(&mut self) {
        // Drop remaining un-consumed elements, then free the buffer.
        for i in self.start..self.end {
            unsafe { self.buf.as_ptr().add(i).drop_in_place(); }
        }
        if self.cap != 0 {
            unsafe { dealloc(self.buf.as_ptr() as *mut u8,
                             Layout::array::<T>(self.cap).unwrap()); }
        }
    }
}
```

When `IntoIterator` is implemented, `MyVec` is consumed by the iterator's
`Drop`, so `MyVec`'s own `Drop` must not double-free. One clean pattern: set
`self.len = 0` and `self.cap = 0` in `into_iter` so `MyVec`'s `Drop` becomes
a no-op.

## 8. Common pitfalls

| Pitfall | Fix |
|---------|-----|
| Running destructor on uninitialized memory | Use `ptr::write` for init, `ptr::read` for consuming reads |
| Double-free: both `MyVec::drop` and `IntoIter::drop` free the same buffer | Zero out `MyVec.cap` when transferring ownership to the iterator |
| `realloc` called with the old allocation size, not the new one | Pass `new_layout.size()` as the third argument |
| `fetch_add(1, Relaxed)` on `Arc` is safe, but `fetch_sub` needs `Release` then `Acquire` fence | See §5 |
| Forgetting to `drop_in_place` elements before `dealloc` | Elements with destructors (e.g. `String`) would leak |

---

## Further Reading

- Nomicon — [Implementing Vec](https://doc.rust-lang.org/nomicon/vec/vec.html)
- Nomicon — [Implementing Arc and Mutex](https://doc.rust-lang.org/nomicon/arc-mutex/arc.html)
- Nomicon — [Uninitialized Memory](https://doc.rust-lang.org/nomicon/uninitialized.html)
- Nomicon — [Ownership Based Resource Management](https://doc.rust-lang.org/nomicon/obrm.html)
- `std` — [`std::alloc`](https://doc.rust-lang.org/std/alloc/index.html)
- `std` — [`std::ptr`](https://doc.rust-lang.org/std/ptr/index.html)
- `std` — [`std::sync::atomic`](https://doc.rust-lang.org/std/sync/atomic/index.html)
