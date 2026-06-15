//! Run with: `cargo run --example examples -p intermediate-09-fearless-concurrency`

use std::sync::{mpsc, Arc, Mutex};
use std::thread;

fn main() {
    // --- thread::spawn + join -----------------------------------------------------
    println!("-- thread::spawn + join --");

    let handle = thread::spawn(|| {
        println!("hello from a spawned thread");
        42
    });
    let result = handle.join().unwrap();
    println!("spawned thread returned: {result}");

    // Spawn several threads, collect JoinHandles, join them all afterward.
    let handles: Vec<_> = (0..4).map(|i| thread::spawn(move || i * i)).collect();
    let squares: Vec<i32> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    println!("squares 0..4: {squares:?}");

    // A panicking thread doesn't crash main -- join() returns Err.
    let panicker = thread::spawn(|| {
        panic!("oops");
    });
    match panicker.join() {
        Ok(_) => println!("panicker finished normally"),
        Err(_) => println!("panicker panicked -- caught via join(), main is fine"),
    }

    // --- mpsc channels --------------------------------------------------------------
    println!("\n-- mpsc channels --");

    let (tx, rx) = mpsc::channel();
    let mut producer_handles = Vec::new();
    for p in 0..3 {
        let tx = tx.clone();
        producer_handles.push(thread::spawn(move || {
            for m in 0..2 {
                tx.send(format!("producer-{p}-msg-{m}")).unwrap();
            }
        }));
    }
    // Drop the original sender so `rx` knows when every producer is done.
    drop(tx);

    let mut received: Vec<String> = rx.iter().collect();
    for handle in producer_handles {
        handle.join().unwrap();
    }
    received.sort();
    println!("received (sorted): {received:?}");

    // --- Arc<Mutex<T>> shared state ---------------------------------------------------
    println!("\n-- Arc<Mutex<T>> shared state --");

    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();
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
    println!("counter after 10 increments: {}", *counter.lock().unwrap());

    // --- Send/Sync: Arc<Mutex<HashMap>> merge pattern ---------------------------------
    println!("\n-- Arc<Mutex<HashMap>> merge --");

    use std::collections::HashMap;
    let shared: Arc<Mutex<HashMap<&'static str, usize>>> = Arc::new(Mutex::new(HashMap::new()));
    let words = vec![vec!["a", "b", "a"], vec!["b", "c"]];
    let mut handles = Vec::new();
    for chunk in words {
        let shared = Arc::clone(&shared);
        handles.push(thread::spawn(move || {
            let mut local = HashMap::new();
            for word in chunk {
                *local.entry(word).or_insert(0usize) += 1;
            }
            let mut shared = shared.lock().unwrap();
            for (word, count) in local {
                *shared.entry(word).or_insert(0) += count;
            }
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
    let counts = Arc::try_unwrap(shared).unwrap().into_inner().unwrap();
    println!("word counts: {counts:?}");

    // Note: `Rc<T>`/`RefCell<T>` are not `Send`/`Sync`, so they can't be used in
    // place of `Arc<T>`/`Mutex<T>` above -- the compiler rejects it at the
    // `thread::spawn` call site, turning a potential data race into a compile error.
}
