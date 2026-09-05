// `HashMap<K, V>` owns its keys and values just like a `Vec` owns its
// elements: inserting a non-Copy value moves it in, and `get` hands back a
// borrow, not ownership.

use std::collections::HashMap;

fn main() {
    inserting_moves_owned_values_in();
    get_returns_a_borrowed_option();
    insert_overwrites_the_existing_value();
    entry_only_inserts_if_absent();
    updating_a_value_based_on_the_old_one();
}

fn inserting_moves_owned_values_in() {
    let key = String::from("Blue");
    let value = String::from("10");

    let mut scores = HashMap::new();
    scores.insert(key, value); // both `key` and `value` are moved into the map
    // println!("{key}: {value}"); // error[E0382]: borrow of moved value: `key`

    println!("{scores:?}");
}

fn get_returns_a_borrowed_option() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    // `get` returns `Option<&V>` - a borrow of the value the map still
    // owns, not the value itself.
    let score = scores.get("Blue").copied().unwrap_or(0);
    println!("Blue: {score}");
    println!("map still owns its entries: {scores:?}");
}

fn insert_overwrites_the_existing_value() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25); // same key, so this replaces (and drops) the old value
    println!("{scores:?}");
}

fn entry_only_inserts_if_absent() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50); // absent: inserted
    scores.entry(String::from("Blue")).or_insert(50); // present: left alone

    println!("{scores:?}");
}

fn updating_a_value_based_on_the_old_one() {
    let text = "hello world wonderful world";
    let mut word_count = HashMap::new();

    for word in text.split_whitespace() {
        // `or_insert` returns a `&mut V` - a mutable borrow straight into
        // the map - so the count can be updated in place without a second
        // lookup.
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{word_count:?}");
}
