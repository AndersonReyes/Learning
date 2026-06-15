use intermediate_03_cli_io_project::{
    grep_report, highlight_matches, parse_args, resolve_ignore_case, search_lines, Config,
};

const SAMPLE: &str = "Rust\nTrust\nrust and dust\nCRUST";

fn args(parts: &[&str]) -> Vec<String> {
    parts.iter().map(|s| s.to_string()).collect()
}

// --- parse_args ---------------------------------------------------------

#[test]
fn parse_args_basic() {
    assert_eq!(
        parse_args(&args(&["minigrep", "to", "poem.txt"])),
        Ok(Config {
            query: "to".to_string(),
            path: "poem.txt".to_string(),
            ignore_case: false,
            line_numbers: false,
        })
    );
}

#[test]
fn parse_args_flag_before_positionals() {
    assert_eq!(
        parse_args(&args(&["minigrep", "-i", "to", "poem.txt"])),
        Ok(Config {
            query: "to".to_string(),
            path: "poem.txt".to_string(),
            ignore_case: true,
            line_numbers: false,
        })
    );
}

#[test]
fn parse_args_flag_between_positionals() {
    assert_eq!(
        parse_args(&args(&["minigrep", "to", "-n", "poem.txt"])),
        Ok(Config {
            query: "to".to_string(),
            path: "poem.txt".to_string(),
            ignore_case: false,
            line_numbers: true,
        })
    );
}

#[test]
fn parse_args_long_flags_after_positionals() {
    assert_eq!(
        parse_args(&args(&[
            "minigrep",
            "to",
            "poem.txt",
            "--ignore-case",
            "--line-numbers"
        ])),
        Ok(Config {
            query: "to".to_string(),
            path: "poem.txt".to_string(),
            ignore_case: true,
            line_numbers: true,
        })
    );
}

#[test]
fn parse_args_too_few() {
    assert_eq!(
        parse_args(&args(&["minigrep", "to"])),
        Err("not enough arguments: expected query and path".to_string())
    );
}

#[test]
fn parse_args_no_args_at_all() {
    assert_eq!(
        parse_args(&[]),
        Err("not enough arguments: expected query and path".to_string())
    );
}

#[test]
fn parse_args_too_many() {
    assert_eq!(
        parse_args(&args(&["minigrep", "to", "poem.txt", "extra"])),
        Err("too many arguments".to_string())
    );
}

#[test]
fn parse_args_unknown_flag() {
    assert_eq!(
        parse_args(&args(&["minigrep", "-x", "to", "poem.txt"])),
        Err("unknown flag: -x".to_string())
    );
}

// --- search_lines ---------------------------------------------------------

#[test]
fn search_lines_case_sensitive() {
    assert_eq!(
        search_lines("rust", SAMPLE, false),
        vec!["Trust", "rust and dust"]
    );
}

#[test]
fn search_lines_case_insensitive() {
    assert_eq!(
        search_lines("rust", SAMPLE, true),
        vec!["Rust", "Trust", "rust and dust", "CRUST"]
    );
}

#[test]
fn search_lines_no_match() {
    assert_eq!(search_lines("xyz", SAMPLE, false), Vec::<&str>::new());
}

#[test]
fn search_lines_empty_query_matches_all() {
    assert_eq!(
        search_lines("", SAMPLE, false),
        vec!["Rust", "Trust", "rust and dust", "CRUST"]
    );
}

#[test]
fn search_lines_empty_contents() {
    assert_eq!(search_lines("rust", "", false), Vec::<&str>::new());
}

// --- highlight_matches ------------------------------------------------------

#[test]
fn highlight_matches_multiple_non_overlapping() {
    assert_eq!(
        highlight_matches("the cat sat on the mat", "at", false),
        "the c**at** s**at** on the m**at**"
    );
}

#[test]
fn highlight_matches_adjacent_matches() {
    assert_eq!(highlight_matches("AAAA", "AA", false), "**AA****AA**");
}

#[test]
fn highlight_matches_case_insensitive_preserves_case() {
    assert_eq!(
        highlight_matches("Hello World", "o", true),
        "Hell**o** W**o**rld"
    );
}

#[test]
fn highlight_matches_empty_query_unchanged() {
    assert_eq!(highlight_matches("test", "", false), "test");
}

#[test]
fn highlight_matches_no_match_unchanged() {
    assert_eq!(highlight_matches("abc", "xyz", false), "abc");
}

#[test]
fn highlight_matches_unicode() {
    assert_eq!(
        highlight_matches("café au lait", "é", false),
        "caf**é** au lait"
    );
}

#[test]
fn highlight_matches_case_insensitive_whole_word() {
    assert_eq!(highlight_matches("ABAB", "ab", true), "**AB****AB**");
}

// --- grep_report ---------------------------------------------------------

#[test]
fn grep_report_case_sensitive_no_line_numbers() {
    assert_eq!(
        grep_report("rust", SAMPLE, false, false),
        "T**rust**\n**rust** and dust\n2 matches found"
    );
}

#[test]
fn grep_report_case_insensitive_with_line_numbers() {
    assert_eq!(
        grep_report("rust", SAMPLE, true, true),
        "1: **Rust**\n2: T**rust**\n3: **rust** and dust\n4: C**RUST**\n4 matches found"
    );
}

#[test]
fn grep_report_no_matches() {
    assert_eq!(grep_report("xyz", SAMPLE, false, false), "0 matches found");
}

#[test]
fn grep_report_single_match_uses_singular() {
    let contents = "alpha\nbeta\ngamma";
    assert_eq!(
        grep_report("beta", contents, false, false),
        "**beta**\n1 match found"
    );
}

#[test]
fn grep_report_empty_contents() -> Result<(), String> {
    let report = grep_report("rust", "", false, false);
    if report == "0 matches found" {
        Ok(())
    } else {
        Err(format!("expected \"0 matches found\", got {report:?}"))
    }
}

// --- resolve_ignore_case ----------------------------------------------------

#[test]
fn resolve_ignore_case_cli_flag_wins() {
    assert!(resolve_ignore_case(true, None));
    assert!(resolve_ignore_case(true, Some("0")));
    assert!(resolve_ignore_case(true, Some("false")));
}

#[test]
fn resolve_ignore_case_env_absent() {
    assert!(!resolve_ignore_case(false, None));
}

#[test]
fn resolve_ignore_case_env_truthy_values() {
    assert!(resolve_ignore_case(false, Some("1")));
    assert!(resolve_ignore_case(false, Some("yes")));
    assert!(resolve_ignore_case(false, Some("true")));
}

#[test]
fn resolve_ignore_case_env_falsy_values() {
    assert!(!resolve_ignore_case(false, Some("0")));
    assert!(!resolve_ignore_case(false, Some("false")));
    assert!(!resolve_ignore_case(false, Some("FALSE")));
    assert!(!resolve_ignore_case(false, Some("")));
    assert!(!resolve_ignore_case(false, Some("  ")));
}
