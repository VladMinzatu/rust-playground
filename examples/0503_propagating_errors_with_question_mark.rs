// The `?` operator is shorthand for "if this is `Err`, return it from the
// enclosing function immediately; otherwise, unwrap the `Ok` value." It only
// works inside a function whose return type is compatible (e.g. `Result`).

use std::fs::File;
use std::io::{self, Read};

fn main() {
    manual_propagation_vs_question_mark();
    chaining_question_marks();
    reading_a_missing_file_returns_err();
}

// The long way: matching on every fallible step and returning early.
fn read_username_manual() -> Result<String, io::Error> {
    let file_result = File::open("username.txt");
    let mut file = match file_result {
        Ok(f) => f,
        Err(e) => return Err(e),
    };

    let mut username = String::new();
    match file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// The same thing with `?`: each fallible call either yields its `Ok` value
// or returns the `Err` immediately.
fn read_username_with_question_mark() -> Result<String, io::Error> {
    let mut file = File::open("username.txt")?;
    let mut username = String::new();
    file.read_to_string(&mut username)?;
    Ok(username)
}

fn manual_propagation_vs_question_mark() {
    println!("manual: {:?}", read_username_manual());
    println!("with ?: {:?}", read_username_with_question_mark());
}

// `?` can even be chained directly onto a call, since both `File::open` and
// `read_to_string` return `Result`.
fn read_username_chained() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("username.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn chaining_question_marks() {
    println!("chained: {:?}", read_username_chained());
}

fn reading_a_missing_file_returns_err() {
    match read_username_with_question_mark() {
        Ok(name) => println!("username: {name}"),
        Err(e) => println!("expected: no username.txt in this directory, got: {e}"),
    }
}
