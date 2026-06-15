//! Intermediate 03 — CLI I/O Project.
//!
//! `notes.md` covers Book ch.12's `minigrep`: `std::env::args`,
//! `std::fs::read_to_string`, `Config::build`, `Box<dyn Error>`, TDD'd
//! `search`/`search_case_insensitive`, `std::env::var`, and `println!` vs.
//! `eprintln!`. The 5 exercises below build the pieces of a small grep-like
//! tool as pure, testable functions: argument parsing, line search, match
//! highlighting, report formatting, and environment-variable resolution.

/// Parsed command-line configuration for the grep-like tool.
#[derive(Debug, PartialEq, Eq)]
pub struct Config {
    pub query: String,
    pub path: String,
    pub ignore_case: bool,
    pub line_numbers: bool,
}

/// Parses command-line arguments (as from [`std::env::args`]) into a
/// [`Config`].
///
/// `args[0]` is the program name and is ignored. The remaining arguments
/// are either:
///
/// - **flags**: `-i`/`--ignore-case` sets [`Config::ignore_case`],
///   `-n`/`--line-numbers` sets [`Config::line_numbers`]. Flags may appear
///   before, after, or between the positional arguments.
/// - **positional arguments**: exactly two are required, in order: the
///   search query, then the file path.
///
/// # Errors
///
/// - `Err("not enough arguments: expected query and path")` if fewer than 2
///   positional arguments are given.
/// - `Err("too many arguments")` if more than 2 positional arguments are
///   given.
/// - `Err("unknown flag: <flag>")` for any argument starting with `-` that
///   isn't `-i`, `--ignore-case`, `-n`, or `--line-numbers`.
///
/// # Examples
///
/// ```ignore
/// use intermediate_03_cli_io_project::{parse_args, Config};
///
/// let args: Vec<String> = ["minigrep", "to", "poem.txt"]
///     .iter().map(|s| s.to_string()).collect();
/// assert_eq!(
///     parse_args(&args),
///     Ok(Config {
///         query: "to".to_string(),
///         path: "poem.txt".to_string(),
///         ignore_case: false,
///         line_numbers: false,
///     })
/// );
///
/// let args: Vec<String> = ["minigrep", "-i", "to", "poem.txt", "--line-numbers"]
///     .iter().map(|s| s.to_string()).collect();
/// assert_eq!(
///     parse_args(&args),
///     Ok(Config {
///         query: "to".to_string(),
///         path: "poem.txt".to_string(),
///         ignore_case: true,
///         line_numbers: true,
///     })
/// );
///
/// let args: Vec<String> = ["minigrep", "to"]
///     .iter().map(|s| s.to_string()).collect();
/// assert_eq!(
///     parse_args(&args),
///     Err("not enough arguments: expected query and path".to_string())
/// );
/// ```
pub fn parse_args(args: &[String]) -> Result<Config, String> {
    todo!()
}

/// Returns every line in `contents` containing `query` (case-sensitively
/// unless `ignore_case` is `true`).
///
/// Returned slices borrow from `contents`. The empty string `""` is
/// contained in every line, so `query == ""` matches every line.
///
/// # Examples
///
/// ```ignore
/// use intermediate_03_cli_io_project::search_lines;
///
/// let contents = "Rust\nTrust\nrust and dust\nCRUST";
///
/// assert_eq!(search_lines("rust", contents, false), vec!["Trust", "rust and dust"]);
/// assert_eq!(
///     search_lines("rust", contents, true),
///     vec!["Rust", "Trust", "rust and dust", "CRUST"]
/// );
/// assert_eq!(search_lines("xyz", contents, false), Vec::<&str>::new());
/// ```
pub fn search_lines<'a>(query: &str, contents: &'a str, ignore_case: bool) -> Vec<&'a str> {
    todo!()
}

/// Wraps every non-overlapping occurrence of `query` in `line` with `**`
/// (Markdown-style bold), scanning left to right and preserving `line`'s
/// original characters and case.
///
/// An empty `query` returns `line` unchanged. If `ignore_case` is `true`,
/// matches are found case-insensitively, but the *original* text from
/// `line` is what gets wrapped.
///
/// # Examples
///
/// ```ignore
/// use intermediate_03_cli_io_project::highlight_matches;
///
/// assert_eq!(
///     highlight_matches("the cat sat on the mat", "at", false),
///     "the c**at** s**at** on the m**at**"
/// );
/// assert_eq!(highlight_matches("AAAA", "AA", false), "**AA****AA**");
/// assert_eq!(highlight_matches("Hello World", "o", true), "Hell**o** W**o**rld");
/// assert_eq!(highlight_matches("test", "", false), "test");
/// assert_eq!(highlight_matches("café au lait", "é", false), "caf**é** au lait");
/// ```
pub fn highlight_matches(line: &str, query: &str, ignore_case: bool) -> String {
    todo!()
}

/// Builds a grep-style report of every line in `contents` matching `query`.
///
/// Each matching line is rendered via [`highlight_matches`] (wrapping
/// matches in `**`); if `line_numbers` is `true`, each line is prefixed
/// with its 1-indexed line number followed by `": "`. Matching lines are
/// joined with `\n`, followed by a final summary line: `"N match(es)
/// found"` -- `"1 match found"` for exactly one match, `"N matches found"`
/// otherwise (including zero).
///
/// # Examples
///
/// ```ignore
/// use intermediate_03_cli_io_project::grep_report;
///
/// let contents = "Rust\nTrust\nrust and dust\nCRUST";
///
/// assert_eq!(
///     grep_report("rust", contents, false, false),
///     "T**rust**\n**rust** and dust\n2 matches found"
/// );
///
/// assert_eq!(
///     grep_report("rust", contents, true, true),
///     "1: **Rust**\n2: T**rust**\n3: **rust** and dust\n4: C**RUST**\n4 matches found"
/// );
///
/// assert_eq!(grep_report("xyz", contents, false, false), "0 matches found");
/// ```
pub fn grep_report(query: &str, contents: &str, ignore_case: bool, line_numbers: bool) -> String {
    todo!()
}

/// Resolves the effective `ignore_case` setting from a CLI flag and an
/// `IGNORE_CASE` environment variable value (as from
/// `std::env::var("IGNORE_CASE").ok()`).
///
/// - If `cli_flag` is `true`, returns `true` -- the CLI flag always wins.
/// - Otherwise, returns `true` if `env_value` is `Some` and, after trimming
///   and lowercasing, is *not* one of `""`, `"0"`, or `"false"`.
/// - Returns `false` if `cli_flag` is `false` and `env_value` is `None`.
///
/// # Examples
///
/// ```ignore
/// use intermediate_03_cli_io_project::resolve_ignore_case;
///
/// assert_eq!(resolve_ignore_case(true, None), true);
/// assert_eq!(resolve_ignore_case(true, Some("0")), true);
/// assert_eq!(resolve_ignore_case(false, None), false);
/// assert_eq!(resolve_ignore_case(false, Some("1")), true);
/// assert_eq!(resolve_ignore_case(false, Some("0")), false);
/// assert_eq!(resolve_ignore_case(false, Some("FALSE")), false);
/// assert_eq!(resolve_ignore_case(false, Some("  ")), false);
/// assert_eq!(resolve_ignore_case(false, Some("yes")), true);
/// ```
pub fn resolve_ignore_case(cli_flag: bool, env_value: Option<&str>) -> bool {
    todo!()
}
