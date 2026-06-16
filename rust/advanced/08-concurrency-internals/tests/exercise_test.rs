use advanced_08_concurrency_internals::{fetch_max, parallel_sum, spin_until, AtomicCounter, ThreadSafeStack};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

// --- Exercise 1: AtomicCounter -----------------------------------------------

#[test]
fn counter_starts_at_zero() {
    let c = AtomicCounter::new();
    assert_eq!(c.get(), 0);
}

#[test]
fn counter_increment_returns_old() {
    let c = AtomicCounter::new();
    assert_eq!(c.increment(), 0); // old=0, now 1
    assert_eq!(c.increment(), 1); // old=1, now 2
    assert_eq!(c.get(), 2);
}

#[test]
fn counter_reset() {
    let c = AtomicCounter::new();
    c.increment();
    c.increment();
    c.reset();
    assert_eq!(c.get(), 0);
}

#[test]
fn counter_concurrent_increments() {
    let c = Arc::new(AtomicCounter::new());
    let mut handles = vec![];
    for _ in 0..100 {
        let c = Arc::clone(&c);
        handles.push(thread::spawn(move || { c.increment(); }));
    }
    for h in handles { h.join().unwrap(); }
    assert_eq!(c.get(), 100);
}

// --- Exercise 2: parallel_sum ------------------------------------------------

#[test]
fn parallel_sum_typical() {
    let data: Vec<u64> = (1..=100).collect();
    assert_eq!(parallel_sum(data, 4), 5050);
}

#[test]
fn parallel_sum_empty() {
    assert_eq!(parallel_sum(vec![], 2), 0);
}

#[test]
fn parallel_sum_zero_threads() {
    let data: Vec<u64> = (1..=10).collect();
    assert_eq!(parallel_sum(data, 0), 0);
}

#[test]
fn parallel_sum_single_thread() {
    let data: Vec<u64> = (1..=100).collect();
    assert_eq!(parallel_sum(data, 1), 5050);
}

#[test]
fn parallel_sum_more_threads_than_elements() {
    let data: Vec<u64> = vec![1, 2, 3];
    assert_eq!(parallel_sum(data, 10), 6);
}

// --- Exercise 3: spin_until --------------------------------------------------

#[test]
fn spin_until_basic() {
    let flag = Arc::new(AtomicBool::new(false));
    let spins = spin_until(Arc::clone(&flag), 20);
    assert!(spins > 0);
    assert!(flag.load(Ordering::Relaxed));
}

#[test]
fn spin_until_returns_positive_spins() {
    let flag = Arc::new(AtomicBool::new(false));
    let spins = spin_until(Arc::clone(&flag), 5);
    assert!(spins >= 1);
}

// --- Exercise 4: fetch_max ---------------------------------------------------

#[test]
fn fetch_max_no_update() {
    let a = AtomicUsize::new(5);
    let old = fetch_max(&a, 3);
    assert_eq!(old, 5);
    assert_eq!(a.load(Ordering::Relaxed), 5);
}

#[test]
fn fetch_max_with_update() {
    let a = AtomicUsize::new(5);
    let old = fetch_max(&a, 10);
    assert_eq!(old, 5);
    assert_eq!(a.load(Ordering::Relaxed), 10);
}

#[test]
fn fetch_max_equal_no_update() {
    let a = AtomicUsize::new(7);
    let old = fetch_max(&a, 7);
    assert_eq!(old, 7);
    assert_eq!(a.load(Ordering::Relaxed), 7);
}

#[test]
fn fetch_max_zero_baseline() {
    let a = AtomicUsize::new(0);
    assert_eq!(fetch_max(&a, 42), 0);
    assert_eq!(a.load(Ordering::Relaxed), 42);
}

#[test]
fn fetch_max_sequential() {
    let a = AtomicUsize::new(0);
    for v in [3, 1, 4, 1, 5, 9, 2, 6] {
        fetch_max(&a, v);
    }
    assert_eq!(a.load(Ordering::Relaxed), 9);
}

// --- Exercise 5: ThreadSafeStack ---------------------------------------------

#[test]
fn stack_push_pop() {
    let s: ThreadSafeStack<i32> = ThreadSafeStack::new();
    s.push(1);
    s.push(2);
    s.push(3);
    assert_eq!(s.pop(), Some(3));
    assert_eq!(s.pop(), Some(2));
    assert_eq!(s.pop(), Some(1));
    assert_eq!(s.pop(), None);
}

#[test]
fn stack_len_and_is_empty() {
    let s: ThreadSafeStack<i32> = ThreadSafeStack::new();
    assert!(s.is_empty());
    s.push(10);
    assert_eq!(s.len(), 1);
    s.push(20);
    assert_eq!(s.len(), 2);
    s.pop();
    assert_eq!(s.len(), 1);
}

#[test]
fn stack_shared_across_threads() {
    let s: ThreadSafeStack<u32> = ThreadSafeStack::new();
    let s2 = s.clone();
    let handle = thread::spawn(move || {
        for i in 0..50 { s2.push(i); }
    });
    handle.join().unwrap();
    assert_eq!(s.len(), 50);
}

#[test]
fn stack_concurrent_push_pop() {
    let s: ThreadSafeStack<u32> = ThreadSafeStack::new();
    let mut handles = vec![];
    for _ in 0..4 {
        let s2 = s.clone();
        handles.push(thread::spawn(move || {
            for i in 0..25 { s2.push(i); }
        }));
    }
    for h in handles { h.join().unwrap(); }
    assert_eq!(s.len(), 100);
}
