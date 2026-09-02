// Borrowing with `&` lets a function read a value without taking ownership.
// Any number of shared (immutable) references may exist at the same time.

fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // borrow, don't move
    println!("The length of '{s1}' is {len}."); // s1 is still usable here

    multiple_shared_borrows_are_fine();
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // `s` goes out of scope, but since it doesn't own the data, nothing is dropped

fn multiple_shared_borrows_are_fine() {
    let s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{r1} and {r2} and {s}"); // any number of readers, no conflict
}
