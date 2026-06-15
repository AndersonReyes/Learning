use advanced_02_patterns_and_matching::{
    balanced_brackets, classify_triangle, longest_run, parse_ipv4, simplify,
    Expr::{self, *},
};

// helper to box an Expr
fn b(e: Expr) -> Box<Expr> {
    Box::new(e)
}

// --- simplify -------------------------------------------------------------------------

#[test]
fn simplify_num_is_unchanged() {
    assert_eq!(simplify(Num(5)), Num(5));
}

#[test]
fn simplify_var_is_unchanged() {
    assert_eq!(simplify(Var("x")), Var("x"));
}

#[test]
fn simplify_add_zero_left() {
    assert_eq!(simplify(Add(b(Num(0)), b(Var("x")))), Var("x"));
}

#[test]
fn simplify_add_zero_right() {
    assert_eq!(simplify(Add(b(Num(5)), b(Num(0)))), Num(5));
}

#[test]
fn simplify_add_no_rule_unchanged() {
    assert_eq!(simplify(Add(b(Num(2)), b(Num(3)))), Add(b(Num(2)), b(Num(3))));
}

#[test]
fn simplify_sub_same_var() {
    assert_eq!(simplify(Sub(b(Var("x")), b(Var("x")))), Num(0));
}

#[test]
fn simplify_sub_different_vars_unchanged() {
    assert_eq!(
        simplify(Sub(b(Var("x")), b(Var("y")))),
        Sub(b(Var("x")), b(Var("y")))
    );
}

#[test]
fn simplify_mul_by_zero_right() {
    assert_eq!(simplify(Mul(b(Var("x")), b(Num(0)))), Num(0));
}

#[test]
fn simplify_mul_by_zero_left() {
    assert_eq!(simplify(Mul(b(Num(0)), b(Var("x")))), Num(0));
}

#[test]
fn simplify_mul_by_one_right() {
    assert_eq!(simplify(Mul(b(Num(5)), b(Num(1)))), Num(5));
}

#[test]
fn simplify_mul_by_one_left() {
    assert_eq!(simplify(Mul(b(Num(1)), b(Var("x")))), Var("x"));
}

#[test]
fn simplify_mul_no_rule_unchanged() {
    assert_eq!(
        simplify(Mul(b(Num(3)), b(Num(4)))),
        Mul(b(Num(3)), b(Num(4)))
    );
}

#[test]
fn simplify_double_negation() {
    assert_eq!(simplify(Neg(b(Neg(b(Num(7)))))), Num(7));
}

#[test]
fn simplify_single_neg_unchanged() {
    assert_eq!(simplify(Neg(b(Num(3)))), Neg(b(Num(3))));
}

#[test]
fn simplify_nested_rules_apply_recursively() {
    // 1 * (0 + x)  →  1 * x  →  x
    let expr = Mul(b(Num(1)), b(Add(b(Num(0)), b(Var("x")))));
    assert_eq!(simplify(expr), Var("x"));
}

#[test]
fn simplify_nested_double_neg_and_add() {
    // 0 + Neg(Neg(3))  →  0 + 3  →  3
    let expr = Add(b(Num(0)), b(Neg(b(Neg(b(Num(3)))))));
    assert_eq!(simplify(expr), Num(3));
}

#[test]
fn simplify_sub_reduces_after_recursive_simplification() {
    // (1 * x) - x  →  x - x  →  0
    let expr = Sub(b(Mul(b(Num(1)), b(Var("x")))), b(Var("x")));
    assert_eq!(simplify(expr), Num(0));
}

// --- parse_ipv4 -----------------------------------------------------------------------

#[test]
fn parse_ipv4_typical_address() {
    assert_eq!(parse_ipv4("192.168.1.1"), Some([192, 168, 1, 1]));
}

#[test]
fn parse_ipv4_all_zeros() {
    assert_eq!(parse_ipv4("0.0.0.0"), Some([0, 0, 0, 0]));
}

#[test]
fn parse_ipv4_max_values() {
    assert_eq!(parse_ipv4("255.255.255.255"), Some([255, 255, 255, 255]));
}

#[test]
fn parse_ipv4_small_address() {
    assert_eq!(parse_ipv4("1.2.3.4"), Some([1, 2, 3, 4]));
}

#[test]
fn parse_ipv4_too_few_octets() {
    assert_eq!(parse_ipv4("192.168.1"), None);
}

#[test]
fn parse_ipv4_too_many_octets() {
    assert_eq!(parse_ipv4("192.168.1.1.1"), None);
}

#[test]
fn parse_ipv4_octet_out_of_range() {
    assert_eq!(parse_ipv4("192.168.1.256"), None);
}

#[test]
fn parse_ipv4_non_numeric_octet() {
    assert_eq!(parse_ipv4("192.168.1.abc"), None);
}

