use advanced_04_advanced_functions_and_macros::{apply_all, call_with_one, fold_with, make_pipeline, sum_of_squares};

// --- Exercise 1: apply_all --------------------------------------------------

fn double(x: i32) -> i32 { x * 2 }
fn negate(x: i32) -> i32 { -x }
fn square(x: i32) -> i32 { x * x }
fn identity(x: i32) -> i32 { x }

#[test]
fn apply_all_typical() {
    assert_eq!(apply_all(&[double, negate, square], &[3, 5, 4]), vec![6, -5, 16]);
}

#[test]
fn apply_all_empty_ops() {
    assert_eq!(apply_all(&[], &[1, 2, 3]), vec![]);
}

#[test]
fn apply_all_fewer_values_than_ops() {
    assert_eq!(apply_all(&[double, negate], &[3]), vec![6]);
}

#[test]
fn apply_all_more_values_than_ops() {
    assert_eq!(apply_all(&[double], &[3, 5, 7]), vec![6]);
}

#[test]
fn apply_all_single_identity() {
    assert_eq!(apply_all(&[identity], &[42]), vec![42]);
}

#[test]
fn apply_all_all_same_fn() {
    assert_eq!(apply_all(&[square, square, square], &[2, 3, 4]), vec![4, 9, 16]);
}

// --- Exercise 2: make_pipeline -----------------------------------------------

#[test]
fn pipeline_three_steps() {
    let add1 = Box::new(|x: i32| x + 1) as Box<dyn Fn(i32) -> i32>;
    let mul2 = Box::new(|x: i32| x * 2) as Box<dyn Fn(i32) -> i32>;
    let sub3 = Box::new(|x: i32| x - 3) as Box<dyn Fn(i32) -> i32>;
    let f = make_pipeline(vec![add1, mul2, sub3]);
    assert_eq!(f(5), (5 + 1) * 2 - 3); // 9
}

#[test]
fn pipeline_empty_is_identity() {
    let f = make_pipeline(vec![]);
    assert_eq!(f(42), 42);
    assert_eq!(f(-7), -7);
}

#[test]
fn pipeline_single_step() {
    let neg = Box::new(|x: i32| -x) as Box<dyn Fn(i32) -> i32>;
    let f = make_pipeline(vec![neg]);
    assert_eq!(f(10), -10);
}

#[test]
fn pipeline_order_matters() {
    let div2 = Box::new(|x: i32| x / 2) as Box<dyn Fn(i32) -> i32>;
    let add10 = Box::new(|x: i32| x + 10) as Box<dyn Fn(i32) -> i32>;
    let f1 = make_pipeline(vec![div2, add10]);
    let div2b = Box::new(|x: i32| x / 2) as Box<dyn Fn(i32) -> i32>;
    let add10b = Box::new(|x: i32| x + 10) as Box<dyn Fn(i32) -> i32>;
    let f2 = make_pipeline(vec![add10b, div2b]);
    assert_eq!(f1(8), 14); // 8/2 + 10
    assert_eq!(f2(8), 9);  // (8+10)/2
}

#[test]
fn pipeline_closure_captures() {
    let factor = 3;
    let mul = Box::new(move |x: i32| x * factor) as Box<dyn Fn(i32) -> i32>;
    let f = make_pipeline(vec![mul]);
    assert_eq!(f(7), 21);
}

// --- Exercise 3: call_with_one -----------------------------------------------

#[test]
fn call_with_one_closure_add() {
    assert_eq!(call_with_one(|x| x + 10), 11);
}

#[test]
fn call_with_one_closure_mul() {
    assert_eq!(call_with_one(|x| x * 42), 42);
}

#[test]
fn call_with_one_fn_ptr() {
    fn triple(x: i32) -> i32 { x * 3 }
    assert_eq!(call_with_one(triple), 3);
}

#[test]
fn call_with_one_constant_closure() {
    assert_eq!(call_with_one(|_| 99), 99);
}

#[test]
fn call_with_one_captures() {
    let offset = 100;
    assert_eq!(call_with_one(|x| x + offset), 101);
}

// --- Exercise 4: sum_of_squares! macro ---------------------------------------
// Note: we bind to a typed `let` variable so the `todo!()` stub in the macro
// has a concrete type to coerce to (the never type `!` needs a type hint here).

#[test]
fn sum_of_squares_single() {
    let r: i64 = sum_of_squares!(3);
    assert_eq!(r, 9);
}

#[test]
fn sum_of_squares_two() {
    let r: i64 = sum_of_squares!(3, 4);
    assert_eq!(r, 25);
}

#[test]
fn sum_of_squares_four() {
    let r: i64 = sum_of_squares!(1, 2, 3, 4);
    assert_eq!(r, 30);
}

#[test]
fn sum_of_squares_zero_arg() {
    let r: i64 = sum_of_squares!(0);
    assert_eq!(r, 0);
}

#[test]
fn sum_of_squares_negative() {
    let r: i64 = sum_of_squares!(-3, -4);
    assert_eq!(r, 25);
}

#[test]
fn sum_of_squares_large() {
    let r: i64 = sum_of_squares!(10);
    assert_eq!(r, 100);
}

// --- Exercise 5: fold_with ---------------------------------------------------

#[test]
fn fold_with_sum() {
    assert_eq!(fold_with(&[1, 2, 3, 4], 0, |acc, &x| acc + x), 10);
}

#[test]
fn fold_with_product() {
    assert_eq!(fold_with(&[1, 2, 3, 4], 1, |acc, &x| acc * x), 24);
}

#[test]
fn fold_with_empty() {
    assert_eq!(fold_with::<i32, i32, _>(&[], 99, |acc, &x| acc + x), 99);
}

#[test]
fn fold_with_string_concat() {
    assert_eq!(
        fold_with(&["a", "b", "c"], String::new(), |mut s, x| { s.push_str(x); s }),
        "abc"
    );
}

#[test]
fn fold_with_max() {
    assert_eq!(
        fold_with(&[3, 1, 4, 1, 5, 9, 2], i32::MIN, |acc, &x| acc.max(x)),
        9
    );
}

#[test]
fn fold_with_count_evens() {
    let count = fold_with(&[1, 2, 3, 4, 5, 6], 0usize, |acc, &x| {
        if x % 2 == 0 { acc + 1 } else { acc }
    });
    assert_eq!(count, 3);
}
