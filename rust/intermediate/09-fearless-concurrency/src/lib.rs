//! Intermediate 09 — Fearless Concurrency (threads, channels, `Mutex`,
//! `Arc`).
//!
//! `notes.md` covers Book ch. 16: `thread::spawn`/`JoinHandle`/`move`
//! closures, `mpsc` channels (multiple producers, channel closing via
//! dropped senders), shared state via `Arc<Mutex<T>>`, and the `Send`/`Sync`
//! marker traits. The 5 exercises below: chunked parallel summation
//! (`sum_with_threads`), a recursive parallel merge sort
//! (`merge_sort_parallel`), a multi-producer channel collector
//! (`collect_messages`), an `Arc<Mutex<HashMap>>`-based word counter
//! (`concurrent_word_count`), and a generic task runner over boxed `FnOnce`
//! closures (`run_in_parallel`).

use std::collections::HashMap;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

// --- 1. sum_with_threads -------------------------------------------------------------

/// Sums `data` by splitting it into up to `num_threads` contiguous chunks,
/// summing each chunk on its own thread (via [`std::thread::spawn`]), and
/// adding the partial sums together.
///
/// - `data.is_empty()` -> `0` (no threads spawned).
/// - `num_threads == 0` is treated as `1`.
/// - Chunk size is `ceil(data.len() / num_threads)`, so the actual number of
///   spawned threads may be less than `num_threads` if `data` is small.
///
/// # Examples
///
/// ```ignore
/// use intermediate_09_fearless_concurrency::sum_with_threads;
///
/// assert_eq!(sum_with_threads(vec![1, 2, 3, 4, 5], 2), 15);
/// assert_eq!(sum_with_threads(vec![], 4), 0);
/// assert_eq!(sum_with_threads(vec![10], 5), 10);
/// assert_eq!(sum_with_threads(vec![1, 2, 3, 4, 5, 6, 7], 3), 28);
/// assert_eq!(sum_with_threads(vec![5, 10, 15], 0), 30); // num_threads == 0 -> 1
/// ```
pub fn sum_with_threads(data: Vec<i64>, num_threads: usize) -> i64 {
    todo!()
}

// --- 2. merge_sort_parallel -----------------------------------------------------------

/// Sorts `data` ascending using a recursive parallel merge sort.
///
/// At each level with `max_depth > 0` and `data.len() > 1`, splits `data` in
/// half, sorts the right half on a new thread (via [`std::thread::spawn`],
/// recursing with `max_depth - 1`), sorts the left half on the current
/// thread (also with `max_depth - 1`), joins the spawned thread, and merges
/// the two sorted halves. At `max_depth == 0` (or `data.len() <= 1`), sorts
/// sequentially without spawning.
///
/// `max_depth` only bounds how many *levels* spawn threads — the result is
/// fully sorted regardless of `max_depth`.
///
/// # Examples
///
/// ```ignore
/// use intermediate_09_fearless_concurrency::merge_sort_parallel;
///
/// assert_eq!(merge_sort_parallel(vec![5, 3, 1, 4, 2], 2), vec![1, 2, 3, 4, 5]);
/// assert_eq!(merge_sort_parallel(Vec::<i32>::new(), 2), Vec::<i32>::new());
/// assert_eq!(merge_sort_parallel(vec![1], 3), vec![1]);
/// assert_eq!(merge_sort_parallel(vec![3, 3, 1, 2, 2], 1), vec![1, 2, 2, 3, 3]);
/// assert_eq!(merge_sort_parallel(vec![5, 4, 3, 2, 1], 0), vec![1, 2, 3, 4, 5]);
/// ```
pub fn merge_sort_parallel(data: Vec<i32>, max_depth: usize) -> Vec<i32> {
    todo!()
}

// --- 3. collect_messages ---------------------------------------------------------------

