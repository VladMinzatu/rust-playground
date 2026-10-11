fn main() {
    let s: String = String::from("hello, world!");
    let upper_s = to_upper_case(&s);          // &String coerces to &str automatically
    println!("{}", upper_s);

    println!("{}", to_upper_case("hi there")); // a string literal is already a &str

    print_lengths("hello");
    print_lengths("héllo");
    print_lengths("🦀🦀");

    indexing_and_slicing("héllo");

    char_methods();
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

fn char_methods() {
    // classification
    for c in ['a', 'Z', '7', ' ', 'é', '🦀'] {
        println!(
            "{:?}: alphabetic={} numeric={} alphanumeric={} whitespace={} ascii={}",
            c, c.is_alphabetic(), c.is_numeric(), c.is_alphanumeric(), c.is_whitespace(), c.is_ascii()
        );
    }

    // digits: to_digit takes a radix and returns an Option
    println!("'7'.to_digit(10) = {:?}", '7'.to_digit(10));
    println!("'f'.to_digit(16) = {:?}", 'f'.to_digit(16));
    println!("'x'.to_digit(10) = {:?}", 'x'.to_digit(10));
    println!("char::from_digit(9, 10) = {:?}", char::from_digit(9, 10));

    // case: to_uppercase returns an iterator because one char can map to several
    println!("'a'.to_uppercase() = {}", 'a'.to_uppercase());
    println!("'ß'.to_uppercase() = {}", 'ß'.to_uppercase()); // "SS"
    println!("'ß'.to_ascii_uppercase() = {}", 'ß'.to_ascii_uppercase()); // non-ASCII is left unchanged

    // a char is a Unicode scalar value, convertible to and from u32
    println!("'A' as u32 = {}", 'A' as u32);
    println!("char::from_u32(0x1F980) = {:?}", char::from_u32(0x1F980));
    println!("char::from_u32(0xD800) = {:?}", char::from_u32(0xD800)); // surrogates aren't valid chars
}
