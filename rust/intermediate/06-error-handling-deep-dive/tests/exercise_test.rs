use intermediate_06_error_handling_deep_dive::{
    describe_error, error_chain_messages, first_valid_port, parse_duration_ms, process_record,
    NoValidPortError, NotFoundError, ParseDurationError, PermissionError, RecordError,
    WrappedError,
};
use std::error::Error;

// --- parse_duration_ms -------------------------------------------------------------

#[test]
fn parse_duration_ms_combined_units() {
    assert_eq!(parse_duration_ms("1h30m45s"), Ok(5_445_000));
}

#[test]
fn parse_duration_ms_single_unit() {
    assert_eq!(parse_duration_ms("500ms"), Ok(500));
}

#[test]
fn parse_duration_ms_all_units() {
    assert_eq!(parse_duration_ms("1h1m1s1ms"), Ok(3_661_001));
}

#[test]
fn parse_duration_ms_empty_string() {
    assert_eq!(parse_duration_ms(""), Err(ParseDurationError::Empty));
}

#[test]
fn parse_duration_ms_unknown_unit() {
    assert_eq!(
        parse_duration_ms("10x"),
        Err(ParseDurationError::UnknownUnit("x".to_string()))
    );
}

#[test]
fn parse_duration_ms_no_unit() {
    assert_eq!(
        parse_duration_ms("100"),
        Err(ParseDurationError::UnknownUnit(String::new()))
    );
}

#[test]
fn parse_duration_ms_zero_seconds() {
    assert_eq!(parse_duration_ms("0s"), Ok(0));
}

#[test]
fn parse_duration_ms_days() {
    assert_eq!(parse_duration_ms("2d"), Ok(172_800_000));
}

#[test]
fn parse_duration_ms_repeated_units_sum() {
    assert_eq!(parse_duration_ms("100ms200ms"), Ok(300));
}

#[test]
fn parse_duration_ms_invalid_number_via_from() {
    let err = parse_duration_ms("99999999999999999999s").unwrap_err();
    assert!(matches!(err, ParseDurationError::InvalidNumber(_)));
    assert!(err.source().is_some());
}

// --- error_chain_messages -----------------------------------------------------------

#[test]
fn error_chain_messages_three_levels() {
    let innermost = WrappedError {
        message: "disk full".to_string(),
        source: None,
    };
    let middle = WrappedError {
        message: "failed to write file".to_string(),
        source: Some(Box::new(innermost)),
    };
    let outer = WrappedError {
        message: "failed to save config".to_string(),
        source: Some(Box::new(middle)),
    };

    assert_eq!(
        error_chain_messages(&outer),
        vec![
            "failed to save config".to_string(),
            "failed to write file".to_string(),
            "disk full".to_string(),
        ]
    );
}

#[test]
fn error_chain_messages_single_error() {
    let single = WrappedError {
        message: "oops".to_string(),
        source: None,
    };
    assert_eq!(error_chain_messages(&single), vec!["oops".to_string()]);
}

#[test]
fn error_chain_messages_two_levels() {
    let inner = WrappedError {
        message: "connection reset".to_string(),
        source: None,
    };
    let outer = WrappedError {
        message: "request failed".to_string(),
        source: Some(Box::new(inner)),
    };
    assert_eq!(
        error_chain_messages(&outer),
        vec!["request failed".to_string(), "connection reset".to_string()]
    );
}

#[test]
fn error_chain_messages_heterogeneous_source() {
    let parse_err = "abc".parse::<i32>().unwrap_err();
    let outer = WrappedError {
        message: "config value invalid".to_string(),
        source: Some(Box::new(parse_err.clone())),
    };
    assert_eq!(
        error_chain_messages(&outer),
        vec!["config value invalid".to_string(), parse_err.to_string()]
    );
}

#[test]
fn error_chain_messages_four_levels() {
    let l4 = WrappedError {
        message: "l4".to_string(),
        source: None,
    };
    let l3 = WrappedError {
        message: "l3".to_string(),
        source: Some(Box::new(l4)),
    };
    let l2 = WrappedError {
        message: "l2".to_string(),
        source: Some(Box::new(l3)),
    };
    let l1 = WrappedError {
        message: "l1".to_string(),
        source: Some(Box::new(l2)),
    };
    assert_eq!(
        error_chain_messages(&l1),
        vec![
            "l1".to_string(),
            "l2".to_string(),
            "l3".to_string(),
            "l4".to_string(),
        ]
    );
}

// --- describe_error -------------------------------------------------------------------

