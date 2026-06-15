use fundamentals_06_structs_and_methods::Polynomial;

#[test]
fn new_trims_trailing_zeros() {
    assert_eq!(Polynomial::new(vec![1.0, 2.0, 0.0, 0.0]).coeffs, vec![1.0, 2.0]);
}

#[test]
fn new_all_zeros_is_empty() {
    assert_eq!(Polynomial::new(vec![0.0, 0.0, 0.0]).coeffs, Vec::<f64>::new());
}

#[test]
fn new_empty_input_is_empty() {
    assert_eq!(Polynomial::new(vec![]).coeffs, Vec::<f64>::new());
}

#[test]
fn new_preserves_interior_zero_coefficient() {
    assert_eq!(Polynomial::new(vec![1.0, 0.0, 3.0]).coeffs, vec![1.0, 0.0, 3.0]);
}

#[test]
fn new_single_nonzero_coefficient_is_unchanged() {
    assert_eq!(Polynomial::new(vec![5.0]).coeffs, vec![5.0]);
}

#[test]
fn evaluate_quadratic() {
    // 1 + 2x + 3x^2 at x = 2 -> 1 + 4 + 12 = 17
    assert_eq!(Polynomial::new(vec![1.0, 2.0, 3.0]).evaluate(2.0), 17.0);
}

#[test]
fn evaluate_at_zero_returns_constant_term() {
    assert_eq!(Polynomial::new(vec![1.0, 2.0, 3.0]).evaluate(0.0), 1.0);
}

#[test]
fn evaluate_zero_polynomial_is_always_zero() {
    assert_eq!(Polynomial::new(vec![]).evaluate(5.0), 0.0);
    assert_eq!(Polynomial::new(vec![]).evaluate(-100.0), 0.0);
}

#[test]
fn evaluate_constant_polynomial() {
    assert_eq!(Polynomial::new(vec![7.0]).evaluate(100.0), 7.0);
}

#[test]
fn evaluate_negative_x() {
    // 1 + 2x + 3x^2 at x = -1 -> 1 - 2 + 3 = 2
    assert_eq!(Polynomial::new(vec![1.0, 2.0, 3.0]).evaluate(-1.0), 2.0);
}

#[test]
fn derivative_quadratic() {
    // d/dx (1 + 2x + 3x^2) = 2 + 6x
    assert_eq!(
        Polynomial::new(vec![1.0, 2.0, 3.0]).derivative(),
        Polynomial::new(vec![2.0, 6.0])
    );
}

#[test]
fn derivative_constant_is_zero_polynomial() {
    assert_eq!(Polynomial::new(vec![5.0]).derivative(), Polynomial::new(vec![]));
}

#[test]
fn derivative_zero_polynomial_is_zero_polynomial() {
    assert_eq!(Polynomial::new(vec![]).derivative(), Polynomial::new(vec![]));
}

#[test]
fn derivative_cubic_with_interior_zero() {
    // d/dx (x + x^3) = 1 + 3x^2
    assert_eq!(
        Polynomial::new(vec![0.0, 1.0, 0.0, 1.0]).derivative(),
        Polynomial::new(vec![1.0, 0.0, 3.0])
    );
}

#[test]
fn add_basic_different_lengths() {
    // (1 + 2x + 3x^2) + (5 - 3x^2 + 7x^3) = 6 + 2x + 0x^2 + 7x^3
    assert_eq!(
        Polynomial::new(vec![1.0, 2.0, 3.0]).add(&Polynomial::new(vec![5.0, 0.0, -3.0, 7.0])),
        Polynomial::new(vec![6.0, 2.0, 0.0, 7.0])
    );
}

#[test]
fn add_to_negation_yields_zero_polynomial() {
    assert_eq!(
        Polynomial::new(vec![1.0, 2.0, 3.0]).add(&Polynomial::new(vec![-1.0, -2.0, -3.0])),
        Polynomial::new(vec![])
    );
}

#[test]
fn add_with_zero_polynomial_is_identity() {
    assert_eq!(
        Polynomial::new(vec![1.0, 2.0]).add(&Polynomial::new(vec![])),
        Polynomial::new(vec![1.0, 2.0])
    );
    assert_eq!(
        Polynomial::new(vec![]).add(&Polynomial::new(vec![1.0, 2.0])),
        Polynomial::new(vec![1.0, 2.0])
    );
}

#[test]
fn add_is_commutative() {
    let a = Polynomial::new(vec![1.0, 2.0, 3.0]);
    let b = Polynomial::new(vec![4.0, -1.0]);
    assert_eq!(a.add(&b), b.add(&a));
}

#[test]
fn multiply_two_linear_factors() {
    // (1 + x) * (1 + x) = 1 + 2x + x^2
    assert_eq!(
        Polynomial::new(vec![1.0, 1.0]).multiply(&Polynomial::new(vec![1.0, 1.0])),
        Polynomial::new(vec![1.0, 2.0, 1.0])
    );
}

#[test]
fn multiply_constant_times_linear() {
    // 2 * (1 + 3x) = 2 + 6x
    assert_eq!(
        Polynomial::new(vec![2.0]).multiply(&Polynomial::new(vec![1.0, 3.0])),
        Polynomial::new(vec![2.0, 6.0])
    );
}

#[test]
fn multiply_by_zero_polynomial_is_zero() {
    assert_eq!(
        Polynomial::new(vec![]).multiply(&Polynomial::new(vec![1.0, 2.0, 3.0])),
        Polynomial::new(vec![])
    );
    assert_eq!(
        Polynomial::new(vec![1.0, 2.0, 3.0]).multiply(&Polynomial::new(vec![])),
        Polynomial::new(vec![])
    );
}

#[test]
fn multiply_difference_of_squares() {
    // (1 - x) * (1 + x) = 1 - x^2
    assert_eq!(
        Polynomial::new(vec![1.0, -1.0]).multiply(&Polynomial::new(vec![1.0, 1.0])),
        Polynomial::new(vec![1.0, 0.0, -1.0])
    );
}

#[test]
fn multiply_is_commutative() {
    let a = Polynomial::new(vec![1.0, 2.0]);
    let b = Polynomial::new(vec![3.0, 0.0, -2.0]);
    assert_eq!(a.multiply(&b), b.multiply(&a));
}
