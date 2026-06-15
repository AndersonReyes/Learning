//! Intermediate 06 — Error Handling Deep Dive.
//!
//! `notes.md` covers `From`-based automatic error conversion via `?`,
//! `Box<dyn Error>` as a type-erased catch-all, error chains via `source()`,
//! and downcasting `dyn Error` back to concrete types. The 5 exercises below:
//! a duration parser with a custom error enum + `From<ParseIntError>`, a
//! `source()`-chain walker, a `dyn Error` downcasting dispatcher, a
//! `Box<dyn Error>`-returning record parser combining a custom enum and
//! `ParseIntError`, and a `Box<dyn Error>`-returning port-picker that
//! distinguishes its failure modes via downcasting.

use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

// --- 1. parse_duration_ms -------------------------------------------------------

/// Error returned by [`parse_duration_ms`].
#[derive(Debug, PartialEq)]
pub enum ParseDurationError {
    /// The input string was empty.
    Empty,
    /// A numeric segment failed to parse as `u64`.
    InvalidNumber(ParseIntError),
    /// A segment's unit wasn't one of `ms`, `s`, `m`, `h`, `d` (includes
    /// `""` for a trailing number with no unit).
    UnknownUnit(String),
}

impl fmt::Display for ParseDurationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseDurationError::Empty => write!(f, "duration string is empty"),
            ParseDurationError::InvalidNumber(e) => write!(f, "invalid number: {e}"),
            ParseDurationError::UnknownUnit(u) => write!(f, "unknown unit: {u:?}"),
        }
    }
}

impl Error for ParseDurationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ParseDurationError::InvalidNumber(e) => Some(e),
            _ => None,
        }
    }
}

impl From<ParseIntError> for ParseDurationError {
    fn from(e: ParseIntError) -> Self {
        ParseDurationError::InvalidNumber(e)
    }
}

/// Parses a duration string like `"1h30m45s"` or `"500ms"` into total
/// milliseconds.
///
/// The string is a sequence of `<digits><unit>` segments (no separators),
/// where `<unit>` is one of `ms`, `s`, `m`, `h`, `d`. Segments are summed. A
/// trailing number with no unit produces `UnknownUnit(String::new())`.
///
/// # Errors
///
/// - `Empty` if `s` is `""`.
/// - `InvalidNumber` (via `?`/`From<ParseIntError>`) if a segment doesn't
///   start with digits that parse as `u64`.
/// - `UnknownUnit(unit)` if a segment's unit isn't recognized.
///
/// # Examples
///
/// ```ignore
/// use intermediate_06_error_handling_deep_dive::{parse_duration_ms, ParseDurationError};
///
/// assert_eq!(parse_duration_ms("1h30m45s"), Ok(5_445_000));
/// assert_eq!(parse_duration_ms("500ms"), Ok(500));
/// assert_eq!(parse_duration_ms("1h1m1s1ms"), Ok(3_661_001));
/// assert_eq!(parse_duration_ms(""), Err(ParseDurationError::Empty));
/// assert_eq!(
///     parse_duration_ms("10x"),
///     Err(ParseDurationError::UnknownUnit("x".to_string()))
/// );
/// ```
pub fn parse_duration_ms(s: &str) -> Result<u64, ParseDurationError> {
    todo!()
}

// --- 2. error_chain_messages -----------------------------------------------------

/// An error that optionally wraps an underlying cause, exposed via
/// [`Error::source`]. Used to build error chains for [`error_chain_messages`].
#[derive(Debug)]
pub struct WrappedError {
    pub message: String,
    pub source: Option<Box<dyn Error>>,
}

impl fmt::Display for WrappedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for WrappedError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_deref()
    }
}

/// Returns the `Display` message of `err`, followed by the `Display` message
/// of each error in its [`Error::source`] chain, outermost first.
///
/// # Examples
///
/// ```ignore
/// use intermediate_06_error_handling_deep_dive::{error_chain_messages, WrappedError};
///
/// let innermost = WrappedError { message: "disk full".to_string(), source: None };
/// let middle = WrappedError {
///     message: "failed to write file".to_string(),
///     source: Some(Box::new(innermost)),
/// };
/// let outer = WrappedError {
///     message: "failed to save config".to_string(),
///     source: Some(Box::new(middle)),
/// };
///
/// assert_eq!(
///     error_chain_messages(&outer),
///     vec![
///         "failed to save config".to_string(),
///         "failed to write file".to_string(),
///         "disk full".to_string(),
///     ]
/// );
///
/// let single = WrappedError { message: "oops".to_string(), source: None };
/// assert_eq!(error_chain_messages(&single), vec!["oops".to_string()]);
/// ```
pub fn error_chain_messages(err: &dyn Error) -> Vec<String> {
    todo!()
}

// --- 3. describe_error -------------------------------------------------------------

/// Error indicating a lookup key was not found.
#[derive(Debug)]
pub struct NotFoundError {
    pub key: String,
}

impl fmt::Display for NotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "key not found: {}", self.key)
    }
}

impl Error for NotFoundError {}

/// Error indicating an action was not permitted.
#[derive(Debug)]
pub struct PermissionError {
    pub action: String,
}

impl fmt::Display for PermissionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "permission denied: {}", self.action)
    }
}

impl Error for PermissionError {}

