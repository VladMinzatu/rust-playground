// Move semantics: assigning or passing a non-Copy value transfers ownership.
// The original binding is invalidated by the compiler and can no longer be used.

fn main() {
    string_is_moved_on_assignment();
    string_is_moved_into_function();
    clone_to_keep_using_the_original();
}

fn string_is_moved_on_assignment() {
    let original = String::from("Ferris");
    let moved = original; // ownership of the heap data moves to `moved`
    // println!("{original}"); // error[E0382]: borrow of moved value: `original`
    println!("moved holds: {moved}");
}

fn string_is_moved_into_function() {
    let name = String::from("Ferris");
    greet(name); // `name` is moved into `greet`
    // println!("{name}"); // error[E0382]: value borrowed here after move
}

fn greet(name: String) {
    println!("Hello, {name}!");
} // `name` goes out of scope here and its heap data is dropped

fn clone_to_keep_using_the_original() {
    let original = String::from("Ferris");
    let copy = original.clone(); // an explicit, possibly expensive, deep copy
    println!("original: {original}, copy: {copy}"); // both are usable
}
