//! Run with: `cargo run --example examples -p intermediate-04-closures-and-iterators`

use std::cell::RefCell;
use std::rc::Rc;

// --- Cacher-style struct (book ch.13.1) -- caches a single value --------------

struct Cacher<F: Fn(u32) -> u32> {
    calculation: F,
    value: Option<u32>,
}

impl<F: Fn(u32) -> u32> Cacher<F> {
    fn new(calculation: F) -> Cacher<F> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

// --- Fn / FnMut / FnOnce ------------------------------------------------------

fn call_fn(f: impl Fn(i32) -> i32, x: i32) -> i32 {
    f(x)
}

fn call_fn_mut(mut f: impl FnMut() -> i32) -> i32 {
    f() + f() + f() // calls f three times -- needs FnMut
}

fn call_fn_once(f: impl FnOnce() -> String) -> String {
    f() // consumes its capture -- can only call once
}

// --- Returning closures --------------------------------------------------------

/// Returns `impl Fn(i32) -> i32` -- one concrete (monomorphized) closure type.
fn adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n
}

/// Returns `Box<dyn Fn(i32) -> i32>` -- different call sites can return
/// *different* underlying closure types behind the same trait object.
fn pick_operation(op: &str, operand: i32) -> Box<dyn Fn(i32) -> i32> {
    match op {
        "add" => Box::new(move |x| x + operand),
        "mul" => Box::new(move |x| x * operand),
        _ => Box::new(|x| x),
    }
}

fn main() {
    // --- Cacher: only caches the *first* result, regardless of arg ---
    let mut cacher = Cacher::new(|x| {
        println!("  (calculating square of {x}...)");
        x * x
    });
    println!("cacher.value(4) = {}", cacher.value(4));
    println!("cacher.value(4) = {}", cacher.value(4)); // cached, no recalculation
    println!(
        "cacher.value(5) = {} (book's basic Cacher: still returns the FIRST result!)",
        cacher.value(5)
    );

    // --- Fn: callable many times, captures by reference ---
    let double = |x: i32| x * 2;
    println!("call_fn(double, 21) = {}", call_fn(double, 21));
    println!("call_fn(double, 10) = {}", call_fn(double, 10)); // double still usable

    // --- FnMut: captures by mutable reference, can be called repeatedly ---
    let mut counter = 0;
    let increment = || {
        counter += 1;
        counter
    };
    println!("call_fn_mut(increment) = {}", call_fn_mut(increment));
    println!("counter after = {counter}"); // 3 -- closure mutated its capture

    // --- FnOnce: captures by value (move), consumed after one call ---
    let s = String::from("hello");
    let consume = move || s; // moves `s` into the closure, then out again
    println!("call_fn_once(consume) = {:?}", call_fn_once(consume));

    // --- move closures and the borrow checker ---
    let owned = String::from("captured");
    let print_owned = move || println!("inside closure: {owned}");
    print_owned();
    // `owned` was moved into `print_owned` -- using it here would be a compile error

    // --- Rc<RefCell<T>> for shared mutable state in an Fn closure ---
    let calls = Rc::new(RefCell::new(0u32));
    let calls_clone = Rc::clone(&calls);
    let tracked = move |x: i32| {
        *calls_clone.borrow_mut() += 1;
        x * x
    };
    println!("tracked(3) = {}", tracked(3));
    println!("tracked(4) = {}", tracked(4));
    println!("calls so far = {}", *calls.borrow());

    // --- Iterators: lazy until consumed ---
    let v = vec![1, 2, 3, 4, 5];
    let iter = v.iter(); // nothing happens yet
    let total: i32 = iter.sum(); // consumes the iterator
    println!("sum of {v:?} = {total}");

    // --- Iterator adaptor chain: filter -> map -> take -> collect ---
    let result: Vec<i32> = (1..=10)
        .filter(|x| x % 2 == 0) // 2,4,6,8,10
        .map(|x| x * x) // 4,16,36,64,100
        .take(3) // 4,16,36
        .collect();
    println!("filter().map().take(3).collect() = {result:?}");

    // --- zip, enumerate, rev, chain ---
    let names = ["Alice", "Bob", "Carol"];
    let ages = [30, 25, 35];
    let pairs: Vec<(&&str, &i32)> = names.iter().zip(ages.iter()).collect();
    println!("zip(names, ages) = {pairs:?}");

    for (i, name) in names.iter().enumerate() {
        println!("  [{i}] {name}");
    }

    let reversed: Vec<i32> = (1..=5).rev().collect();
    println!("(1..=5).rev().collect() = {reversed:?}");

    let chained: Vec<i32> = (1..=3).chain(10..=12).collect();
    println!("(1..=3).chain(10..=12).collect() = {chained:?}");

    // --- scan: running total ---
    let totals: Vec<i32> = [1, 2, 3, 4].iter().scan(0, |sum, &x| {
        *sum += x;
        Some(*sum)
    }).collect();
    println!("running totals of [1,2,3,4] = {totals:?}");

    // --- returning closures: impl Fn vs Box<dyn Fn> ---
    let add5 = adder(5);
    println!("adder(5)(10) = {}", add5(10));

    let op = pick_operation("mul", 3);
    println!("pick_operation(\"mul\", 3)(7) = {}", op(7));
}
