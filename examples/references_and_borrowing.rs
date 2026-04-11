// Boxes and ownership don't just support move-only semantics. They also support borrowing. In this example, we show how to borrow a value instead of moving it.

fn main() {
    let first = String::from("First" );
    let second = String::from("Second");
    sub(&first, &second); // Just borrowing the values instead of moving them
    println!("[main]first is {first} and second is {second}");
}

fn sub(first: &String, second: &String) {
    println!("[sub]first is {first} and second is {second}");
}