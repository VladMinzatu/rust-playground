fn main() {
    we_can_use_first_here();
    we_cannot_reuse_first_once_it_is_moved();
}

fn we_can_use_first_here(){
    let first = String::from("Ferris");
    let first_clone = first.clone();
    let full = add_suffix(first_clone);
    println!("{full}, originally {first}");
}

fn we_cannot_reuse_first_once_it_is_moved(){
    let first = String::from("Ferris");
    let full = add_suffix(first);
    println!("{full}");
    // println!("first is {first}"); // error: value borrowed here after move
}

fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
}