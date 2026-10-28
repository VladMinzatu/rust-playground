fn main() {
    init_and_move();

    move_to_variable();

    move_from_indexed_copy();

    move_from_indexed_move();
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

fn move_from_indexed_copy() {
    let mut v = vec![1, 2, 3];
    let x = v[0]; // This is valid because x is a copy of the value at index 0
    println!("{}", x); // This is valid

    v[0] = 10; // This is valid because we are modifying the vector
    println!("{:?}", v); // This is valid
}

fn move_from_indexed_move() {
    let mut v = vec![String::from("Hello"), String::from("World"), String::from("again"), String::from("and again")];
    // let s = v[0]; // This would cause a compile error because s is moved from v[0]
    
    let last = v.pop(); // This is valid because pop() returns an Option<String>
    assert_eq!(last, Some(String::from("and again")));

    let second = v.swap_remove(1); // This is valid because swap_remove() returns the removed element - removes the element at index 1 and replaces it with the last element
    assert_eq!(second, String::from("World"));

    let third = std::mem::replace(&mut v[1], String::from("new")); // This is valid because replace() returns the old value and replaces it with a new value. Swapping the value at index 1 with a new String
    assert_eq!(third, String::from("again"));
}