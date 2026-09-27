// Ownership can be moved into a function and moved back out through the
// return value. That works, but it's awkward - passing a reference
// is usually nicer when the callee only
// needs to read or update the value, not own it.

fn main() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length_by_value(s1);
    // `s1` was moved into the function and is no longer usable here.
    println!("The length of '{s2}' is {len}.");
}

fn calculate_length_by_value(s: String) -> (String, usize) {
    let length = s.len();
    (s, length) // hand ownership back to the caller alongside the result
}
