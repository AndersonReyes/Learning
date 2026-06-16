//! Advanced 09 — Async/Await & Futures (Book ch. 17, adapted).

/// Returns `x * 2` asynchronously.
///
/// # Example
/// ```ignore
/// assert_eq!(pollster::block_on(async_double(21)), 42);
/// ```
pub async fn async_double(x: i64) -> i64 {
    todo!()
}

/// Applies an async function `f` to each element of `items` in order,
/// collecting results into a `Vec`.
///
/// # Example
/// ```ignore
/// let v = pollster::block_on(async_map(&[1_i32, 2, 3], |x| async move { x * 10 }));
/// assert_eq!(v, vec![10, 20, 30]);
/// ```
pub async fn async_map<A, B, F, Fut>(items: &[A], f: F) -> Vec<B>
where
    A: Copy,
    F: Fn(A) -> Fut,
    Fut: std::future::Future<Output = B>,
{
    todo!()
}

/// Returns every element of `items` for which the async predicate `pred`
/// returns `true`, preserving order.
///
/// # Example
/// ```ignore
/// let v = pollster::block_on(async_filter(&[1_i32, 2, 3, 4, 5], |x| async move { x % 2 == 0 }));
/// assert_eq!(v, vec![2, 4]);
/// ```
pub async fn async_filter<A, F, Fut>(items: &[A], pred: F) -> Vec<A>
where
    A: Copy,
    F: Fn(A) -> Fut,
    Fut: std::future::Future<Output = bool>,
{
    todo!()
}

/// Returns the first element of `items` for which the async predicate `pred`
/// returns `true`, or `None` if no element matches.
///
/// # Example
/// ```ignore
/// let v = pollster::block_on(async_first_match(&[1_i32, 3, 4, 7], |x| async move { x % 2 == 0 }));
/// assert_eq!(v, Some(4));
/// ```
pub async fn async_first_match<A, F, Fut>(items: &[A], pred: F) -> Option<A>
where
    A: Copy,
    F: Fn(A) -> Fut,
    Fut: std::future::Future<Output = bool>,
{
    todo!()
}

/// Folds `items` left-to-right using an async `combiner`, starting from
/// `init`.  Each call to `combiner` receives the current accumulator and the
/// current element and returns a `Future` yielding the next accumulator.
///
/// # Example
/// ```ignore
/// let sum = pollster::block_on(async_fold(&[1_i32, 2, 3, 4], 0, |acc, x| async move { acc + x }));
/// assert_eq!(sum, 10);
/// ```
pub async fn async_fold<A, B, F, Fut>(items: &[A], init: B, combiner: F) -> B
where
    A: Copy,
    F: Fn(B, A) -> Fut,
    Fut: std::future::Future<Output = B>,
{
    todo!()
}