#[test]
fn describe_error_not_found() {
    let e = NotFoundError {
        key: "foo".to_string(),
    };
    assert_eq!(describe_error(&e), "not found: please check the key 'foo'");
}

#[test]
fn describe_error_permission() {
    let e = PermissionError {
        action: "delete".to_string(),
    };
    assert_eq!(describe_error(&e), "permission denied while trying to delete");
}

#[test]
fn describe_error_unknown() {
    let e = "x".parse::<i32>().unwrap_err();
    assert_eq!(describe_error(&e), format!("unknown error: {e}"));
}

#[test]
fn describe_error_not_found_different_key() {
    let e = NotFoundError {
        key: "user_id_42".to_string(),
    };
    assert_eq!(
        describe_error(&e),
        "not found: please check the key 'user_id_42'"
    );
}

#[test]
fn describe_error_record_error_falls_through_to_unknown() {
    let e = RecordError::MissingField;
    assert_eq!(describe_error(&e), format!("unknown error: {e}"));
}

// --- process_record -------------------------------------------------------------------

#[test]
fn process_record_valid() {
    assert_eq!(
        process_record("Alice:30").unwrap(),
        ("Alice".to_string(), 30)
    );
}

#[test]
fn process_record_missing_field() {
    let err = process_record("Bob").unwrap_err();
    assert_eq!(
        err.downcast_ref::<RecordError>(),
        Some(&RecordError::MissingField)
    );
}

#[test]
fn process_record_too_many_fields() {
    let err = process_record("Carol:30:extra").unwrap_err();
    assert_eq!(
        err.downcast_ref::<RecordError>(),
        Some(&RecordError::TooManyFields)
    );
}

#[test]
fn process_record_invalid_age_number() {
    let err = process_record("Dave:abc").unwrap_err();
    assert!(err.downcast_ref::<std::num::ParseIntError>().is_some());
}

#[test]
fn process_record_age_out_of_range() {
    let err = process_record("Eve:200").unwrap_err();
    assert_eq!(
        err.downcast_ref::<RecordError>(),
        Some(&RecordError::AgeOutOfRange(200))
    );
}

#[test]
fn process_record_trims_whitespace() {
    assert_eq!(
        process_record("  Frank : 45 ").unwrap(),
        ("Frank".to_string(), 45)
    );
}

#[test]
fn process_record_age_boundary_150_is_valid() {
    assert_eq!(
        process_record("Grace:150").unwrap(),
        ("Grace".to_string(), 150)
    );
}

#[test]
fn process_record_age_boundary_151_is_out_of_range() {
    let err = process_record("Henry:151").unwrap_err();
    assert_eq!(
        err.downcast_ref::<RecordError>(),
        Some(&RecordError::AgeOutOfRange(151))
    );
}

#[test]
fn process_record_empty_string_is_missing_field() {
    let err = process_record("").unwrap_err();
    assert_eq!(
        err.downcast_ref::<RecordError>(),
        Some(&RecordError::MissingField)
    );
}

// --- first_valid_port -------------------------------------------------------------------

#[test]
fn first_valid_port_skips_invalid_then_picks_valid() {
    assert_eq!(first_valid_port(&["abc", "80", "8080"]).unwrap(), 8080);
}

#[test]
fn first_valid_port_returns_first_valid_not_largest() {
    assert_eq!(first_valid_port(&["8080", "9090"]).unwrap(), 8080);
}

#[test]
fn first_valid_port_all_parse_but_none_in_range() {
    let err = first_valid_port(&["80", "100"]).unwrap_err();
    assert!(err.downcast_ref::<NoValidPortError>().is_some());
}

#[test]
fn first_valid_port_none_parse() {
    let err = first_valid_port(&["abc", "def"]).unwrap_err();
    assert!(err.downcast_ref::<std::num::ParseIntError>().is_some());
}

#[test]
fn first_valid_port_empty_candidates() {
    let err = first_valid_port(&[]).unwrap_err();
    assert!(err.downcast_ref::<NoValidPortError>().is_some());
}

#[test]
fn first_valid_port_boundary_1024_is_valid() {
    assert_eq!(first_valid_port(&["1023", "1024"]).unwrap(), 1024);
}

#[test]
fn first_valid_port_max_u16_is_valid() {
    assert_eq!(first_valid_port(&["65535"]).unwrap(), 65535);
}

#[test]
fn first_valid_port_overflowing_u16_is_parse_error() {
    let err = first_valid_port(&["65536"]).unwrap_err();
    assert!(err.downcast_ref::<std::num::ParseIntError>().is_some());
}
