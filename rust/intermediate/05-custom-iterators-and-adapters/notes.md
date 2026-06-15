# Custom Iterators & Adapters

## The `Iterator` trait

```rust
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
    // ~70 more methods, all with default implementations
    // (map, filter, fold, take, zip, scan, collect, ...)
}
```

Implement `type Item` and `next` for your type, and you get **every** adaptor
and consuming method (`.map()`, `.filter()`, `.take()`, `.collect()`, `.sum()`,
...) for free — they're all defined in terms of `next`.

```rust
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

// for free, once `next` exists:
let sum: u32 = Counter::new().zip(Counter::new().skip(1))
    .map(|(a, b)| a * b)
    .filter(|x| x % 3 == 0)
    .sum();
```

## Infinite iterators

`next` can always return `Some` — the iterator never signals exhaustion. This
is fine and common (e.g. natural numbers, Fibonacci), but the caller **must**
bound it with `.take(n)`, `.take_while(...)`, or similar before `.collect()`
or any other consuming method — otherwise it loops forever / OOMs.

```rust
struct Naturals(u64);

impl Iterator for Naturals {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        self.0 += 1;
        Some(self.0) // always Some -- infinite
    }
}

let first_five: Vec<u64> = Naturals(0).take(5).collect(); // [1,2,3,4,5]
```

## `IntoIterator`: powering `for` loops

```rust
pub trait IntoIterator {
    type Item;
    type IntoIter: Iterator<Item = Self::Item>;
    fn into_iter(self) -> Self::IntoIter;
}
```

`for x in value { ... }` desugars to `for x in value.into_iter() { ... }`.
Implementing `IntoIterator` for a custom collection makes it work with `for`
loops, `.collect()`'s source position, and anywhere an `IntoIterator` bound is
required.

`Vec<T>::into_iter()` returns `std::vec::IntoIter<T>` (owned `T` items,
consumes the `Vec`). A common pattern for a collection wrapping `Vec<Vec<T>>`
(e.g. rows of a grid) is to flatten lazily:

```rust
struct GridIntoIter {
    rows: std::vec::IntoIter<Vec<i32>>,
    current_row: std::vec::IntoIter<i32>,
}

impl Iterator for GridIntoIter {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        loop {
            if let Some(item) = self.current_row.next() {
                return Some(item);
            }
            self.current_row = self.rows.next()?.into_iter(); // `?` -> None when rows exhausted
        }
    }
}
```

The `loop` + early-return-on-`?` pattern skips empty rows automatically:
if `current_row` is empty, pull the next row and try again; if there are no
more rows, `?` propagates `None`.

## Custom adapters: wrapping an inner iterator

An *adapter* is a struct holding another iterator (`iter: I`) plus whatever
extra state it needs, implementing `Iterator` itself by pulling from `iter` in
`next`.

```rust
struct Pairwise<I: Iterator> {
    iter: I,
    prev: Option<I::Item>,
}

fn pairwise<I: Iterator>(iter: I) -> Pairwise<I> {
    Pairwise { iter, prev: None }
}

impl<I: Iterator> Iterator for Pairwise<I>
where
    I::Item: Clone,
{
    type Item = (I::Item, I::Item);

    fn next(&mut self) -> Option<Self::Item> {
        if self.prev.is_none() {
            self.prev = self.iter.next();
        }
        let prev = self.prev.clone()?;
        let curr = self.iter.next()?;
        self.prev = Some(curr.clone());
        Some((prev, curr))
    }
}
```

### Lookahead / "peeking" without `Peekable`

To group or compare an item against the *next* one, you sometimes need to
consume an item from the inner iterator before you're ready to yield it. Store
it in a `peeked: Option<I::Item>` field and check that field first on the next
call — this is exactly what `std::iter::Peekable` does internally, but you
can build the same shape for adapters that need more than one slot of
lookahead or extra accumulated state (e.g. a running count).

```rust
struct RunLength<I: Iterator> {
    iter: I,
    peeked: Option<I::Item>,
}

impl<I: Iterator> Iterator for RunLength<I>
where
    I::Item: PartialEq + Clone,
{
    type Item = (I::Item, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.peeked.take().or_else(|| self.iter.next())?;
        let mut count = 1;
        loop {
            match self.iter.next() {
                Some(item) if item == current => count += 1,
                Some(item) => {
                    self.peeked = Some(item); // save for the *next* call
                    break;
                }
                None => break,
            }
        }
        Some((current, count))
    }
}
```

`I::Item: Clone` bounds show up whenever an adapter needs to hold on to an
item *and* yield a copy of it (or compare it to a later item) — `next`
consumes items from `iter` by value, so anything kept around must be cloned
or moved.

## Gotchas

- Forgetting `.take(n)` on an infinite iterator before `.collect()` hangs the
  program.
- `next` mutates `&mut self` — adapter state (e.g. `peeked`, `prev`, running
  totals) must live in fields, not locals, or it resets every call.
- `type Item` is part of the trait impl, not a generic parameter you choose
  per call — one `impl Iterator for X` fixes `X::Item` for all uses of `X`.

## Further Reading (Book)

- [Ch. 13.2 — Processing a Series of Items with Iterators](https://doc.rust-lang.org/book/ch13-02-iterators.html) (the `Counter` example, "Methods that Call `next`")
- [`std::iter::Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html) — full list of default methods
- [`std::iter::IntoIterator`](https://doc.rust-lang.org/std/iter/trait.IntoIterator.html)
- [`std::vec::IntoIter`](https://doc.rust-lang.org/std/vec/struct.IntoIter.html)
- [`std::iter::Peekable`](https://doc.rust-lang.org/std/iter/struct.Peekable.html)
