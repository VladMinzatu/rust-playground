// Boxes and ownership don't just support move-only semantics. They also support borrowing. In this example, we show how to borrow a value instead of moving it.

fn main() {
    // first and second own their respective String values
    let first = String::from("First" );
    let second = String::from("Second");
    sub(&first, &second); // Just borrowing the values instead of moving them
    println!("[main]first is {first} and second is {second}");

    pointers_and_dereferencing();

    permissions();
}

fn sub(first: &String, second: &String) {
    // first and second here are merely borrowed, not owned
    println!("[sub]first is {first} and second is {second}");
    // no deallocation happens here, since we don't own the values
}

fn pointers_and_dereferencing() {
    let x: Box<i32> = Box::new(5);
    let y: i32 = *x; // dereference the box to get the value
    println!("x is {x} and y is {y}");

    let z: &Box<i32> = &x; // we can also borrow the box itself
    let w: i32 = **z; // dereference the box to get the value
    println!("z is {z} and w is {w}");
}

fn permissions() {
    let mut v = vec![1, 2, 3];
    let num: &mut i32 = &mut v[2];

    *num += 1;
    
    println!("num is {}", *num);
    println!("Vector is now {:?}", v);
}

fn return_ref_to_stack() -> String { // could not return &String here (reference to stack value)
    let s = String::from("Hello world");
    s // s is returned by value, so it is moved to the caller, i.e. ownership is mvoed outside of the function
}