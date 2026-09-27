
fn main() {
    let mut vec = vec![1, 2, 3];
    let item = vec.pop();
    match item {
        Some(value) => println!("Popped value: {}", value),
        None => println!("Vector is empty, no value to pop."),
    }
}
