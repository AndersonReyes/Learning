//! Run with: `cargo run --example examples -p advanced-02-patterns-and-matching`

fn main() {
    // --- Pattern positions: let, for, fn, if let, while let, let...else --------------
    println!("-- pattern positions --");

    let (x, y) = (3, 7); // let with a tuple pattern (irrefutable)
    println!("x={x}, y={y}");

    let mut nums = vec![Some(1), None, Some(3), None, Some(5)];
    let mut sum = 0;
    while let Some(Some(n)) = nums.pop() {
        // while let: refutable, keeps looping while the pattern matches
        sum += n;
    }
    println!("sum of Some values (from back, until None): {sum}");

    // let...else: refutable let that diverges on mismatch
    fn first_even(slice: &[i32]) -> Option<i32> {
        let [first, rest @ ..] = slice else {
            return None;
        };
        if first % 2 == 0 {
            Some(*first)
        } else {
            first_even(rest)
        }
    }
    println!("first even in [1,3,5,6,7]: {:?}", first_even(&[1, 3, 5, 6, 7]));
    println!("first even in [1,3,5]: {:?}", first_even(&[1, 3, 5]));

    // --- Literals, | alternatives, ..= ranges ----------------------------------------
    println!("\n-- literals, |, ..= ranges --");

    for score in [0, 55, 65, 75, 90] {
        let grade = match score {
            0..=49 => "fail",
            50..=64 => "pass",
            65..=79 => "merit",
            80..=100 => "distinction",
            _ => "invalid",
        };
        println!("{score} -> {grade}");
    }

    for n in [0, 1, 2, 3, 4, 99] {
        let desc = match n {
            1 | 2 | 3 => "small",
            4..=10 => "medium",
            0 => "zero",
            _ => "large",
        };
        println!("{n} -> {desc}");
    }

    // --- Struct destructuring --------------------------------------------------------
    println!("\n-- struct destructuring --");

    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };
    match p {
        Point { x: 0, y } => println!("on y-axis at {y}"),
        Point { x, y: 0 } => println!("on x-axis at {x}"),
        Point { x, y } => println!("at ({x}, {y})"),
    }

    // -- (fields we don't need: `..`) -
    struct Foo {
        x: i32,
        _y: i32,
        _z: i32,
    }
    let f = Foo { x: 1, _y: 2, _z: 3 };
    let Foo { x, .. } = f; // ignore _y and _z with ..
    println!("x from Foo: {x}");

    // --- Slice patterns & @ bindings -------------------------------------------------
    println!("\n-- slice patterns & @ bindings --");

    let data = [1, 2, 3, 4, 5_i32];
    match data.as_slice() {
        [] => println!("empty"),
        [single] => println!("one: {single}"),
        [first, .., last] => println!("first={first}, last={last}"),
    }

    for n in [0, 3, 7, 15, 42] {
        let label = match n {
            0 => "zero",
            x @ 1..=5 => {
                println!("  small @ binding: {x}");
                "small"
            }
            x @ 6..=10 => {
                println!("  medium @ binding: {x}");
                "medium"
            }
            _ => "large",
        };
        println!("{n} -> {label}");
    }

    // --- Match guards ----------------------------------------------------------------
    println!("\n-- match guards --");

    for (a, b) in [(1, 1), (2, 3), (-1, 5)] {
        match (a, b) {
            (x, y) if x == y => println!("({x}, {y}) are equal"),
            (x, _) if x < 0 => println!("({x}, {b}) has negative x"),
            (x, y) => println!("({x}, {y}) — other"),
        }
    }

    // --- Tuple patterns + | across alternatives (balanced-bracket style) -------------
    println!("\n-- tuple patterns with | across alternatives --");

    let mut stack: Vec<char> = Vec::new();
    let input = "{[()]}";
    let mut valid = true;
    for c in input.chars() {
        match (stack.last().copied(), c) {
            (_, '(' | '[' | '{') => stack.push(c),
            (Some('('), ')') | (Some('['), ']') | (Some('{'), '}') => {
                stack.pop();
            }
            (_, ')' | ']' | '}') => {
                valid = false;
                break;
            }
            _ => {}
        }
    }
    valid = valid && stack.is_empty();
    println!("{input:?} balanced: {valid}");
}
