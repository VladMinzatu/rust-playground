// Types that implement `Copy` (the simple stack-only types: integers,
// floats, bool, char, and tuples/arrays of Copy types) don't move on
// assignment or when passed to a function - they're copied instead, so the
// original binding stays valid.

fn main() {
    integers_are_copied_not_moved();
    strings_cannot_implement_copy();
}

fn integers_are_copied_not_moved() {
    let x = 5;
    let y = x; // `x` is copied into `y`, not moved
    println!("x: {x}, y: {y}"); // both still valid

    takes_a_copy(x); // x is copied into the function too
    println!("x is still usable: {x}");
}

fn takes_a_copy(_n: i32) {}

fn strings_cannot_implement_copy() {
    // `String` owns a heap allocation, so it can't be bitwise-copied cheaply:
    // it only implements `Clone`, not `Copy`. That's why moves exist for it.
    let s = String::from("Ferris");
    let _s2 = s.clone(); // must be explicit
    println!("cloning is explicit and visible: {s}");
}
