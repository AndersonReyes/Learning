use std::collections::HashMap;

use fundamentals_10_error_handling::{
    checked_transfer, eval_rpn, parse_all_or_first_error, parse_csv_row, validate_password,
    CalcError, RowError, TransferError,
};

#[test]
fn eval_rpn_basic_addition() {
    assert_eq!(eval_rpn(&["2", "3", "+"]), Ok(5.0));
}

#[test]
fn eval_rpn_division() {
    assert_eq!(eval_rpn(&["4", "2", "/"]), Ok(2.0));
}

#[test]
fn eval_rpn_classic_expression() {
    // 5 + ((1 + 2) * 4) - 3 = 14
    assert_eq!(
        eval_rpn(&["5", "1", "2", "+", "4", "*", "+", "3", "-"]),
        Ok(14.0)
    );
}

#[test]
fn eval_rpn_division_by_zero() {
    assert_eq!(eval_rpn(&["4", "0", "/"]), Err(CalcError::DivisionByZero));
}

#[test]
fn eval_rpn_invalid_number() {
    assert_eq!(
        eval_rpn(&["2", "abc", "+"]),
        Err(CalcError::InvalidNumber("abc".to_string()))
    );
}

#[test]
fn eval_rpn_stack_underflow_one_operand() {
    assert_eq!(eval_rpn(&["2", "+"]), Err(CalcError::StackUnderflow));
}

#[test]
fn eval_rpn_extra_operands() {
    assert_eq!(eval_rpn(&["2", "3"]), Err(CalcError::ExtraOperands(1)));
}

#[test]
fn eval_rpn_unknown_operator() {
    assert_eq!(
        eval_rpn(&["2", "3", "%"]),
        Err(CalcError::UnknownOperator("%".to_string()))
    );
}

#[test]
fn eval_rpn_empty_input_is_underflow() {
    assert_eq!(eval_rpn(&[]), Err(CalcError::StackUnderflow));
}

#[test]
fn eval_rpn_negative_number_literal() {
    // "-3" (length 2) is a negative number, not the "-" operator.
    assert_eq!(eval_rpn(&["-3", "2", "+"]), Ok(-1.0));
}

#[test]
fn parse_csv_row_basic() {
    assert_eq!(
        parse_csv_row("1.5, 2.0, 3.25", 3),
        Ok(vec![1.5, 2.0, 3.25])
    );
}

#[test]
fn parse_csv_row_wrong_column_count() {
    assert_eq!(
        parse_csv_row("1, 2", 3),
        Err(RowError::WrongColumnCount {
            expected: 3,
            actual: 2
        })
    );
}

#[test]
fn parse_csv_row_invalid_number_reports_column() {
    assert_eq!(
        parse_csv_row("1, x, 3", 3),
        Err(RowError::InvalidNumber {
            column: 1,
            value: "x".to_string()
        })
    );
}

#[test]
fn parse_csv_row_trims_whitespace() {
    assert_eq!(parse_csv_row("  4.0  ,5", 2), Ok(vec![4.0, 5.0]));
}

#[test]
fn parse_csv_row_empty_string() {
    assert_eq!(
        parse_csv_row("", 1),
        Err(RowError::InvalidNumber {
            column: 0,
            value: "".to_string()
        })
    );
}

#[test]
fn parse_csv_row_negative_numbers() {
    assert_eq!(parse_csv_row("-1.5,2.5", 2), Ok(vec![-1.5, 2.5]));
}

#[test]
fn checked_transfer_success() {
    let mut balances = HashMap::from([("alice".to_string(), 100), ("bob".to_string(), 50)]);
    assert_eq!(checked_transfer(&mut balances, "alice", "bob", 30), Ok(()));
    assert_eq!(balances["alice"], 70);
    assert_eq!(balances["bob"], 80);
}

#[test]
fn checked_transfer_from_not_found() {
    let mut balances = HashMap::from([("bob".to_string(), 50)]);
    assert_eq!(
        checked_transfer(&mut balances, "alice", "bob", 10),
        Err(TransferError::AccountNotFound("alice".to_string()))
    );
    assert_eq!(balances["bob"], 50);
    assert!(!balances.contains_key("alice"));
}

#[test]
fn checked_transfer_to_not_found() {
    let mut balances = HashMap::from([("alice".to_string(), 100)]);
    assert_eq!(
        checked_transfer(&mut balances, "alice", "bob", 10),
        Err(TransferError::AccountNotFound("bob".to_string()))
    );
    assert_eq!(balances["alice"], 100);
}

