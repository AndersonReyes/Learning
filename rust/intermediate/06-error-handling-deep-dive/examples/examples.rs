//! Run with: `cargo run --example examples -p intermediate-06-error-handling-deep-dive`

use std::error::Error;
use std::fmt;

// --- A minimal custom error: Debug + Display, default source() -------------------

#[derive(Debug)]
struct EmptyInputError;

impl fmt::Display for EmptyInputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "input is empty")
    }
}

impl Error for EmptyInputError {}

// --- From-based conversion: `?` turns ParseIntError into ConfigError -------------

use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
enum ConfigError {
    Empty,
    BadNumber(ParseIntError),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::Empty => write!(f, "config value is empty"),
            ConfigError::BadNumber(e) => write!(f, "bad number: {e}"),
        }
    }
}

impl Error for ConfigError {}

impl From<ParseIntError> for ConfigError {
    fn from(e: ParseIntError) -> Self {
        ConfigError::BadNumber(e)
    }
}

fn parse_count(s: &str) -> Result<u32, ConfigError> {
    if s.is_empty() {
        return Err(ConfigError::Empty);
    }
    let n = s.parse::<u32>()?; // ParseIntError -> ConfigError via From
    Ok(n)
}

// --- Box<dyn Error>: heterogeneous errors from one function -----------------------

fn sum_fields(line: &str) -> Result<i64, Box<dyn Error>> {
    if line.trim().is_empty() {
        return Err(Box::new(EmptyInputError)); // custom error, boxed
    }
    let mut total = 0;
    for field in line.split(',') {
        total += field.trim().parse::<i64>()?; // ParseIntError, auto-boxed
    }
    Ok(total)
}

// --- source(): error chains --------------------------------------------------------

#[derive(Debug)]
struct WrappedError {
    message: String,
    source: Option<Box<dyn Error>>,
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

fn print_chain(err: &dyn Error) {
    println!("  {err}");
    let mut cause = err.source();
    while let Some(e) = cause {
        println!("  caused by: {e}");
        cause = e.source();
    }
}

// --- Downcasting dyn Error ----------------------------------------------------------

fn handle(err: &(dyn Error + 'static)) {
    if let Some(e) = err.downcast_ref::<EmptyInputError>() {
        println!("  empty input: {e}");
    } else if let Some(e) = err.downcast_ref::<ParseIntError>() {
        println!("  bad number: {e}");
    } else {
        println!("  other error: {err}");
    }
}

fn main() {
    // --- Display + Debug + default source() ---
    let e = EmptyInputError;
    println!("EmptyInputError display: {e}");
    println!("EmptyInputError debug: {e:?}");
    println!("EmptyInputError source: {:?}", e.source());

    // --- From-based `?` conversion ---
    println!("\nparse_count(\"42\") = {:?}", parse_count("42"));
    println!("parse_count(\"\") = {:?}", parse_count(""));
    match parse_count("abc") {
        Ok(n) => println!("parse_count(\"abc\") = Ok({n})"),
        Err(ConfigError::BadNumber(e)) => println!("parse_count(\"abc\") = BadNumber({e})"),
        Err(e) => println!("parse_count(\"abc\") = {e}"),
    }

    // --- Box<dyn Error>: any error type via `?` or Box::new ---
    println!("\nsum_fields(\"1, 2, 3\") = {:?}", sum_fields("1, 2, 3"));
    match sum_fields("") {
        Ok(n) => println!("sum_fields(\"\") = Ok({n})"),
        Err(e) => println!("sum_fields(\"\") = Err({e})"),
    }
    match sum_fields("1, x, 3") {
        Ok(n) => println!("sum_fields(\"1, x, 3\") = Ok({n})"),
        Err(e) => println!("sum_fields(\"1, x, 3\") = Err({e})"),
    }

    // --- source() chains ---
    println!("\nerror chain:");
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
    print_chain(&outer);

    // --- downcasting dyn Error ---
    println!("\ndowncasting:");
    let boxed_empty: Box<dyn Error> = Box::new(EmptyInputError);
    handle(&*boxed_empty);

    let boxed_parse: Box<dyn Error> = match "x".parse::<i32>() {
        Ok(_) => unreachable!(),
        Err(e) => Box::new(e),
    };
    handle(&*boxed_parse);

    let boxed_other: Box<dyn Error> = Box::new(WrappedError {
        message: "something else".to_string(),
        source: None,
    });
    handle(&*boxed_other);

    // downcast_ref also works directly on Box<dyn Error> via Deref
    if let Some(e) = boxed_parse.downcast_ref::<ParseIntError>() {
        println!("  boxed_parse.downcast_ref::<ParseIntError>() = Some({e})");
    }
}
