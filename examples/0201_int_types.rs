fn main() {
    assert_eq!(10_i8 as u16, 10_u16);
    assert_eq!(0xcafedad_u64 as u8, 0xad_u8); // last byte
    assert_eq!(2_u16.pow(4), 16);
    println!("{}", (-4_i32).abs()); // println!("{}", (-4).abs()); won't compile 
}