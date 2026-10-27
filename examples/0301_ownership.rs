use std::vec;

fn main() {
    own_and_drop();
}

fn own_and_drop() {
    let mut v = vec![1, 2, 3]; // allocated here on the heap, but v's pointer, capacity, and length are on the stack
    for i in 4..6 {
        v.push(i);
    }
    println!("{:?}", v); 
} // dropped here 