# Async/Await & Futures

Book ch. 17 (adapted). Uses `pollster` as a minimal single-threaded runtime.

---

## The `Future` trait

```rust
pub trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

- `Poll::Ready(value)` — computation is done, value is returned.
- `Poll::Pending` — not done yet; the executor should wake this task when
  progress is possible.

A runtime (executor) drives `Future`s to completion by calling `poll()` in a
loop (or when a `Waker` fires).

## `async fn` and `.await`

```rust
async fn fetch_data() -> String {
    // ... some async work
    "result".to_string()
}

async fn use_data() {
    let s = fetch_data().await;  // suspends until fetch_data is ready
    println!("{s}");
}
```

An `async fn` returns an anonymous type that implements `Future<Output = T>`.
`.await` inside an `async fn` yields control to the runtime until the inner
future completes.

## `async` blocks

```rust
let fut = async {
    let a = first().await;
    let b = second(a).await;
    a + b
};
```

## Running a future: block_on

Without a runtime, use `pollster::block_on` or `futures::executor::block_on`
to synchronously drive a future to completion:

```rust
let result = pollster::block_on(async { 42 });
assert_eq!(result, 42);
```

## Joining futures concurrently

`futures::join!` / `futures::future::join_all` run multiple futures
concurrently on the same thread (cooperative multitasking, no parallelism):

```rust
// Using manual join via async block (no futures crate needed):
async fn both() -> (i32, i32) {
    let a = first();
    let b = second();
    (a.await, b.await) // sequential, not concurrent — .await suspends immediately
}
```

For true concurrent execution on a single thread, you need `select!` or
`join!` from the `futures` crate, or an async runtime like `tokio`.

## Common patterns

### `async` closures (nightly) vs. returning `impl Future`

Stable way to make a higher-order async function:

```rust
fn map_async<F, Fut>(items: &[i32], f: F) -> impl Future<Output = Vec<i32>> + '_
where
    F: Fn(i32) -> Fut,
    Fut: Future<Output = i32>,
{
    async move {
        let mut out = vec![];
        for &item in items {
            out.push(f(item).await);
        }
        out
    }
}
```

### `Box<dyn Future<Output = T> + Send>`

Used when returning a future from a trait method or when the concrete type
is erased. Requires `Pin<Box<...>>` for safe polling.

## Gotchas

- Futures are lazy — they do nothing until polled.
- `.await` is only valid inside `async` context.
- `async fn` bodies must be `Send` if you want to send them across threads
  (all captured state must be `Send`).
- Long non-async blocking calls inside `async fn` block the executor thread —
  use `spawn_blocking` in real runtimes.
- `pollster` is single-threaded and deterministic — fine for these exercises.

## Further Reading

- [Book ch. 17 — Async and Await](https://doc.rust-lang.org/book/ch17-00-async-await.html)
- [`std::future::Future`](https://doc.rust-lang.org/std/future/trait.Future.html)
- [`std::task::Poll`](https://doc.rust-lang.org/std/task/enum.Poll.html)
- [`pollster` crate](https://docs.rs/pollster/latest/pollster/)
