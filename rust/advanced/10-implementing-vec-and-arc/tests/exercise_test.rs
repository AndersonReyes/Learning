use advanced_10_implementing_vec_and_arc::{MyArc, MyVec};

// ── Exercise 1: MyVec push / pop / len / is_empty / Index ────────────────────

#[test]
fn myvec_new_is_empty() {
    let v: MyVec<i32> = MyVec::new();
    assert!(v.is_empty());
    assert_eq!(v.len(), 0);
}

#[test]
fn myvec_push_and_len() {
    let mut v: MyVec<i32> = MyVec::new();
    v.push(10);
    v.push(20);
    v.push(30);
    assert_eq!(v.len(), 3);
    assert!(!v.is_empty());
}

#[test]
fn myvec_index() {
    let mut v: MyVec<i32> = MyVec::new();
    v.push(100);
    v.push(200);
    v.push(300);
    assert_eq!(v[0], 100);
    assert_eq!(v[1], 200);
    assert_eq!(v[2], 300);
}

#[test]
fn myvec_pop_order() {
    let mut v: MyVec<i32> = MyVec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    assert_eq!(v.pop(), Some(3));
    assert_eq!(v.pop(), Some(2));
    assert_eq!(v.pop(), Some(1));
    assert_eq!(v.pop(), None);
}

#[test]
fn myvec_pop_empty() {
    let mut v: MyVec<i32> = MyVec::new();
    assert_eq!(v.pop(), None);
}

#[test]
fn myvec_push_many_triggers_growth() {
    // Forces at least 4 doublings (1→2→4→8→16).
    let mut v: MyVec<i32> = MyVec::new();
    for i in 0..20 {
        v.push(i);
    }
    assert_eq!(v.len(), 20);
    for i in 0..20 {
        assert_eq!(v[i], i as i32);
    }
}

#[test]
fn myvec_drop_runs_element_dtors() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    let count = Arc::new(AtomicUsize::new(0));
    struct Bomb(Arc<AtomicUsize>);
    impl Drop for Bomb {
        fn drop(&mut self) {
            self.0.fetch_add(1, Ordering::Relaxed);
        }
    }

    {
        let mut v: MyVec<Bomb> = MyVec::new();
        for _ in 0..5 {
            v.push(Bomb(Arc::clone(&count)));
        }
        assert_eq!(count.load(Ordering::Relaxed), 0);
    } // v is dropped here
    assert_eq!(count.load(Ordering::Relaxed), 5);
}

// ── Exercise 2: MyVec insert / remove ────────────────────────────────────────

#[test]
fn myvec_insert_at_start() {
    let mut v: MyVec<i32> = MyVec::new();
    v.push(2);
    v.push(3);
    v.insert(0, 1);
    assert_eq!(v[0], 1);
    assert_eq!(v[1], 2);
    assert_eq!(v[2], 3);
    assert_eq!(v.len(), 3);
}

#[test]
fn myvec_insert_at_middle() {
    let mut v: MyVec<i32> = MyVec::new();
    v.push(1);
    v.push(3);
    v.insert(1, 2);
    assert_eq!(v[0], 1);
    assert_eq!(v[1], 2);
    assert_eq!(v[2], 3);
}