/// Returns a human-readable description of `err`, branching on its concrete
/// type via [`std::error::Error::downcast_ref`]:
///
/// - [`NotFoundError`] -> `"not found: please check the key '{key}'"`
/// - [`PermissionError`] -> `"permission denied while trying to {action}"`
/// - anything else -> `"unknown error: {err}"` (using `err`'s `Display`)
///
/// # Examples
///
/// ```ignore
/// use intermediate_06_error_handling_deep_dive::{describe_error, NotFoundError, PermissionError};
///
/// let e1 = NotFoundError { key: "foo".to_string() };
/// assert_eq!(describe_error(&e1), "not found: please check the key 'foo'");
///
/// let e2 = PermissionError { action: "delete".to_string() };
/// assert_eq!(describe_error(&e2), "permission denied while trying to delete");
///
/// let e3 = "x".parse::<i32>().unwrap_err();
/// assert_eq!(describe_error(&e3), format!("unknown error: {e3}"));
/// ```
pub fn describe_error(err: &(dyn Error + 'static)) -> String {
    todo!()
}

// --- 4. process_record -------------------------------------------------------------

/// Error returned by [`process_record`] for malformed records (as opposed to
/// `ParseIntError`, which propagates directly via `Box<dyn Error>`).
#[derive(Debug, PartialEq)]
pub enum RecordError {
    /// The record didn't contain a `:`.
    MissingField,
    /// The record contained more than one `:`.
    TooManyFields,
    /// The parsed age was greater than 150.
    AgeOutOfRange(u32),
}

impl fmt::Display for RecordError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RecordError::MissingField => write!(f, "missing field: expected \"name:age\""),
            RecordError::TooManyFields => write!(f, "too many fields: expected \"name:age\""),
            RecordError::AgeOutOfRange(age) => write!(f, "age out of range: {age}"),
        }
    }
}

impl Error for RecordError {}

/// Parses a `"name:age"` record into `(name, age)`.
///
/// `name` and `age` are trimmed of surrounding whitespace. `age` must parse
/// as `u32` and be `<= 150`.
///
/// # Errors
///
/// - [`RecordError::MissingField`] if `line` has no `:`.
/// - [`RecordError::TooManyFields`] if `line` has more than one `:`.
/// - A boxed [`std::num::ParseIntError`] (via `?`) if the age segment isn't a
///   valid `u32`.
/// - [`RecordError::AgeOutOfRange`] if the parsed age is `> 150`.
///
/// # Examples
///
/// ```ignore
/// use intermediate_06_error_handling_deep_dive::{process_record, RecordError};
///
/// assert_eq!(process_record("Alice:30").unwrap(), ("Alice".to_string(), 30));
///
/// let err = process_record("Bob").unwrap_err();
/// assert_eq!(err.downcast_ref::<RecordError>(), Some(&RecordError::MissingField));
///
/// let err = process_record("Carol:30:extra").unwrap_err();
/// assert_eq!(err.downcast_ref::<RecordError>(), Some(&RecordError::TooManyFields));
///
/// let err = process_record("Dave:abc").unwrap_err();
/// assert!(err.downcast_ref::<std::num::ParseIntError>().is_some());
///
/// let err = process_record("Eve:200").unwrap_err();
/// assert_eq!(err.downcast_ref::<RecordError>(), Some(&RecordError::AgeOutOfRange(200)));
/// ```
pub fn process_record(line: &str) -> Result<(String, u32), Box<dyn Error>> {
    todo!()
}

// --- 5. first_valid_port -------------------------------------------------------------

/// Error returned by [`first_valid_port`] when no candidate parses as a port
/// in the valid range, and none failed to parse as a number either.
#[derive(Debug, PartialEq)]
pub struct NoValidPortError;

impl fmt::Display for NoValidPortError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "no candidate is a valid port (1024-65535)")
    }
}

impl Error for NoValidPortError {}

/// Returns the first of `candidates` that parses as a `u16` in
/// `1024..=65535` (the unprivileged port range).
///
/// # Errors
///
/// - If every candidate parses as a `u16` but none is in `1024..=65535`,
///   returns a boxed [`NoValidPortError`].
/// - If `candidates` is empty, also returns a boxed [`NoValidPortError`].
/// - If at least one candidate fails to parse as `u16` *and* no candidate is
///   a valid port, returns the boxed [`std::num::ParseIntError`] from the
///   **last** such candidate.
///
/// # Examples
///
/// ```ignore
/// use intermediate_06_error_handling_deep_dive::{first_valid_port, NoValidPortError};
///
/// assert_eq!(first_valid_port(&["abc", "80", "8080"]).unwrap(), 8080);
/// assert_eq!(first_valid_port(&["8080", "9090"]).unwrap(), 8080);
///
/// let err = first_valid_port(&["80", "100"]).unwrap_err();
/// assert!(err.downcast_ref::<NoValidPortError>().is_some());
///
/// let err = first_valid_port(&["abc", "def"]).unwrap_err();
/// assert!(err.downcast_ref::<std::num::ParseIntError>().is_some());
///
/// let err = first_valid_port(&[]).unwrap_err();
/// assert!(err.downcast_ref::<NoValidPortError>().is_some());
/// ```
pub fn first_valid_port(candidates: &[&str]) -> Result<u16, Box<dyn Error>> {
    todo!()
}
