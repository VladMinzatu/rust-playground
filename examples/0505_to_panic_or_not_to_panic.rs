// `Result` and `panic!` aren't interchangeable - they answer different
// questions. `Result` says "the caller should decide how to handle this
// failure." `panic!` says "this state should be impossible; something is
// broken." Knowing which one applies is what "error handling" is really
// about, more than the mechanics of `?` on their own.

use std::net::IpAddr;

fn main() {
    unwrap_when_failure_is_logically_impossible();
    panic_to_enforce_an_invariant_at_construction();
    result_when_the_caller_should_decide();
}

fn unwrap_when_failure_is_logically_impossible() {
    // This literal is hardcoded and known-valid, so a parse failure here
    // would mean a typo in the code, not bad input - `expect` documents
    // that reasoning, rather than silently swallowing a real bug.
    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("hardcoded IP address should always be valid");
    println!("home: {home}");
}

struct Percentage(u8);

impl Percentage {
    // A constructor is a natural place to enforce an invariant: if the
    // value is out of range, every other method on `Percentage` can then
    // assume 0..=100 without re-checking. Panicking here turns a bad call
    // site into an immediate, loud failure instead of letting quietly-wrong
    // data spread through the program.
    fn new(value: u8) -> Percentage {
        if value > 100 {
            panic!("percentage must be between 0 and 100, got {value}");
        }
        Percentage(value)
    }
}

fn panic_to_enforce_an_invariant_at_construction() {
    let p = Percentage::new(75);
    println!("percentage: {}", p.0);
    // Percentage::new(150); // panics: "percentage must be between 0 and 100, got 150"
}

// When failure is expected and the caller has real options - retry, fall
// back, report to a user - `Result` is the better fit: it forces the
// caller to at least acknowledge the failure case, unlike a panic they
// can't intercept.
fn parse_percentage(raw: &str) -> Result<Percentage, String> {
    let value: u8 = raw
        .trim()
        .parse()
        .map_err(|_| format!("'{raw}' is not a valid number"))?;
    if value > 100 {
        return Err(format!("{value} is out of the 0-100 range"));
    }
    Ok(Percentage::new(value))
}

fn result_when_the_caller_should_decide() {
    match parse_percentage("85") {
        Ok(p) => println!("parsed percentage: {}", p.0),
        Err(e) => println!("error: {e}"),
    }
    match parse_percentage("150") {
        Ok(p) => println!("parsed percentage: {}", p.0),
        Err(e) => println!("error: {e}"),
    }
}
