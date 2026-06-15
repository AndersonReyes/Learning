//! Run with: `cargo run --example examples -p intermediate-03-cli-io-project`

use std::env;
use std::error::Error;
use std::fmt;

// --- Config::build -- parsing argv into a struct ----------------------------

#[derive(Debug)]
struct Config {
    query: String,
    path: String,
    ignore_case: bool,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments: expected <query> <path>");
        }
        Ok(Config {
            query: args[1].clone(),
            path: args[2].clone(),
            // Real code: env::var("IGNORE_CASE").is_ok()
            ignore_case: false,
        })
    }
}

// --- A custom error + Box<dyn Error> -----------------------------------------

#[derive(Debug)]
struct EmptyQueryError;

impl fmt::Display for EmptyQueryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "query must not be empty")
    }
}

impl Error for EmptyQueryError {}

/// Mimics `run(config) -> Result<(), Box<dyn Error>>` from ch.12.3, but
/// takes `contents` directly instead of reading a file -- keeps this
/// example self-contained and offline.
fn run<'a>(config: &Config, contents: &'a str) -> Result<Vec<&'a str>, Box<dyn Error>> {
    if config.query.is_empty() {
        return Err(Box::new(EmptyQueryError));
    }
    println!("searching {:?} for {:?}", config.path, config.query);
    Ok(search(&config.query, contents, config.ignore_case))
}

// --- TDD'd search / search_case_insensitive (ch.12.4) ------------------------

fn search<'a>(query: &str, contents: &'a str, ignore_case: bool) -> Vec<&'a str> {
    if ignore_case {
        let query = query.to_lowercase();
        contents
            .lines()
            .filter(|line| line.to_lowercase().contains(&query))
            .collect()
    } else {
        contents.lines().filter(|line| line.contains(query)).collect()
    }
}

fn main() {
    // --- Config::build ---
    let good_args = vec![
        "minigrep".to_string(),
        "rust".to_string(),
        "poem.txt".to_string(),
    ];
    let bad_args = vec!["minigrep".to_string()];

    match Config::build(&good_args) {
        Ok(config) => println!("parsed config: {config:?}"),
        Err(e) => eprintln!("Problem parsing arguments: {e}"),
    }
    match Config::build(&bad_args) {
        Ok(config) => println!("parsed config: {config:?}"),
        Err(e) => eprintln!("Problem parsing arguments: {e}"),
    }

    // --- search / search_case_insensitive ---
    let contents = "Rust:\nsafe, fast, productive.\nPick three.\nTrust me.";
    println!("search(\"rust\", ..., false) = {:?}", search("rust", contents, false));
    println!("search(\"rust\", ..., true)  = {:?}", search("rust", contents, true));

    // --- run + Box<dyn Error> ---
    let config = Config::build(&good_args).unwrap();
    match run(&config, contents) {
        Ok(matches) => println!("run(..) matches = {matches:?}"),
        Err(e) => eprintln!("Application error: {e}"),
    }

    let empty_query_config = Config {
        query: String::new(),
        path: "poem.txt".to_string(),
        ignore_case: false,
    };
    match run(&empty_query_config, contents) {
        Ok(matches) => println!("run(..) matches = {matches:?}"),
        Err(e) => eprintln!("Application error: {e}"),
    }

    // --- environment variables ---
    // Real code: env::var("IGNORE_CASE").is_ok()
    let ignore_case_set = env::var("IGNORE_CASE").is_ok();
    println!("IGNORE_CASE env var set? {ignore_case_set}");

    // --- println! vs eprintln! ---
    println!("normal output goes to stdout");
    eprintln!("error output goes to stderr -- visible even if stdout is redirected");
}