#[test]
fn checked_transfer_insufficient_funds() {
    let mut balances = HashMap::from([("alice".to_string(), 50), ("bob".to_string(), 0)]);
    assert_eq!(
        checked_transfer(&mut balances, "alice", "bob", 100),
        Err(TransferError::InsufficientFunds {
            available: 50,
            requested: 100
        })
    );
    assert_eq!(balances["alice"], 50);
    assert_eq!(balances["bob"], 0);
}

#[test]
fn checked_transfer_invalid_amount() {
    let mut balances = HashMap::from([("alice".to_string(), 50), ("bob".to_string(), 0)]);
    assert_eq!(
        checked_transfer(&mut balances, "alice", "bob", 0),
        Err(TransferError::InvalidAmount(0))
    );
    assert_eq!(
        checked_transfer(&mut balances, "alice", "bob", -10),
        Err(TransferError::InvalidAmount(-10))
    );
    assert_eq!(balances["alice"], 50);
    assert_eq!(balances["bob"], 0);
}

#[test]
fn checked_transfer_invalid_amount_checked_before_accounts() {
    let mut balances: HashMap<String, i64> = HashMap::new();
    assert_eq!(
        checked_transfer(&mut balances, "alice", "bob", -5),
        Err(TransferError::InvalidAmount(-5))
    );
}

#[test]
fn checked_transfer_self_transfer_is_noop() {
    let mut balances = HashMap::from([("alice".to_string(), 100)]);
    assert_eq!(
        checked_transfer(&mut balances, "alice", "alice", 30),
        Ok(())
    );
    assert_eq!(balances["alice"], 100);
}

#[test]
fn parse_all_or_first_error_all_valid() {
    assert_eq!(
        parse_all_or_first_error(&["1", "2", "3"]),
        Ok(vec![1, 2, 3])
    );
}

#[test]
fn parse_all_or_first_error_empty() {
    assert_eq!(parse_all_or_first_error(&[]), Ok(vec![]));
}

#[test]
fn parse_all_or_first_error_reports_index_and_error() {
    assert_eq!(
        parse_all_or_first_error(&["1", "x", "3"]),
        Err((1, "x".parse::<i64>().unwrap_err()))
    );
}

#[test]
fn parse_all_or_first_error_negative_numbers() {
    assert_eq!(
        parse_all_or_first_error(&["10", "-5", "0"]),
        Ok(vec![10, -5, 0])
    );
}

#[test]
fn parse_all_or_first_error_whitespace_not_trimmed() {
    assert_eq!(
        parse_all_or_first_error(&["  4", "5"]),
        Err((0, "  4".parse::<i64>().unwrap_err()))
    );
}

#[test]
fn validate_password_all_rules_pass() {
    assert_eq!(validate_password("Abcdef1!"), Ok(()));
}

#[test]
fn validate_password_all_rules_fail() {
    assert_eq!(
        validate_password(""),
        Err(vec![
            "password must be at least 8 characters long".to_string(),
            "password must contain an uppercase letter".to_string(),
            "password must contain a lowercase letter".to_string(),
            "password must contain a digit".to_string(),
            "password must contain a special character (!@#$%^&*)".to_string(),
        ])
    );
}

#[test]
fn validate_password_too_short_lowercase_only() {
    assert_eq!(
        validate_password("abc"),
        Err(vec![
            "password must be at least 8 characters long".to_string(),
            "password must contain an uppercase letter".to_string(),
            "password must contain a digit".to_string(),
            "password must contain a special character (!@#$%^&*)".to_string(),
        ])
    );
}

#[test]
fn validate_password_all_uppercase_long_enough() {
    assert_eq!(
        validate_password("ABCDEFGH"),
        Err(vec![
            "password must contain a lowercase letter".to_string(),
            "password must contain a digit".to_string(),
            "password must contain a special character (!@#$%^&*)".to_string(),
        ])
    );
}

#[test]
fn validate_password_missing_only_digit() {
    assert_eq!(
        validate_password("Password!"),
        Err(vec!["password must contain a digit".to_string()])
    );
}

#[test]
fn validate_password_with_dollar_sign_special_char() {
    assert_eq!(validate_password("Password1$"), Ok(()));
}
