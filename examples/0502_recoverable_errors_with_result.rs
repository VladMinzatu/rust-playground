// `Result<T, E>` represents an operation that might fail without treating
// failure as a bug: `Ok(value)` on success, `Err(error)` on failure. Unlike
// `panic!`, it lets the caller decide how to respond.

use std::num::ParseIntError;

fn main() {
    matching_on_result();
    mapping_the_error_case();
    unwrap_and_expect_on_result();
}

fn parse_number(s: &str) -> Result<i32, ParseIntError> {
    s.trim().parse()
}

fn matching_on_result() {
    match parse_number("42") {
        Ok(n) => println!("parsed: {n}"),
        Err(e) => println!("failed to parse: {e}"),
    }

    match parse_number("not a number") {
        Ok(n) => println!("parsed: {n}"),
        Err(e) => println!("failed to parse: {e}"),
    }
}

fn mapping_the_error_case() {
    // `unwrap_or_else` runs a closure over the error to produce a fallback,
    // without panicking and without a full `match`.
    let n = parse_number("oops").unwrap_or_else(|_| {
        println!("couldn't parse, defaulting to 0");
        0
    });
    println!("n: {n}");
}

fn unwrap_and_expect_on_result() {
    let n = parse_number("7").unwrap(); // fine: known-good input
    println!("unwrap: {n}");

    // parse_number("oops").unwrap(); // panics: "called `Result::unwrap()` on an `Err` value: ParseIntError { .. }"
    // parse_number("oops").expect("input should always be numeric here"); // same panic, custom message
}
