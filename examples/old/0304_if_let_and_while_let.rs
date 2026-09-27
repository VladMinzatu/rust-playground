// `if let` is sugar for a `match` that only cares about one pattern,
// trading exhaustiveness checking for brevity. `while let` does the same
// thing but loops for as long as the pattern keeps matching.

fn main() {
    if_let_as_shorthand_for_match();
    if_let_with_else();
    while_let_drains_a_stack();
}

fn if_let_as_shorthand_for_match() {
    let config_max: Option<u8> = Some(3);

    // Equivalent to:
    // match config_max {
    //     Some(max) => println!("max is {max}"),
    //     _ => (),
    // }
    if let Some(max) = config_max {
        println!("max is {max}");
    }
}

fn if_let_with_else() {
    let config_max: Option<u8> = None;

    if let Some(max) = config_max {
        println!("max is {max}");
    } else {
        println!("no max configured");
    }
}

fn while_let_drains_a_stack() {
    // `Vec<T>` is a growable list, covered properly later - here it's just
    // a convenient source for `pop()`, which returns `Option<T>`.
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    // `pop` yields `Some(value)` while there's something left and `None`
    // once the stack is empty - which is exactly when the loop should end.
    while let Some(top) = stack.pop() {
        println!("popped {top}");
    }
}
