use intermediate_09_fearless_concurrency::{
    collect_messages, concurrent_word_count, merge_sort_parallel, run_in_parallel,
    sum_with_threads,
};
use std::collections::HashMap;

// --- sum_with_threads -------------------------------------------------------------

#[test]
fn sum_with_threads_basic_two_threads() {
    assert_eq!(sum_with_threads(vec![1, 2, 3, 4, 5], 2), 15);
}

#[test]
fn sum_with_threads_empty_data() {
    assert_eq!(sum_with_threads(vec![], 4), 0);
}

#[test]
fn sum_with_threads_more_threads_than_elements() {
    assert_eq!(sum_with_threads(vec![10], 5), 10);
}

#[test]
fn sum_with_threads_uneven_chunks() {
    assert_eq!(sum_with_threads(vec![1, 2, 3, 4, 5, 6, 7], 3), 28);
}

#[test]
fn sum_with_threads_zero_threads_treated_as_one() {
    assert_eq!(sum_with_threads(vec![5, 10, 15], 0), 30);
}

#[test]
fn sum_with_threads_negative_numbers() {
    assert_eq!(sum_with_threads(vec![-1, -2, 3], 2), 0);
}

#[test]
fn sum_with_threads_single_thread() {
    assert_eq!(sum_with_threads(vec![1, 2, 3, 4, 5], 1), 15);
}

// --- merge_sort_parallel -----------------------------------------------------------

#[test]
fn merge_sort_parallel_basic() {
    assert_eq!(merge_sort_parallel(vec![5, 3, 1, 4, 2], 2), vec![1, 2, 3, 4, 5]);
}

#[test]
fn merge_sort_parallel_empty() {
    assert_eq!(merge_sort_parallel(Vec::new(), 2), Vec::<i32>::new());
}

#[test]
fn merge_sort_parallel_single_element() {
    assert_eq!(merge_sort_parallel(vec![1], 3), vec![1]);
}

#[test]
fn merge_sort_parallel_with_duplicates() {
    assert_eq!(merge_sort_parallel(vec![3, 3, 1, 2, 2], 1), vec![1, 2, 2, 3, 3]);
}

#[test]
fn merge_sort_parallel_depth_zero_still_sorts() {
    assert_eq!(merge_sort_parallel(vec![5, 4, 3, 2, 1], 0), vec![1, 2, 3, 4, 5]);
}

#[test]
fn merge_sort_parallel_already_sorted() {
    assert_eq!(merge_sort_parallel(vec![1, 2, 3, 4, 5], 2), vec![1, 2, 3, 4, 5]);
}

#[test]
fn merge_sort_parallel_negative_numbers() {
    assert_eq!(merge_sort_parallel(vec![-5, 3, -1, 0, 2], 2), vec![-5, -1, 0, 2, 3]);
}

#[test]
fn merge_sort_parallel_depth_exceeding_recursion_still_sorts() {
    // max_depth is far larger than needed for 4 elements -- recursion still
    // bottoms out at len <= 1 regardless.
    assert_eq!(merge_sort_parallel(vec![4, 2, 1, 3], 10), vec![1, 2, 3, 4]);
}

// --- collect_messages ---------------------------------------------------------------

#[test]
fn collect_messages_two_producers_two_messages() {
    assert_eq!(
        collect_messages(2, 2),
        vec![
            "producer-0-msg-0",
            "producer-0-msg-1",
            "producer-1-msg-0",
            "producer-1-msg-1",
        ]
    );
}

#[test]
fn collect_messages_zero_producers() {
    assert_eq!(collect_messages(0, 5), Vec::<String>::new());
}

#[test]
fn collect_messages_zero_messages_per_producer() {
    assert_eq!(collect_messages(3, 0), Vec::<String>::new());
}

#[test]
fn collect_messages_single_producer() {
    assert_eq!(
        collect_messages(1, 3),
        vec!["producer-0-msg-0", "producer-0-msg-1", "producer-0-msg-2"]
    );
}

#[test]
fn collect_messages_returns_sorted_for_multiple_producers() {
    assert_eq!(
        collect_messages(3, 3),
        vec![
            "producer-0-msg-0",
            "producer-0-msg-1",
            "producer-0-msg-2",
            "producer-1-msg-0",
            "producer-1-msg-1",
            "producer-1-msg-2",
            "producer-2-msg-0",
            "producer-2-msg-1",
            "producer-2-msg-2",
        ]
    );
}

