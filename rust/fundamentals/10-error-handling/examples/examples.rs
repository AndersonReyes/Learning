//! Run with: `cargo run --example examples -p fundamentals-10-error-handling`

use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
enum ConfigError {
    MissingKey(String),
    InvalidValue { key: String, value: String },
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::MissingKey(key) => write!(f, "missing key: {key}"),
            ConfigError::InvalidValue { key, value } => {
                write!(f, "invalid value for {key}: {value:?}")
            }
        }
    }
}

impl Error for ConfigError {}

// Looks up `key` and parses its value as i32 -- demonstrates building a
// Result with a custom error enum and `?`/`.ok_or()`.
fn lookup_port(config: &[(&str, &str)], key: &str) -> Result<i32, ConfigError> {
    let raw = config
        .iter()
        .find(|(k, _)| *k == key)
        .map(|(_, v)| *v)
        .ok_or_else(|| ConfigError::MissingKey(key.to_string()))?;

    raw.parse::<i32>().map_err(|_| ConfigError::InvalidValue {
        key: key.to_string(),
        value: raw.to_string(),
    })
}

// `?` with Option: short-circuits to None instead of returning Err.
fn first_char_upper(s: &str) -> Option<char> {
    let c = s.chars().next()?;
    Some(c.to_ascii_uppercase())
}

// Guard clauses keep the happy path unindented.
fn withdraw(balance: i64, amount: i64) -> Result<i64, String> {
    if amount <= 0 {
        return Err("amount must be positive".to_string());
    }
    if amount > balance {
        return Err("insufficient funds".to_string());
    }
    Ok(balance - amount)
}

// Accumulating ALL errors instead of stopping at the first.
fn validate_even_and_nonnegative(n: i32) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();
    if n < 0 {
        errors.push("must be non-negative".to_string());
    }
    if n % 2 != 0 {
        errors.push("must be even".to_string());
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn main() {
    // --- Result<T, E> basics ---------------------------------------------

    let config = [("host", "localhost"), ("port", "8080"), ("debug", "yes")];

    match lookup_port(&config, "port") {
        Ok(port) => println!("port = {port}"),
        Err(e) => println!("error: {e}"),
    }

    // Shortcut methods
    println!("unwrap_or default: {}", lookup_port(&config, "missing").unwrap_or(-1));
    println!(
        "unwrap_or_else: {}",
        lookup_port(&config, "debug").unwrap_or_else(|e| {
            println!("  (recovered from: {e})");
            0
        })
    );

    // .ok() discards the error, turning Result into Option
    println!("lookup('port').ok() = {:?}", lookup_port(&config, "port").ok());
    println!("lookup('missing').ok() = {:?}", lookup_port(&config, "missing").ok());

    // --- ? with Result and Option -----------------------------------------

    match lookup_port(&config, "host") {
        Ok(port) => println!("host parsed as port?! {port}"),
        Err(ConfigError::InvalidValue { key, value }) => {
            println!("'{key}' = {value:?} is not a valid port")
        }
        Err(e) => println!("error: {e}"),
    }

    println!("first_char_upper(\"hello\") = {:?}", first_char_upper("hello"));
    println!("first_char_upper(\"\") = {:?}", first_char_upper(""));

    // --- Guard clauses ------------------------------------------------------

    println!("withdraw(100, 30) = {:?}", withdraw(100, 30));
    println!("withdraw(100, 200) = {:?}", withdraw(100, 200));
    println!("withdraw(100, -5) = {:?}", withdraw(100, -5));

    // --- Accumulating multiple errors ---------------------------------------

    println!("validate(4) = {:?}", validate_even_and_nonnegative(4));
    println!("validate(-3) = {:?}", validate_even_and_nonnegative(-3));

    // --- panic! / .unwrap() / .expect() (commented out -- would abort) ------
    //
    // let v = vec![1, 2, 3];
    // let _ = v[10];                          // panics: index out of bounds
    // let _: i32 = "not a number".parse().unwrap(); // panics with ParseIntError
    // let _: i32 = "not a number".parse().expect("config value must be a number");
}