#[test]
fn parse_ipv4_empty_string() {
    assert_eq!(parse_ipv4(""), None);
}

#[test]
fn parse_ipv4_trailing_dot() {
    assert_eq!(parse_ipv4("1.2.3."), None); // last segment "" can't parse
}

// --- balanced_brackets ----------------------------------------------------------------

#[test]
fn brackets_empty_string() {
    assert!(balanced_brackets(""));
}

#[test]
fn brackets_single_pair_parens() {
    assert!(balanced_brackets("()"));
}

#[test]
fn brackets_single_pair_square() {
    assert!(balanced_brackets("[]"));
}

#[test]
fn brackets_single_pair_curly() {
    assert!(balanced_brackets("{}"));
}

#[test]
fn brackets_multiple_sequential_pairs() {
    assert!(balanced_brackets("()[]{}"));
}

#[test]
fn brackets_nested_valid() {
    assert!(balanced_brackets("{[()]}"));
}

#[test]
fn brackets_with_text_inside() {
    assert!(balanced_brackets("(hello [world])"));
}

#[test]
fn brackets_wrong_nesting_order() {
    assert!(!balanced_brackets("([)]"));
}

#[test]
fn brackets_unclosed() {
    assert!(!balanced_brackets("("));
}

#[test]
fn brackets_extra_close() {
    assert!(!balanced_brackets(")"));
}

#[test]
fn brackets_extra_close_square() {
    assert!(!balanced_brackets("]"));
}

#[test]
fn brackets_wrong_close_type() {
    assert!(!balanced_brackets("{[}"));
}

#[test]
fn brackets_deeply_nested_valid() {
    assert!(balanced_brackets("((()))"));
}

// --- longest_run ---------------------------------------------------------------------

#[test]
fn longest_run_empty_slice() {
    assert_eq!(longest_run::<i32>(&[]), None);
}

#[test]
fn longest_run_single_element() {
    assert_eq!(longest_run(&[42_i32]), Some((&42, 1)));
}

#[test]
fn longest_run_all_same() {
    assert_eq!(longest_run(&[3_i32, 3, 3]), Some((&3, 3)));
}

#[test]
fn longest_run_all_distinct() {
    assert_eq!(longest_run(&[1_i32, 2, 3]), Some((&1, 1)));
}

#[test]
fn longest_run_run_in_middle() {
    assert_eq!(longest_run(&[1_i32, 2, 2, 2, 3]), Some((&2, 3)));
}

#[test]
fn longest_run_run_at_end() {
    assert_eq!(longest_run(&[1_i32, 1, 2, 2, 2]), Some((&2, 3)));
}

#[test]
fn longest_run_run_at_start() {
    assert_eq!(longest_run(&[5_i32, 5, 5, 1, 2]), Some((&5, 3)));
}

#[test]
fn longest_run_first_longest_wins_on_tie() {
    assert_eq!(longest_run(&[1_i32, 1, 2, 2]), Some((&1, 2)));
}

#[test]
fn longest_run_works_with_strings() {
    assert_eq!(
        longest_run(&["a", "a", "b", "b", "b"]),
        Some((&"b", 3))
    );
}

// --- classify_triangle ---------------------------------------------------------------

#[test]
fn triangle_right_3_4_5() {
    assert_eq!(classify_triangle(3, 4, 5), "right");
}

#[test]
fn triangle_right_6_8_10() {
    assert_eq!(classify_triangle(6, 8, 10), "right");
}

#[test]
fn triangle_right_5_12_13() {
    assert_eq!(classify_triangle(5, 12, 13), "right");
}

#[test]
fn triangle_equilateral() {
    assert_eq!(classify_triangle(5, 5, 5), "equilateral");
}

#[test]
fn triangle_isosceles() {
    assert_eq!(classify_triangle(5, 5, 7), "isosceles");
}

#[test]
fn triangle_isosceles_other_pair() {
    assert_eq!(classify_triangle(7, 5, 7), "isosceles");
}

#[test]
fn triangle_scalene() {
    assert_eq!(classify_triangle(3, 4, 6), "scalene");
}

#[test]
fn triangle_invalid_zero_side() {
    assert_eq!(classify_triangle(0, 3, 4), "invalid");
}

#[test]
fn triangle_invalid_degenerate() {
    assert_eq!(classify_triangle(1, 2, 3), "invalid"); // 1+2 == 3, not >
}

#[test]
fn triangle_invalid_too_short() {
    assert_eq!(classify_triangle(1, 1, 10), "invalid");
}

#[test]
fn triangle_right_order_independent() {
    // 3-4-5 in any order should be "right"
    assert_eq!(classify_triangle(5, 3, 4), "right");
    assert_eq!(classify_triangle(4, 5, 3), "right");
}
