// `Vec<T>` owns its elements: pushing a non-Copy value moves it in, and
// dropping the vector drops everything it holds. Because a vector can
// reallocate when it grows, the borrow checker won't let a reference to one
// element coexist with anything that could add more.

fn main() {
    pushing_moves_the_value_in();
    indexing_vs_get();
    push_cannot_alias_with_an_existing_reference();
    iterating_by_shared_reference();
    iterating_by_mutable_reference();
    iterating_by_value_consumes_the_vector();
}

fn pushing_moves_the_value_in() {
    let name = String::from("Ferris");
    let mut v = Vec::new();
    v.push(name); // `name` is moved into the vector
    // println!("{name}"); // error[E0382]: borrow of moved value: `name`
    println!("v[0]: {}", v[0]);
}

fn indexing_vs_get() {
    let v = vec![1, 2, 3];

    let third = v[2]; // panics if the index is out of bounds
    println!("third: {third}");

    match v.get(10) {
        Some(value) => println!("value: {value}"),
        None => println!("index 10 is out of bounds, get() returned None instead of panicking"),
    }
}

fn push_cannot_alias_with_an_existing_reference() {
    let mut v = vec![1, 2, 3];
    let first = &v[0]; // shared borrow of one element
    // v.push(4); // error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
    // `push` might have to reallocate the whole backing buffer, which would
    // leave `first` pointing at freed memory - so the borrow checker forbids
    // it while `first` is still in use.
    println!("first: {first}");

    v.push(4); // fine now: `first`'s borrow ended at its last use above
    println!("v: {v:?}");
}

fn iterating_by_shared_reference() {
    let v = vec![100, 32, 57];
    for n in &v {
        println!("shared: {n}");
    }
    println!("still usable: {v:?}"); // `v` wasn't consumed by the loop
}

fn iterating_by_mutable_reference() {
    let mut v = vec![100, 32, 57];
    for n in &mut v {
        *n += 50; // deref to reach through the mutable reference and mutate in place
    }
    println!("mutated: {v:?}");
}

fn iterating_by_value_consumes_the_vector() {
    let v = vec![String::from("a"), String::from("b")];
    for s in v {
        // each `s` is moved out of the vector as we go
        println!("owned: {s}");
    }
    // println!("{v:?}"); // error[E0382]: borrow of moved value: `v`
}
