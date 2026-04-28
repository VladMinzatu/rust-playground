struct User {
    name: String,
    age: u32,
}

struct Point(f64, f64); // tuple struct

fn main() {
    let user1 = User {
        name: String::from("Alice"),
        age: 30,
    };
    println!("User1 is {} and is {} years old", user1.name, user1.age);

    let age = 26;
    let mut user2 = User { // the whole struct must be mutable to modify its fields
        name: String::from("Bob"),
        age, // field init shorthand, equivalent to age: age
    };
    user2.age += 1;
    println!("User2 is {} and is {} years old", user2.name, user2.age);

    let point = Point(3.0, 4.0);
    println!("Point is at ({}, {})", point.0, point.1);
    let Point(x, y) = point; // destructuring the tuple struct
    println!("(x, y) is at ({}, {})", x, y);
}