#[test]
fn myvec_insert_at_end() {
    let mut v: MyVec<i32> = MyVec::new();
    v.push(1);
    v.push(2);
    v.insert(2, 3);
    assert_eq!(v[2], 3);
    assert_eq!(v.len(), 3);
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn myvec_insert_out_of_bounds() {
    let mut v: MyVec<i32> = MyVec::new();
    v.push(1);
    v.insert(5, 99);
}

#[test]
fn myvec_remove_middle() {
    let mut v: MyVec<i32> = MyVec::new();
    v.push(10);
    v.push(20);
    v.push(30);
    let removed = v.remove(1);
    assert_eq!(removed, 20);
    assert_eq!(v.len(), 2);
    assert_eq!(v[0], 10);
    assert_eq!(v[1], 30);
}

#[test]
fn myvec_remove_first() {
    let mut v: MyVec<i32> = MyVec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    assert_eq!(v.remove(0), 1);
    assert_eq!(v[0], 2);
    assert_eq!(v[1], 3);
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn myvec_remove_out_of_bounds() {
    let mut v: MyVec<i32> = MyVec::new();
    v.push(1);
    v.remove(5);
}

// ── Exercise 3: IntoIterator ──────────────────────────────────────────────────

#[test]
fn myvec_into_iter_collects() {
    let mut v: MyVec<i32> = MyVec::new();
    for i in 1..=5 {
        v.push(i);
    }
    let collected: Vec<i32> = v.into_iter().collect();
    assert_eq!(collected, vec![1, 2, 3, 4, 5]);
}

#[test]
fn myvec_into_iter_empty() {
    let v: MyVec<i32> = MyVec::new();
    let collected: Vec<i32> = v.into_iter().collect();
    assert!(collected.is_empty());
}

#[test]
fn myvec_into_iter_drops_remaining() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    let count = Arc::new(AtomicUsize::new(0));
    struct Bomb(Arc<AtomicUsize>);
    impl Drop for Bomb {
        fn drop(&mut self) {
            self.0.fetch_add(1, Ordering::Relaxed);
        }
    }

    let mut v: MyVec<Bomb> = MyVec::new();
    for _ in 0..4 {
        v.push(Bomb(Arc::clone(&count)));
    }

    let mut iter = v.into_iter();
    // consume 2, leave 2
    iter.next();
    iter.next();
    assert_eq!(count.load(Ordering::Relaxed), 2);
    drop(iter); // should drop the remaining 2
    assert_eq!(count.load(Ordering::Relaxed), 4);
}

#[test]
fn myvec_into_iter_sum() {
    let mut v: MyVec<i64> = MyVec::new();
    for i in 1..=100 {
        v.push(i);
    }
    let sum: i64 = v.into_iter().sum();
    assert_eq!(sum, 5050);
}

// ── Exercise 4: MyArc new / Deref / Clone ────────────────────────────────────

#[test]
fn myarc_deref() {
    let a = MyArc::new(42_i32);
    assert_eq!(*a, 42);
}

#[test]
fn myarc_clone_increments_count() {
    let a = MyArc::new(10_i32);
    assert_eq!(a.ref_count(), 1);
    let b = a.clone();
    assert_eq!(a.ref_count(), 2);
    assert_eq!(b.ref_count(), 2);
}

#[test]
fn myarc_multiple_clones() {
    let a = MyArc::new(99_i32);
    let b = a.clone();
    let c = a.clone();
    let d = b.clone();
    assert_eq!(a.ref_count(), 4);
    drop(b);
    assert_eq!(a.ref_count(), 3);
    drop(c);
    assert_eq!(a.ref_count(), 2);
    drop(d);
    assert_eq!(a.ref_count(), 1);
}

#[test]
fn myarc_deref_string() {
    let a = MyArc::new(String::from("hello"));
    assert_eq!(a.len(), 5); // deref coercion to &String then &str
    let b = a.clone();
    assert_eq!(*b, "hello");
}

// ── Exercise 5: MyArc Drop — refcount, dealloc, thread-safety ────────────────

#[test]
fn myarc_drop_runs_destructor() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    let count = Arc::new(AtomicUsize::new(0));
    struct Bomb(Arc<AtomicUsize>);
    impl Drop for Bomb {
        fn drop(&mut self) {
            self.0.fetch_add(1, Ordering::Relaxed);
        }
    }

    let a = MyArc::new(Bomb(Arc::clone(&count)));
    let b = a.clone();
    assert_eq!(count.load(Ordering::Relaxed), 0);
    drop(a);
    assert_eq!(count.load(Ordering::Relaxed), 0); // still one reference
    drop(b);
    assert_eq!(count.load(Ordering::Relaxed), 1); // last ref dropped
}

#[test]
fn myarc_send_to_thread() {
    use std::thread;

    let a = MyArc::new(vec![1_i32, 2, 3]);
    let b = a.clone();

    let handle = thread::spawn(move || {
        assert_eq!(*b, vec![1, 2, 3]);
        b.ref_count() // may be 1 or 2 depending on timing
    });

    let _thread_count = handle.join().unwrap();
    assert_eq!(a.ref_count(), 1);
    assert_eq!(*a, vec![1, 2, 3]);
}

#[test]
fn myarc_many_threads_share() {
    use std::thread;

    let a = MyArc::new(42_i32);
    let mut handles = vec![];

    for _ in 0..8 {
        let clone = a.clone();
        handles.push(thread::spawn(move || {
            assert_eq!(*clone, 42);
        }));
    }

    for h in handles {
        h.join().unwrap();
    }
    assert_eq!(a.ref_count(), 1);
}
