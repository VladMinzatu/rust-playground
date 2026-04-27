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

    let mut user2 = User { // the whole struct must be mutable to modify its fields
        name: String::from("Bob"),
        age: 25,
    };
    user2.age += 1;
    println!("User2 is {} and is {} years old", user2.name, user2.age);
}