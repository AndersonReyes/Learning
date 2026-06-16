//! Run with: `cargo run --example examples -p advanced-08-concurrency-internals`

use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // --- AtomicUsize basics --------------------------------------------------
    println!("-- atomic basics --");

    static GLOBAL: AtomicUsize = AtomicUsize::new(0);
    let a = AtomicUsize::new(10);
    let old = a.fetch_add(5, Ordering::AcqRel);
    println!("fetch_add(5) old={old}, new={}", a.load(Ordering::Acquire));

    // concurrent increments
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];
    for _ in 0..4 {
        let c = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..25 {
                c.fetch_add(1, Ordering::AcqRel);
            }
        }));
    }
    for h in handles { h.join().unwrap(); }
    println!("4 threads × 25 increments = {}", counter.load(Ordering::Acquire));

    // --- compare_exchange (CAS) ----------------------------------------------
    println!("\n-- compare_exchange (CAS) --");

    let val = AtomicUsize::new(5);
    match val.compare_exchange(5, 10, Ordering::AcqRel, Ordering::Acquire) {
        Ok(old) => println!("CAS ok: was {old}, now {}", val.load(Ordering::Acquire)),
        Err(actual) => println!("CAS failed: actual={actual}"),
    }
    // expected value now wrong — CAS fails
    match val.compare_exchange(5, 99, Ordering::AcqRel, Ordering::Acquire) {
        Ok(old) => println!("CAS ok: was {old}"),
        Err(actual) => println!("CAS failed: actual={actual}, value still {}", val.load(Ordering::Acquire)),
    }

    // --- AtomicBool flag -----------------------------------------------------
    println!("\n-- AtomicBool flag --");

    let ready = Arc::new(AtomicBool::new(false));
    let r = Arc::clone(&ready);
    let worker = thread::spawn(move || {
        thread::sleep(std::time::Duration::from_millis(10));
        r.store(true, Ordering::Release);
        println!("  worker: flag set");
    });
    let mut spins = 0u64;
    while !ready.load(Ordering::Acquire) {
        spins += 1;
        std::hint::spin_loop();
    }
    worker.join().unwrap();
    println!("main: spun {} times", spins);

    // --- Arc<Mutex<T>> shared state ------------------------------------------
    println!("\n-- Arc<Mutex<T>> --");

    let shared = Arc::new(Mutex::new(Vec::<u32>::new()));
    let mut handles = vec![];
    for i in 0..4_u32 {
        let s = Arc::clone(&shared);
        handles.push(thread::spawn(move || {
            s.lock().unwrap().push(i * 10);
        }));
    }
    for h in handles { h.join().unwrap(); }
    let mut v = shared.lock().unwrap().clone();
    v.sort();
    println!("shared vec: {:?}", v);

    // --- Send / Sync ---------------------------------------------------------
    println!("\n-- Send / Sync --");

    // Arc<T>: Send + Sync when T: Send + Sync
    let arc_str: Arc<String> = Arc::new("hello".into());
    let arc2 = Arc::clone(&arc_str);
    let h = thread::spawn(move || println!("  from thread: {}", arc2));
    h.join().unwrap();

    // Rc is NOT Send — would fail at compile time
    // let rc = std::rc::Rc::new(5);
    // thread::spawn(move || println!("{}", rc)); // error: Rc<i32>: !Send
}
