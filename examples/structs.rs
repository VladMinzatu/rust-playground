struct User {
    name: String,
    age: u32,
}
fn main() {
    let user1 = User {
        name: String::from("Alice"),
        age: 30,
    };
    println!("User1 is {} and is {} years old", user1.name, user1.age);
}