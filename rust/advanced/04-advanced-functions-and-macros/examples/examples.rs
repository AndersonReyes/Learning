//! Run with: `cargo run --example examples -p advanced-04-advanced-functions-and-macros`

fn main() {
    // --- fn pointers ---------------------------------------------------------
    println!("-- fn pointers --");

    fn double(x: i32) -> i32 { x * 2 }
    fn square(x: i32) -> i32 { x * x }

    let ops: &[fn(i32) -> i32] = &[double, square];
    for (f, &n) in ops.iter().zip([3, 4].iter()) {
        println!("  f({}) = {}", n, f(n));
    }

    // Enum variant constructors as fn pointers
    let optionals: Vec<Option<i32>> = (0..3).map(Some).collect();
    println!("map(Some): {:?}", optionals);

    // --- returning closures --------------------------------------------------
    println!("\n-- returning closures --");

    fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
        move |x| x + n
    }

    fn make_adder_dyn(n: i32) -> Box<dyn Fn(i32) -> i32> {
        Box::new(move |x| x + n)
    }

    let add5 = make_adder(5);
    let add10 = make_adder_dyn(10);
    println!("add5(3) = {}", add5(3));
    println!("add10(3) = {}", add10(3));

    // Pipeline via composition
    let steps: Vec<Box<dyn Fn(i32) -> i32>> = vec![
        Box::new(|x| x + 1),
        Box::new(|x| x * 2),
        Box::new(|x| x - 3),
    ];
    let pipeline = move |mut x: i32| -> i32 { for f in &steps { x = f(x); } x };
    println!("pipeline(5) = {} (expect {})", pipeline(5), (5 + 1) * 2 - 3);

    // --- Fn / FnMut / FnOnce -------------------------------------------------
    println!("\n-- Fn / FnMut / FnOnce --");

    fn call_twice<F: Fn(i32) -> i32>(f: F, x: i32) -> (i32, i32) {
        (f(x), f(x))  // called twice: needs Fn (not FnOnce)
    }

    let base = 100;
    let (a, b) = call_twice(|x| x + base, 5);
    println!("call_twice(|x| x+100, 5) = ({}, {})", a, b);

    let mut count = 0;
    let mut counter = || { count += 1; count };
    println!("FnMut counter: {}, {}, {}", counter(), counter(), counter());

    // --- dispatch table with fn pointers -------------------------------------
    println!("\n-- dispatch table --");

    use std::collections::HashMap;
    let mut table: HashMap<&str, fn(i32) -> i32> = HashMap::new();
    table.insert("double", double);
    table.insert("square", square);

    for name in ["double", "square"] {
        println!("  {}(6) = {}", name, table[name](6));
    }

    // --- macro_rules! --------------------------------------------------------
    println!("\n-- macro_rules! --");

    macro_rules! make_vec {
        ($($elem:expr),*) => {
            vec![$($elem),*]
        };
    }

    let v = make_vec![1, 2, 3, 4];
    println!("make_vec![1,2,3,4] = {:?}", v);

    macro_rules! repeat_print {
        ($msg:literal, $n:expr) => {
            for _ in 0..$n {
                println!("  {}", $msg);
            }
        };
    }

    repeat_print!("hello from macro", 3);
}
