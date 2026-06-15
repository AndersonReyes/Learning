//! Fundamentals 06 — Structs & Methods.
//!
//! A single exercise type, `Polynomial`, exercises struct definitions,
//! associated functions (constructors), `&self` methods, and returning
//! owned `Self` from `notes.md`.

/// A polynomial with `f64` coefficients, stored in order of increasing
/// degree: `coeffs[i]` is the coefficient of `x^i`.
///
/// Canonical form has no trailing zero coefficients — the zero polynomial
/// is represented as `coeffs == vec![]`. All constructors and operations
/// below must preserve this invariant so that two equal polynomials compare
/// equal with `==` (via the derived `PartialEq`).
#[derive(Debug, Clone, PartialEq)]
pub struct Polynomial {
    pub coeffs: Vec<f64>,
}

impl Polynomial {
    /// Constructs a new `Polynomial` from coefficients in order of
    /// increasing degree, trimming any trailing zero coefficients so the
    /// representation is canonical.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// assert_eq!(Polynomial::new(vec![1.0, 2.0, 0.0, 0.0]).coeffs, vec![1.0, 2.0]);
    /// assert_eq!(Polynomial::new(vec![0.0, 0.0]).coeffs, Vec::<f64>::new());
    /// assert_eq!(Polynomial::new(vec![]).coeffs, Vec::<f64>::new());
    /// // an interior zero coefficient is NOT trimmed:
    /// assert_eq!(Polynomial::new(vec![1.0, 0.0, 3.0]).coeffs, vec![1.0, 0.0, 3.0]);
    /// ```
    pub fn new(coeffs: Vec<f64>) -> Self {
        todo!()
    }

    /// Evaluates the polynomial at `x` using Horner's method.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // 1 + 2x + 3x^2 at x = 2 -> 1 + 4 + 12 = 17
    /// assert_eq!(Polynomial::new(vec![1.0, 2.0, 3.0]).evaluate(2.0), 17.0);
    /// // the zero polynomial evaluates to 0 everywhere
    /// assert_eq!(Polynomial::new(vec![]).evaluate(5.0), 0.0);
    /// // a constant polynomial evaluates to that constant everywhere
    /// assert_eq!(Polynomial::new(vec![7.0]).evaluate(100.0), 7.0);
    /// ```
    pub fn evaluate(&self, x: f64) -> f64 {
        todo!()
    }

    /// Returns the derivative of `self` as a new `Polynomial`.
    ///
    /// The derivative of `sum(coeffs[i] * x^i)` is `sum(i * coeffs[i] *
    /// x^(i-1))` for `i >= 1`. The derivative of a constant (or the zero
    /// polynomial) is the zero polynomial.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // d/dx (1 + 2x + 3x^2) = 2 + 6x
    /// assert_eq!(Polynomial::new(vec![1.0, 2.0, 3.0]).derivative(), Polynomial::new(vec![2.0, 6.0]));
    /// // d/dx (constant) = 0
    /// assert_eq!(Polynomial::new(vec![5.0]).derivative(), Polynomial::new(vec![]));
    /// assert_eq!(Polynomial::new(vec![]).derivative(), Polynomial::new(vec![]));
    /// // d/dx (x + x^3) = 1 + 3x^2
    /// assert_eq!(Polynomial::new(vec![0.0, 1.0, 0.0, 1.0]).derivative(), Polynomial::new(vec![1.0, 0.0, 3.0]));
    /// ```
    pub fn derivative(&self) -> Self {
        todo!()
    }

    /// Returns `self + other` as a new `Polynomial` (coefficient-wise sum,
    /// padding the shorter polynomial with zeros).
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // (1 + 2x + 3x^2) + (5 - 3x^2 + 7x^3) = 6 + 2x + 0x^2 + 7x^3
    /// assert_eq!(
    ///     Polynomial::new(vec![1.0, 2.0, 3.0]).add(&Polynomial::new(vec![5.0, 0.0, -3.0, 7.0])),
    ///     Polynomial::new(vec![6.0, 2.0, 0.0, 7.0])
    /// );
    /// // a polynomial plus its negation is the zero polynomial
    /// assert_eq!(
    ///     Polynomial::new(vec![1.0, 2.0, 3.0]).add(&Polynomial::new(vec![-1.0, -2.0, -3.0])),
    ///     Polynomial::new(vec![])
    /// );
    /// ```
    pub fn add(&self, other: &Self) -> Self {
        todo!()
    }

    /// Returns `self * other` as a new `Polynomial` (polynomial
    /// multiplication via convolution of the coefficient vectors).
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // (1 + x) * (1 + x) = 1 + 2x + x^2
    /// assert_eq!(
    ///     Polynomial::new(vec![1.0, 1.0]).multiply(&Polynomial::new(vec![1.0, 1.0])),
    ///     Polynomial::new(vec![1.0, 2.0, 1.0])
    /// );
    /// // multiplying by the zero polynomial gives the zero polynomial
    /// assert_eq!(
    ///     Polynomial::new(vec![]).multiply(&Polynomial::new(vec![1.0, 2.0, 3.0])),
    ///     Polynomial::new(vec![])
    /// );
    /// // (1 - x) * (1 + x) = 1 - x^2
    /// assert_eq!(
    ///     Polynomial::new(vec![1.0, -1.0]).multiply(&Polynomial::new(vec![1.0, 1.0])),
    ///     Polynomial::new(vec![1.0, 0.0, -1.0])
    /// );
    /// ```
    pub fn multiply(&self, other: &Self) -> Self {
        todo!()
    }
}
