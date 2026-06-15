//! Intermediate 01 — Generics, Traits & Lifetimes.
//!
//! Exercises combine generic data types, trait bounds (incl. `std::ops::Add`),
//! and lifetime-annotated structs from `notes.md`.

use std::ops::Add;

/// A node in a [`Bst`]'s tree, holding `value` plus optional left/right
/// children. Not exported; only `Bst`'s methods touch it.
struct Node<T: Ord> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

/// A generic binary search tree over any `T: Ord`.
///
/// Duplicate values (per `Ord`) are ignored on [`insert`](Bst::insert) — the
/// tree contains at most one of each distinct value.
pub struct Bst<T: Ord> {
    root: Option<Box<Node<T>>>,
}

impl<T: Ord> Bst<T> {
    /// Creates an empty tree.
    pub fn new() -> Self {
        Bst { root: None }
    }

    /// Inserts `value`, maintaining the BST ordering invariant. If an equal
    /// value (per `Ord`) is already present, the tree is left unchanged.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use intermediate_01_generics_traits_and_lifetimes::Bst;
    ///
    /// let mut tree = Bst::new();
    /// for v in [5, 3, 8, 1, 4, 7, 9, 3, 5] {
    ///     tree.insert(v);
    /// }
    /// assert_eq!(tree.in_order(), vec![&1, &3, &4, &5, &7, &8, &9]);
    /// ```
    pub fn insert(&mut self, value: T) {
        todo!()
    }

    /// Returns `true` if `value` (per `Ord`) is present in the tree.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use intermediate_01_generics_traits_and_lifetimes::Bst;
    ///
    /// let mut tree = Bst::new();
    /// tree.insert(5);
    /// tree.insert(3);
    /// assert!(tree.contains(&3));
    /// assert!(!tree.contains(&4));
    /// ```
    pub fn contains(&self, value: &T) -> bool {
        todo!()
    }

    /// Returns references to every value in ascending order (in-order
    /// traversal).
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use intermediate_01_generics_traits_and_lifetimes::Bst;
    ///
    /// let mut tree: Bst<i32> = Bst::new();
    /// assert_eq!(tree.in_order(), Vec::<&i32>::new());
    ///
    /// tree.insert(2);
    /// tree.insert(1);
    /// assert_eq!(tree.in_order(), vec![&1, &2]);
    /// ```
    pub fn in_order(&self) -> Vec<&T> {
        todo!()
    }
}

impl<T: Ord> Default for Bst<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// A monetary amount in integer cents, supporting `+` via [`Add`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Money {
    pub cents: i64,
}

impl Add for Money {
    type Output = Money;

    fn add(self, other: Money) -> Money {
        Money {
            cents: self.cents + other.cents,
        }
    }
}

/// Sums `items` using `+`, starting from `T::default()`.
///
/// Works for any `T` implementing `Add<Output = T> + Copy + Default` — both
/// numeric primitives and custom types like [`Money`].
///
/// # Examples
///
/// ```ignore
/// use intermediate_01_generics_traits_and_lifetimes::{sum_all, Money};
///
/// assert_eq!(sum_all(&[1, 2, 3, 4]), 10);
/// assert_eq!(sum_all::<i32>(&[]), 0);
/// assert_eq!(sum_all(&[1.5_f64, 2.5]), 4.0);
///
/// let amounts = [Money { cents: 100 }, Money { cents: 250 }, Money { cents: 50 }];
/// assert_eq!(sum_all(&amounts), Money { cents: 400 });
/// ```
pub fn sum_all<T>(items: &[T]) -> T
where
    T: Add<Output = T> + Copy + Default,
{
    todo!()
}

/// A lexer over `&'a str` that yields maximal runs of ASCII alphanumeric
/// characters as slices borrowed from the original input.
pub struct Tokenizer<'a> {
    remaining: &'a str,
}

impl<'a> Tokenizer<'a> {
    /// Creates a tokenizer over `input`.
    pub fn new(input: &'a str) -> Self {
        Tokenizer { remaining: input }
    }

    /// Returns the next token: a maximal run of `char::is_ascii_alphanumeric`
    /// characters, skipping any other characters (whitespace, punctuation,
    /// non-ASCII) before it. Returns `None` once no characters remain, and
    /// continues returning `None` on subsequent calls.
    ///
    /// The returned `&str` is a slice of the original input passed to
    /// [`new`](Tokenizer::new), not an owned copy.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use intermediate_01_generics_traits_and_lifetimes::Tokenizer;
    ///
    /// let mut t = Tokenizer::new("Hello, world! 123-abc");
    /// assert_eq!(t.next_token(), Some("Hello"));
    /// assert_eq!(t.next_token(), Some("world"));
    /// assert_eq!(t.next_token(), Some("123"));
    /// assert_eq!(t.next_token(), Some("abc"));
    /// assert_eq!(t.next_token(), None);
    /// assert_eq!(t.next_token(), None);
    ///
    /// // non-ASCII characters act as separators too
    /// let mut t = Tokenizer::new("café 42");
    /// assert_eq!(t.next_token(), Some("caf"));
    /// assert_eq!(t.next_token(), Some("42"));
    /// ```
    pub fn next_token(&mut self) -> Option<&'a str> {
        todo!()
    }
}
