//! Intermediate 04 — Closures & Iterators.
//!
//! `notes.md` covers Book ch.13.1-13.2: closures (`Fn`/`FnMut`/`FnOnce`,
//! `move`, storing closures in structs, `Rc<RefCell<T>>` for shared mutable
//! captures), and iterators (`Iterator`, consuming vs. lazy adaptors,
//! `.scan()`, returning `impl Fn`/`Box<dyn Fn>`). The 5 exercises below
//! combine both: a generalized `Cacher` (`Memoizer`), function composition,
//! a retrying higher-order function, a generic top-N-by-key sort, and a
//! `.scan()`-based running min/max.

use std::collections::HashMap;

/// A generalized `Cacher` (book ch.13.1): wraps a `calculation: F` and caches
/// its result per distinct `arg`, so `calculation` runs at most once for each
/// `arg` ever passed to [`Memoizer::value`].
pub struct Memoizer<F: Fn(u64) -> u64> {
    calculation: F,
    cache: HashMap<u64, u64>,
}

impl<F: Fn(u64) -> u64> Memoizer<F> {
    /// Creates a new `Memoizer` wrapping `calculation`, with an empty cache.
    pub fn new(calculation: F) -> Self {
        Memoizer {
            calculation,
            cache: HashMap::new(),
        }
    }

    /// Returns `calculation(arg)`, computing it only on the first call for a
    /// given `arg`; subsequent calls with the same `arg` return the cached
    /// value without calling `calculation` again.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use intermediate_04_closures_and_iterators::Memoizer;
    /// use std::cell::RefCell;
    /// use std::rc::Rc;
    ///
    /// let calls = Rc::new(RefCell::new(0u32));
    /// let calls_clone = Rc::clone(&calls);
    /// let mut memo = Memoizer::new(move |x: u64| {
    ///     *calls_clone.borrow_mut() += 1;
    ///     x * x
    /// });
    ///
    /// assert_eq!(memo.value(4), 16);
    /// assert_eq!(memo.value(4), 16); // cached -- calculation not called again
    /// assert_eq!(memo.value(5), 25);
    /// assert_eq!(*calls.borrow(), 2); // called once for 4, once for 5
    /// ```
    pub fn value(&mut self, arg: u64) -> u64 {
        todo!()
    }
}

/// Returns a boxed closure equivalent to `|x| g(f(x))`.
///
/// # Examples
///
/// ```ignore
/// use intermediate_04_closures_and_iterators::compose;
///
/// let add_one = |x: i32| x + 1;
/// let double = |x: i32| x * 2;
/// let add_then_double = compose(add_one, double);
/// assert_eq!(add_then_double(3), 8); // (3 + 1) * 2 = 8
///
/// let negate = |x: i32| -x;
/// let combo = compose(add_then_double, negate);
/// assert_eq!(combo(2), -6); // ((2 + 1) * 2) = 6, negated = -6
/// ```
pub fn compose<A, B, C>(
    f: impl Fn(A) -> B + 'static,
    g: impl Fn(B) -> C + 'static,
) -> Box<dyn Fn(A) -> C> {
    todo!()
}

/// Calls `f(attempt)` for `attempt` in `1..=max_attempts` (in order),
/// returning the first `Ok` result. If every call returns `Err`, returns the
/// `Err` from the *last* attempt.
///
/// `f` is `FnMut` so it may track state (e.g. a call counter) across calls.
///
/// # Errors
///
/// Returns `Err("no attempts allowed".to_string())` immediately if
/// `max_attempts == 0`, without calling `f`.
///
/// # Examples
///
/// ```ignore
/// use intermediate_04_closures_and_iterators::retry;
///
/// let mut tries = 0;
/// let result = retry(5, |attempt| {
///     tries += 1;
///     if attempt < 3 {
///         Err(format!("attempt {attempt} failed"))
///     } else {
///         Ok(attempt * 10)
///     }
/// });
/// assert_eq!(result, Ok(30));
/// assert_eq!(tries, 3);
///
/// let result: Result<i32, String> = retry(0, |_| Ok(42));
/// assert_eq!(result, Err("no attempts allowed".to_string()));
/// ```
pub fn retry<T>(max_attempts: u32, f: impl FnMut(u32) -> Result<T, String>) -> Result<T, String> {
    todo!()
}

/// Returns references to the `n` items of `items` with the largest `key`,
/// sorted by `key` descending. Ties keep their original relative order
/// (stable sort). If `n > items.len()`, returns all items sorted.
///
/// # Examples
///
/// ```ignore
/// use intermediate_04_closures_and_iterators::top_n_by;
/// use std::cmp::Reverse;
///
/// let scores = [("Alice", 50), ("Bob", 80), ("Carol", 80), ("Dave", 30)];
///
/// let top2 = top_n_by(&scores, 2, |&(_, score)| score);
/// assert_eq!(top2, vec![&("Bob", 80), &("Carol", 80)]);
///
/// // smallest 2, via Reverse
/// let bottom2 = top_n_by(&scores, 2, |&(_, score)| Reverse(score));
/// assert_eq!(bottom2, vec![&("Dave", 30), &("Alice", 50)]);
///
/// assert_eq!(top_n_by(&scores, 0, |&(_, score)| score), Vec::<&(&str, i32)>::new());
/// ```
pub fn top_n_by<T, K, F>(items: &[T], n: usize, key: F) -> Vec<&T>
where
    K: Ord,
    F: Fn(&T) -> K,
{
    todo!()
}

/// Returns, for each prefix `values[..=i]`, the `(min, max)` of that prefix.
///
/// # Examples
///
/// ```ignore
/// use intermediate_04_closures_and_iterators::running_stats;
///
/// assert_eq!(
///     running_stats(&[3.0, 1.0, 4.0, 1.0, 5.0]),
///     vec![(3.0, 3.0), (1.0, 3.0), (1.0, 4.0), (1.0, 4.0), (1.0, 5.0)]
/// );
/// assert_eq!(running_stats(&[42.0]), vec![(42.0, 42.0)]);
/// assert_eq!(running_stats(&[]), Vec::<(f64, f64)>::new());
/// ```
pub fn running_stats(values: &[f64]) -> Vec<(f64, f64)> {
    todo!()
}
