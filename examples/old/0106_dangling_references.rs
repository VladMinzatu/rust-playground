// The borrow checker also prevents dangling references: a reference can
// never outlive the data it points to. Lifetime annotations let a function
// signature describe how long a returned reference stays valid, tied to
// one of its inputs.

fn main() {
    let owned = return_owned_value_instead_of_a_dangling_reference();
    println!("got an owned value back: {owned}");

    longest_ties_the_return_lifetime_to_its_inputs();
}

// fn dangle() -> &String {   // error[E0106]: missing lifetime specifier
//     let s = String::from("hello");
//     &s                    // `s` is dropped at the end of this scope;
// }                          // returning a reference to it would dangle

fn return_owned_value_instead_of_a_dangling_reference() -> String {
    let s = String::from("hello");
    s // move ownership out instead of returning a reference to local data
}

// The `'a` lifetime says: the returned reference is valid for as long as
// both `x` and `y` are - i.e. no longer than the shorter-lived of the two
// borrows the caller passed in.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn longest_ties_the_return_lifetime_to_its_inputs() {
    let s1 = String::from("long string is long");
    let result;
    {
        let s2 = String::from("xyz");
        result = longest(s1.as_str(), &s2);
        println!("The longest string is '{result}'"); // must be used while s2 is still alive
    }
}
