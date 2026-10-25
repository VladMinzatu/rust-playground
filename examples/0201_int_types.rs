fn main() {
    assert_eq!(10_i8 as u16, 10_u16);
    assert_eq!(0xcafedad_u64 as u8, 0xad_u8); // last byte
    assert_eq!(2_u16.pow(4), 16);
    println!("{}", (-4_i32).abs()); // println!("{}", (-4).abs()); won't compile 

    // println!("{}", i32::MAX + 1); -> panics on overflow in debug mode, wraps around in release mode
    assert_eq!(500_i16.wrapping_mul(500), -12144);
}
