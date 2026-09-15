// `panic!` signals an unrecoverable error: the program prints a message,
// unwinds the stack (dropping values as it goes), and exits. It's for bugs
// and broken invariants, not for conditions a caller should be expected to
// handle - that's what `Result` is for.

fn main() {
    explicit_panic_would_abort();
    indexing_out_of_bounds_panics();
    unwrap_panics_on_none();
    expect_panics_with_a_custom_message();
}

fn explicit_panic_would_abort() {
    // panic!("crash and burn"); // would abort here with the message plus a backtrace hint
    println!("panic! stops the program immediately - commented out so the rest of the file still runs");
}

fn indexing_out_of_bounds_panics() {
    let v = vec![1, 2, 3];
    println!("v[0]: {}", v[0]); // in bounds: fine

    // println!("{}", v[99]); // panics: "index out of bounds: the len is 3 but the index is 99"
}

fn unwrap_panics_on_none() {
    let present: Option<i32> = Some(5);
    println!("unwrap on Some: {}", present.unwrap()); // fine: unwraps to the inner value

    let absent: Option<i32> = None;
    // absent.unwrap(); // panics: "called `Option::unwrap()` on a `None` value"
    println!("absent is {absent:?}, left un-unwrapped");
}

fn expect_panics_with_a_custom_message() {
    let present: Option<i32> = Some(5);
    // `expect` behaves like `unwrap` but lets you choose the panic message,
    // which should explain why the value is expected to be present here.
    let value = present.expect("present was just set to Some(5) above");
    println!("expect on Some: {value}");
}
