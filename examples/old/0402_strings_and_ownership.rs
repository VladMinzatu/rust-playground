// `String` is a growable, UTF-8 encoded collection of bytes - not an array
// of characters. That's why it can't be indexed by position, and why
// concatenating strings has its own ownership rules.

fn main() {
    concatenation_with_plus_moves_the_left_operand();
    format_borrows_instead_of_moving();
    no_indexing_by_position();
    iterating_chars_vs_bytes();
}

fn concatenation_with_plus_moves_the_left_operand() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");

    // `+` calls `fn add(self, s: &str) -> String`: it takes `self` by
    // value, so `s1` is moved in, while `s2` is only borrowed (and
    // coerced from `&String` to `&str`).
    let s3 = s1 + &s2;
    // println!("{s1}"); // error[E0382]: borrow of moved value: `s1`
    println!("s3: {s3}, s2 still usable: {s2}");
}

fn format_borrows_instead_of_moving() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // Unlike `+`, `format!` only borrows its arguments, so all three
    // remain usable afterwards.
    let combined = format!("{s1}-{s2}-{s3}");
    println!("{combined}, originals still usable: {s1} {s2} {s3}");
}

fn no_indexing_by_position() {
    let hello = String::from("Здравствуйте");
    // let first = hello[0]; // error[E0277]: the type `String` cannot be indexed by `{integer}`
    // Each of these Cyrillic characters is 2 bytes in UTF-8, so "the byte
    // at index 0" wouldn't be a whole character - indexing is disallowed
    // rather than silently returning something wrong.
    println!("byte length: {}", hello.len());

    let slice = &hello[0..4]; // byte-range slicing is allowed as long as it lands on char boundaries
    println!("first two chars as a slice: {slice}");
}

fn iterating_chars_vs_bytes() {
    let s = "Hi!";
    for c in s.chars() {
        println!("char: {c}");
    }
    for b in s.bytes() {
        println!("byte: {b}");
    }
}
