use intermediate_04_closures_and_iterators::{compose, retry, running_stats, top_n_by, Memoizer};
use std::cell::RefCell;
use std::cmp::Reverse;
use std::rc::Rc;

// --- Memoizer ----------------------------------------------------------------

#[test]
fn memoizer_caches_repeated_calls() {
    let calls = Rc::new(RefCell::new(0u32));
    let calls_clone = Rc::clone(&calls);
    let mut memo = Memoizer::new(move |x: u64| {
        *calls_clone.borrow_mut() += 1;
        x * x
    });

    assert_eq!(memo.value(4), 16);
    assert_eq!(memo.value(4), 16); // cached
    assert_eq!(memo.value(5), 25);
    assert_eq!(*calls.borrow(), 2);
}

#[test]
fn memoizer_repeated_calls_same_arg_only_computes_once() {
    let calls = Rc::new(RefCell::new(0u32));
    let calls_clone = Rc::clone(&calls);
    let mut memo = Memoizer::new(move |x: u64| {
        *calls_clone.borrow_mut() += 1;
        x * 2
    });

    for _ in 0..10 {
        assert_eq!(memo.value(7), 14);
    }
    assert_eq!(*calls.borrow(), 1);
}

#[test]
fn memoizer_each_distinct_arg_computed_once() {
    let calls = Rc::new(RefCell::new(0u32));
    let calls_clone = Rc::clone(&calls);
    let mut memo = Memoizer::new(move |x: u64| {
        *calls_clone.borrow_mut() += 1;
        x + 100
    });

    for arg in [0u64, 1, 2, 3] {
        memo.value(arg);
    }
    // call again, in a different order -- everything should be cached now
    for arg in [3u64, 2, 1, 0] {
        assert_eq!(memo.value(arg), arg + 100);
    }
    assert_eq!(*calls.borrow(), 4);
}

#[test]
fn memoizer_handles_zero_arg() {
    let mut memo = Memoizer::new(|x: u64| x * x * x);
    assert_eq!(memo.value(0), 0);
    assert_eq!(memo.value(0), 0);
    assert_eq!(memo.value(3), 27);
}

// --- compose -------------------------------------------------------------------

#[test]
fn compose_add_then_double() {
    let add_one = |x: i32| x + 1;
    let double = |x: i32| x * 2;
    let add_then_double = compose(add_one, double);
    assert_eq!(add_then_double(3), 8);
    assert_eq!(add_then_double(0), 2);
    assert_eq!(add_then_double(-1), 0);
}

#[test]
fn compose_chained_three_functions() {
    let add_one = |x: i32| x + 1;
    let double = |x: i32| x * 2;
    let add_then_double = compose(add_one, double);

    let negate = |x: i32| -x;
    let combo = compose(add_then_double, negate);
    assert_eq!(combo(2), -6);
    assert_eq!(combo(-1), 0);
}

#[test]
fn compose_double_then_square() {
    let double = |x: i32| x * 2;
    let square = |x: i32| x * x;
    let double_then_square = compose(double, square);
    assert_eq!(double_then_square(3), 36);
    assert_eq!(double_then_square(0), 0);
    assert_eq!(double_then_square(-2), 16);
}

#[test]
fn compose_with_move_captured_closure() {
    let offset = 10;
    let add_offset = move |x: i32| x + offset;
    let negate = |x: i32| -x;
    let combo = compose(add_offset, negate);
    assert_eq!(combo(5), -15);
    assert_eq!(combo(-10), 0);
}

#[test]
fn compose_across_different_types() {
    let to_string = |x: i32| x.to_string();
    let len = |s: String| s.len();
    let digit_count = compose(to_string, len);
    assert_eq!(digit_count(12345), 5);
    assert_eq!(digit_count(0), 1);
    assert_eq!(digit_count(-100), 4); // "-100"
}

// --- retry -----------------------------------------------------------------------

#[test]
fn retry_succeeds_after_failures() {
    let mut tries = 0;
    let result = retry(5, |attempt| {
        tries += 1;
        if attempt < 3 {
            Err(format!("attempt {attempt} failed"))
        } else {
            Ok(attempt * 10)
        }
    });
    assert_eq!(result, Ok(30));
    assert_eq!(tries, 3);
}

#[test]
fn retry_zero_max_attempts_returns_err_without_calling_f() {
    let mut called = false;
    let result: Result<i32, String> = retry(0, |_| {
        called = true;
        Ok(42)
    });
    assert_eq!(result, Err("no attempts allowed".to_string()));
    assert!(!called);
}

