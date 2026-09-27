// `match` compares a value against a series of patterns and runs the code
// for the first one that matches. Unlike `if`, the compiler requires match
// arms to be exhaustive - every possible value must be handled.

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alaska,
    Alabama,
}

fn main() {
    match_returns_a_value();
    binding_to_data_inside_a_variant();
    matching_multiple_patterns();
    catch_all_with_a_binding();
    match_must_be_exhaustive();
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(_state) => 25,
    }
}

fn match_returns_a_value() {
    println!("a dime is worth {} cents", value_in_cents(&Coin::Dime));
    println!("a nickel is worth {} cents", value_in_cents(&Coin::Nickel));
}

fn binding_to_data_inside_a_variant() {
    for coin in [Coin::Quarter(UsState::Alaska), Coin::Quarter(UsState::Alabama)] {
        match coin {
            Coin::Quarter(state) => println!("quarter from {state:?}!"), // `state` is bound from the variant's payload
            _ => println!("not a quarter"),
        }
    }
}

fn matching_multiple_patterns() {
    let n = 4;
    match n {
        1 | 2 => println!("one or two"),
        3..=5 => println!("three through five"), // inclusive range pattern
        _ => println!("something else"),
    }
}

fn catch_all_with_a_binding() {
    let n = 9;
    match n {
        1 => println!("one"),
        other => println!("something else: {other}"), // catch-all that binds the value instead of discarding it
    }
}

fn match_must_be_exhaustive() {
    let coin = Coin::Penny;
    // Removing the `Coin::Quarter(_)` arm below would fail to compile:
    // error[E0004]: non-exhaustive patterns: `Coin::Quarter(_)` not covered
    let name = match coin {
        Coin::Penny => "penny",
        Coin::Nickel => "nickel",
        Coin::Dime => "dime",
        Coin::Quarter(_) => "quarter",
    };
    println!("that's a {name}");
}
