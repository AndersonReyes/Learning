//! Advanced 10 — Implementing `Vec` and `Arc` from Scratch (Nomicon).
//!
//! Five exercises:
//!   1. `MyVec<T>` — `push` / `pop` / `len` / `is_empty` / `Index`
//!   2. `MyVec<T>` — `insert` / `remove`
//!   3. `IntoIterator for MyVec<T>` (via `MyVecIntoIter<T>`)
//!   4. `MyArc<T>` — `new` / `Deref` / `Clone`
//!   5. `MyArc<T>` — `Drop` (atomic refcount, fence, dealloc)

use std::alloc::{alloc, dealloc, realloc, Layout};
use std::ptr::NonNull;
use std::sync::atomic::{AtomicUsize, Ordering};

// ---------------------------------------------------------------------------
// Exercises 1–3: MyVec<T>
// ---------------------------------------------------------------------------

/// A minimal growable array that manages its own heap allocation.
///
/// Invariants:
/// - `cap == 0` ↔ `ptr == NonNull::dangling()` (no heap allocation).
/// - `len ≤ cap` always.
/// - Elements at indices `0..len` are initialized; `len..cap` are not.
pub struct MyVec<T> {
    ptr: NonNull<T>,
    len: usize,
    cap: usize,
}

// SAFETY: like `Vec<T>`, we require `T: Send`/`T: Sync`.
unsafe impl<T: Send> Send for MyVec<T> {}
unsafe impl<T: Sync> Sync for MyVec<T> {}

impl<T> MyVec<T> {
    /// Creates an empty `MyVec` with no heap allocation.
    pub fn new() -> Self {
        MyVec {
            ptr: NonNull::dangling(),
            len: 0,
            cap: 0,
        }
    }

    /// Returns the number of elements.
    pub fn len(&self) -> usize {
        todo!()
    }

    /// Returns `true` if the vector contains no elements.
    pub fn is_empty(&self) -> bool {
        todo!()
    }

    fn grow(&mut self) {
        let new_cap = if self.cap == 0 { 1 } else { self.cap * 2 };
        let new_layout = Layout::array::<T>(new_cap).expect("capacity overflow");
        let raw = if self.cap == 0 {
            unsafe { alloc(new_layout) }
        } else {
            let old_layout = Layout::array::<T>(self.cap).unwrap();
            unsafe { realloc(self.ptr.as_ptr() as *mut u8, old_layout, new_layout.size()) }
        };
        self.ptr = NonNull::new(raw as *mut T).expect("allocation failed");
        self.cap = new_cap;
    }

    /// Appends `val` to the end, growing the buffer if needed.
    ///
    /// # Example
    /// ```ignore
    /// let mut v: MyVec<i32> = MyVec::new();
    /// v.push(1); v.push(2); v.push(3);
    /// assert_eq!(v.len(), 3);
    /// assert_eq!(v[0], 1);
    /// ```
    pub fn push(&mut self, val: T) {
        todo!()
    }

    /// Removes and returns the last element, or `None` if empty.
    ///
    /// # Example
    /// ```ignore
    /// let mut v: MyVec<i32> = MyVec::new();
    /// v.push(10); v.push(20);
    /// assert_eq!(v.pop(), Some(20));
    /// assert_eq!(v.pop(), Some(10));
    /// assert_eq!(v.pop(), None);
    /// ```
    pub fn pop(&mut self) -> Option<T> {
        todo!()
    }

    /// Inserts `val` at `index`, shifting elements `[index..len]` one slot right.
    ///
    /// # Panics
    /// Panics with `"index out of bounds"` if `index > len`.
    ///
    /// # Example
    /// ```ignore
    /// let mut v: MyVec<i32> = MyVec::new();
    /// v.push(1); v.push(3);
    /// v.insert(1, 2);
    /// assert_eq!(v[0], 1); assert_eq!(v[1], 2); assert_eq!(v[2], 3);
    /// ```
    pub fn insert(&mut self, index: usize, val: T) {
        todo!()
    }

    /// Removes and returns the element at `index`, shifting elements left.
    ///
    /// # Panics
    /// Panics with `"index out of bounds"` if `index >= len`.
    ///
    /// # Example
    /// ```ignore
    /// let mut v: MyVec<i32> = MyVec::new();
    /// v.push(10); v.push(20); v.push(30);
    /// assert_eq!(v.remove(1), 20);
    /// assert_eq!(v.len(), 2);
    /// assert_eq!(v[0], 10); assert_eq!(v[1], 30);
    /// ```
    pub fn remove(&mut self, index: usize) -> T {
        todo!()
    }
}

