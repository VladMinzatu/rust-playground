// `?` doesn't require the error type of a fallible call to match the
// enclosing function's error type exactly - it converts via `From`. A
// custom error enum can implement `From<SourceError>` for each underlying
// error it wraps, letting `?` unify them into one type.

use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
enum ConfigError {
    Empty,
    NotANumber(ParseIntError),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::Empty => write!(f, "config value was empty"),
            ConfigError::NotANumber(e) => write!(f, "config value wasn't a number: {e}"),
        }
    }
}

impl std::error::Error for ConfigError {}

// This impl is what lets `?` turn a `ParseIntError` into a `ConfigError`
// automatically inside `parse_config`.
impl From<ParseIntError> for ConfigError {
    fn from(e: ParseIntError) -> Self {
        ConfigError::NotANumber(e)
    }
}

fn parse_config(raw: &str) -> Result<i32, ConfigError> {
    if raw.trim().is_empty() {
        return Err(ConfigError::Empty);
    }
    let value = raw.trim().parse::<i32>()?; // ParseIntError -> ConfigError via From
    Ok(value)
}

fn main() {
    custom_error_unifies_multiple_failure_kinds();
    box_dyn_error_as_a_catch_all();
}

fn custom_error_unifies_multiple_failure_kinds() {
    println!("{:?}", parse_config("42"));
    println!("{}", parse_config("").unwrap_err());
    println!("{}", parse_config("abc").unwrap_err());
}

// `Box<dyn Error>` is a trait object: a catch-all error type for code (like
// `main`, or a quick script) that doesn't need callers to distinguish
// between specific failure kinds - any error implementing
// `std::error::Error` can be `?`-propagated into it.
fn parse_and_double(raw: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let n: i32 = raw.trim().parse()?; // ParseIntError coerces into Box<dyn Error>
    Ok(n * 2)
}

fn box_dyn_error_as_a_catch_all() {
    match parse_and_double("21") {
        Ok(n) => println!("doubled: {n}"),
        Err(e) => println!("error: {e}"),
    }
    match parse_and_double("nope") {
        Ok(n) => println!("doubled: {n}"),
        Err(e) => println!("error: {e}"),
    }
}
