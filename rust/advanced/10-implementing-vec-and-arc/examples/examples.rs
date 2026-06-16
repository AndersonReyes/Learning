use std::alloc::{alloc, dealloc, realloc, Layout};
use std::ptr::NonNull;
use std::sync::atomic::{fence, AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

// ── 1. Raw allocation ─────────────────────────────────────────────────────────
fn allocation_demo() {
    println!("=== Raw allocation ===");

    // Allocate space for 4 i32s.
    let cap = 4_usize;
    let layout = Layout::array::<i32>(cap).unwrap();
    let ptr = unsafe { alloc(layout) as *mut i32 };

    // Write values with ptr::write (the slots are uninitialized — no assignment!).
    for i in 0..cap {
        unsafe { ptr.add(i).write(i as i32 * 10) };
    }

    // Read back.
    for i in 0..cap {
        let val = unsafe { ptr.add(i).read() };
        println!("  slot[{i}] = {val}");
    }

    // Grow: double capacity via realloc.
    let new_cap = cap * 2;
    let new_layout = Layout::array::<i32>(new_cap).unwrap();
    let ptr = unsafe { realloc(ptr as *mut u8, layout, new_layout.size()) as *mut i32 };
    println!("  grown to cap {new_cap}, ptr still valid");

    // Dealloc.
    unsafe { dealloc(ptr as *mut u8, new_layout) };
    println!("  deallocated");
}

// ── 2. ptr::write vs assignment, ptr::read, drop_in_place ────────────────────
fn ptr_write_read_demo() {
    println!("\n=== ptr::write / ptr::read / drop_in_place ===");

    let layout = Layout::new::<String>();
    let ptr = unsafe { alloc(layout) as *mut String };

    // Use ptr::write — not assignment — because the slot is uninitialized.
    // Assignment would try to drop the "old" value, which is UB on uninit memory.
    unsafe { ptr.write(String::from("hello")) };
    println!("  wrote: {}", unsafe { &*ptr });

    // ptr::read moves the value out without running its destructor.
    let s: String = unsafe { ptr.read() };
    println!("  read back: {s}");
    // The slot is now logically uninitialized again; `s` owns the String.

    // Re-initialize and use drop_in_place to destroy without freeing.
    unsafe { ptr.write(String::from("world")) };
    unsafe { ptr.drop_in_place() };
    println!("  drop_in_place ran (String freed its heap buffer)");

    unsafe { dealloc(ptr as *mut u8, layout) };
}

// ── 3. Minimal MyVec<T> ───────────────────────────────────────────────────────
struct MyVec<T> {
    ptr: NonNull<T>,
    len: usize,
    cap: usize,
}

impl<T> MyVec<T> {
    fn new() -> Self {
        MyVec { ptr: NonNull::dangling(), len: 0, cap: 0 }
    }

    fn grow(&mut self) {
        let new_cap = if self.cap == 0 { 1 } else { self.cap * 2 };
        let new_layout = Layout::array::<T>(new_cap).unwrap();
        let raw = if self.cap == 0 {
            unsafe { alloc(new_layout) }
        } else {
            let old = Layout::array::<T>(self.cap).unwrap();
            unsafe { realloc(self.ptr.as_ptr() as *mut u8, old, new_layout.size()) }
        };
        self.ptr = NonNull::new(raw as *mut T).expect("alloc failed");
        self.cap = new_cap;
    }

    fn push(&mut self, val: T) {
        if self.len == self.cap { self.grow(); }
        unsafe { self.ptr.as_ptr().add(self.len).write(val) };
        self.len += 1;
    }

    fn pop(&mut self) -> Option<T> {
        if self.len == 0 { return None; }
        self.len -= 1;
        Some(unsafe { self.ptr.as_ptr().add(self.len).read() })
    }
}

impl<T> Drop for MyVec<T> {
    fn drop(&mut self) {
        unsafe {
            for i in 0..self.len { self.ptr.as_ptr().add(i).drop_in_place(); }
            if self.cap != 0 {
                dealloc(self.ptr.as_ptr() as *mut u8, Layout::array::<T>(self.cap).unwrap());
            }
        }
    }
}

fn myvec_demo() {
    println!("\n=== MyVec<i32> ===");
    let mut v: MyVec<i32> = MyVec::new();
    for i in 1..=5 { v.push(i * 10); }
    println!("  len={}, cap={}", v.len, v.cap);
    while let Some(x) = v.pop() {
        print!("  popped {x}");
    }
    println!();
}

// ── 4. MyVec::insert / remove demo ───────────────────────────────────────────
struct MyVec2<T> {
    ptr: NonNull<T>,
    len: usize,
    cap: usize,
}

impl<T: std::fmt::Debug> MyVec2<T> {
    fn new() -> Self { MyVec2 { ptr: NonNull::dangling(), len: 0, cap: 0 } }

    fn grow(&mut self) {
        let new_cap = if self.cap == 0 { 1 } else { self.cap * 2 };
        let nl = Layout::array::<T>(new_cap).unwrap();
        let raw = if self.cap == 0 {
            unsafe { alloc(nl) }
        } else {
            unsafe { realloc(self.ptr.as_ptr() as *mut u8, Layout::array::<T>(self.cap).unwrap(), nl.size()) }
        };
        self.ptr = NonNull::new(raw as *mut T).unwrap();
        self.cap = new_cap;
    }

    fn push(&mut self, val: T) {
        if self.len == self.cap { self.grow(); }
        unsafe { self.ptr.as_ptr().add(self.len).write(val) };
        self.len += 1;
    }

    fn insert(&mut self, index: usize, val: T) {
        assert!(index <= self.len, "out of bounds");
        if self.len == self.cap { self.grow(); }
        unsafe {
            let p = self.ptr.as_ptr().add(index);
            std::ptr::copy(p, p.add(1), self.len - index);
            p.write(val);
        }
        self.len += 1;
    }

    fn remove(&mut self, index: usize) -> T {
        assert!(index < self.len, "out of bounds");
        let val = unsafe { self.ptr.as_ptr().add(index).read() };
        unsafe {
            let p = self.ptr.as_ptr().add(index);
            std::ptr::copy(p.add(1), p, self.len - index - 1);
        }
        self.len -= 1;
        val
    }

    fn as_slice(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(self.ptr.as_ptr(), self.len) }
    }
}

