use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

fn main() {
    // ── 1. async fn and .await ────────────────────────────────────────────────
    // An `async fn` returns an opaque type that implements `Future`.
    // Calling it does NOT run it; you must drive it to completion.
    async fn double(x: i64) -> i64 {
        x * 2
    }

    let result = pollster::block_on(double(21));
    println!("double(21) = {result}"); // 42

    // ── 2. async blocks ───────────────────────────────────────────────────────
    // An `async { ... }` block is an anonymous future.
    let fut = async { 6 * 7 };
    println!("async block = {}", pollster::block_on(fut)); // 42

    // ── 3. Sequencing awaits (sequential, not concurrent) ────────────────────
    async fn fetch_and_double(x: i64) -> i64 {
        let a = double(x).await;    // first future completes
        let b = double(a).await;    // then second
        b
    }
    println!("fetch_and_double(5) = {}", pollster::block_on(fetch_and_double(5))); // 20

    // ── 4. async map over a slice ─────────────────────────────────────────────
    async fn async_map<A: Copy, B, F, Fut>(items: &[A], f: F) -> Vec<B>
    where
        F: Fn(A) -> Fut,
        Fut: Future<Output = B>,
    {
        let mut out = Vec::with_capacity(items.len());
        for &item in items {
            out.push(f(item).await);
        }
        out
    }

    let squares = pollster::block_on(async_map(&[1_i32, 2, 3, 4], |x| async move { x * x }));
    println!("squares = {squares:?}"); // [1, 4, 9, 16]

    // ── 5. async filter ───────────────────────────────────────────────────────
    async fn async_filter<A: Copy, F, Fut>(items: &[A], pred: F) -> Vec<A>
    where
        F: Fn(A) -> Fut,
        Fut: Future<Output = bool>,
    {
        let mut out = vec![];
        for &item in items {
            if pred(item).await {
                out.push(item);
            }
        }
        out
    }

    let evens = pollster::block_on(async_filter(&[1_i32, 2, 3, 4, 5], |x| async move { x % 2 == 0 }));
    println!("evens = {evens:?}"); // [2, 4]

    // ── 6. async fold ─────────────────────────────────────────────────────────
    async fn async_fold<A: Copy, B, F, Fut>(items: &[A], init: B, combiner: F) -> B
    where
        F: Fn(B, A) -> Fut,
        Fut: Future<Output = B>,
    {
        let mut acc = init;
        for &item in items {
            acc = combiner(acc, item).await;
        }
        acc
    }

    let sum = pollster::block_on(async_fold(&[1_i32, 2, 3, 4, 5], 0, |a, x| async move { a + x }));
    println!("sum = {sum}"); // 15

    let concat = pollster::block_on(async_fold(
        &["Hello", ", ", "world", "!"],
        String::new(),
        |mut acc, s| async move { acc.push_str(s); acc },
    ));
    println!("concat = {concat}"); // "Hello, world!"

    // ── 7. Implementing Future manually ──────────────────────────────────────
    // Under the hood every async fn compiles into a state machine that
    // implements `Future`. Here is the simplest possible manual Future:
    struct ReadyFuture<T>(Option<T>);

    impl<T: Unpin> Future for ReadyFuture<T> {
        type Output = T;

        fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<T> {
            Poll::Ready(self.0.take().expect("polled after completion"))
        }
    }

    let answer = pollster::block_on(ReadyFuture(Some(42_u32)));
    println!("ReadyFuture output = {answer}"); // 42

    // ── 8. Returning a boxed future from a function ───────────────────────────
    // When you need to erase the concrete future type (e.g. to store in a Vec
    // or return from a trait method), use `Pin<Box<dyn Future<...>>>`.
    fn make_future(x: i64) -> Pin<Box<dyn Future<Output = i64>>> {
        Box::pin(async move { x * 3 })
    }

    let v = pollster::block_on(make_future(7));
    println!("boxed future(7) = {v}"); // 21
}
