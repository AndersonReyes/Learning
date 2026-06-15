use fundamentals_07_enums_and_pattern_matching::{
    classify_triangle, first_non_repeating_char, walk, Direction, Expr, TriangleKind,
};

#[test]
fn eval_single_number() {
    assert_eq!(Expr::Num(5.0).eval(), Some(5.0));
}

#[test]
fn eval_add() {
    let e = Expr::Add(Box::new(Expr::Num(2.0)), Box::new(Expr::Num(3.0)));
    assert_eq!(e.eval(), Some(5.0));
}

#[test]
fn eval_sub_can_be_negative() {
    let e = Expr::Sub(Box::new(Expr::Num(2.0)), Box::new(Expr::Num(5.0)));
    assert_eq!(e.eval(), Some(-3.0));
}

#[test]
fn eval_mul_and_div() {
    let mul = Expr::Mul(Box::new(Expr::Num(3.0)), Box::new(Expr::Num(4.0)));
    assert_eq!(mul.eval(), Some(12.0));

    let div = Expr::Div(Box::new(Expr::Num(10.0)), Box::new(Expr::Num(2.0)));
    assert_eq!(div.eval(), Some(5.0));
}

#[test]
fn eval_division_by_zero_is_none() {
    let e = Expr::Div(Box::new(Expr::Num(10.0)), Box::new(Expr::Num(0.0)));
    assert_eq!(e.eval(), None);
}

#[test]
fn eval_nested_expression() {
    // (2 + 3) * (4 - 1) = 15
    let e = Expr::Mul(
        Box::new(Expr::Add(Box::new(Expr::Num(2.0)), Box::new(Expr::Num(3.0)))),
        Box::new(Expr::Sub(Box::new(Expr::Num(4.0)), Box::new(Expr::Num(1.0)))),
    );
    assert_eq!(e.eval(), Some(15.0));
}

#[test]
fn eval_none_propagates_from_nested_division() {
    // 1 + (1 / 0) -> None
    let e = Expr::Add(
        Box::new(Expr::Num(1.0)),
        Box::new(Expr::Div(Box::new(Expr::Num(1.0)), Box::new(Expr::Num(0.0)))),
    );
    assert_eq!(e.eval(), None);
}

#[test]
fn classify_triangle_equilateral() {
    assert_eq!(classify_triangle(3.0, 3.0, 3.0), TriangleKind::Equilateral);
}

#[test]
fn classify_triangle_isosceles_any_pair() {
    assert_eq!(classify_triangle(3.0, 3.0, 4.0), TriangleKind::Isosceles);
    assert_eq!(classify_triangle(3.0, 4.0, 4.0), TriangleKind::Isosceles);
    assert_eq!(classify_triangle(4.0, 3.0, 4.0), TriangleKind::Isosceles);
}

#[test]
fn classify_triangle_scalene() {
    assert_eq!(classify_triangle(3.0, 4.0, 5.0), TriangleKind::Scalene);
}

#[test]
fn classify_triangle_degenerate_is_invalid() {
    // 1 + 1 == 2: fails the strict triangle inequality
    assert_eq!(classify_triangle(1.0, 1.0, 2.0), TriangleKind::Invalid);
}

#[test]
fn classify_triangle_zero_or_negative_side_is_invalid() {
    assert_eq!(classify_triangle(0.0, 1.0, 1.0), TriangleKind::Invalid);
    assert_eq!(classify_triangle(-1.0, 2.0, 2.0), TriangleKind::Invalid);
}

#[test]
fn classify_triangle_grossly_violates_inequality() {
    assert_eq!(classify_triangle(1.0, 2.0, 10.0), TriangleKind::Invalid);
}

#[test]
fn direction_from_token_valid_letters() {
    assert_eq!(Direction::from_token("N"), Some(Direction::North));
    assert_eq!(Direction::from_token("S"), Some(Direction::South));
    assert_eq!(Direction::from_token("E"), Some(Direction::East));
    assert_eq!(Direction::from_token("W"), Some(Direction::West));
}

#[test]
fn direction_from_token_invalid_input() {
    assert_eq!(Direction::from_token("X"), None);
    assert_eq!(Direction::from_token("north"), None);
    assert_eq!(Direction::from_token(""), None);
}

#[test]
fn walk_basic_path() {
    assert_eq!(
        walk((0, 0), &[Direction::North, Direction::North, Direction::East]),
        (1, 2)
    );
}

#[test]
fn walk_with_negatives() {
    assert_eq!(
        walk((5, 5), &[Direction::South, Direction::West, Direction::West]),
        (3, 4)
    );
}

#[test]
fn walk_empty_path_is_identity() {
    assert_eq!(walk((0, 0), &[]), (0, 0));
}

#[test]
fn walk_round_trip_cancels_out() {
    assert_eq!(
        walk(
            (0, 0),
            &[Direction::North, Direction::South, Direction::East, Direction::West]
        ),
        (0, 0)
    );
}

#[test]
fn walk_from_negative_start() {
    assert_eq!(
        walk((-1, -1), &[Direction::East, Direction::East, Direction::North]),
        (1, 0)
    );
}

#[test]
fn first_non_repeating_char_basic() {
    assert_eq!(first_non_repeating_char("swiss"), Some('w'));
}

#[test]
fn first_non_repeating_char_all_repeats_is_none() {
    assert_eq!(first_non_repeating_char("aabbcc"), None);
}

#[test]
fn first_non_repeating_char_empty_is_none() {
    assert_eq!(first_non_repeating_char(""), None);
}

#[test]
fn first_non_repeating_char_single_char() {
    assert_eq!(first_non_repeating_char("x"), Some('x'));
}

#[test]
fn first_non_repeating_char_returns_first_in_order() {
    assert_eq!(first_non_repeating_char("aabbc"), Some('c'));
    assert_eq!(first_non_repeating_char("abcabcd"), Some('d'));
}

#[test]
fn first_non_repeating_char_handles_multibyte_unicode() {
    // a, b, 🦀 each appear twice; c appears once.
    assert_eq!(first_non_repeating_char("ab🦀ab🦀c"), Some('c'));
}
