fn main() {
    let assign_first = true;
    let mut base = 2;

    let name; // declared but not initialized
    if assign_first {
        name = "Alice"; // assign a value to name
    } else {
        name = "Bob"; // assign a value to name
    }
    println!("name = {}", name);

    let age = if assign_first{
        base = base * 10;
        base + base
    } else {
        base + base
    };
    println!("age = {}", age);
}