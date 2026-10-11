
fn main() {
    let s: String = String::from("hello, world!");
    let upper_s = to_upper_case(&s);          // &String coerces to &str automatically
    println!("{}", upper_s);

    println!("{}", to_upper_case("hi there")); // a string literal is already a &str
}

fn to_upper_case(s: &str) -> String {
    s.chars().map(|c| c.to_ascii_uppercase()).collect()
}