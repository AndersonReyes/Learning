//! Run with: `cargo run --example examples -p intermediate-05-custom-iterators-and-adapters`

// --- The Counter example from the book (ch.13.2) --------------------------------

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

// --- Infinite iterator, bounded by .take() ----------------------------------------

struct Naturals(u64);

impl Iterator for Naturals {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        self.0 += 1;
        Some(self.0) // always Some -- infinite
    }
}

// --- Custom adapter wrapping an inner iterator -------------------------------------

struct Doubler<I: Iterator<Item = i32>> {
    iter: I,
}

fn doubler<I: Iterator<Item = i32>>(iter: I) -> Doubler<I> {
    Doubler { iter }
}

impl<I: Iterator<Item = i32>> Iterator for Doubler<I> {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        self.iter.next().map(|x| x * 2)
    }
}

// --- IntoIterator for a custom collection -------------------------------------------

struct Bag {
    items: Vec<String>,
}

struct BagIntoIter {
    inner: std::vec::IntoIter<String>,
}

impl Iterator for BagIntoIter {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        self.inner.next()
    }
}

impl IntoIterator for Bag {
    type Item = String;
    type IntoIter = BagIntoIter;

    fn into_iter(self) -> BagIntoIter {
        BagIntoIter {
            inner: self.items.into_iter(),
        }
    }
}

fn main() {
    // --- implementing Iterator (next) gives every default method for free ---
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    println!("Counter zip/map/filter/sum = {sum}");
    println!(
        "Counter::new().collect() = {:?}",
        Counter::new().collect::<Vec<u32>>()
    );

    // --- infinite iterator bounded by .take() ---
    let naturals: Vec<u64> = Naturals(0).take(5).collect();
    println!("Naturals(0).take(5).collect() = {naturals:?}");

    // --- custom adapter wrapping an inner iterator ---
    let doubled: Vec<i32> = doubler(vec![1, 2, 3].into_iter()).collect();
    println!("doubler([1,2,3]).collect() = {doubled:?}");

    // adapters compose: chain more iterator methods on top of a custom one
    let total: i32 = doubler(1..=5).sum();
    println!("doubler(1..=5).sum() = {total}");

    // --- IntoIterator for a custom collection -- works with `for` ---
    let bag = Bag {
        items: vec!["a".to_string(), "b".to_string(), "c".to_string()],
    };
    for item in bag {
        println!("bag item: {item}");
    }

    let bag2 = Bag {
        items: vec!["x".to_string(), "y".to_string()],
    };
    let collected: Vec<String> = bag2.into_iter().collect();
    println!("bag2.into_iter().collect() = {collected:?}");
}
