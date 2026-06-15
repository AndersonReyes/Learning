//! Fundamentals 07 — Enums & Pattern Matching.
//!
//! Exercises cover defining enums (including a recursive AST type via
//! `Box`), `Option<T>`, and `match`/`if let` from `notes.md`.

/// A simple arithmetic expression tree.
///
/// `Add`/`Sub`/`Mul`/`Div` hold boxed sub-expressions (recursive types need
/// indirection — see `notes.md`).
#[derive(Debug, PartialEq)]
pub enum Expr {
    Num(f64),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
}

impl Expr {
    /// Recursively evaluates the expression, returning `None` if a `Div`
    /// node anywhere in the tree divides by zero.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use fundamentals_07_enums_and_pattern_matching::Expr;
    ///
    /// // 2 + 3 = 5
    /// let e = Expr::Add(Box::new(Expr::Num(2.0)), Box::new(Expr::Num(3.0)));
    /// assert_eq!(e.eval(), Some(5.0));
    ///
    /// // 10 / 0 -> None
    /// let e = Expr::Div(Box::new(Expr::Num(10.0)), Box::new(Expr::Num(0.0)));
    /// assert_eq!(e.eval(), None);
    ///
    /// // (2 + 3) * (4 - 1) = 15
    /// let e = Expr::Mul(
    ///     Box::new(Expr::Add(Box::new(Expr::Num(2.0)), Box::new(Expr::Num(3.0)))),
    ///     Box::new(Expr::Sub(Box::new(Expr::Num(4.0)), Box::new(Expr::Num(1.0)))),
    /// );
    /// assert_eq!(e.eval(), Some(15.0));
    ///
    /// // a None deep in the tree propagates out
    /// let e = Expr::Add(
    ///     Box::new(Expr::Num(1.0)),
    ///     Box::new(Expr::Div(Box::new(Expr::Num(1.0)), Box::new(Expr::Num(0.0)))),
    /// );
    /// assert_eq!(e.eval(), None);
    /// ```
    pub fn eval(&self) -> Option<f64> {
        todo!()
    }
}

/// The classification of a triangle by its three side lengths.
#[derive(Debug, PartialEq)]
pub enum TriangleKind {
    Equilateral,
    Isosceles,
    Scalene,
    /// The side lengths don't form a valid triangle (non-positive, or
    /// fail the triangle inequality).
    Invalid,
}

/// Classifies the triangle with side lengths `a`, `b`, `c`.
///
/// # Examples
///
/// ```ignore
/// use fundamentals_07_enums_and_pattern_matching::{classify_triangle, TriangleKind};
///
/// assert_eq!(classify_triangle(3.0, 3.0, 3.0), TriangleKind::Equilateral);
/// assert_eq!(classify_triangle(3.0, 3.0, 4.0), TriangleKind::Isosceles);
/// assert_eq!(classify_triangle(3.0, 4.0, 5.0), TriangleKind::Scalene);
/// // 1 + 1 == 2, which does not exceed 3 -> not a valid triangle
/// assert_eq!(classify_triangle(1.0, 1.0, 2.0), TriangleKind::Invalid);
/// assert_eq!(classify_triangle(0.0, 1.0, 1.0), TriangleKind::Invalid);
/// ```
pub fn classify_triangle(a: f64, b: f64, c: f64) -> TriangleKind {
    todo!()
}

/// A compass direction.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    /// Parses a single-letter token (`"N"`, `"S"`, `"E"`, `"W"`) into a
    /// `Direction`, or returns `None` for any other input.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use fundamentals_07_enums_and_pattern_matching::Direction;
    ///
    /// assert_eq!(Direction::from_token("N"), Some(Direction::North));
    /// assert_eq!(Direction::from_token("W"), Some(Direction::West));
    /// assert_eq!(Direction::from_token("X"), None);
    /// assert_eq!(Direction::from_token("north"), None);
    /// ```
    pub fn from_token(s: &str) -> Option<Direction> {
        todo!()
    }
}

/// Starting at `start`, applies each direction in `path` as a unit step
/// (`North`/`South` change the second coordinate, `East`/`West` change the
/// first), returning the final position.
///
/// # Examples
///
/// ```ignore
/// use fundamentals_07_enums_and_pattern_matching::{walk, Direction};
///
/// assert_eq!(
///     walk((0, 0), &[Direction::North, Direction::North, Direction::East]),
///     (1, 2)
/// );
/// assert_eq!(
///     walk((5, 5), &[Direction::South, Direction::West, Direction::West]),
///     (3, 4)
/// );
/// assert_eq!(walk((0, 0), &[]), (0, 0));
/// ```
pub fn walk(start: (i32, i32), path: &[Direction]) -> (i32, i32) {
    todo!()
}

/// Returns the first character in `s` that appears exactly once, in order
/// of first occurrence, or `None` if every character repeats (or `s` is
/// empty).
///
/// # Examples
///
/// ```ignore
/// use fundamentals_07_enums_and_pattern_matching::first_non_repeating_char;
///
/// assert_eq!(first_non_repeating_char("swiss"), Some('w'));
/// assert_eq!(first_non_repeating_char("aabbcc"), None);
/// assert_eq!(first_non_repeating_char(""), None);
/// assert_eq!(first_non_repeating_char("x"), Some('x'));
/// assert_eq!(first_non_repeating_char("aabbc"), Some('c'));
/// ```
pub fn first_non_repeating_char(s: &str) -> Option<char> {
    todo!()
}