impl<T> std::ops::Index<usize> for MyVec<T> {
    type Output = T;
    fn index(&self, i: usize) -> &T {
        todo!()
    }
}

impl<T> Drop for MyVec<T> {
    fn drop(&mut self) {
        // Hint: drop_in_place each live element, then dealloc the buffer.
        // If len == 0 and cap == 0, nothing to do (dangling pointer, never allocated).
        unsafe {
            for i in 0..self.len {
                self.ptr.as_ptr().add(i).drop_in_place();
            }
            if self.cap != 0 {
                dealloc(
                    self.ptr.as_ptr() as *mut u8,
                    Layout::array::<T>(self.cap).unwrap(),
                );
            }
        }
    }
}

// ── Exercise 3: IntoIterator ─────────────────────────────────────────────────

/// Consuming iterator for `MyVec<T>`.
pub struct MyVecIntoIter<T> {
    buf: NonNull<T>,
    cap: usize,
    start: usize,
    end: usize,
}

unsafe impl<T: Send> Send for MyVecIntoIter<T> {}
unsafe impl<T: Sync> Sync for MyVecIntoIter<T> {}

impl<T> Iterator for MyVecIntoIter<T> {
    type Item = T;

    /// Yields the next element, or `None` when exhausted.
    fn next(&mut self) -> Option<T> {
        todo!()
    }
}

impl<T> Drop for MyVecIntoIter<T> {
    fn drop(&mut self) {
        // Hint: drop_in_place remaining elements [start..end], then dealloc if cap != 0.
        for i in self.start..self.end {
            unsafe { self.buf.as_ptr().add(i).drop_in_place() };
        }
        if self.cap != 0 {
            unsafe {
                dealloc(
                    self.buf.as_ptr() as *mut u8,
                    Layout::array::<T>(self.cap).unwrap(),
                );
            }
        }
    }
}

impl<T> IntoIterator for MyVec<T> {
    type Item = T;
    type IntoIter = MyVecIntoIter<T>;

    /// Consumes `self` and returns an iterator over its elements.
    ///
    /// Transfer buffer ownership to the iterator; zero out `MyVec` fields so
    /// its `Drop` impl becomes a no-op.
    fn into_iter(mut self) -> MyVecIntoIter<T> {
        todo!()
    }
}

// ---------------------------------------------------------------------------
// Exercises 4–5: MyArc<T>
// ---------------------------------------------------------------------------

struct ArcInner<T> {
    rc: AtomicUsize,
    data: T,
}

/// A thread-safe reference-counted pointer (simplified `std::sync::Arc<T>`).
///
/// `T` must be `Send + Sync` so the shared reference can cross thread
/// boundaries safely.
pub struct MyArc<T> {
    ptr: NonNull<ArcInner<T>>,
}

unsafe impl<T: Send + Sync> Send for MyArc<T> {}
unsafe impl<T: Send + Sync> Sync for MyArc<T> {}

impl<T> MyArc<T> {
    /// Allocates a new inner node with `rc = 1` and returns a `MyArc`
    /// pointing to it.
    ///
    /// # Example
    /// ```ignore
    /// let a = MyArc::new(42_i32);
    /// assert_eq!(*a, 42);
    /// ```
    pub fn new(data: T) -> Self {
        todo!()
    }

    /// Returns the current reference count (for testing only).
    pub fn ref_count(&self) -> usize {
        unsafe { self.ptr.as_ref() }.rc.load(Ordering::Acquire)
    }
}

impl<T> std::ops::Deref for MyArc<T> {
    type Target = T;

    /// Returns a shared reference to the inner data.
    fn deref(&self) -> &T {
        todo!()
    }
}

impl<T> Clone for MyArc<T> {
    /// Increments the reference count and returns a new `MyArc` pointing to
    /// the same allocation.
    ///
    /// # Example
    /// ```ignore
    /// let a = MyArc::new(1_i32);
    /// let b = a.clone();
    /// assert_eq!(a.ref_count(), 2);
    /// ```
    fn clone(&self) -> Self {
        todo!()
    }
}

impl<T> Drop for MyArc<T> {
    /// Decrements the reference count. When it reaches zero, runs `T`'s
    /// destructor and frees the heap allocation.
    ///
    /// Use `Release` ordering on the decrement and an `Acquire` fence before
    /// freeing to synchronize with other threads' `Drop`s.
    fn drop(&mut self) {
        todo!()
    }
}
