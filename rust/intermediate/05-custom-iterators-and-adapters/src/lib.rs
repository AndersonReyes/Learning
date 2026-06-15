//! Intermediate 05 — Custom Iterators & Adapters.
//!
//! `notes.md` covers Book ch.13.2 (deepening): implementing the `Iterator`
//! trait (`type Item`, `next`) to get every adaptor/consuming method for
//! free, infinite iterators bounded by `.take()`, `IntoIterator` for custom
//! collections, and custom adapters that wrap an inner iterator (with
//! `peeked`/`prev` state for lookahead). The 5 exercises below: an infinite
//! `Fibonacci` iterator, a `Pairwise` adapter (consecutive pairs), a
//! `RunLength` adapter (run-length encoding via lookahead), a
//! `ChunksIterator` adapter (fixed-size chunks), and a `Grid` type with a
//! custom `IntoIterator` impl (row-major flattening).

/// An infinite iterator over the Fibonacci sequence, starting `0, 1, 1, 2,
/// 3, 5, 8, ...`.
///
/// Created via [`fibonacci`]. Always returns `Some` -- bound with `.take(n)`
/// before collecting.
pub struct Fibonacci {
    curr: u64,
    next: u64,
}

/// Returns a new [`Fibonacci`] iterator starting at `0, 1, 1, 2, 3, ...`.
pub fn fibonacci() -> Fibonacci {
    Fibonacci { curr: 0, next: 1 }
}

impl Iterator for Fibonacci {
    type Item = u64;

    /// Returns the next Fibonacci number, advancing the sequence.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use intermediate_05_custom_iterators_and_adapters::fibonacci;
    ///
    /// let first10: Vec<u64> = fibonacci().take(10).collect();
    /// assert_eq!(first10, vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
    /// ```
    fn next(&mut self) -> Option<u64> {
        todo!()
    }
}

/// A custom iterator adapter yielding consecutive pairs `(prev, curr)` of
/// `iter`'s items -- `n` items in, `n - 1` pairs out.
///
/// Created via [`pairwise`].
pub struct Pairwise<I: Iterator> {
    iter: I,
    prev: Option<I::Item>,
}

/// Wraps `iter` to yield consecutive pairs of its items.
pub fn pairwise<I: Iterator>(iter: I) -> Pairwise<I> {
    Pairwise { iter, prev: None }
}

impl<I: Iterator> Iterator for Pairwise<I>
where
    I::Item: Clone,
{
    type Item = (I::Item, I::Item);

    /// Returns the next consecutive pair, or `None` once fewer than 2 items
    /// remain.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use intermediate_05_custom_iterators_and_adapters::pairwise;
    ///
    /// let pairs: Vec<(i32, i32)> = pairwise(vec![1, 2, 3, 4].into_iter()).collect();
    /// assert_eq!(pairs, vec![(1, 2), (2, 3), (3, 4)]);
    ///
    /// let none: Vec<(i32, i32)> = pairwise(vec![1].into_iter()).collect();
    /// assert_eq!(none, Vec::<(i32, i32)>::new());
    /// ```
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

/// A custom iterator adapter performing run-length encoding: groups
/// consecutive equal items from `iter` into `(item, count)` pairs.
///
/// Created via [`run_length`].
pub struct RunLength<I: Iterator> {
    iter: I,
    peeked: Option<I::Item>,
}

/// Wraps `iter` to yield `(item, count)` for each run of consecutive equal
/// items.
pub fn run_length<I: Iterator>(iter: I) -> RunLength<I> {
    RunLength { iter, peeked: None }
}

impl<I: Iterator> Iterator for RunLength<I>
where
    I::Item: PartialEq + Clone,
{
    type Item = (I::Item, usize);

    /// Returns the next `(item, count)` run, or `None` once `iter` is
    /// exhausted.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use intermediate_05_custom_iterators_and_adapters::run_length;
    ///
    /// let runs: Vec<(char, usize)> = run_length("aaabccccd".chars()).collect();
    /// assert_eq!(runs, vec![('a', 3), ('b', 1), ('c', 4), ('d', 1)]);
    ///
    /// let empty: Vec<(char, usize)> = run_length("".chars()).collect();
    /// assert_eq!(empty, Vec::<(char, usize)>::new());
    /// ```
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

/// A custom iterator adapter yielding fixed-size, non-overlapping chunks of
/// `iter`'s items as `Vec<I::Item>`. The final chunk may be shorter than
/// `size` if `iter`'s length isn't a multiple of `size`. If `size == 0`,
/// yields nothing.
///
/// Created via [`chunks`].
pub struct ChunksIterator<I: Iterator> {
    iter: I,
    size: usize,
}

/// Wraps `iter` to yield chunks of `size` items at a time.
pub fn chunks<I: Iterator>(iter: I, size: usize) -> ChunksIterator<I> {
    ChunksIterator { iter, size }
}

impl<I: Iterator> Iterator for ChunksIterator<I> {
    type Item = Vec<I::Item>;

    /// Returns the next chunk, or `None` once `iter` is exhausted.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use intermediate_05_custom_iterators_and_adapters::chunks;
    ///
    /// let result: Vec<Vec<i32>> = chunks(vec![1, 2, 3, 4, 5].into_iter(), 2).collect();
    /// assert_eq!(result, vec![vec![1, 2], vec![3, 4], vec![5]]);
    ///
    /// let exact: Vec<Vec<i32>> = chunks(vec![1, 2, 3, 4].into_iter(), 2).collect();
    /// assert_eq!(exact, vec![vec![1, 2], vec![3, 4]]);
    /// ```
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

/// A 2D grid of `i32` stored as rows of (possibly uneven-length) `Vec<i32>`.
pub struct Grid {
    rows: Vec<Vec<i32>>,
}

impl Grid {
    /// Creates a new `Grid` from `rows`.
    pub fn new(rows: Vec<Vec<i32>>) -> Self {
        Grid { rows }
    }
}

/// The iterator produced by [`Grid`]'s [`IntoIterator`] impl, yielding every
/// element in row-major order (empty rows are skipped).
pub struct GridIntoIter {
    rows: std::vec::IntoIter<Vec<i32>>,
    current_row: std::vec::IntoIter<i32>,
}

impl IntoIterator for Grid {
    type Item = i32;
    type IntoIter = GridIntoIter;

    fn into_iter(self) -> GridIntoIter {
        let mut rows = self.rows.into_iter();
        let current_row = rows.next().unwrap_or_default().into_iter();
        GridIntoIter { rows, current_row }
    }
}

impl Iterator for GridIntoIter {
    type Item = i32;

    /// Returns the next element in row-major order, or `None` once every row
    /// is exhausted.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use intermediate_05_custom_iterators_and_adapters::Grid;
    ///
    /// let grid = Grid::new(vec![vec![1, 2], vec![3, 4, 5]]);
    /// let flat: Vec<i32> = grid.into_iter().collect();
    /// assert_eq!(flat, vec![1, 2, 3, 4, 5]);
    ///
    /// let with_empty_rows = Grid::new(vec![vec![], vec![1], vec![]]);
    /// let flat2: Vec<i32> = with_empty_rows.into_iter().collect();
    /// assert_eq!(flat2, vec![1]);
    ///
    /// let empty = Grid::new(vec![]);
    /// let flat3: Vec<i32> = empty.into_iter().collect();
    /// assert_eq!(flat3, Vec::<i32>::new());
    /// ```
    fn next(&mut self) -> Option<i32> {
        todo!()
    }
}