impl<T> Drop for MyVec2<T> {
    fn drop(&mut self) {
        unsafe {
            for i in 0..self.len { self.ptr.as_ptr().add(i).drop_in_place(); }
            if self.cap != 0 { dealloc(self.ptr.as_ptr() as *mut u8, Layout::array::<T>(self.cap).unwrap()); }
        }
    }
}

fn insert_remove_demo() {
    println!("\n=== insert / remove ===");
    let mut v: MyVec2<i32> = MyVec2::new();
    v.push(1); v.push(3); v.push(5);
    v.insert(1, 2);
    v.insert(3, 4);
    println!("  after inserts: {:?}", v.as_slice()); // [1,2,3,4,5]
    let r = v.remove(2);
    println!("  removed index 2 ({}), remaining: {:?}", r, v.as_slice()); // [1,2,4,5]
}

// ── 5. MyArc<T> with atomic refcount ─────────────────────────────────────────
struct ArcInner<T> { rc: AtomicUsize, data: T }

struct MyArc<T> { ptr: NonNull<ArcInner<T>> }

unsafe impl<T: Send + Sync> Send for MyArc<T> {}
unsafe impl<T: Send + Sync> Sync for MyArc<T> {}

impl<T> MyArc<T> {
    fn new(data: T) -> Self {
        let inner = Box::new(ArcInner { rc: AtomicUsize::new(1), data });
        MyArc { ptr: NonNull::new(Box::into_raw(inner)).unwrap() }
    }

    fn ref_count(&self) -> usize {
        unsafe { self.ptr.as_ref() }.rc.load(Ordering::Acquire)
    }
}

impl<T> Clone for MyArc<T> {
    fn clone(&self) -> Self {
        unsafe { self.ptr.as_ref() }.rc.fetch_add(1, Ordering::Relaxed);
        MyArc { ptr: self.ptr }
    }
}

impl<T> std::ops::Deref for MyArc<T> {
    type Target = T;
    fn deref(&self) -> &T { &unsafe { self.ptr.as_ref() }.data }
}

impl<T> Drop for MyArc<T> {
    fn drop(&mut self) {
        let inner = unsafe { self.ptr.as_ref() };
        if inner.rc.fetch_sub(1, Ordering::Release) != 1 { return; }
        // Acquire fence ensures all `Release` decrements from other threads
        // are visible before we destroy the inner node.
        fence(Ordering::Acquire);
        unsafe { drop(Box::from_raw(self.ptr.as_ptr())) };
    }
}

fn myarc_demo() {
    println!("\n=== MyArc<i32> ===");
    let a = MyArc::new(42_i32);
    println!("  created: value={}, rc={}", *a, a.ref_count());

    let b = a.clone();
    let c = a.clone();
    println!("  after 2 clones: rc={}", a.ref_count()); // 3

    drop(b);
    println!("  after drop b: rc={}", a.ref_count()); // 2
    drop(c);
    println!("  after drop c: rc={}", a.ref_count()); // 1
}

fn myarc_thread_demo() {
    println!("\n=== MyArc shared across threads ===");
    let counter = Arc::new(AtomicUsize::new(0));
    let a = MyArc::new(100_i32);

    let mut handles = vec![];
    for _ in 0..4 {
        let clone = a.clone();
        let c = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            c.fetch_add(*clone as usize, Ordering::Relaxed);
        }));
    }
    for h in handles { h.join().unwrap(); }

    println!("  sum from 4 threads: {}", counter.load(Ordering::Relaxed)); // 400
    println!("  final rc: {}", a.ref_count()); // 1
}

fn main() {
    allocation_demo();
    ptr_write_read_demo();
    myvec_demo();
    insert_remove_demo();
    myarc_demo();
    myarc_thread_demo();
}
