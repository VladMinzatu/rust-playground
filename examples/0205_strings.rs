fn main() {
    let s: &str = "This is a multiline
        string";
    println!("{}", s.len());
    let mut s_the_string = s.to_string();
    println!("{}", s_the_string);
    s_the_string += " and it goes on";
    println!("{}", s_the_string);

    assert!("ONE".to_lowercase() == "one");
    assert!("aaabbb".contains("ab"));

    assert_eq!('β'.is_alphabetic(), true);
}