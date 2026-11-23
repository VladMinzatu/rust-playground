pub struct MyTuple(pub i32, pub String);
fn main() {
    let tuple = MyTuple(5, "Hello".to_string());
    println!("Value 1: {}", tuple.0);
    println!("Value 2: {}", tuple.1);
}