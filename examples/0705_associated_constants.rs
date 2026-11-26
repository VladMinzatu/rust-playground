
pub struct MyStruct {
    x: i32,
    y: i32,
}

impl MyStruct {
    const ZERO: MyStruct = MyStruct { x: 0, y: 0 }; // associated constant, can be accessed as MyStruct::ZERO
    const ONE: MyStruct = MyStruct { x: 1, y: 1 }; // associated constant, can be accessed as MyStruct::ONE
    const ID: i32 = 42; // associated constant can be of different types, can be accessed as MyStruct::ID
}
fn main() {
    let zero = MyStruct::ZERO;
    let one = MyStruct::ONE;
    let id = MyStruct::ID;
    println!("Zero: ({}, {})", zero.x, zero.y);
    println!("One: ({}, {})", one.x, one.y);
    println!("ID: {}", id);
}