#[test]
fn collect_messages_correct_total_count_and_uniqueness() {
    let messages = collect_messages(4, 5);
    assert_eq!(messages.len(), 20);

    // sorted ascending
    assert!(messages.windows(2).all(|w| w[0] <= w[1]));

    // all unique
    let unique: std::collections::HashSet<_> = messages.iter().collect();
    assert_eq!(unique.len(), 20);
}

// --- concurrent_word_count -----------------------------------------------------------

#[test]
fn concurrent_word_count_basic() {
    let chunks = vec![
        vec!["a".to_string(), "b".to_string(), "a".to_string()],
        vec!["b".to_string(), "c".to_string()],
    ];
    let counts = concurrent_word_count(chunks);
    assert_eq!(counts.get("a"), Some(&2));
    assert_eq!(counts.get("b"), Some(&2));
    assert_eq!(counts.get("c"), Some(&1));
    assert_eq!(counts.len(), 3);
}

#[test]
fn concurrent_word_count_empty_chunks_vec() {
    assert_eq!(concurrent_word_count(vec![]), HashMap::new());
}

#[test]
fn concurrent_word_count_single_empty_chunk() {
    assert_eq!(concurrent_word_count(vec![vec![]]), HashMap::new());
}

#[test]
fn concurrent_word_count_single_chunk() {
    let chunks = vec![vec![
        "x".to_string(),
        "y".to_string(),
        "x".to_string(),
        "x".to_string(),
    ]];
    let counts = concurrent_word_count(chunks);
    assert_eq!(counts.get("x"), Some(&3));
    assert_eq!(counts.get("y"), Some(&1));
    assert_eq!(counts.len(), 2);
}

#[test]
fn concurrent_word_count_many_chunks_same_word() {
    let chunks = vec![
        vec!["dup".to_string()],
        vec!["dup".to_string()],
        vec!["dup".to_string()],
    ];
    let counts = concurrent_word_count(chunks);
    assert_eq!(counts.get("dup"), Some(&3));
    assert_eq!(counts.len(), 1);
}

#[test]
fn concurrent_word_count_no_duplicates() {
    let chunks = vec![
        vec!["a".to_string(), "b".to_string()],
        vec!["c".to_string(), "d".to_string()],
    ];
    let counts = concurrent_word_count(chunks);
    assert_eq!(counts.len(), 4);
    for word in ["a", "b", "c", "d"] {
        assert_eq!(counts.get(word), Some(&1));
    }
}

// --- run_in_parallel -------------------------------------------------------------------

#[test]
fn run_in_parallel_basic_order_preserved() {
    let tasks: Vec<Box<dyn FnOnce() -> i32 + Send>> = vec![
        Box::new(|| 1 + 1),
        Box::new(|| 2 * 3),
        Box::new(|| 100 - 1),
    ];
    assert_eq!(run_in_parallel(tasks), vec![2, 6, 99]);
}

#[test]
fn run_in_parallel_empty() {
    let tasks: Vec<Box<dyn FnOnce() -> i32 + Send>> = vec![];
    assert_eq!(run_in_parallel(tasks), Vec::<i32>::new());
}

#[test]
fn run_in_parallel_single_task() {
    let tasks: Vec<Box<dyn FnOnce() -> i32 + Send>> = vec![Box::new(|| 7)];
    assert_eq!(run_in_parallel(tasks), vec![7]);
}

#[test]
fn run_in_parallel_with_strings() {
    let tasks: Vec<Box<dyn FnOnce() -> String + Send>> = vec![
        Box::new(|| "a".to_string()),
        Box::new(|| "b".to_string()),
        Box::new(|| "c".to_string()),
    ];
    assert_eq!(run_in_parallel(tasks), vec!["a", "b", "c"]);
}

#[test]
fn run_in_parallel_order_preserved_with_varying_work() {
    let tasks: Vec<Box<dyn FnOnce() -> i32 + Send>> = vec![
        Box::new(|| (0..1000).sum::<i32>()), // more "work"
        Box::new(|| 42),
        Box::new(|| (0..100).sum::<i32>()),
    ];
    assert_eq!(run_in_parallel(tasks), vec![499_500, 42, 4_950]);
}

#[test]
fn run_in_parallel_many_tasks() {
    let tasks: Vec<Box<dyn FnOnce() -> i32 + Send>> = (0..10)
        .map(|i| Box::new(move || i * i) as Box<dyn FnOnce() -> i32 + Send>)
        .collect();
    assert_eq!(
        run_in_parallel(tasks),
        vec![0, 1, 4, 9, 16, 25, 36, 49, 64, 81]
    );
}
