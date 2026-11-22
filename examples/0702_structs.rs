struct User {
    first_name: String,
    last_name: String,
    age: u32,
}
fn main() {
    let user = User {
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        age: 30,
    };
    println!("User: {} {}", user.first_name, user.last_name);
    println!("Age: {}", user.age);
}

