// Rust has no null. Instead, an optional value is expressed with the
// built-in `Option<T>` enum: `Some(value)` or `None`. The compiler forces
// you to handle both cases before you can use the value inside.

fn main() {
    matching_some_and_none();
    option_does_not_coerce_to_t();
    unwrap_or_provides_a_default();
}

fn checked_half(n: i32) -> Option<i32> {
    if n % 2 == 0 {
        Some(n / 2)
    } else {
        None
    }
}

fn matching_some_and_none() {
    match checked_half(6) {
        Some(half) => println!("half of 6 is {half}"),
        None => println!("6 doesn't split evenly"),
    }

    match checked_half(7) {
        Some(half) => println!("half of 7 is {half}"),
        None => println!("7 doesn't split evenly"),
    }
}

fn option_does_not_coerce_to_t() {
    let some_number = Some(5);
    // let sum = some_number + 1; // error[E0277]: cannot add `{integer}` to `Option<i32>`
    // `Option<T>` and `T` are different types, so the compiler stops you
    // from using a maybe-absent value as if it were always present.
    let sum = match some_number {
        Some(n) => n + 1,
        None => 0,
    };
    println!("sum: {sum}");
}

fn unwrap_or_provides_a_default() {
    let absent: Option<i32> = None;
    // A shorthand for the "match and fall back to a default" pattern above.
    println!("{}", absent.unwrap_or(0));
}
