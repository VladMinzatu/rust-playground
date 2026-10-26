fn main() {
    let s: &str = "This is a multiline
        string";
    println!("{}", s.len());
    let mut s_the_string = s.to_string();
    println!("{}", s_the_string);
    s_the_string += " and it goes on";
    println!("{}", s_the_string);
}