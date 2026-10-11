fn main() {
    let s: String = String::from("hello, world!");
    let upper_s = to_upper_case(&s);          // &String coerces to &str automatically
    println!("{}", upper_s);

    println!("{}", to_upper_case("hi there")); // a string literal is already a &str

    print_lengths("hello");
    print_lengths("héllo");
    print_lengths("🦀🦀");

    indexing_and_slicing("héllo");
}

fn to_upper_case(s: &str) -> String {
    s.chars().map(|c| c.to_ascii_uppercase()).collect()
}

// len() counts UTF-8 bytes, chars().count() counts Unicode scalar values
fn print_lengths(s: &str) {
    println!("{:?}: {} bytes, {} chars", s, s.len(), s.chars().count());
}

fn indexing_and_slicing(s: &str) {
    // let c = s[0]; // compile error: `str` cannot be indexed by `{integer}`

    // nth char: walks the string from the start, so it's O(n)
    println!("chars().nth(1) = {:?}", s.chars().nth(1));

    // slices take byte ranges, not char ranges
    println!("&s[0..1] = {:?}", &s[0..1]); // "h"
    println!("&s[1..3] = {:?}", &s[1..3]); // "é" is 2 bytes

    // &s[0..2] would panic: byte 2 is in the middle of 'é'
    // get() returns None instead of panicking
    println!("s.get(0..2) = {:?}", s.get(0..2));
    println!("is_char_boundary(2) = {}", s.is_char_boundary(2));
}