/// Spawns `num_producers` threads, each sending `messages_per_producer`
/// messages of the form `"producer-{p}-msg-{m}"` (`p` = producer index, `m`
/// = message index, both 0-based) through a shared [`mpsc::channel`], then
/// collects and sorts all received messages.
///
/// Each producer gets its own clone of the [`mpsc::Sender`]; the original is
/// dropped before reading from the [`mpsc::Receiver`] so the receiving
/// iterator terminates once every producer finishes (and drops its clone).
///
/// # Examples
///
/// ```ignore
/// use intermediate_09_fearless_concurrency::collect_messages;
///
/// assert_eq!(
///     collect_messages(2, 2),
///     vec![
///         "producer-0-msg-0",
///         "producer-0-msg-1",
///         "producer-1-msg-0",
///         "producer-1-msg-1",
///     ]
/// );
/// assert_eq!(collect_messages(0, 5), Vec::<String>::new());
/// assert_eq!(collect_messages(3, 0), Vec::<String>::new());
/// assert_eq!(
///     collect_messages(1, 3),
///     vec!["producer-0-msg-0", "producer-0-msg-1", "producer-0-msg-2"]
/// );
/// ```
pub fn collect_messages(num_producers: usize, messages_per_producer: usize) -> Vec<String> {
    todo!()
}

// --- 4. concurrent_word_count -----------------------------------------------------------

/// Counts word occurrences across `chunks`, processing each chunk on its own
/// thread and merging the per-thread counts into a single map via
/// `Arc<Mutex<HashMap<String, usize>>>`.
///
/// Each thread builds a local `HashMap` for its chunk (no contention while
/// counting), then locks the shared map once to merge its local counts in.
///
/// # Examples
///
/// ```ignore
/// use intermediate_09_fearless_concurrency::concurrent_word_count;
/// use std::collections::HashMap;
///
/// let chunks = vec![
///     vec!["a".to_string(), "b".to_string(), "a".to_string()],
///     vec!["b".to_string(), "c".to_string()],
/// ];
/// let counts = concurrent_word_count(chunks);
/// assert_eq!(counts.get("a"), Some(&2));
/// assert_eq!(counts.get("b"), Some(&2));
/// assert_eq!(counts.get("c"), Some(&1));
/// assert_eq!(counts.len(), 3);
///
/// assert_eq!(concurrent_word_count(vec![]), HashMap::new());
/// assert_eq!(concurrent_word_count(vec![vec![]]), HashMap::new());
/// ```
pub fn concurrent_word_count(chunks: Vec<Vec<String>>) -> HashMap<String, usize> {
    todo!()
}

// --- 5. run_in_parallel -------------------------------------------------------------------

/// Runs each task in `tasks` on its own thread (via [`std::thread::spawn`]),
/// joins all of them, and returns their results **in the same order as
/// `tasks`** (regardless of which thread finishes first).
///
/// # Examples
///
/// ```ignore
/// use intermediate_09_fearless_concurrency::run_in_parallel;
///
/// let tasks: Vec<Box<dyn FnOnce() -> i32 + Send>> = vec![
///     Box::new(|| 1 + 1),
///     Box::new(|| 2 * 3),
///     Box::new(|| 100 - 1),
/// ];
/// assert_eq!(run_in_parallel(tasks), vec![2, 6, 99]);
///
/// let empty: Vec<Box<dyn FnOnce() -> i32 + Send>> = vec![];
/// assert_eq!(run_in_parallel(empty), Vec::<i32>::new());
///
/// let strings: Vec<Box<dyn FnOnce() -> String + Send>> = vec![
///     Box::new(|| "a".to_string()),
///     Box::new(|| "b".to_string()),
///     Box::new(|| "c".to_string()),
/// ];
/// assert_eq!(run_in_parallel(strings), vec!["a", "b", "c"]);
/// ```
pub fn run_in_parallel<T: Send + 'static>(
    tasks: Vec<Box<dyn FnOnce() -> T + Send + 'static>>,
) -> Vec<T> {
    todo!()
}