#[test]
fn retry_always_fails_returns_last_error() {
    let mut tries = 0;
    let result: Result<i32, String> = retry(3, |attempt| {
        tries += 1;
        Err(format!("attempt {attempt} failed"))
    });
    assert_eq!(result, Err("attempt 3 failed".to_string()));
    assert_eq!(tries, 3);
}

#[test]
fn retry_succeeds_first_try_calls_f_once() {
    let mut tries = 0;
    let result = retry(5, |_attempt| {
        tries += 1;
        Ok::<i32, String>(99)
    });
    assert_eq!(result, Ok(99));
    assert_eq!(tries, 1);
}

#[test]
fn retry_passes_sequential_attempt_numbers() {
    let mut attempts_seen = Vec::new();
    let result: Result<i32, String> = retry(4, |attempt| {
        attempts_seen.push(attempt);
        if attempt < 4 {
            Err(format!("fail {attempt}"))
        } else {
            Ok(attempt as i32 * 100)
        }
    });
    assert_eq!(result, Ok(400));
    assert_eq!(attempts_seen, vec![1, 2, 3, 4]);
}

#[test]
fn retry_one_max_attempt_success() {
    let result: Result<&str, String> = retry(1, |_attempt| Ok("done"));
    assert_eq!(result, Ok("done"));
}

// --- top_n_by --------------------------------------------------------------------

#[test]
fn top_n_by_basic_descending() {
    let scores = [("Alice", 50), ("Bob", 80), ("Carol", 80), ("Dave", 30)];
    let top2 = top_n_by(&scores, 2, |&(_, score)| score);
    assert_eq!(top2, vec![&("Bob", 80), &("Carol", 80)]);
}

#[test]
fn top_n_by_reverse_key_for_smallest() {
    let scores = [("Alice", 50), ("Bob", 80), ("Carol", 80), ("Dave", 30)];
    let bottom2 = top_n_by(&scores, 2, |&(_, score)| Reverse(score));
    assert_eq!(bottom2, vec![&("Dave", 30), &("Alice", 50)]);
}

#[test]
fn top_n_by_zero_returns_empty() {
    let scores = [("Alice", 50), ("Bob", 80), ("Carol", 80), ("Dave", 30)];
    assert_eq!(
        top_n_by(&scores, 0, |&(_, score)| score),
        Vec::<&(&str, i32)>::new()
    );
}

#[test]
fn top_n_by_n_greater_than_len_returns_all_sorted() {
    let scores = [("Alice", 50), ("Bob", 80), ("Carol", 80), ("Dave", 30)];
    let all = top_n_by(&scores, 10, |&(_, score)| score);
    assert_eq!(
        all,
        vec![&("Bob", 80), &("Carol", 80), &("Alice", 50), &("Dave", 30)]
    );
}

#[test]
fn top_n_by_empty_items_returns_empty() {
    let items: [(&str, i32); 0] = [];
    assert_eq!(
        top_n_by(&items, 3, |&(_, score)| score),
        Vec::<&(&str, i32)>::new()
    );
}

#[derive(Debug, PartialEq)]
struct Player {
    name: &'static str,
    score: i32,
}

#[test]
fn top_n_by_with_struct_field() {
    let players = [
        Player { name: "Alice", score: 50 },
        Player { name: "Bob", score: 80 },
        Player { name: "Carol", score: 80 },
        Player { name: "Dave", score: 30 },
    ];
    let top2 = top_n_by(&players, 2, |p| p.score);
    assert_eq!(top2, vec![&players[1], &players[2]]);
}

// --- running_stats -----------------------------------------------------------------

#[test]
fn running_stats_basic() {
    assert_eq!(
        running_stats(&[3.0, 1.0, 4.0, 1.0, 5.0]),
        vec![(3.0, 3.0), (1.0, 3.0), (1.0, 4.0), (1.0, 4.0), (1.0, 5.0)]
    );
}

#[test]
fn running_stats_single_element() {
    assert_eq!(running_stats(&[42.0]), vec![(42.0, 42.0)]);
}

#[test]
fn running_stats_empty() {
    assert_eq!(running_stats(&[]), Vec::<(f64, f64)>::new());
}

#[test]
fn running_stats_negative_values() {
    assert_eq!(
        running_stats(&[-1.0, -5.0, -3.0]),
        vec![(-1.0, -1.0), (-5.0, -1.0), (-5.0, -1.0)]
    );
}

#[test]
fn running_stats_mixed_signs() {
    assert_eq!(
        running_stats(&[0.0, -2.0, 3.0, -1.0]),
        vec![(0.0, 0.0), (-2.0, 0.0), (-2.0, 3.0), (-2.0, 3.0)]
    );
}
