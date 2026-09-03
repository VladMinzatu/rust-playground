// A struct groups related, named fields into a single type. Fields are
// accessed with dot notation, and the whole instance must be `mut` for any
// field to be mutable - Rust has no per-field mutability.

struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

fn main() {
    basic_instantiation();
    field_init_shorthand();
    struct_update_syntax();
    the_whole_instance_must_be_mutable();
}

fn basic_instantiation() {
    let user1 = User {
        username: String::from("ferris"),
        email: String::from("ferris@rust-lang.org"),
        active: true,
        sign_in_count: 1,
    };
    println!(
        "{} <{}>, sign-ins: {}",
        user1.username, user1.email, user1.sign_in_count
    );
}

fn build_user(username: String, email: String) -> User {
    User {
        username, // field init shorthand: same as `username: username`
        email,
        active: true,
        sign_in_count: 1,
    }
}

fn field_init_shorthand() {
    let user = build_user(String::from("ferris"), String::from("ferris@rust-lang.org"));
    println!("{} <{}>", user.username, user.email);
}

fn struct_update_syntax() {
    let user1 = build_user(String::from("ferris"), String::from("ferris@rust-lang.org"));

    // `..user1` fills any field not explicitly set from user1. `username`
    // isn't overridden, so it's moved out of user1 into user2. That's a
    // partial move: user1 as a whole is no longer usable, but Copy fields
    // like `active` remain accessible on it individually.
    let user2 = User {
        email: String::from("crab@rust-lang.org"),
        ..user1
    };
    // println!("{}", user1.username); // error[E0382]: borrow of partially moved value
    println!("user1 is still active: {}", user1.active); // Copy field survives the partial move

    println!("{} <{}>", user2.username, user2.email);
}

fn the_whole_instance_must_be_mutable() {
    let mut user = build_user(String::from("ferris"), String::from("ferris@rust-lang.org"));
    user.email = String::from("new_email@rust-lang.org"); // fine: `user` is mut
    println!("updated email: {}", user.email);
}
