fn main() {
    init_and_move();

    move_to_variable();
}

fn init_and_move() {
    let v = vec![1, 2, 3];
    let w = v; // v is moved to w
    // println!("{:?}", v); // This would cause a compile error because v is no longer valid
    println!("{:?}", w); // This is valid
}

fn move_to_variable() {
    let mut s = String::from("Hello");
    println!("{}", s); // This is valid
    
    s = String::from("World"); // s is moved to a new String - Hello is dropped here!
    println!("{}", s); // This is valid
}